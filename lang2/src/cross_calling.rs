// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ink_core::env2::EnvTypes;

/// Implemented by contracts that are compiled as dependencies.
///
/// This allows to forward `&self` calls to a call forwarder
/// that encodes and dispatches the calls to the chain.
pub trait ForwardCall {
    /// The call forwarder that handles `&self` messages.
    type Forwarder;

    /// Instantiates a call forwarder to forward `&self` messages.
    fn call(self) -> Self::Forwarder;
}

/// Implemented by contracts that are compiled as dependencies.
///
/// This allows to forward `&mut self` calls to a call forwarder
/// that encodes and dispatches the calls to the chain.
pub trait ForwardCallMut {
    /// The call forwarder that handles `&mut self` messages.
    type Forwarder;

    /// Instantiates a call forwarder to forward `&mut self` messages.
    fn call_mut(self) -> Self::Forwarder;
}

/// Implemented by contracts that are compiled as dependencies.
///
/// Allows them to return their underlying account identifier.
pub trait ToAccountId<Env>
where
    Env: EnvTypes,
{
    /// Returns the underlying account identifier of the instantiated contract.
    fn to_account_id(&self) -> <Env as EnvTypes>::AccountId;
}
