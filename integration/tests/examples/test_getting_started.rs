use serial_test::serial;

#[tokio::test]
#[serial]
async fn should_succeed() {
    assert_eq!(1, 1);
}
