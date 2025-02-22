#[test]
#[cfg(all(feature = "serde_json", feature = "rand", feature = "tracing"))]
fn regression_issue_2() {
    use tracing::subscriber::with_default;
    use tracing_subscriber::layer::SubscriberExt as _;

    with_default(
        tracing_subscriber::Registry::default()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::fmt::layer()),
        || {
            let v: Vec<i32> = deser_incomplete::from_json_str("[1,2,3").unwrap();

            assert_eq!(v, vec![1, 2, 3]);
        },
    );
}
