use std::cell::RefCell;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::rc::Rc;

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt, TryFutureExt};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_util::compat::Tokio02AsyncReadCompatExt;

use capnp_test::iomod_capnp::{agent, iomod, registry};

struct AgentImpl;

impl agent::Server for AgentImpl {}

struct RegistryImpl {
    pub modules: Rc<RefCell<HashMap<&'static str, iomod::Client>>>,
}

impl RegistryImpl {
    pub fn new() -> RegistryImpl {
        let modules = Rc::new(RefCell::new(HashMap::default()));

        RegistryImpl { modules }
    }
}

impl registry::Server for RegistryImpl {
    fn register(
        &mut self,
        params: registry::RegisterParams,
        mut results: registry::RegisterResults,
    ) -> Promise<(), capnp::Error> {
        let module = params.get().unwrap().get_iomod().unwrap();

        let mut modules = self.modules.borrow_mut();
        modules.entry("default").or_insert(module.clone());

        results.get().set_agent(capnp_rpc::new_client(AgentImpl));

        Promise::from_future(async move {
            let req = module.get_declaration_request();
            let res = req.send().promise.await.unwrap();
            let name = res.get().unwrap().get_decl().unwrap().get_name().unwrap();

            println!("Registered IOmod: {}", name);

            Ok(())
        })
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} ADDRESS[:PORT]", args[0]);
        return;
    }

    let addr = args[1]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    let mut listener = TcpListener::bind(addr).await.unwrap();
    let registry: registry::Client = capnp_rpc::new_client(RegistryImpl::new());

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            println!("listening on {}...", addr.port());

            loop {
                let (stream, _) = listener.accept().await.unwrap();
                stream.set_nodelay(true).unwrap();

                let (reader, writer) =
                    tokio_util::compat::Tokio02AsyncReadCompatExt::compat(stream).split();

                let rpc_network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(rpc_network), Some(registry.clone().client));

                tokio::task::spawn_local(Box::pin(
                    rpc_system
                        .map_err(|e| println!("error: {:?}", e))
                        .map(|_| ()),
                ));
            }
        })
        .await;
}
