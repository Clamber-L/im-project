use nacos_sdk::api::{
    naming::{NamingService, NamingServiceBuilder, ServiceInstance},
    props::ClientProps,
};

pub async fn nacos_center(
    addr: &str,
    namespace: &str,
    app_name: &str,
    service_name: String,
    group_name: String,
) {
    let client_props = ClientProps::new()
        .server_addr(addr)
        .namespace(namespace)
        .app_name(app_name);

    let naming_service = NamingServiceBuilder::new(client_props).build().unwrap();

    let service_instance = ServiceInstance {
        ip: "127.0.0.1".to_string(),
        port: 9090,
        ..Default::default()
    };
    let _register_instance_ret = naming_service
        .register_instance(service_name, Some(group_name), service_instance)
        .await;
}
