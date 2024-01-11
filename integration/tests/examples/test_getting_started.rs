use crate::examples::IggyExampleTest;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn should_succeed() {
    let mut iggy_example_test = IggyExampleTest::new("getting-started");

    iggy_example_test.execute_test().await;

    assert_eq!(1, 1);
}
