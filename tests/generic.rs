mod testing_utilities;

pub use testing_utilities::*;

use rstest::rstest;

#[rstest]
#[case::todos(TestingApp::new().todos())]
#[tokio::test]
async fn is_versioned<Response>(#[case] endpoint: TestRequest<Response>) {
    assert!(endpoint.base_url.starts_with("/v1/"));
}
