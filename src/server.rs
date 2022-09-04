use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

struct Parth<T: ToString> {
    name: T
}

struct Pee<T: ToString>{
    point: Parth<T>
}

async fn hello_world<T: ToString>(_req: Request<Body>, p: Parth<T>) -> Result<Response<Body>, Infallible> {

    let pe = Pee{
        point: p
    };

    Ok(Response::new(pe.point.name.to_string().into()))
}

pub async fn run_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(|req|{
        //
        let q = Parth{
            name: "7456"
        };
        hello_world(req, q)
    })) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
