// Copyright (c) 2023 Elektrobit Automotive GmbH
//
// This program and the accompanying materials are made available under the
// terms of the Apache License, Version 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.
//
// SPDX-License-Identifier: Apache-2.0

use api::proto::cli_connection_server::CliConnectionServer;
use common::communications_error::CommunicationMiddlewareError;
use common::communications_server::CommunicationsServer;

use tonic::transport::{Certificate, Identity, Server};

use std::net::SocketAddr;

use crate::agent_senders_map::AgentSendersMap;
use crate::grpc_cli_connection::GRPCCliConnection;
use crate::grpc_middleware_error::GrpcMiddlewareError;
use crate::security::TLSConfig;
use api::proto::agent_connection_server::AgentConnectionServer;

use crate::from_server_proxy;
use crate::grpc_agent_connection::GRPCAgentConnection;

use common::from_server_interface::FromServerReceiver;
use common::to_server_interface::ToServerSender;

use async_trait::async_trait;

#[derive(Debug)]
pub struct GRPCCommunicationsServer {
    sender: ToServerSender,
    agent_senders: AgentSendersMap,
    tls_config: Option<TLSConfig>,
}

#[async_trait]
impl CommunicationsServer for GRPCCommunicationsServer {
    async fn start(
        &mut self,
        mut receiver: FromServerReceiver,
        addr: SocketAddr,
    ) -> Result<(), CommunicationMiddlewareError> {
        // [impl->swdd~grpc-server-creates-agent-connection~1]
        let my_connection =
            GRPCAgentConnection::new(self.agent_senders.clone(), self.sender.clone());

        // [impl->swdd~grpc-server-creates-cli-connection~1]
        let my_cli_connection =
            GRPCCliConnection::new(self.agent_senders.clone(), self.sender.clone());

        let agent_senders_clone = self.agent_senders.clone();

        match &self.tls_config {
            Some(tls_config) => {
                let client_ca_cert = std::fs::read_to_string(".certs/ca.pem").unwrap();
                let client_ca_cert = Certificate::from_pem(client_ca_cert);
                let crt_pem = &tls_config.path_to_crt_pem; // ".certs/server-crt.pem"
                let key_pem = &tls_config.path_to_key_pem; // ".certs/server-key.pem"
                let cert = std::fs::read_to_string(crt_pem)
                    .map_err(|err| CommunicationMiddlewareError(err.to_string()))?;
                let key = std::fs::read_to_string(key_pem)
                    .map_err(|err| CommunicationMiddlewareError(err.to_string()))?;
                let server_identity = Identity::from_pem(cert, key);
                let tls = tonic::transport::ServerTlsConfig::new()
                    .client_ca_root(client_ca_cert)
                    .identity(server_identity);
                tokio::select! {
                    // [impl->swdd~grpc-server-spawns-tonic-service~1]
                    // [impl->swdd~grpc-delegate-workflow-to-external-library~1]
                    result = Server::builder()
                        .tls_config(tls).map_err(|err| CommunicationMiddlewareError(err.to_string()))?
                        .add_service(AgentConnectionServer::new(my_connection))
                        // [impl->swdd~grpc-server-provides-endpoint-for-cli-connection-handling~1]
                        .add_service(CliConnectionServer::new(my_cli_connection))
                        .serve(addr) => {
                            result.map_err(|err| {
                                GrpcMiddlewareError::StartError(format!("{err:?}"))
                            })?
                        }
                    // [impl->swdd~grpc-server-forwards-from-server-messages-to-grpc-client~1]
                    _ = from_server_proxy::forward_from_ankaios_to_proto(
                        &agent_senders_clone,
                        &mut receiver,
                    ) => {
                        Err(GrpcMiddlewareError::ConnectionInterrupted(
                            "Connection between Ankaios server and the communication middleware dropped.".into())
                        )?
                    }

                }
            }
            None => {
                tokio::select! {
                    // [impl->swdd~grpc-server-spawns-tonic-service~1]
                    // [impl->swdd~grpc-delegate-workflow-to-external-library~1]
                    result = Server::builder()
                        // .tls_config(tls).map_err(|err| CommunicationMiddlewareError(err.to_string()))?
                        .add_service(AgentConnectionServer::new(my_connection))
                        // [impl->swdd~grpc-server-provides-endpoint-for-cli-connection-handling~1]
                        .add_service(CliConnectionServer::new(my_cli_connection))
                        .serve(addr) => {
                            result.map_err(|err| {
                                GrpcMiddlewareError::StartError(format!("{err:?}"))
                            })?
                        }
                    // [impl->swdd~grpc-server-forwards-from-server-messages-to-grpc-client~1]
                    _ = from_server_proxy::forward_from_ankaios_to_proto(
                        &agent_senders_clone,
                        &mut receiver,
                    ) => {
                        Err(GrpcMiddlewareError::ConnectionInterrupted(
                            "Connection between Ankaios server and the communication middleware dropped.".into())
                        )?
                    }

                }
            }
        }

        // tokio::select! {
        //     // [impl->swdd~grpc-server-spawns-tonic-service~1]
        //     // [impl->swdd~grpc-delegate-workflow-to-external-library~1]
        //     result = Server::builder()
        //         // .tls_config(tls).map_err(|err| CommunicationMiddlewareError(err.to_string()))?
        //         .add_service(AgentConnectionServer::new(my_connection))
        //         // [impl->swdd~grpc-server-provides-endpoint-for-cli-connection-handling~1]
        //         .add_service(CliConnectionServer::new(my_cli_connection))
        //         .serve(addr) => {
        //             result.map_err(|err| {
        //                 GrpcMiddlewareError::StartError(format!("{err:?}"))
        //             })?
        //         }
        //     // [impl->swdd~grpc-server-forwards-from-server-messages-to-grpc-client~1]
        //     _ = from_server_proxy::forward_from_ankaios_to_proto(
        //         &agent_senders_clone,
        //         &mut receiver,
        //     ) => {
        //         Err(GrpcMiddlewareError::ConnectionInterrupted(
        //             "Connection between Ankaios server and the communication middleware dropped.".into())
        //         )?
        //     }

        // }
        Ok(())
    }
}

impl GRPCCommunicationsServer {
    pub fn new(sender: ToServerSender, tls_config: Option<TLSConfig>) -> Self {
        GRPCCommunicationsServer {
            agent_senders: AgentSendersMap::new(),
            sender,
            tls_config,
        }
    }
}
