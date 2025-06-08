use anyhow::Result;
use chrono::FixedOffset;
use lib_entity::mysql::{litemall_checkpoint, litemall_device, litemall_record};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::time::Duration;

const COOKIE:&'static str = "SESSION=88ee3cac-a52c-4075-b89c-8b672114bc4e; \
Hm_lvt_78a708634c1f5addd19932d35e5f1868=1732841960; smsuser=3stWsCBpHOHYqh62raJLcQ==; JSESSIONID=0BDE2C23281E4ED9FDB1B19FFCA75425; autologinkey=925ed04d8c9b01d8e3e4dd7255836014:1742188263795:3stWsCBpHOHYqh62raJLcQ==";

const DATABASE_URL: &str = "mysql://ads_admin:SD0535%21%40%23408@rm-m5e1oh57839c9b2dq8o.mysql.rds.aliyuncs.com:3306/litemall_disease_control";
// const DATABASE_URL: &str = "mysql://root:lsw0516@82.156.175.47:3300/litemall_disease_control";

#[tokio::main]
async fn main() -> Result<()> {
    let mut option = ConnectOptions::new(DATABASE_URL.to_string());

    option
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .sqlx_logging(true);

    let connection = Database::connect(option).await?;

    let result = request_record().await?;
    add_data(&connection, result).await;
    Ok(())
}

async fn request_record() -> reqwest::Result<RootData> {
    let client = Client::new();

    let mut header_map = HeaderMap::new();
    header_map.insert(
        HeaderName::from_static("cookie"),
        HeaderValue::from_str(COOKIE).unwrap(),
    );

    let mut query_map = Map::new();
    query_map.insert(
        "sort".to_string(),
        Value::String(String::from("cpl.create_time")),
    );
    query_map.insert("order".to_string(), Value::String(String::from("desc")));
    query_map.insert("limit".to_string(), Value::Number(Number::from(100)));
    query_map.insert("offset".to_string(), Value::Number(Number::from(0)));
    query_map.insert("areaIds".to_string(), Value::String(String::from("35058")));
    query_map.insert(
        "startTime".to_string(),
        Value::String("1739462400000".to_string()),
    );
    query_map.insert(
        "endTime".to_string(),
        Value::String("1742054400000".to_string()),
    );
    query_map.insert("deviceId".to_string(), Value::String(String::from("")));
    query_map.insert("patrolmanId".to_string(), Value::String(String::from("")));
    query_map.insert("checkpointId".to_string(), Value::String(String::from("")));

    client
        .get("https://www.5ixun.com/exun/checkpointLog/query")
        .headers(header_map)
        .query(&query_map)
        .send()
        .await?
        .json::<RootData>()
        .await
}

async fn add_data(conn: &DatabaseConnection, root_data: RootData) {
    if !root_data.rows.is_empty() {
        for rows in root_data.rows {
            let tz = FixedOffset::east_opt(8 * 3600).unwrap();
            let date_time = sea_orm::prelude::DateTimeLocal::from_naive_utc_and_offset(
                sea_orm::prelude::DateTime::from_timestamp_millis(rows.create_time).unwrap(),
                tz,
            )
            .naive_local();

            let record = litemall_record::ActiveModel {
                createtime: Set(None),
                patrolmanid: Set(Some(rows.patrolman_id.to_string())),
                checkpointid: Set(Some(rows.checkpoint_id.to_string())),
                deviceid: Set(Some(rows.device_id.to_string())),
                customerid: Set(Some(rows.customer_id.to_string())),
                uuid: Set(Some(String::from(rows.uuid))),
                uploadtime: Set(Some(date_time)),
                areaname: Set(Some(rows.area_name)),
                username: Set(Some(rows.user_name)),
                create_time: Set(Some(date_time)),
                ..Default::default()
            };

            println!("time:{:?}", record.uploadtime);

            let record_id = litemall_record::Entity::insert(record)
                .exec(conn)
                .await
                .unwrap()
                .last_insert_id;

            if let Some(device) = rows.device {
                let device_model = litemall_device::ActiveModel {
                    name: Set(Some(device.name)),
                    code: Set(Some(device.code)),
                    re_code: Set(Some(device.re_code)),
                    status: Set(Some(0)),
                    addcheckpoint: Set(Some(String::from("0"))),
                    model: Set(Some(String::from("0"))),
                    smsstatus: Set(Some(0)),
                    recordid: Set(Some(record_id)),
                    ..Default::default()
                };

                device_model.insert(conn).await.unwrap();
            }

            if let Some(check_point) = rows.checkpoint {
                let check_point_model = litemall_checkpoint::ActiveModel {
                    name: Set(Some(check_point.name)),
                    card: Set(Some(check_point.card)),
                    recard: Set(Some(check_point.re_card)),
                    r#type: Set(Some(0)),
                    mapx: Set(Some(check_point.map_x.to_string())),
                    mapy: Set(Some(check_point.map_y.to_string())),
                    recordid: Set(Some(record_id)),
                    ..Default::default()
                };
                check_point_model.insert(conn).await.unwrap();
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootData {
    limit: i32,
    offset: i32,
    total: i32,
    pub rows: Vec<Rows>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rows {
    pub id: i64,
    pub create_time: i64,
    pub patrolman_id: i32,
    pub checkpoint_id: i32,
    pub device_id: i32,
    pub customer_id: i32,
    pub uuid: String,
    pub upload_time: i64,
    pub area_name: String,
    pub user_name: String,
    pub device: Option<Device>,
    pub checkpoint: Option<CheckPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckPoint {
    pub name: String,
    pub card: String,
    pub re_card: String,
    pub map_x: f64,
    pub map_y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub code: String,
    pub re_code: String,
    pub status: i32,
    pub add_checkpoint: i32,
    pub download_fingerprint: bool,
    pub check_usage: bool,
}
