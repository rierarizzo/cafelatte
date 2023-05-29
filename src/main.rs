use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use salvo_skeleton::infrastructure::api::controllers::user_controller;

#[tokio::main]
async fn main() {
    let router: Router = Router::with_path("").push(user_controller::router());
    let acceptor: TcpAcceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
