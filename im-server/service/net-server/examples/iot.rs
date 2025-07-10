use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct IotOnlineResponse {
    errno: usize,
    error: String,
    data: IotOnlineResponseInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct IotOnlineResponseInfo {
    online: bool,
    id: String,
    observe_status: bool,
    title: String,
}

#[tokio::main]
async fn main() {
    let data: Vec<usize> = vec![
        866504062909894,
        860423063289345,
        864035060581163,
        864035060578169,
        864035060982528,
        864035060584175,
        864035060861052,
        864035060958213,
        864035060268555,
        864035060903284,
        864035060927242,
        864035060578128,
        864035060967735,
        864035060409753,
        864035060957645,
        864035060850451,
        864035060816692,
        864035060833432,
        864035060903854,
        864035060610079,
        864035060830818,
        864035060927044,
        864035060606960,
        864035060942845,
        864035060957678,
        864035060041218,
        864035060830776,
        864035060820504,
        864035060397875,
        864035060807998,
        864035060162907,
        866512063984311,
        866512063972647,
        867677062717662,
        867677062947251,
        867677062947830,
        867677062945859,
        867677062947848,
        867677062716862,
        867677062717159,
        867677062947814,
        867677062718132,
        867677062716912,
        867677062716813,
        867677062946089,
        867677062947210,
        867677062716870,
        867677062945982,
        867677062717753,
        867677062716888,
        867677062714727,
        867677062947186,
        867677062947772,
        867677062947350,
        867677062947806,
        867677062716102,
        867677062945974,
        867677062716961,
        867677062718264,
        867677062945867,
        867677062718298,
        867677062716052,
        867677062716920,
        866512064063040,
        866512064062844,
        866062066926356,
        865581068124455,
        865581067638364,
        865581068427304,
    ];

    // 创建一个 reqwest 客户端
    let client = Client::new();
    let mut header_map = HeaderMap::new();
    header_map.insert("api-key", "WnNSoFIkhONDoTFyShRur3=xjmE=".parse().unwrap());

    let start = Instant::now();
    //创建异步任务
    let tasks = data.iter().map(|item| {
        let client = client.clone(); // 克隆客户端，供每个任务使用
        let header_map = header_map.clone();
        let item = item.clone(); // 克隆数据项，将其移动到闭包中

        tokio::spawn(async move {
            // 在异步任务中执行 HTTP 请求
            let url = format!("https://api.heclouds.com/devices/getbyimei?imei={}", item);
            let response = client
                .get(&url)
                .headers(header_map)
                .send()
                .await
                .unwrap()
                .json::<IotOnlineResponse>()
                .await
                .unwrap();

            // 返回响应
            response
        })
    });

    // 等待所有任务完成，并收集结果
    let responses = futures::future::join_all(tasks).await;

    let mut online_num = 0;
    // 处理所有响应
    for response in responses {
        // 在这里处理每个响应
        match response {
            Ok(res) => {
                if res.data.online {
                    online_num += 1;
                }
                println!("{:?}", res.data);
            }
            Err(_) => {}
        }
    }

    println!("online_num:{}", online_num);
    let end = Instant::now();

    println!("time:{:?}", (end - start).as_secs());
}
