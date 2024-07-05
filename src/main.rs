#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::fs;

use pingora::server::{configuration::Opt, Server};

use pingora::services::Service;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use tcp::proxy::proxy_service;

mod tcp;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomConf {
    pub upstream: Vec<String>,
    pub listen: String,
}

impl Default for CustomConf {
    fn default() -> Self {
        CustomConf {
            upstream: vec![],
            listen: "0.0.0.0:80".to_string(),
        }
    }
}

impl CustomConf {
    pub fn load(path: &String) -> Self {
        let custom_conf_str = fs::read_to_string(&path).expect("Read CustomConfig Fail");
        let custom_conf: CustomConf =
            serde_yaml::from_str(custom_conf_str.as_str()).expect("Parse CustomConfig Fail");
        custom_conf
    }
}

pub fn main() {
    let opt = Some(Opt::from_args());
    let custom_conf_path = opt
        .as_ref()
        .expect("No CustomConfig file position specified")
        .conf
        .as_ref()
        .expect("No CustomConfig file position specified");
    let custom_conf = CustomConf::load(&custom_conf_path);
    let mut proxy_server = Server::new(opt).unwrap();
    proxy_server.bootstrap();
    println!("Server started");
    println!("custom CustomConf: {:?}", custom_conf);
    println!("official CustomConf: {:?}", proxy_server.configuration);
    let mut services: Vec<Box<dyn Service>> = vec![];
    for upstream in &custom_conf.upstream {
        let proxy_service = proxy_service(
            &custom_conf.listen.as_str(), // listen
            &upstream.as_str(),           // proxy to
        );
        services.push(Box::new(proxy_service));
    }
    proxy_server.add_services(services);
    proxy_server.run_forever();
}
