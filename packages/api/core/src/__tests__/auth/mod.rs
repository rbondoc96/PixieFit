mod login;
mod register;
mod logout;

use crate::__tests__::prelude::*;

pub(self) async fn perform_standard_login(server: &TestServer, payload: &Value) -> TestResponse {
    server.post("/api/auth").json(payload).await
}

pub(self) async fn perform_admin_login(server: &TestServer, payload: &Value) -> TestResponse {
    server.post("/api/auth/admin").json(payload).await
}
