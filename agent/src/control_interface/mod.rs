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

#[cfg(not(test))]
mod authorizer;
#[cfg(test)]
pub mod authorizer;
mod directory;
mod fifo;
mod filesystem;
mod from_server_channels;
mod input_output;
mod pipes_channel_context;
mod pipes_channel_context_info;
mod pipes_channel_task;
mod reopen_file;
mod to_ankaios;

#[cfg(not(test))]
pub use authorizer::Authorizer;
#[cfg(test)]
pub use authorizer::MockAuthorizer;
pub use directory::*;
pub use fifo::*;
pub use filesystem::*;
pub use from_server_channels::*;
#[cfg(test)]
pub use input_output::*;
pub use pipes_channel_context::*;
pub use pipes_channel_context_info::*;
#[cfg(test)]
pub use pipes_channel_task::*;
pub use reopen_file::*;
pub use to_ankaios::ToAnkaios;
