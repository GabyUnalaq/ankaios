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

pub mod ankaios_streaming {
    use tonic::async_trait;

    #[async_trait]
    pub trait GRPCStreaming<T> {
        async fn message(&mut self) -> Result<Option<T>, tonic::Status>;
    }
}

pub mod security {
    #[derive(Debug, Default, Clone)]
    pub struct TLSConfig {
        pub path_to_ca_pem: String,
        pub path_to_crt_pem: String,
        pub path_to_key_pem: String,
    }
}

mod agent_senders_map;
pub mod client;
mod from_server_proxy;
mod grpc_agent_connection;
mod grpc_cli_connection;
pub mod grpc_middleware_error;
pub mod server;
mod to_server_proxy;
