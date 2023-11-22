use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/gof", method = "get")]
    async fn index(
        &self,
        neigbors: Query<Option<String>>,
        state: Query<Option<String>>,
    ) -> PlainText<String> {
        let neigbors = neigbors.0.unwrap().parse::<i32>().unwrap();
        let state = state.0.unwrap().parse::<bool>().unwrap();

        match neigbors {
            1 | 4 => match state {
                true => PlainText(false.to_string()),
                false => PlainText(state.to_string()),
            },

            3 => match state {
                true => PlainText(state.to_string()),
                false => PlainText(true.to_string()),
            },
            _ => PlainText(state.to_string()),
        }
    }

    #[oai(path = "/hello", method = "get")]
    async fn great(
        &self,
        name: Query<Option<String>>,
        num: Query<Option<String>>,
    ) -> PlainText<String> {
        let num = num.0.unwrap().parse::<i32>().unwrap();

        match name.0 {
            Some(name) => PlainText(format!("hello, {} {}!", name, num)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Game Of Life API", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
