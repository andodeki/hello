use crate::configuration::*;
use crate::db::{connection::Repo, Repository};
use crate::routes::*;

// use hyper::server::conn::AddrIncoming;
use hyper::{Body, Request, Response, Server};
use warp::{
    hyper::{Method, StatusCode},
    reject, Filter, Rejection, Reply,
};

use listenfd::ListenFd;
use std::convert::Infallible;
use std::net::TcpListener;
// use warp::Filter;
// const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct AppState {
    pub repository: Repository,
    pub jwt_secret: String,
}
pub struct Application {
    port: u16,
    addr: TcpListener,
    configuration: Settings, // server: Server<AddrIncoming, T>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Application, anyhow::Error> {
        let port = configuration.application.port;

        let mut listenfd = ListenFd::from_env();
        let addr: TcpListener;

        if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
            // Server::from_tcp(l)?
            addr = l;
        } else {
            let address = format!(
                "{}:{}",
                configuration.application.host, configuration.application.port
            );
            let listener = TcpListener::bind(&address)?;
            let _port = listener.local_addr().unwrap();

            // Server::bind(&port)
            addr = listener;
        };
        // run(configuration).await?;
        // Ok(Self { port, server })
        Ok(Application {
            port,
            addr,
            configuration,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), hyper::Error> {
        let database_url = self.configuration.database.database_name;
        let repository = Repository(Repo::new(&database_url));
        
        let cors = warp::cors()
            .allow_methods(vec!["GET", "POST"])
            .allow_header("content-type")
            .allow_header("authorization")
            .allow_any_origin()
            .build();
        let log = warp::log("api::request");
        // Match any request and return hello world!
        // let routes = warp::any().map(|| "Hello, WorldDEAD!\n");
        let hello = warp::path("hi")
            .and(warp::get())
            .and(warp::path::end())
            .map(|| format!("Hello, World"))
            // .map(|| hello_world)
            .recover(handle_not_found);
        let status = warp::path("status")
            .and(warp::get())
            .and(warp::path::end())
            .map(|| format!("OK"))
            .recover(handle_not_found);

        
        let app_state = AppState {
            repository,
            jwt_secret: "jjjjskj".to_owned(),
        };

        // let routes = routes(app_state).with(warp::log(self.configuration.application.name.as_str()));

        // let svc = warp::service(hello.or(status).with(cors).with(log));
        let svc = warp::service(
            routes(app_state)
                .with(warp::log("hello"))
                .with(cors)
                .with(log),
        );

        let make_svc = hyper::service::make_service_fn(|_: _| {
            let svc = svc.clone();
            async move { Ok::<_, Infallible>(svc) }
        });

        // let port = listener.local_addr().unwrap();
        println!(
            "You can access the server at {}",
            self.addr.local_addr().unwrap()
        );

        let server = Server::from_tcp(self.addr)?.serve(make_svc);

        server.await
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

pub async fn handle_not_found(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.is_not_found() {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Err(reject)
    }
}
