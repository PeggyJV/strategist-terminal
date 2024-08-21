use eyre::Result;

use crate::app::AppContext;

pub(crate) struct StewardVersion {
    pub(crate) endpoint: String,
    pub(crate) version: String,
}

pub(crate) async fn get_all_steward_versions(app_context: &AppContext) -> Vec<Result<StewardVersion>> {
    let Some(subscribers) = app_context.subscribers.clone() else {
        log::error!("no subscribers found when attempting to query steward versions");

        return vec![];
    };

    let futures: Vec<_> = subscribers.into_iter().map(|s| get_steward_version(s.push_url)).collect();

    futures::future::join_all(futures).await
}

pub(crate) async fn get_steward_version(grpc_endpoint: String) -> Result<StewardVersion> {
    let mut client = steward_proto::proto::status_service_client::StatusServiceClient::connect(grpc_endpoint.clone()).await?;
    let request = tonic::Request::new(steward_proto::proto::VersionRequest {});

    match client.version(request).await {
        Ok(response) => {
            let response = response.into_inner();
            let version = response.version;

            log::trace!(version = version.as_str(), push_url = grpc_endpoint.as_str(); "steward version retrieved");

            Ok(StewardVersion {
                endpoint: grpc_endpoint,
                version,
            })
        },
        Err(e) => {
            log::error!(code = e.code() as i32, push_url = grpc_endpoint.as_str(); "failed to get steward version");

            Err(e.into())
        }
    }
}
