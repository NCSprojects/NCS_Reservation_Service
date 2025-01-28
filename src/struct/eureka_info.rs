use serde::Serialize;

#[derive(Serialize)]
pub struct EurekaInfo {
    pub instance: EurekaDetails,
}

#[derive(Serialize)]
pub struct EurekaDetails {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "app")]
    pub app: String,
    #[serde(rename = "ipAddr")]
    pub ip_addr: String,
    #[serde(rename = "vipAddress")]
    pub vip_address: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "port")]
    pub port: EurekaPortDetails,
    #[serde(rename = "dataCenterInfo")]
    pub data_center_info: DataCenterInfo, 
}

#[derive(Serialize)]
pub struct EurekaPortDetails {
    #[serde(rename = "$")]
    pub port: u16,
    #[serde(rename = "@enabled")]
    pub enabled: String,
}

// `dataCenterInfo` 객체 구조
#[derive(Serialize)]
pub struct DataCenterInfo {
    #[serde(rename = "@class")]
    pub class: String,
    pub name: String,
}