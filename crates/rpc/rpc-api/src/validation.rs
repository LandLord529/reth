//! API for block submission validation.

use alloy_rpc_types_beacon::relay::{
    BuilderBlockValidationRequest, BuilderBlockValidationRequestV2,
    BuilderBlockValidationRequestV3, BuilderBlockValidationRequestV4,
    BuilderBlockValidationRequestV5,
};
use jsonrpsee::proc_macros::rpc;

/// Block validation rpc interface.
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "flashbots"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "flashbots"))]
pub trait BlockSubmissionValidationApi {
    /// A Request to validate a block submission.
    #[method(name = "validateBuilderSubmissionV1")]
    async fn validate_builder_submission_v1(
        &self,
        request: BuilderBlockValidationRequest,
    ) -> jsonrpsee::core::RpcResult<()>;

    /// A Request to validate a block submission.
    #[method(name = "validateBuilderSubmissionV2")]
    async fn validate_builder_submission_v2(
        &self,
        request: BuilderBlockValidationRequestV2,
    ) -> jsonrpsee::core::RpcResult<()>;

    /// A Request to validate a block submission.
    #[method(name = "validateBuilderSubmissionV3")]
    async fn validate_builder_submission_v3(
        &self,
        request: BuilderBlockValidationRequestV3,
    ) -> jsonrpsee::core::RpcResult<()>;

    /// A Request to validate a block submission.
    #[method(name = "validateBuilderSubmissionV4")]
    async fn validate_builder_submission_v4(
        &self,
        request: BuilderBlockValidationRequestV4,
    ) -> jsonrpsee::core::RpcResult<()>;

    /// A Request to validate a block submission.
    #[method(name = "validateBuilderSubmissionV5")]
    async fn validate_builder_submission_v5(
        &self,
        request: BuilderBlockValidationRequestV5,
    ) -> jsonrpsee::core::RpcResult<()>;
}
