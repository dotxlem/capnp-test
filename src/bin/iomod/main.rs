use std::net::ToSocketAddrs;

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio_util::compat::Tokio02AsyncReadCompatExt;

use capnp_test::iomod_capnp::{iomod, registry};

struct IomodImpl;

impl iomod::Server for IomodImpl {
    fn get_declaration(
        &mut self,
        _params: iomod::GetDeclarationParams,
        mut results: iomod::GetDeclarationResults,
    ) -> Promise<(), capnp::Error> {
        let mut decl = results.get().init_decl();
        decl.set_name("CapnTest");

        Promise::ok(())
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} HOST:PORT", args[0]);
        return;
    }

    let addr = args[1]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    let stream = TcpStream::connect(addr).await.unwrap();
    stream.set_nodelay(true).unwrap();

    let (reader, writer) = tokio_util::compat::Tokio02AsyncReadCompatExt::compat(stream).split();

    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));

    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let registry: registry::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

            let mut register = registry.register_request();
            register
                .get()
                .set_iomod(capnp_rpc::new_client(IomodImpl {}));
            register.send().promise.await.unwrap();
        })
        .await;
}
