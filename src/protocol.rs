#[repr(u16)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtocolVersion {
    V1_12_2 = 340,
    V1_16_5 = 754,
    V1_21_1 = 767,
}
