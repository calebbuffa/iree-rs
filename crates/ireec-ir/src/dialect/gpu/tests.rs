//! Laws for GPU enum discriminant conversions.
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn mma_values_round_trip(index in 0usize..MmaIntrinsic::ALL.len()) {
        let item = MmaIntrinsic::ALL[index];
        prop_assert_eq!(MmaIntrinsic::from_value(item as u32).unwrap(), item);
    }
    #[test]
    fn virtual_mma_values_round_trip(index in 0usize..VirtualMmaIntrinsic::ALL.len()) {
        let item = VirtualMmaIntrinsic::ALL[index];
        prop_assert_eq!(VirtualMmaIntrinsic::from_value(item as u32).unwrap(), item);
    }
}
