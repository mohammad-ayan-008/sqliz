#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let test_cases = trybuild::TestCases::new();
    test_cases.pass("Tests/testcase_1.rs");
}
