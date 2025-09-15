// Copyright 2023 RobustMQ Team
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

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdminServerResponse<T>
where
    T: Serialize,
{
    code: u64,
    data: T,
}

impl<T: Serialize> AdminServerResponse<T> {
    pub fn new(code: u64, data: T) -> Self {
        Self { code, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, data)
    }

    pub fn err(data: T) -> Self {
        Self::new(100, data)
    }

    pub fn code(&self) -> u64 {
        self.code
    }

    pub fn data(self) -> T {
        self.data
    }

    pub fn is_ok(&self) -> bool {
        self.code == 0
    }
}

impl AdminServerResponse<String> {
    pub fn success() -> Self {
        Self::new(0, "success".to_string())
    }
}

impl<T: Serialize> IntoResponse for AdminServerResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);

        (StatusCode::OK, body).into_response()
    }
}
