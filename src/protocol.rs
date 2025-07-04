use crate::packet::Packet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtocolVersion {
    V1_12         = 335,
    V1_12_1       = 338,
    V1_12_2       = 340,
    
    V1_13         = 393,
    V1_13_1       = 401,
    V1_13_2       = 404,
    
    V1_14         = 477,
    V1_14_1       = 480,
    V1_14_2       = 485,
    V1_14_3       = 490,
    V1_14_4       = 498,
    
    V1_15         = 573,
    V1_15_1       = 575,
    V1_15_2       = 578,
    
    V1_16         = 735,
    V1_16_1       = 736,
    V1_16_2       = 751,
    V1_16_3       = 753,
    V1_16_4_5     = 754,
    
    V1_17         = 755,
    V1_17_1       = 756,
    
    V1_18_1       = 757,
    V1_18_2       = 758,
    
    V1_19         = 759,
    V1_19_1_2     = 760,
    V1_19_3       = 761,
    V1_19_4       = 762,
    
    V1_20_1       = 763,
    V1_20_2       = 764,
    V1_20_3_4_5_6 = 766,
    
    V1_21_1_2_3   = 768,
    V1_21_4       = 769,
    V1_21_5       = 770,
    V1_21_6       = 771,
    V1_21_7       = 772
}

impl ProtocolVersion {
    pub fn fron_string_version<S>(version: S) -> Option<Self>
    where
        S: AsRef<str>
    {
        let version = match version.as_ref() {
            "1.12" => ProtocolVersion::V1_12,
            "1.12.1" => ProtocolVersion::V1_12_1,
            "1.12.2" => ProtocolVersion::V1_12_2,
            "1.13" => ProtocolVersion::V1_13,
            "1.13.1" => ProtocolVersion::V1_13_1,
            "1.13.2" => ProtocolVersion::V1_13_2,
            "1.14" => ProtocolVersion::V1_14,
            "1.14.1" => ProtocolVersion::V1_14_1,
            "1.14.2" => ProtocolVersion::V1_14_2,
            "1.14.3" => ProtocolVersion::V1_14_3,
            "1.14.4" => ProtocolVersion::V1_14_4,
            "1.15" => ProtocolVersion::V1_15,
            "1.15.1" => ProtocolVersion::V1_15_1,
            "1.15.2" => ProtocolVersion::V1_15_2,
            "1.16" => ProtocolVersion::V1_16,
            "1.16.1" => ProtocolVersion::V1_16_1,
            "1.16.2" => ProtocolVersion::V1_16_2,
            "1.16.3" => ProtocolVersion::V1_16_3,
            "1.16.4" | "1.16.5" => ProtocolVersion::V1_16_4_5,
            "1.17" => ProtocolVersion::V1_17,
            "1.17.1" => ProtocolVersion::V1_17_1,
            "1.18" | "1.18.1" => ProtocolVersion::V1_18_1,
            "1.18.2" => ProtocolVersion::V1_18_2,
            "1.19" => ProtocolVersion::V1_19,
            "1.19.1" | "1.19.2" => ProtocolVersion::V1_19_1_2,
            "1.19.3" => ProtocolVersion::V1_19_3,
            "1.19.4" => ProtocolVersion::V1_19_4,
            "1.20" | "1.20.1" => ProtocolVersion::V1_20_1,
            "1.20.2" => ProtocolVersion::V1_20_2,
            "1.20.3" | "1.20.4" | "1.20.5" | "1.20.6" => ProtocolVersion::V1_20_3_4_5_6,
            "1.21" | "1.21.1" | "1.21.2" | "1.21.3" => ProtocolVersion::V1_21_1_2_3,
            "1.21.4" => ProtocolVersion::V1_21_4,
            "1.21.5" => ProtocolVersion::V1_21_5,
            "1.21.6" => ProtocolVersion::V1_21_6,
            "1.21.7" => ProtocolVersion::V1_21_7,
            _ => return None
        };
        
        Some(version)
    }
}
