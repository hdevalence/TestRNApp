use penumbra_proto::{
    core::app::v1::{
        query_service_client::QueryServiceClient as AppQueryServiceClient, AppParametersRequest,
    },
    custody::v1::custody_service_server::CustodyServiceServer,
    view::v1::view_service_server::ViewServiceServer,
};
use penumbra_view::{Storage, ViewServer};

const ENDPOINT: &str = "https://testnet.plinfra.net";
