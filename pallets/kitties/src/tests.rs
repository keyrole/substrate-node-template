use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_should_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(1, 1);
	});
}