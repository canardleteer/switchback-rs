mod common;

use common::run_buf_lint_format;
use switchback_protobuf::examples::fixtures_proto_dir;

#[test]
fn pristine_fixtures_pass_buf_lint_and_format() {
    run_buf_lint_format(&fixtures_proto_dir());
}
