#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1fd0],
    #[doc = "0x1fd0 - AXI interconnect - peripheral ID4 register"]
    pub periph_id_4: crate::Reg<periph_id_4::PERIPH_ID_4_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x1fe0 - AXI interconnect - peripheral ID0 register"]
    pub periph_id_0: crate::Reg<periph_id_0::PERIPH_ID_0_SPEC>,
    #[doc = "0x1fe4 - AXI interconnect - peripheral ID1 register"]
    pub periph_id_1: crate::Reg<periph_id_1::PERIPH_ID_1_SPEC>,
    #[doc = "0x1fe8 - AXI interconnect - peripheral ID2 register"]
    pub periph_id_2: crate::Reg<periph_id_2::PERIPH_ID_2_SPEC>,
    #[doc = "0x1fec - AXI interconnect - peripheral ID3 register"]
    pub periph_id_3: crate::Reg<periph_id_3::PERIPH_ID_3_SPEC>,
    #[doc = "0x1ff0 - AXI interconnect - component ID0 register"]
    pub comp_id_0: crate::Reg<comp_id_0::COMP_ID_0_SPEC>,
    #[doc = "0x1ff4 - AXI interconnect - component ID1 register"]
    pub comp_id_1: crate::Reg<comp_id_1::COMP_ID_1_SPEC>,
    #[doc = "0x1ff8 - AXI interconnect - component ID2 register"]
    pub comp_id_2: crate::Reg<comp_id_2::COMP_ID_2_SPEC>,
    #[doc = "0x1ffc - AXI interconnect - component ID3 register"]
    pub comp_id_3: crate::Reg<comp_id_3::COMP_ID_3_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x2008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ1_fn_mod_iss_bm: crate::Reg<targ1_fn_mod_iss_bm::TARG1_FN_MOD_ISS_BM_SPEC>,
    _reserved10: [u8; 0x18],
    #[doc = "0x2024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub targ1_fn_mod2: crate::Reg<targ1_fn_mod2::TARG1_FN_MOD2_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x202c - AXI interconnect - TARG x long burst functionality modification"]
    pub targ1_fn_mod_lb: crate::Reg<targ1_fn_mod_lb::TARG1_FN_MOD_LB_SPEC>,
    _reserved12: [u8; 0xd8],
    #[doc = "0x2108 - AXI interconnect - TARG x long burst functionality modification"]
    pub targ1_fn_mod: crate::Reg<targ1_fn_mod::TARG1_FN_MOD_SPEC>,
    _reserved13: [u8; 0x0efc],
    #[doc = "0x3008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ2_fn_mod_iss_bm: crate::Reg<targ2_fn_mod_iss_bm::TARG2_FN_MOD_ISS_BM_SPEC>,
    _reserved14: [u8; 0x18],
    #[doc = "0x3024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub targ2_fn_mod2: crate::Reg<targ2_fn_mod2::TARG2_FN_MOD2_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x302c - AXI interconnect - TARG x long burst functionality modification"]
    pub targ2_fn_mod_lb: crate::Reg<targ2_fn_mod_lb::TARG2_FN_MOD_LB_SPEC>,
    _reserved16: [u8; 0xd8],
    #[doc = "0x3108 - AXI interconnect - TARG x long burst functionality modification"]
    pub targ2_fn_mod: crate::Reg<targ2_fn_mod::TARG2_FN_MOD_SPEC>,
    _reserved17: [u8; 0x0efc],
    #[doc = "0x4008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ3_fn_mod_iss_bm: crate::Reg<targ3_fn_mod_iss_bm::TARG3_FN_MOD_ISS_BM_SPEC>,
    _reserved18: [u8; 0x0ffc],
    #[doc = "0x5008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ4_fn_mod_iss_bm: crate::Reg<targ4_fn_mod_iss_bm::TARG4_FN_MOD_ISS_BM_SPEC>,
    _reserved19: [u8; 0x0ffc],
    #[doc = "0x6008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ5_fn_mod_iss_bm: crate::Reg<targ5_fn_mod_iss_bm::TARG5_FN_MOD_ISS_BM_SPEC>,
    _reserved20: [u8; 0x0ffc],
    #[doc = "0x7008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ6_fn_mod_iss_bm: crate::Reg<targ6_fn_mod_iss_bm::TARG6_FN_MOD_ISS_BM_SPEC>,
    _reserved21: [u8; 0x1000],
    #[doc = "0x800c - AXI interconnect - TARG x bus matrix issuing functionality register"]
    pub targ7_fn_mod_iss_bm: crate::Reg<targ7_fn_mod_iss_bm::TARG7_FN_MOD_ISS_BM_SPEC>,
    _reserved22: [u8; 0x14],
    #[doc = "0x8024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    pub targ7_fn_mod2: crate::Reg<targ7_fn_mod2::TARG7_FN_MOD2_SPEC>,
    _reserved23: [u8; 0xe0],
    #[doc = "0x8108 - AXI interconnect - TARG x long burst functionality modification"]
    pub targ7_fn_mod: crate::Reg<targ7_fn_mod::TARG7_FN_MOD_SPEC>,
    _reserved24: [u8; 0x0003_9f18],
    #[doc = "0x42024 - AXI interconnect - INI x functionality modification 2 register"]
    pub ini1_fn_mod2: crate::Reg<ini1_fn_mod2::INI1_FN_MOD2_SPEC>,
    #[doc = "0x42028 - AXI interconnect - INI x AHB functionality modification register"]
    pub ini1_fn_mod_ahb: crate::Reg<ini1_fn_mod_ahb::INI1_FN_MOD_AHB_SPEC>,
    _reserved26: [u8; 0xd4],
    #[doc = "0x42100 - AXI interconnect - INI x read QoS register"]
    pub ini1_read_qos: crate::Reg<ini1_read_qos::INI1_READ_QOS_SPEC>,
    #[doc = "0x42104 - AXI interconnect - INI x write QoS register"]
    pub ini1_write_qos: crate::Reg<ini1_write_qos::INI1_WRITE_QOS_SPEC>,
    #[doc = "0x42108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini1_fn_mod: crate::Reg<ini1_fn_mod::INI1_FN_MOD_SPEC>,
    _reserved29: [u8; 0x0ff4],
    #[doc = "0x43100 - AXI interconnect - INI x read QoS register"]
    pub ini2_read_qos: crate::Reg<ini2_read_qos::INI2_READ_QOS_SPEC>,
    #[doc = "0x43104 - AXI interconnect - INI x write QoS register"]
    pub ini2_write_qos: crate::Reg<ini2_write_qos::INI2_WRITE_QOS_SPEC>,
    #[doc = "0x43108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini2_fn_mod: crate::Reg<ini2_fn_mod::INI2_FN_MOD_SPEC>,
    _reserved32: [u8; 0x0f18],
    #[doc = "0x44024 - AXI interconnect - INI x functionality modification 2 register"]
    pub ini3_fn_mod2: crate::Reg<ini3_fn_mod2::INI3_FN_MOD2_SPEC>,
    #[doc = "0x44028 - AXI interconnect - INI x AHB functionality modification register"]
    pub ini3_fn_mod_ahb: crate::Reg<ini3_fn_mod_ahb::INI3_FN_MOD_AHB_SPEC>,
    _reserved34: [u8; 0xd4],
    #[doc = "0x44100 - AXI interconnect - INI x read QoS register"]
    pub ini3_read_qos: crate::Reg<ini3_read_qos::INI3_READ_QOS_SPEC>,
    #[doc = "0x44104 - AXI interconnect - INI x write QoS register"]
    pub ini3_write_qos: crate::Reg<ini3_write_qos::INI3_WRITE_QOS_SPEC>,
    #[doc = "0x44108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini3_fn_mod: crate::Reg<ini3_fn_mod::INI3_FN_MOD_SPEC>,
    _reserved37: [u8; 0x0ff4],
    #[doc = "0x45100 - AXI interconnect - INI x read QoS register"]
    pub ini4_read_qos: crate::Reg<ini4_read_qos::INI4_READ_QOS_SPEC>,
    #[doc = "0x45104 - AXI interconnect - INI x write QoS register"]
    pub ini4_write_qos: crate::Reg<ini4_write_qos::INI4_WRITE_QOS_SPEC>,
    #[doc = "0x45108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini4_fn_mod: crate::Reg<ini4_fn_mod::INI4_FN_MOD_SPEC>,
    _reserved40: [u8; 0x0ff4],
    #[doc = "0x46100 - AXI interconnect - INI x read QoS register"]
    pub ini5_read_qos: crate::Reg<ini5_read_qos::INI5_READ_QOS_SPEC>,
    #[doc = "0x46104 - AXI interconnect - INI x write QoS register"]
    pub ini5_write_qos: crate::Reg<ini5_write_qos::INI5_WRITE_QOS_SPEC>,
    #[doc = "0x46108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini5_fn_mod: crate::Reg<ini5_fn_mod::INI5_FN_MOD_SPEC>,
    _reserved43: [u8; 0x0ff4],
    #[doc = "0x47100 - AXI interconnect - INI x read QoS register"]
    pub ini6_read_qos: crate::Reg<ini6_read_qos::INI6_READ_QOS_SPEC>,
    #[doc = "0x47104 - AXI interconnect - INI x write QoS register"]
    pub ini6_write_qos: crate::Reg<ini6_write_qos::INI6_WRITE_QOS_SPEC>,
    #[doc = "0x47108 - AXI interconnect - INI x issuing functionality modification register"]
    pub ini6_fn_mod: crate::Reg<ini6_fn_mod::INI6_FN_MOD_SPEC>,
}
#[doc = "PERIPH_ID_4 register accessor: an alias for `Reg<PERIPH_ID_4_SPEC>`"]
pub type PERIPH_ID_4 = crate::Reg<periph_id_4::PERIPH_ID_4_SPEC>;
#[doc = "AXI interconnect - peripheral ID4 register"]
pub mod periph_id_4;
#[doc = "PERIPH_ID_0 register accessor: an alias for `Reg<PERIPH_ID_0_SPEC>`"]
pub type PERIPH_ID_0 = crate::Reg<periph_id_0::PERIPH_ID_0_SPEC>;
#[doc = "AXI interconnect - peripheral ID0 register"]
pub mod periph_id_0;
#[doc = "PERIPH_ID_1 register accessor: an alias for `Reg<PERIPH_ID_1_SPEC>`"]
pub type PERIPH_ID_1 = crate::Reg<periph_id_1::PERIPH_ID_1_SPEC>;
#[doc = "AXI interconnect - peripheral ID1 register"]
pub mod periph_id_1;
#[doc = "PERIPH_ID_2 register accessor: an alias for `Reg<PERIPH_ID_2_SPEC>`"]
pub type PERIPH_ID_2 = crate::Reg<periph_id_2::PERIPH_ID_2_SPEC>;
#[doc = "AXI interconnect - peripheral ID2 register"]
pub mod periph_id_2;
#[doc = "PERIPH_ID_3 register accessor: an alias for `Reg<PERIPH_ID_3_SPEC>`"]
pub type PERIPH_ID_3 = crate::Reg<periph_id_3::PERIPH_ID_3_SPEC>;
#[doc = "AXI interconnect - peripheral ID3 register"]
pub mod periph_id_3;
#[doc = "COMP_ID_0 register accessor: an alias for `Reg<COMP_ID_0_SPEC>`"]
pub type COMP_ID_0 = crate::Reg<comp_id_0::COMP_ID_0_SPEC>;
#[doc = "AXI interconnect - component ID0 register"]
pub mod comp_id_0;
#[doc = "COMP_ID_1 register accessor: an alias for `Reg<COMP_ID_1_SPEC>`"]
pub type COMP_ID_1 = crate::Reg<comp_id_1::COMP_ID_1_SPEC>;
#[doc = "AXI interconnect - component ID1 register"]
pub mod comp_id_1;
#[doc = "COMP_ID_2 register accessor: an alias for `Reg<COMP_ID_2_SPEC>`"]
pub type COMP_ID_2 = crate::Reg<comp_id_2::COMP_ID_2_SPEC>;
#[doc = "AXI interconnect - component ID2 register"]
pub mod comp_id_2;
#[doc = "COMP_ID_3 register accessor: an alias for `Reg<COMP_ID_3_SPEC>`"]
pub type COMP_ID_3 = crate::Reg<comp_id_3::COMP_ID_3_SPEC>;
#[doc = "AXI interconnect - component ID3 register"]
pub mod comp_id_3;
#[doc = "TARG1_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG1_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG1_FN_MOD_ISS_BM = crate::Reg<targ1_fn_mod_iss_bm::TARG1_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ1_fn_mod_iss_bm;
#[doc = "TARG2_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG2_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG2_FN_MOD_ISS_BM = crate::Reg<targ2_fn_mod_iss_bm::TARG2_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ2_fn_mod_iss_bm;
#[doc = "TARG3_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG3_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG3_FN_MOD_ISS_BM = crate::Reg<targ3_fn_mod_iss_bm::TARG3_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ3_fn_mod_iss_bm;
#[doc = "TARG4_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG4_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG4_FN_MOD_ISS_BM = crate::Reg<targ4_fn_mod_iss_bm::TARG4_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ4_fn_mod_iss_bm;
#[doc = "TARG5_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG5_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG5_FN_MOD_ISS_BM = crate::Reg<targ5_fn_mod_iss_bm::TARG5_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ5_fn_mod_iss_bm;
#[doc = "TARG6_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG6_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG6_FN_MOD_ISS_BM = crate::Reg<targ6_fn_mod_iss_bm::TARG6_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ6_fn_mod_iss_bm;
#[doc = "TARG7_FN_MOD_ISS_BM register accessor: an alias for `Reg<TARG7_FN_MOD_ISS_BM_SPEC>`"]
pub type TARG7_FN_MOD_ISS_BM = crate::Reg<targ7_fn_mod_iss_bm::TARG7_FN_MOD_ISS_BM_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ7_fn_mod_iss_bm;
#[doc = "TARG1_FN_MOD2 register accessor: an alias for `Reg<TARG1_FN_MOD2_SPEC>`"]
pub type TARG1_FN_MOD2 = crate::Reg<targ1_fn_mod2::TARG1_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ1_fn_mod2;
#[doc = "TARG2_FN_MOD2 register accessor: an alias for `Reg<TARG2_FN_MOD2_SPEC>`"]
pub type TARG2_FN_MOD2 = crate::Reg<targ2_fn_mod2::TARG2_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ2_fn_mod2;
#[doc = "TARG7_FN_MOD2 register accessor: an alias for `Reg<TARG7_FN_MOD2_SPEC>`"]
pub type TARG7_FN_MOD2 = crate::Reg<targ7_fn_mod2::TARG7_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ7_fn_mod2;
#[doc = "TARG1_FN_MOD_LB register accessor: an alias for `Reg<TARG1_FN_MOD_LB_SPEC>`"]
pub type TARG1_FN_MOD_LB = crate::Reg<targ1_fn_mod_lb::TARG1_FN_MOD_LB_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ1_fn_mod_lb;
#[doc = "TARG2_FN_MOD_LB register accessor: an alias for `Reg<TARG2_FN_MOD_LB_SPEC>`"]
pub type TARG2_FN_MOD_LB = crate::Reg<targ2_fn_mod_lb::TARG2_FN_MOD_LB_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ2_fn_mod_lb;
#[doc = "TARG1_FN_MOD register accessor: an alias for `Reg<TARG1_FN_MOD_SPEC>`"]
pub type TARG1_FN_MOD = crate::Reg<targ1_fn_mod::TARG1_FN_MOD_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ1_fn_mod;
#[doc = "TARG2_FN_MOD register accessor: an alias for `Reg<TARG2_FN_MOD_SPEC>`"]
pub type TARG2_FN_MOD = crate::Reg<targ2_fn_mod::TARG2_FN_MOD_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ2_fn_mod;
#[doc = "TARG7_FN_MOD register accessor: an alias for `Reg<TARG7_FN_MOD_SPEC>`"]
pub type TARG7_FN_MOD = crate::Reg<targ7_fn_mod::TARG7_FN_MOD_SPEC>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ7_fn_mod;
#[doc = "INI1_FN_MOD2 register accessor: an alias for `Reg<INI1_FN_MOD2_SPEC>`"]
pub type INI1_FN_MOD2 = crate::Reg<ini1_fn_mod2::INI1_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod ini1_fn_mod2;
#[doc = "INI3_FN_MOD2 register accessor: an alias for `Reg<INI3_FN_MOD2_SPEC>`"]
pub type INI3_FN_MOD2 = crate::Reg<ini3_fn_mod2::INI3_FN_MOD2_SPEC>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod ini3_fn_mod2;
#[doc = "INI1_FN_MOD_AHB register accessor: an alias for `Reg<INI1_FN_MOD_AHB_SPEC>`"]
pub type INI1_FN_MOD_AHB = crate::Reg<ini1_fn_mod_ahb::INI1_FN_MOD_AHB_SPEC>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod ini1_fn_mod_ahb;
#[doc = "INI3_FN_MOD_AHB register accessor: an alias for `Reg<INI3_FN_MOD_AHB_SPEC>`"]
pub type INI3_FN_MOD_AHB = crate::Reg<ini3_fn_mod_ahb::INI3_FN_MOD_AHB_SPEC>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod ini3_fn_mod_ahb;
#[doc = "INI1_READ_QOS register accessor: an alias for `Reg<INI1_READ_QOS_SPEC>`"]
pub type INI1_READ_QOS = crate::Reg<ini1_read_qos::INI1_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini1_read_qos;
#[doc = "INI2_READ_QOS register accessor: an alias for `Reg<INI2_READ_QOS_SPEC>`"]
pub type INI2_READ_QOS = crate::Reg<ini2_read_qos::INI2_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini2_read_qos;
#[doc = "INI3_READ_QOS register accessor: an alias for `Reg<INI3_READ_QOS_SPEC>`"]
pub type INI3_READ_QOS = crate::Reg<ini3_read_qos::INI3_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini3_read_qos;
#[doc = "INI4_READ_QOS register accessor: an alias for `Reg<INI4_READ_QOS_SPEC>`"]
pub type INI4_READ_QOS = crate::Reg<ini4_read_qos::INI4_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini4_read_qos;
#[doc = "INI5_READ_QOS register accessor: an alias for `Reg<INI5_READ_QOS_SPEC>`"]
pub type INI5_READ_QOS = crate::Reg<ini5_read_qos::INI5_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini5_read_qos;
#[doc = "INI6_READ_QOS register accessor: an alias for `Reg<INI6_READ_QOS_SPEC>`"]
pub type INI6_READ_QOS = crate::Reg<ini6_read_qos::INI6_READ_QOS_SPEC>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini6_read_qos;
#[doc = "INI1_WRITE_QOS register accessor: an alias for `Reg<INI1_WRITE_QOS_SPEC>`"]
pub type INI1_WRITE_QOS = crate::Reg<ini1_write_qos::INI1_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini1_write_qos;
#[doc = "INI2_WRITE_QOS register accessor: an alias for `Reg<INI2_WRITE_QOS_SPEC>`"]
pub type INI2_WRITE_QOS = crate::Reg<ini2_write_qos::INI2_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini2_write_qos;
#[doc = "INI3_WRITE_QOS register accessor: an alias for `Reg<INI3_WRITE_QOS_SPEC>`"]
pub type INI3_WRITE_QOS = crate::Reg<ini3_write_qos::INI3_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini3_write_qos;
#[doc = "INI4_WRITE_QOS register accessor: an alias for `Reg<INI4_WRITE_QOS_SPEC>`"]
pub type INI4_WRITE_QOS = crate::Reg<ini4_write_qos::INI4_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini4_write_qos;
#[doc = "INI5_WRITE_QOS register accessor: an alias for `Reg<INI5_WRITE_QOS_SPEC>`"]
pub type INI5_WRITE_QOS = crate::Reg<ini5_write_qos::INI5_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini5_write_qos;
#[doc = "INI6_WRITE_QOS register accessor: an alias for `Reg<INI6_WRITE_QOS_SPEC>`"]
pub type INI6_WRITE_QOS = crate::Reg<ini6_write_qos::INI6_WRITE_QOS_SPEC>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini6_write_qos;
#[doc = "INI1_FN_MOD register accessor: an alias for `Reg<INI1_FN_MOD_SPEC>`"]
pub type INI1_FN_MOD = crate::Reg<ini1_fn_mod::INI1_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini1_fn_mod;
#[doc = "INI2_FN_MOD register accessor: an alias for `Reg<INI2_FN_MOD_SPEC>`"]
pub type INI2_FN_MOD = crate::Reg<ini2_fn_mod::INI2_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini2_fn_mod;
#[doc = "INI3_FN_MOD register accessor: an alias for `Reg<INI3_FN_MOD_SPEC>`"]
pub type INI3_FN_MOD = crate::Reg<ini3_fn_mod::INI3_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini3_fn_mod;
#[doc = "INI4_FN_MOD register accessor: an alias for `Reg<INI4_FN_MOD_SPEC>`"]
pub type INI4_FN_MOD = crate::Reg<ini4_fn_mod::INI4_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini4_fn_mod;
#[doc = "INI5_FN_MOD register accessor: an alias for `Reg<INI5_FN_MOD_SPEC>`"]
pub type INI5_FN_MOD = crate::Reg<ini5_fn_mod::INI5_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini5_fn_mod;
#[doc = "INI6_FN_MOD register accessor: an alias for `Reg<INI6_FN_MOD_SPEC>`"]
pub type INI6_FN_MOD = crate::Reg<ini6_fn_mod::INI6_FN_MOD_SPEC>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini6_fn_mod;
