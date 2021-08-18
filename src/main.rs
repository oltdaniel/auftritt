use std::net::{SocketAddr, IpAddr};

use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use actix_web::http::{StatusCode};
use r2d2::{ManageConnection};
use maxminddb::geoip2;
use serde::{Serialize};

type MaxmindPool = r2d2::Pool<MaxmindManager>;

#[derive(Serialize)]
struct IpLookupResponse<'a> {
    details: geoip2::Country<'a>,
    ip: IpAddr
}

async fn handle_ip(pool: web::Data<MaxmindPool>, ip: web::Path<String>) -> impl Responder {
    let ip_parsed: Result<IpAddr, _> = ip.into_inner().parse();
    if ip_parsed.is_err() {
        return HttpResponse::new(StatusCode::BAD_REQUEST);
    }
    let ip = ip_parsed.unwrap();
    let conn = pool.get().expect("couldnt get connection");
    let data: Result<geoip2::Country, maxminddb::MaxMindDBError> = conn.lookup(ip);
    match data {
        Ok(v) => HttpResponse::Ok().json(IpLookupResponse{details: v, ip: ip}),
        Err(_) => HttpResponse::new(StatusCode::BAD_REQUEST),
    }
}

async fn handle_me(pool: web::Data<MaxmindPool>, req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let remote_addr = conn_info.realip_remote_addr();
    if None == remote_addr {
        return HttpResponse::new(StatusCode::BAD_REQUEST);
    }
    let remote_addr_parsed: Result<SocketAddr, _> = remote_addr.unwrap().parse();
    if remote_addr_parsed.is_err() {
        return HttpResponse::new(StatusCode::BAD_REQUEST);
    }
    let ip_parsed = remote_addr_parsed.unwrap().ip();
    let conn = pool.get().expect("couldnt get connection");
    let data: Result<geoip2::Country, maxminddb::MaxMindDBError> = conn.lookup(ip_parsed);
    match data {
        Ok(v) => HttpResponse::Ok().json(IpLookupResponse{details: v, ip: ip_parsed}),
        Err(_) => HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}

struct MaxmindManager;

impl ManageConnection for MaxmindManager {
    type Connection = maxminddb::Reader<Vec<u8>>;
    type Error = maxminddb::MaxMindDBError;

    fn connect(&self) -> Result<maxminddb::Reader<Vec<u8>>, maxminddb::MaxMindDBError> {
        let mut args = std::env::args().skip(1);
        let reader = maxminddb::Reader::open_readfile(
            args.next()
                .ok_or("First argument must be the path to the IP database").unwrap(),
        ).unwrap();
        return Ok(reader);
    }

    fn is_valid(&self, _: &mut maxminddb::Reader<Vec<u8>>) -> Result<(), maxminddb::MaxMindDBError> {
        Ok(())
    }

    fn has_broken(&self, _: &mut maxminddb::Reader<Vec<u8>>) -> bool {
        false
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = r2d2::Pool::builder()
        .build(MaxmindManager)
        .expect("Failed to create pool.");
    HttpServer::new(move || {
        App::new().data(pool.clone())
            .route("/me", web::get().to(handle_me))
            .route("/{ip}", web::get().to(handle_ip))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}