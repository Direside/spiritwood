use rocket_contrib::json::Json;
use rocket::request::Request;
use rocket::response::{self, Response,Responder};
use rocket::http::{ContentType, Status};

pub fn not_found(message: &str) -> FailResponse {
    FailResponse::fail(404, message)
}

pub fn conflict(message: &str) -> FailResponse {
    FailResponse::fail(409, message)
}

pub fn bad_request(message: &str) -> FailResponse {
    FailResponse::fail(400, message)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailResponse {
    status: u16,
    message: String
}

impl FailResponse {
    fn fail(status: u16, message: &str) -> FailResponse {
        FailResponse {
            status: status,
            message: message.to_string()
        }
    }

    pub fn http_status(&self) -> Status {
        match self.status {
            200 => Status::Ok,
            404 => Status::NotFound,
            409 => Status::Conflict,
            _ => Status::InternalServerError
        }
    }
}

impl<'r> Responder<'r> for FailResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(Json(&self).respond_to(&req).unwrap())
            .header(ContentType::JSON)
            .status(self.http_status().clone())
            .ok()
    }
}
