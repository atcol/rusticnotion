use dotenv::dotenv;
use notion::NotionApi;

pub fn test_token() -> String {
    dotenv().ok();

    let token = {
        if let Some(token) = std::env::var("NOTION_API_TOKEN").ok() {
            token
        } else if let Some(token) = std::fs::read_to_string(".api_token").ok() {
            token
        } else {
            panic!("No API Token found in environment variable 'NOTION_API_TOKEN'!")
        }
    };
    token.trim().to_string()
}

pub fn test_client() -> NotionApi {
    NotionApi::new(test_token()).unwrap()
}
