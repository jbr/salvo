use salvo::prelude::*;

#[handler]
async fn set_user(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    depot.insert("user", "client");
    ctrl.call_next(req, depot, res).await;
}
#[handler]
async fn hello(depot: &mut Depot) -> String {
    format!("Hello {}", depot.get::<&str>("user").copied().unwrap_or_default())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().hoop(set_user).handle(hello);

    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    Server::new(acceptor).serve(router).await;
}
