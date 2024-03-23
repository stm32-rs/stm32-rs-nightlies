#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1fd0],
    periph_id_4: PERIPH_ID_4,
    _reserved1: [u8; 0x0c],
    periph_id_0: PERIPH_ID_0,
    periph_id_1: PERIPH_ID_1,
    periph_id_2: PERIPH_ID_2,
    periph_id_3: PERIPH_ID_3,
    comp_id_0: COMP_ID_0,
    comp_id_1: COMP_ID_1,
    comp_id_2: COMP_ID_2,
    comp_id_3: COMP_ID_3,
    _reserved9: [u8; 0x08],
    targ1_fn_mod_iss_bm: TARG1_FN_MOD_ISS_BM,
    _reserved10: [u8; 0x18],
    targ1_fn_mod2: TARG1_FN_MOD2,
    _reserved11: [u8; 0x04],
    targ1_fn_mod_lb: TARG1_FN_MOD_LB,
    _reserved12: [u8; 0xd8],
    targ1_fn_mod: TARG1_FN_MOD,
    _reserved13: [u8; 0x0efc],
    targ2_fn_mod_iss_bm: TARG2_FN_MOD_ISS_BM,
    _reserved14: [u8; 0x18],
    targ2_fn_mod2: TARG2_FN_MOD2,
    _reserved15: [u8; 0x04],
    targ2_fn_mod_lb: TARG2_FN_MOD_LB,
    _reserved16: [u8; 0xd8],
    targ2_fn_mod: TARG2_FN_MOD,
    _reserved17: [u8; 0x0efc],
    targ3_fn_mod_iss_bm: TARG3_FN_MOD_ISS_BM,
    _reserved18: [u8; 0x0ffc],
    targ4_fn_mod_iss_bm: TARG4_FN_MOD_ISS_BM,
    _reserved19: [u8; 0x0ffc],
    targ5_fn_mod_iss_bm: TARG5_FN_MOD_ISS_BM,
    _reserved20: [u8; 0x0ffc],
    targ6_fn_mod_iss_bm: TARG6_FN_MOD_ISS_BM,
    _reserved21: [u8; 0x1000],
    targ7_fn_mod_iss_bm: TARG7_FN_MOD_ISS_BM,
    _reserved22: [u8; 0x14],
    targ7_fn_mod2: TARG7_FN_MOD2,
    _reserved23: [u8; 0xe0],
    targ7_fn_mod: TARG7_FN_MOD,
    _reserved24: [u8; 0x0003_9f18],
    ini1_fn_mod2: INI1_FN_MOD2,
    ini1_fn_mod_ahb: INI1_FN_MOD_AHB,
    _reserved26: [u8; 0xd4],
    ini1_read_qos: INI1_READ_QOS,
    ini1_write_qos: INI1_WRITE_QOS,
    ini1_fn_mod: INI1_FN_MOD,
    _reserved29: [u8; 0x0ff4],
    ini2_read_qos: INI2_READ_QOS,
    ini2_write_qos: INI2_WRITE_QOS,
    ini2_fn_mod: INI2_FN_MOD,
    _reserved32: [u8; 0x0f18],
    ini3_fn_mod2: INI3_FN_MOD2,
    ini3_fn_mod_ahb: INI3_FN_MOD_AHB,
    _reserved34: [u8; 0xd4],
    ini3_read_qos: INI3_READ_QOS,
    ini3_write_qos: INI3_WRITE_QOS,
    ini3_fn_mod: INI3_FN_MOD,
    _reserved37: [u8; 0x0ff4],
    ini4_read_qos: INI4_READ_QOS,
    ini4_write_qos: INI4_WRITE_QOS,
    ini4_fn_mod: INI4_FN_MOD,
    _reserved40: [u8; 0x0ff4],
    ini5_read_qos: INI5_READ_QOS,
    ini5_write_qos: INI5_WRITE_QOS,
    ini5_fn_mod: INI5_FN_MOD,
    _reserved43: [u8; 0x0ff4],
    ini6_read_qos: INI6_READ_QOS,
    ini6_write_qos: INI6_WRITE_QOS,
    ini6_fn_mod: INI6_FN_MOD,
}
impl RegisterBlock {
    #[doc = "0x1fd0 - AXI interconnect - peripheral ID4 register"]
    #[inline(always)]
    pub const fn periph_id_4(&self) -> &PERIPH_ID_4 {
        &self.periph_id_4
    }
    #[doc = "0x1fe0 - AXI interconnect - peripheral ID0 register"]
    #[inline(always)]
    pub const fn periph_id_0(&self) -> &PERIPH_ID_0 {
        &self.periph_id_0
    }
    #[doc = "0x1fe4 - AXI interconnect - peripheral ID1 register"]
    #[inline(always)]
    pub const fn periph_id_1(&self) -> &PERIPH_ID_1 {
        &self.periph_id_1
    }
    #[doc = "0x1fe8 - AXI interconnect - peripheral ID2 register"]
    #[inline(always)]
    pub const fn periph_id_2(&self) -> &PERIPH_ID_2 {
        &self.periph_id_2
    }
    #[doc = "0x1fec - AXI interconnect - peripheral ID3 register"]
    #[inline(always)]
    pub const fn periph_id_3(&self) -> &PERIPH_ID_3 {
        &self.periph_id_3
    }
    #[doc = "0x1ff0 - AXI interconnect - component ID0 register"]
    #[inline(always)]
    pub const fn comp_id_0(&self) -> &COMP_ID_0 {
        &self.comp_id_0
    }
    #[doc = "0x1ff4 - AXI interconnect - component ID1 register"]
    #[inline(always)]
    pub const fn comp_id_1(&self) -> &COMP_ID_1 {
        &self.comp_id_1
    }
    #[doc = "0x1ff8 - AXI interconnect - component ID2 register"]
    #[inline(always)]
    pub const fn comp_id_2(&self) -> &COMP_ID_2 {
        &self.comp_id_2
    }
    #[doc = "0x1ffc - AXI interconnect - component ID3 register"]
    #[inline(always)]
    pub const fn comp_id_3(&self) -> &COMP_ID_3 {
        &self.comp_id_3
    }
    #[doc = "0x2008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ1_fn_mod_iss_bm(&self) -> &TARG1_FN_MOD_ISS_BM {
        &self.targ1_fn_mod_iss_bm
    }
    #[doc = "0x2024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    #[inline(always)]
    pub const fn targ1_fn_mod2(&self) -> &TARG1_FN_MOD2 {
        &self.targ1_fn_mod2
    }
    #[doc = "0x202c - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn targ1_fn_mod_lb(&self) -> &TARG1_FN_MOD_LB {
        &self.targ1_fn_mod_lb
    }
    #[doc = "0x2108 - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn targ1_fn_mod(&self) -> &TARG1_FN_MOD {
        &self.targ1_fn_mod
    }
    #[doc = "0x3008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ2_fn_mod_iss_bm(&self) -> &TARG2_FN_MOD_ISS_BM {
        &self.targ2_fn_mod_iss_bm
    }
    #[doc = "0x3024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    #[inline(always)]
    pub const fn targ2_fn_mod2(&self) -> &TARG2_FN_MOD2 {
        &self.targ2_fn_mod2
    }
    #[doc = "0x302c - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn targ2_fn_mod_lb(&self) -> &TARG2_FN_MOD_LB {
        &self.targ2_fn_mod_lb
    }
    #[doc = "0x3108 - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn targ2_fn_mod(&self) -> &TARG2_FN_MOD {
        &self.targ2_fn_mod
    }
    #[doc = "0x4008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ3_fn_mod_iss_bm(&self) -> &TARG3_FN_MOD_ISS_BM {
        &self.targ3_fn_mod_iss_bm
    }
    #[doc = "0x5008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ4_fn_mod_iss_bm(&self) -> &TARG4_FN_MOD_ISS_BM {
        &self.targ4_fn_mod_iss_bm
    }
    #[doc = "0x6008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ5_fn_mod_iss_bm(&self) -> &TARG5_FN_MOD_ISS_BM {
        &self.targ5_fn_mod_iss_bm
    }
    #[doc = "0x7008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ6_fn_mod_iss_bm(&self) -> &TARG6_FN_MOD_ISS_BM {
        &self.targ6_fn_mod_iss_bm
    }
    #[doc = "0x800c - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn targ7_fn_mod_iss_bm(&self) -> &TARG7_FN_MOD_ISS_BM {
        &self.targ7_fn_mod_iss_bm
    }
    #[doc = "0x8024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    #[inline(always)]
    pub const fn targ7_fn_mod2(&self) -> &TARG7_FN_MOD2 {
        &self.targ7_fn_mod2
    }
    #[doc = "0x8108 - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn targ7_fn_mod(&self) -> &TARG7_FN_MOD {
        &self.targ7_fn_mod
    }
    #[doc = "0x42024 - AXI interconnect - INI x functionality modification 2 register"]
    #[inline(always)]
    pub const fn ini1_fn_mod2(&self) -> &INI1_FN_MOD2 {
        &self.ini1_fn_mod2
    }
    #[doc = "0x42028 - AXI interconnect - INI x AHB functionality modification register"]
    #[inline(always)]
    pub const fn ini1_fn_mod_ahb(&self) -> &INI1_FN_MOD_AHB {
        &self.ini1_fn_mod_ahb
    }
    #[doc = "0x42100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn ini1_read_qos(&self) -> &INI1_READ_QOS {
        &self.ini1_read_qos
    }
    #[doc = "0x42104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn ini1_write_qos(&self) -> &INI1_WRITE_QOS {
        &self.ini1_write_qos
    }
    #[doc = "0x42108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn ini1_fn_mod(&self) -> &INI1_FN_MOD {
        &self.ini1_fn_mod
    }
    #[doc = "0x43100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn ini2_read_qos(&self) -> &INI2_READ_QOS {
        &self.ini2_read_qos
    }
    #[doc = "0x43104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn ini2_write_qos(&self) -> &INI2_WRITE_QOS {
        &self.ini2_write_qos
    }
    #[doc = "0x43108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn ini2_fn_mod(&self) -> &INI2_FN_MOD {
        &self.ini2_fn_mod
    }
    #[doc = "0x44024 - AXI interconnect - INI x functionality modification 2 register"]
    #[inline(always)]
    pub const fn ini3_fn_mod2(&self) -> &INI3_FN_MOD2 {
        &self.ini3_fn_mod2
    }
    #[doc = "0x44028 - AXI interconnect - INI x AHB functionality modification register"]
    #[inline(always)]
    pub const fn ini3_fn_mod_ahb(&self) -> &INI3_FN_MOD_AHB {
        &self.ini3_fn_mod_ahb
    }
    #[doc = "0x44100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn ini3_read_qos(&self) -> &INI3_READ_QOS {
        &self.ini3_read_qos
    }
    #[doc = "0x44104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn ini3_write_qos(&self) -> &INI3_WRITE_QOS {
        &self.ini3_write_qos
    }
    #[doc = "0x44108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn ini3_fn_mod(&self) -> &INI3_FN_MOD {
        &self.ini3_fn_mod
    }
    #[doc = "0x45100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn ini4_read_qos(&self) -> &INI4_READ_QOS {
        &self.ini4_read_qos
    }
    #[doc = "0x45104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn ini4_write_qos(&self) -> &INI4_WRITE_QOS {
        &self.ini4_write_qos
    }
    #[doc = "0x45108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn ini4_fn_mod(&self) -> &INI4_FN_MOD {
        &self.ini4_fn_mod
    }
    #[doc = "0x46100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn ini5_read_qos(&self) -> &INI5_READ_QOS {
        &self.ini5_read_qos
    }
    #[doc = "0x46104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn ini5_write_qos(&self) -> &INI5_WRITE_QOS {
        &self.ini5_write_qos
    }
    #[doc = "0x46108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn ini5_fn_mod(&self) -> &INI5_FN_MOD {
        &self.ini5_fn_mod
    }
    #[doc = "0x47100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn ini6_read_qos(&self) -> &INI6_READ_QOS {
        &self.ini6_read_qos
    }
    #[doc = "0x47104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn ini6_write_qos(&self) -> &INI6_WRITE_QOS {
        &self.ini6_write_qos
    }
    #[doc = "0x47108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn ini6_fn_mod(&self) -> &INI6_FN_MOD {
        &self.ini6_fn_mod
    }
}
#[doc = "PERIPH_ID_4 (r) register accessor: AXI interconnect - peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_4`]
module"]
pub type PERIPH_ID_4 = crate::Reg<periph_id_4::PERIPH_ID_4rs>;
#[doc = "AXI interconnect - peripheral ID4 register"]
pub mod periph_id_4;
#[doc = "PERIPH_ID_0 (r) register accessor: AXI interconnect - peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_0`]
module"]
pub type PERIPH_ID_0 = crate::Reg<periph_id_0::PERIPH_ID_0rs>;
#[doc = "AXI interconnect - peripheral ID0 register"]
pub mod periph_id_0;
#[doc = "PERIPH_ID_1 (r) register accessor: AXI interconnect - peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_1`]
module"]
pub type PERIPH_ID_1 = crate::Reg<periph_id_1::PERIPH_ID_1rs>;
#[doc = "AXI interconnect - peripheral ID1 register"]
pub mod periph_id_1;
#[doc = "PERIPH_ID_2 (r) register accessor: AXI interconnect - peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_2`]
module"]
pub type PERIPH_ID_2 = crate::Reg<periph_id_2::PERIPH_ID_2rs>;
#[doc = "AXI interconnect - peripheral ID2 register"]
pub mod periph_id_2;
#[doc = "PERIPH_ID_3 (r) register accessor: AXI interconnect - peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_3`]
module"]
pub type PERIPH_ID_3 = crate::Reg<periph_id_3::PERIPH_ID_3rs>;
#[doc = "AXI interconnect - peripheral ID3 register"]
pub mod periph_id_3;
#[doc = "COMP_ID_0 (r) register accessor: AXI interconnect - component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_id_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_id_0`]
module"]
pub type COMP_ID_0 = crate::Reg<comp_id_0::COMP_ID_0rs>;
#[doc = "AXI interconnect - component ID0 register"]
pub mod comp_id_0;
#[doc = "COMP_ID_1 (r) register accessor: AXI interconnect - component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_id_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_id_1`]
module"]
pub type COMP_ID_1 = crate::Reg<comp_id_1::COMP_ID_1rs>;
#[doc = "AXI interconnect - component ID1 register"]
pub mod comp_id_1;
#[doc = "COMP_ID_2 (r) register accessor: AXI interconnect - component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_id_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_id_2`]
module"]
pub type COMP_ID_2 = crate::Reg<comp_id_2::COMP_ID_2rs>;
#[doc = "AXI interconnect - component ID2 register"]
pub mod comp_id_2;
#[doc = "COMP_ID_3 (r) register accessor: AXI interconnect - component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_id_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_id_3`]
module"]
pub type COMP_ID_3 = crate::Reg<comp_id_3::COMP_ID_3rs>;
#[doc = "AXI interconnect - component ID3 register"]
pub mod comp_id_3;
#[doc = "TARG1_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ1_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ1_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ1_fn_mod_iss_bm`]
module"]
pub type TARG1_FN_MOD_ISS_BM = crate::Reg<targ1_fn_mod_iss_bm::TARG1_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ1_fn_mod_iss_bm;
#[doc = "TARG2_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ2_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ2_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ2_fn_mod_iss_bm`]
module"]
pub type TARG2_FN_MOD_ISS_BM = crate::Reg<targ2_fn_mod_iss_bm::TARG2_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ2_fn_mod_iss_bm;
#[doc = "TARG3_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ3_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ3_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ3_fn_mod_iss_bm`]
module"]
pub type TARG3_FN_MOD_ISS_BM = crate::Reg<targ3_fn_mod_iss_bm::TARG3_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ3_fn_mod_iss_bm;
#[doc = "TARG4_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ4_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ4_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ4_fn_mod_iss_bm`]
module"]
pub type TARG4_FN_MOD_ISS_BM = crate::Reg<targ4_fn_mod_iss_bm::TARG4_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ4_fn_mod_iss_bm;
#[doc = "TARG5_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ5_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ5_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ5_fn_mod_iss_bm`]
module"]
pub type TARG5_FN_MOD_ISS_BM = crate::Reg<targ5_fn_mod_iss_bm::TARG5_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ5_fn_mod_iss_bm;
#[doc = "TARG6_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ6_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ6_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ6_fn_mod_iss_bm`]
module"]
pub type TARG6_FN_MOD_ISS_BM = crate::Reg<targ6_fn_mod_iss_bm::TARG6_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ6_fn_mod_iss_bm;
#[doc = "TARG7_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ7_fn_mod_iss_bm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ7_fn_mod_iss_bm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ7_fn_mod_iss_bm`]
module"]
pub type TARG7_FN_MOD_ISS_BM = crate::Reg<targ7_fn_mod_iss_bm::TARG7_FN_MOD_ISS_BMrs>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod targ7_fn_mod_iss_bm;
#[doc = "TARG1_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ1_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ1_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ1_fn_mod2`]
module"]
pub type TARG1_FN_MOD2 = crate::Reg<targ1_fn_mod2::TARG1_FN_MOD2rs>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ1_fn_mod2;
#[doc = "TARG2_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ2_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ2_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ2_fn_mod2`]
module"]
pub type TARG2_FN_MOD2 = crate::Reg<targ2_fn_mod2::TARG2_FN_MOD2rs>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ2_fn_mod2;
#[doc = "TARG7_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ7_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ7_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ7_fn_mod2`]
module"]
pub type TARG7_FN_MOD2 = crate::Reg<targ7_fn_mod2::TARG7_FN_MOD2rs>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod targ7_fn_mod2;
#[doc = "TARG1_FN_MOD_LB (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ1_fn_mod_lb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ1_fn_mod_lb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ1_fn_mod_lb`]
module"]
pub type TARG1_FN_MOD_LB = crate::Reg<targ1_fn_mod_lb::TARG1_FN_MOD_LBrs>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ1_fn_mod_lb;
#[doc = "TARG2_FN_MOD_LB (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ2_fn_mod_lb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ2_fn_mod_lb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ2_fn_mod_lb`]
module"]
pub type TARG2_FN_MOD_LB = crate::Reg<targ2_fn_mod_lb::TARG2_FN_MOD_LBrs>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ2_fn_mod_lb;
#[doc = "TARG1_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ1_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ1_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ1_fn_mod`]
module"]
pub type TARG1_FN_MOD = crate::Reg<targ1_fn_mod::TARG1_FN_MODrs>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ1_fn_mod;
#[doc = "TARG2_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ2_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ2_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ2_fn_mod`]
module"]
pub type TARG2_FN_MOD = crate::Reg<targ2_fn_mod::TARG2_FN_MODrs>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ2_fn_mod;
#[doc = "TARG7_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ7_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ7_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@targ7_fn_mod`]
module"]
pub type TARG7_FN_MOD = crate::Reg<targ7_fn_mod::TARG7_FN_MODrs>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod targ7_fn_mod;
#[doc = "INI1_FN_MOD2 (rw) register accessor: AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini1_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini1_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini1_fn_mod2`]
module"]
pub type INI1_FN_MOD2 = crate::Reg<ini1_fn_mod2::INI1_FN_MOD2rs>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod ini1_fn_mod2;
#[doc = "INI3_FN_MOD2 (rw) register accessor: AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini3_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini3_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini3_fn_mod2`]
module"]
pub type INI3_FN_MOD2 = crate::Reg<ini3_fn_mod2::INI3_FN_MOD2rs>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod ini3_fn_mod2;
#[doc = "INI1_FN_MOD_AHB (rw) register accessor: AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini1_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini1_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini1_fn_mod_ahb`]
module"]
pub type INI1_FN_MOD_AHB = crate::Reg<ini1_fn_mod_ahb::INI1_FN_MOD_AHBrs>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod ini1_fn_mod_ahb;
#[doc = "INI3_FN_MOD_AHB (rw) register accessor: AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini3_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini3_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini3_fn_mod_ahb`]
module"]
pub type INI3_FN_MOD_AHB = crate::Reg<ini3_fn_mod_ahb::INI3_FN_MOD_AHBrs>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod ini3_fn_mod_ahb;
#[doc = "INI1_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini1_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini1_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini1_read_qos`]
module"]
pub type INI1_READ_QOS = crate::Reg<ini1_read_qos::INI1_READ_QOSrs>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini1_read_qos;
#[doc = "INI2_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini2_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini2_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini2_read_qos`]
module"]
pub type INI2_READ_QOS = crate::Reg<ini2_read_qos::INI2_READ_QOSrs>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini2_read_qos;
#[doc = "INI3_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini3_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini3_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini3_read_qos`]
module"]
pub type INI3_READ_QOS = crate::Reg<ini3_read_qos::INI3_READ_QOSrs>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini3_read_qos;
#[doc = "INI4_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini4_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini4_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini4_read_qos`]
module"]
pub type INI4_READ_QOS = crate::Reg<ini4_read_qos::INI4_READ_QOSrs>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini4_read_qos;
#[doc = "INI5_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini5_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini5_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini5_read_qos`]
module"]
pub type INI5_READ_QOS = crate::Reg<ini5_read_qos::INI5_READ_QOSrs>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini5_read_qos;
#[doc = "INI6_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini6_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini6_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini6_read_qos`]
module"]
pub type INI6_READ_QOS = crate::Reg<ini6_read_qos::INI6_READ_QOSrs>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod ini6_read_qos;
#[doc = "INI1_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini1_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini1_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini1_write_qos`]
module"]
pub type INI1_WRITE_QOS = crate::Reg<ini1_write_qos::INI1_WRITE_QOSrs>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini1_write_qos;
#[doc = "INI2_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini2_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini2_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini2_write_qos`]
module"]
pub type INI2_WRITE_QOS = crate::Reg<ini2_write_qos::INI2_WRITE_QOSrs>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini2_write_qos;
#[doc = "INI3_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini3_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini3_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini3_write_qos`]
module"]
pub type INI3_WRITE_QOS = crate::Reg<ini3_write_qos::INI3_WRITE_QOSrs>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini3_write_qos;
#[doc = "INI4_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini4_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini4_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini4_write_qos`]
module"]
pub type INI4_WRITE_QOS = crate::Reg<ini4_write_qos::INI4_WRITE_QOSrs>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini4_write_qos;
#[doc = "INI5_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini5_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini5_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini5_write_qos`]
module"]
pub type INI5_WRITE_QOS = crate::Reg<ini5_write_qos::INI5_WRITE_QOSrs>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini5_write_qos;
#[doc = "INI6_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini6_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini6_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini6_write_qos`]
module"]
pub type INI6_WRITE_QOS = crate::Reg<ini6_write_qos::INI6_WRITE_QOSrs>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod ini6_write_qos;
#[doc = "INI1_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini1_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini1_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini1_fn_mod`]
module"]
pub type INI1_FN_MOD = crate::Reg<ini1_fn_mod::INI1_FN_MODrs>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini1_fn_mod;
#[doc = "INI2_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini2_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini2_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini2_fn_mod`]
module"]
pub type INI2_FN_MOD = crate::Reg<ini2_fn_mod::INI2_FN_MODrs>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini2_fn_mod;
#[doc = "INI3_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini3_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini3_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini3_fn_mod`]
module"]
pub type INI3_FN_MOD = crate::Reg<ini3_fn_mod::INI3_FN_MODrs>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini3_fn_mod;
#[doc = "INI4_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini4_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini4_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini4_fn_mod`]
module"]
pub type INI4_FN_MOD = crate::Reg<ini4_fn_mod::INI4_FN_MODrs>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini4_fn_mod;
#[doc = "INI5_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini5_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini5_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini5_fn_mod`]
module"]
pub type INI5_FN_MOD = crate::Reg<ini5_fn_mod::INI5_FN_MODrs>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini5_fn_mod;
#[doc = "INI6_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini6_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini6_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ini6_fn_mod`]
module"]
pub type INI6_FN_MOD = crate::Reg<ini6_fn_mod::INI6_FN_MODrs>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod ini6_fn_mod;
