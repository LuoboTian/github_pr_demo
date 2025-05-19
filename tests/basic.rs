use github_pr_demo::add;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(-1, -2), -3);
}
