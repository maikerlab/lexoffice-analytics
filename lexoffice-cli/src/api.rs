use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use log::info;
use serde::Deserialize;
use warp::Filter;
use warp::http::StatusCode;
use crate::sync_vouchers;
use crate::utils::parse_date_string;

#[derive(Deserialize, Debug)]
struct QueryParams {
    from: Option<String>,
    to: Option<String>,
    vouchers: Option<String>
}

pub async fn run(port: u16) {
    let health_route = warp::path("health")
        .and(warp::get())
        .and_then(health_check);

    let sync_route = warp::path("sync")
        .and(warp::post())
        .and(warp::query::<QueryParams>())
        .and_then(sync);

    warp::serve(health_route.or(sync_route))
        .run(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port))
        .await;
}

async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("I am healthy!", StatusCode::OK))
}

async fn sync(params: QueryParams) -> Result<impl warp::Reply, warp::Rejection> {
    let from = params.from.map(|from_str| parse_date_string(from_str)).unwrap();
    let to = params.to.map(|to_str| parse_date_string(to_str)).unwrap();
    info!("Syncing from {:?} to {:?}", from, to);
    let vouchers = params.vouchers.unwrap_or("invoices".to_string());
    let vouchers = vouchers.split(',')
        .map(|s| s.to_string())
        .collect();
    sync_vouchers(vouchers, from, to).await;
    Ok(warp::reply::reply())
}