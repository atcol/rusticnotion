mod common;
use common::test_client;
use notion::models::search::{FilterProperty, FilterValue, NotionSearch};

#[tokio::test]
async fn list_databases() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    dbg!(api.list_databases().await?);

    Ok(())
}

#[tokio::test]
async fn search_databases() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let response = api
        .search(NotionSearch::Filter {
            property: FilterProperty::Object,
            value: FilterValue::Database,
        })
        .await?;

    assert!(response.results.len() > 0);

    Ok(())
}

#[tokio::test]
async fn search_pages() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_client();

    let response = api
        .search(NotionSearch::Filter {
            property: FilterProperty::Object,
            value: FilterValue::Page,
        })
        .await?;

    assert!(response.results.len() > 0);

    Ok(())
}
