use super::handler::{Handler, PageNotFoundHandle, StaticPageHandler,WebServiceHandle};
use http::{httprequest, httpresponse::HttpResponse, httprequest::HttpRequest};
use std::io::prelude::*;
pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandle::handle(&req);
                            let _ = resp.send_response(stream);
                        }  
                      _ => {
                          let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandle::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}