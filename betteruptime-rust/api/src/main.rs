use poem::{get, handler, listener::TcpListener, post, Route, Server};
use poem::web::{Path, Json};
use store::Store;
use req_input::CreateWebsiteInput;
use req_output::CreateWebsiteOutput;

pub mod req_input;
pub mod req_output;

#[handler]
fn get_website(Path(website_id):Path<String>)->String{
    s.create_website();
    format!("website: {}",website_id)
}
#[handler]
fn create_website(Json(data):Json<CreateWebsiteInput>)->Json<CreateWebsiteOutput>{
    let url=data.url;
    let response=CreateWebsiteOutput {
        id:String::from("ID")
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
                    .at("/status/:website_id", get(get_website))
                    .at("/website",post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(app)
    .await
}