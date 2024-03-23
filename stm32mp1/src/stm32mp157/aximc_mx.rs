#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aximc_m0_fn_mod2: AXIMC_M0_FN_MOD2,
    _reserved1: [u8; 0xd8],
    aximc_m0_read_qos: AXIMC_M0_READ_QOS,
    aximc_m0_fn_mod: AXIMC_M0_FN_MOD,
    aximc_m0_write_qos: AXIMC_M0_WRITE_QOS,
    _reserved4: [u8; 0x0f18],
    aximc_m1_fn_mod2: AXIMC_M1_FN_MOD2,
    _reserved5: [u8; 0xd8],
    aximc_m1_read_qos: AXIMC_M1_READ_QOS,
    aximc_m1_write_qos: AXIMC_M1_WRITE_QOS,
    aximc_m1_fn_mod: AXIMC_M1_FN_MOD,
    _reserved8: [u8; 0x0ee8],
    aximc_periph_id_4: AXIMC_PERIPH_ID_4,
    aximc_periph_id_5: AXIMC_PERIPH_ID_5,
    aximc_periph_id_6: AXIMC_PERIPH_ID_6,
    aximc_periph_id_7: AXIMC_PERIPH_ID_7,
    aximc_periph_id_0: AXIMC_PERIPH_ID_0,
    aximc_periph_id_1: AXIMC_PERIPH_ID_1,
    aximc_periph_id_2: AXIMC_PERIPH_ID_2,
    aximc_periph_id_3: AXIMC_PERIPH_ID_3,
    aximc_comp_id_0: AXIMC_COMP_ID_0,
    aximc_comp_id_1: AXIMC_COMP_ID_1,
    aximc_comp_id_2: AXIMC_COMP_ID_2,
    aximc_comp_id_3: AXIMC_COMP_ID_3,
    aximc_m2_fn_mod2: AXIMC_M2_FN_MOD2,
    _reserved21: [u8; 0xd8],
    aximc_m2_read_qos: AXIMC_M2_READ_QOS,
    aximc_m2_write_qos: AXIMC_M2_WRITE_QOS,
    aximc_m2_fn_mod: AXIMC_M2_FN_MOD,
    _reserved24: [u8; 0x0f18],
    aximc_m5_fn_mod2: AXIMC_M5_FN_MOD2,
    _reserved25: [u8; 0xd8],
    aximc_m5_read_qos: AXIMC_M5_READ_QOS,
    aximc_m5_write_qos: AXIMC_M5_WRITE_QOS,
    aximc_m5_fn_mod: AXIMC_M5_FN_MOD,
    _reserved28: [u8; 0x0ff4],
    aximc_m3_read_qos: AXIMC_M3_READ_QOS,
    aximc_m3_write_qos: AXIMC_M3_WRITE_QOS,
    aximc_m3_fn_mod: AXIMC_M3_FN_MOD,
    _reserved31: [u8; 0x0ff4],
    aximc_m7_read_qos: AXIMC_M7_READ_QOS,
    aximc_m7_write_qos: AXIMC_M7_WRITE_QOS,
    aximc_m7_fn_mod: AXIMC_M7_FN_MOD,
    _reserved34: [u8; 0x0ff4],
    aximc_m8_read_qos: AXIMC_M8_READ_QOS,
    aximc_m8_write_qos: AXIMC_M8_WRITE_QOS,
    aximc_m8_fn_mod: AXIMC_M8_FN_MOD,
    _reserved37: [u8; 0x1f18],
    aximc_m4_fn_mod2: AXIMC_M4_FN_MOD2,
    _reserved38: [u8; 0xd8],
    aximc_m4_read_qos: AXIMC_M4_READ_QOS,
    aximc_m4_write_qos: AXIMC_M4_WRITE_QOS,
    aximc_m4_fn_mod: AXIMC_M4_FN_MOD,
    _reserved41: [u8; 0x0ff4],
    aximc_m9_read_qos: AXIMC_M9_READ_QOS,
    aximc_m9_write_qos: AXIMC_M9_WRITE_QOS,
    aximc_m9_fn_mod: AXIMC_M9_FN_MOD,
    _reserved44: [u8; 0x0ff4],
    aximc_m10_read_qos: AXIMC_M10_READ_QOS,
    aximc_m10_write_qos: AXIMC_M10_WRITE_QOS,
    aximc_m10_fn_mod: AXIMC_M10_FN_MOD,
    _reserved47: [u8; 0x0f18],
    aximc_m6_fn_mod2: AXIMC_M6_FN_MOD2,
    _reserved48: [u8; 0xd8],
    aximc_m6_read_qos: AXIMC_M6_READ_QOS,
    aximc_m6_write_qos: AXIMC_M6_WRITE_QOS,
    aximc_m6_fn_mod: AXIMC_M6_FN_MOD,
    _reserved51: [u8; 0x0003_6f40],
    aximc_m0_fn_mod_ahb: AXIMC_M0_FN_MOD_AHB,
    _reserved52: [u8; 0x0ffc],
    aximc_m1_fn_mod_ahb: AXIMC_M1_FN_MOD_AHB,
    _reserved53: [u8; 0x0ffc],
    aximc_m2_fn_mod_ahb: AXIMC_M2_FN_MOD_AHB,
    _reserved54: [u8; 0x0ffc],
    aximc_m5_fn_mod_ahb: AXIMC_M5_FN_MOD_AHB,
    _reserved55: [u8; 0x5000],
    aximc_fn_mod_lb: AXIMC_FN_MOD_LB,
    _reserved56: [u8; 0x2ff8],
    aximc_m6_fn_mod_ahb: AXIMC_M6_FN_MOD_AHB,
}
impl RegisterBlock {
    #[doc = "0x00 - AXIMC master 0 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m0_fn_mod2(&self) -> &AXIMC_M0_FN_MOD2 {
        &self.aximc_m0_fn_mod2
    }
    #[doc = "0xdc - AXIMC master 0 read priority register"]
    #[inline(always)]
    pub const fn aximc_m0_read_qos(&self) -> &AXIMC_M0_READ_QOS {
        &self.aximc_m0_read_qos
    }
    #[doc = "0xe0 - AXIMC master 0 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m0_fn_mod(&self) -> &AXIMC_M0_FN_MOD {
        &self.aximc_m0_fn_mod
    }
    #[doc = "0xe4 - AXIMC master 0 write priority register"]
    #[inline(always)]
    pub const fn aximc_m0_write_qos(&self) -> &AXIMC_M0_WRITE_QOS {
        &self.aximc_m0_write_qos
    }
    #[doc = "0x1000 - AXIMC master 1 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m1_fn_mod2(&self) -> &AXIMC_M1_FN_MOD2 {
        &self.aximc_m1_fn_mod2
    }
    #[doc = "0x10dc - AXIMC master 1 read priority register"]
    #[inline(always)]
    pub const fn aximc_m1_read_qos(&self) -> &AXIMC_M1_READ_QOS {
        &self.aximc_m1_read_qos
    }
    #[doc = "0x10e0 - AXIMC master 1 write priority register"]
    #[inline(always)]
    pub const fn aximc_m1_write_qos(&self) -> &AXIMC_M1_WRITE_QOS {
        &self.aximc_m1_write_qos
    }
    #[doc = "0x10e4 - AXIMC master 1 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m1_fn_mod(&self) -> &AXIMC_M1_FN_MOD {
        &self.aximc_m1_fn_mod
    }
    #[doc = "0x1fd0 - AXIMC peripheral ID4 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_4(&self) -> &AXIMC_PERIPH_ID_4 {
        &self.aximc_periph_id_4
    }
    #[doc = "0x1fd4 - AXIMC peripheral ID5 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_5(&self) -> &AXIMC_PERIPH_ID_5 {
        &self.aximc_periph_id_5
    }
    #[doc = "0x1fd8 - AXIMC peripheral ID6 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_6(&self) -> &AXIMC_PERIPH_ID_6 {
        &self.aximc_periph_id_6
    }
    #[doc = "0x1fdc - AXIMC peripheral ID7 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_7(&self) -> &AXIMC_PERIPH_ID_7 {
        &self.aximc_periph_id_7
    }
    #[doc = "0x1fe0 - AXIMC peripheral ID0 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_0(&self) -> &AXIMC_PERIPH_ID_0 {
        &self.aximc_periph_id_0
    }
    #[doc = "0x1fe4 - AXIMC peripheral ID1 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_1(&self) -> &AXIMC_PERIPH_ID_1 {
        &self.aximc_periph_id_1
    }
    #[doc = "0x1fe8 - AXIMC peripheral ID2 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_2(&self) -> &AXIMC_PERIPH_ID_2 {
        &self.aximc_periph_id_2
    }
    #[doc = "0x1fec - AXIMC peripheral ID3 register"]
    #[inline(always)]
    pub const fn aximc_periph_id_3(&self) -> &AXIMC_PERIPH_ID_3 {
        &self.aximc_periph_id_3
    }
    #[doc = "0x1ff0 - AXIMC component ID0 register"]
    #[inline(always)]
    pub const fn aximc_comp_id_0(&self) -> &AXIMC_COMP_ID_0 {
        &self.aximc_comp_id_0
    }
    #[doc = "0x1ff4 - AXIMC component ID1 register"]
    #[inline(always)]
    pub const fn aximc_comp_id_1(&self) -> &AXIMC_COMP_ID_1 {
        &self.aximc_comp_id_1
    }
    #[doc = "0x1ff8 - AXIMC component ID2 register"]
    #[inline(always)]
    pub const fn aximc_comp_id_2(&self) -> &AXIMC_COMP_ID_2 {
        &self.aximc_comp_id_2
    }
    #[doc = "0x1ffc - AXIMC component ID3 register"]
    #[inline(always)]
    pub const fn aximc_comp_id_3(&self) -> &AXIMC_COMP_ID_3 {
        &self.aximc_comp_id_3
    }
    #[doc = "0x2000 - AXIMC master 2 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m2_fn_mod2(&self) -> &AXIMC_M2_FN_MOD2 {
        &self.aximc_m2_fn_mod2
    }
    #[doc = "0x20dc - AXIMC master 2 read priority register"]
    #[inline(always)]
    pub const fn aximc_m2_read_qos(&self) -> &AXIMC_M2_READ_QOS {
        &self.aximc_m2_read_qos
    }
    #[doc = "0x20e0 - AXIMC master 2 write priority register"]
    #[inline(always)]
    pub const fn aximc_m2_write_qos(&self) -> &AXIMC_M2_WRITE_QOS {
        &self.aximc_m2_write_qos
    }
    #[doc = "0x20e4 - AXIMC master 2 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m2_fn_mod(&self) -> &AXIMC_M2_FN_MOD {
        &self.aximc_m2_fn_mod
    }
    #[doc = "0x3000 - AXIMC master 5 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m5_fn_mod2(&self) -> &AXIMC_M5_FN_MOD2 {
        &self.aximc_m5_fn_mod2
    }
    #[doc = "0x30dc - AXIMC master 5 read priority register"]
    #[inline(always)]
    pub const fn aximc_m5_read_qos(&self) -> &AXIMC_M5_READ_QOS {
        &self.aximc_m5_read_qos
    }
    #[doc = "0x30e0 - AXIMC master 5 write priority register"]
    #[inline(always)]
    pub const fn aximc_m5_write_qos(&self) -> &AXIMC_M5_WRITE_QOS {
        &self.aximc_m5_write_qos
    }
    #[doc = "0x30e4 - AXIMC master 5 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m5_fn_mod(&self) -> &AXIMC_M5_FN_MOD {
        &self.aximc_m5_fn_mod
    }
    #[doc = "0x40dc - AXIMC master 3 read priority register"]
    #[inline(always)]
    pub const fn aximc_m3_read_qos(&self) -> &AXIMC_M3_READ_QOS {
        &self.aximc_m3_read_qos
    }
    #[doc = "0x40e0 - AXIMC master 3 write priority register"]
    #[inline(always)]
    pub const fn aximc_m3_write_qos(&self) -> &AXIMC_M3_WRITE_QOS {
        &self.aximc_m3_write_qos
    }
    #[doc = "0x40e4 - AXIMC master 3 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m3_fn_mod(&self) -> &AXIMC_M3_FN_MOD {
        &self.aximc_m3_fn_mod
    }
    #[doc = "0x50dc - AXIMC master 7 read priority register"]
    #[inline(always)]
    pub const fn aximc_m7_read_qos(&self) -> &AXIMC_M7_READ_QOS {
        &self.aximc_m7_read_qos
    }
    #[doc = "0x50e0 - AXIMC master 7 write priority register"]
    #[inline(always)]
    pub const fn aximc_m7_write_qos(&self) -> &AXIMC_M7_WRITE_QOS {
        &self.aximc_m7_write_qos
    }
    #[doc = "0x50e4 - AXIMC master 7 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m7_fn_mod(&self) -> &AXIMC_M7_FN_MOD {
        &self.aximc_m7_fn_mod
    }
    #[doc = "0x60dc - AXIMC master 8 read priority register"]
    #[inline(always)]
    pub const fn aximc_m8_read_qos(&self) -> &AXIMC_M8_READ_QOS {
        &self.aximc_m8_read_qos
    }
    #[doc = "0x60e0 - AXIMC master 8 write priority register"]
    #[inline(always)]
    pub const fn aximc_m8_write_qos(&self) -> &AXIMC_M8_WRITE_QOS {
        &self.aximc_m8_write_qos
    }
    #[doc = "0x60e4 - AXIMC master 8 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m8_fn_mod(&self) -> &AXIMC_M8_FN_MOD {
        &self.aximc_m8_fn_mod
    }
    #[doc = "0x8000 - AXIMC master 4 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m4_fn_mod2(&self) -> &AXIMC_M4_FN_MOD2 {
        &self.aximc_m4_fn_mod2
    }
    #[doc = "0x80dc - AXIMC master 4 read priority register"]
    #[inline(always)]
    pub const fn aximc_m4_read_qos(&self) -> &AXIMC_M4_READ_QOS {
        &self.aximc_m4_read_qos
    }
    #[doc = "0x80e0 - AXIMC master 4 write priority register"]
    #[inline(always)]
    pub const fn aximc_m4_write_qos(&self) -> &AXIMC_M4_WRITE_QOS {
        &self.aximc_m4_write_qos
    }
    #[doc = "0x80e4 - AXIMC master 4 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m4_fn_mod(&self) -> &AXIMC_M4_FN_MOD {
        &self.aximc_m4_fn_mod
    }
    #[doc = "0x90dc - AXIMC master 9 read priority register"]
    #[inline(always)]
    pub const fn aximc_m9_read_qos(&self) -> &AXIMC_M9_READ_QOS {
        &self.aximc_m9_read_qos
    }
    #[doc = "0x90e0 - AXIMC master 9 write priority register"]
    #[inline(always)]
    pub const fn aximc_m9_write_qos(&self) -> &AXIMC_M9_WRITE_QOS {
        &self.aximc_m9_write_qos
    }
    #[doc = "0x90e4 - AXIMC master 9 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m9_fn_mod(&self) -> &AXIMC_M9_FN_MOD {
        &self.aximc_m9_fn_mod
    }
    #[doc = "0xa0dc - AXIMC master 10 read priority register"]
    #[inline(always)]
    pub const fn aximc_m10_read_qos(&self) -> &AXIMC_M10_READ_QOS {
        &self.aximc_m10_read_qos
    }
    #[doc = "0xa0e0 - AXIMC master 10 write priority register"]
    #[inline(always)]
    pub const fn aximc_m10_write_qos(&self) -> &AXIMC_M10_WRITE_QOS {
        &self.aximc_m10_write_qos
    }
    #[doc = "0xa0e4 - AXIMC master 10 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m10_fn_mod(&self) -> &AXIMC_M10_FN_MOD {
        &self.aximc_m10_fn_mod
    }
    #[doc = "0xb000 - AXIMC master 6 packing functionality register"]
    #[inline(always)]
    pub const fn aximc_m6_fn_mod2(&self) -> &AXIMC_M6_FN_MOD2 {
        &self.aximc_m6_fn_mod2
    }
    #[doc = "0xb0dc - AXIMC master 6 read priority register"]
    #[inline(always)]
    pub const fn aximc_m6_read_qos(&self) -> &AXIMC_M6_READ_QOS {
        &self.aximc_m6_read_qos
    }
    #[doc = "0xb0e0 - AXIMC master 6 write priority register"]
    #[inline(always)]
    pub const fn aximc_m6_write_qos(&self) -> &AXIMC_M6_WRITE_QOS {
        &self.aximc_m6_write_qos
    }
    #[doc = "0xb0e4 - AXIMC master 6 issuing capability override functionality register"]
    #[inline(always)]
    pub const fn aximc_m6_fn_mod(&self) -> &AXIMC_M6_FN_MOD {
        &self.aximc_m6_fn_mod
    }
    #[doc = "0x42028 - AXIMC master 0 AHB conversion override functionality register"]
    #[inline(always)]
    pub const fn aximc_m0_fn_mod_ahb(&self) -> &AXIMC_M0_FN_MOD_AHB {
        &self.aximc_m0_fn_mod_ahb
    }
    #[doc = "0x43028 - AXIMC master 1 AHB conversion override functionality register"]
    #[inline(always)]
    pub const fn aximc_m1_fn_mod_ahb(&self) -> &AXIMC_M1_FN_MOD_AHB {
        &self.aximc_m1_fn_mod_ahb
    }
    #[doc = "0x44028 - AXIMC master 2 AHB conversion override functionality register"]
    #[inline(always)]
    pub const fn aximc_m2_fn_mod_ahb(&self) -> &AXIMC_M2_FN_MOD_AHB {
        &self.aximc_m2_fn_mod_ahb
    }
    #[doc = "0x45028 - AXIMC master 5 AHB conversion override functionality register"]
    #[inline(always)]
    pub const fn aximc_m5_fn_mod_ahb(&self) -> &AXIMC_M5_FN_MOD_AHB {
        &self.aximc_m5_fn_mod_ahb
    }
    #[doc = "0x4a02c - AXIMC long burst capability inhibition register"]
    #[inline(always)]
    pub const fn aximc_fn_mod_lb(&self) -> &AXIMC_FN_MOD_LB {
        &self.aximc_fn_mod_lb
    }
    #[doc = "0x4d028 - AXIMC master 6 AHB conversion override functionality register"]
    #[inline(always)]
    pub const fn aximc_m6_fn_mod_ahb(&self) -> &AXIMC_M6_FN_MOD_AHB {
        &self.aximc_m6_fn_mod_ahb
    }
}
#[doc = "AXIMC_M0_FN_MOD2 (rw) register accessor: AXIMC master 0 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m0_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m0_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m0_fn_mod2`]
module"]
pub type AXIMC_M0_FN_MOD2 = crate::Reg<aximc_m0_fn_mod2::AXIMC_M0_FN_MOD2rs>;
#[doc = "AXIMC master 0 packing functionality register"]
pub mod aximc_m0_fn_mod2;
#[doc = "AXIMC_M0_READ_QOS (rw) register accessor: AXIMC master 0 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m0_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m0_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m0_read_qos`]
module"]
pub type AXIMC_M0_READ_QOS = crate::Reg<aximc_m0_read_qos::AXIMC_M0_READ_QOSrs>;
#[doc = "AXIMC master 0 read priority register"]
pub mod aximc_m0_read_qos;
#[doc = "AXIMC_M0_WRITE_QOS (rw) register accessor: AXIMC master 0 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m0_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m0_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m0_write_qos`]
module"]
pub type AXIMC_M0_WRITE_QOS = crate::Reg<aximc_m0_write_qos::AXIMC_M0_WRITE_QOSrs>;
#[doc = "AXIMC master 0 write priority register"]
pub mod aximc_m0_write_qos;
#[doc = "AXIMC_M0_FN_MOD (rw) register accessor: AXIMC master 0 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m0_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m0_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m0_fn_mod`]
module"]
pub type AXIMC_M0_FN_MOD = crate::Reg<aximc_m0_fn_mod::AXIMC_M0_FN_MODrs>;
#[doc = "AXIMC master 0 issuing capability override functionality register"]
pub mod aximc_m0_fn_mod;
#[doc = "AXIMC_M1_FN_MOD2 (rw) register accessor: AXIMC master 1 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m1_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m1_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m1_fn_mod2`]
module"]
pub type AXIMC_M1_FN_MOD2 = crate::Reg<aximc_m1_fn_mod2::AXIMC_M1_FN_MOD2rs>;
#[doc = "AXIMC master 1 packing functionality register"]
pub mod aximc_m1_fn_mod2;
#[doc = "AXIMC_M1_READ_QOS (rw) register accessor: AXIMC master 1 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m1_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m1_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m1_read_qos`]
module"]
pub type AXIMC_M1_READ_QOS = crate::Reg<aximc_m1_read_qos::AXIMC_M1_READ_QOSrs>;
#[doc = "AXIMC master 1 read priority register"]
pub mod aximc_m1_read_qos;
#[doc = "AXIMC_M1_WRITE_QOS (rw) register accessor: AXIMC master 1 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m1_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m1_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m1_write_qos`]
module"]
pub type AXIMC_M1_WRITE_QOS = crate::Reg<aximc_m1_write_qos::AXIMC_M1_WRITE_QOSrs>;
#[doc = "AXIMC master 1 write priority register"]
pub mod aximc_m1_write_qos;
#[doc = "AXIMC_M1_FN_MOD (rw) register accessor: AXIMC master 1 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m1_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m1_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m1_fn_mod`]
module"]
pub type AXIMC_M1_FN_MOD = crate::Reg<aximc_m1_fn_mod::AXIMC_M1_FN_MODrs>;
#[doc = "AXIMC master 1 issuing capability override functionality register"]
pub mod aximc_m1_fn_mod;
#[doc = "AXIMC_M2_FN_MOD2 (rw) register accessor: AXIMC master 2 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m2_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m2_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m2_fn_mod2`]
module"]
pub type AXIMC_M2_FN_MOD2 = crate::Reg<aximc_m2_fn_mod2::AXIMC_M2_FN_MOD2rs>;
#[doc = "AXIMC master 2 packing functionality register"]
pub mod aximc_m2_fn_mod2;
#[doc = "AXIMC_M2_READ_QOS (rw) register accessor: AXIMC master 2 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m2_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m2_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m2_read_qos`]
module"]
pub type AXIMC_M2_READ_QOS = crate::Reg<aximc_m2_read_qos::AXIMC_M2_READ_QOSrs>;
#[doc = "AXIMC master 2 read priority register"]
pub mod aximc_m2_read_qos;
#[doc = "AXIMC_M2_WRITE_QOS (rw) register accessor: AXIMC master 2 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m2_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m2_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m2_write_qos`]
module"]
pub type AXIMC_M2_WRITE_QOS = crate::Reg<aximc_m2_write_qos::AXIMC_M2_WRITE_QOSrs>;
#[doc = "AXIMC master 2 write priority register"]
pub mod aximc_m2_write_qos;
#[doc = "AXIMC_M2_FN_MOD (rw) register accessor: AXIMC master 2 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m2_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m2_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m2_fn_mod`]
module"]
pub type AXIMC_M2_FN_MOD = crate::Reg<aximc_m2_fn_mod::AXIMC_M2_FN_MODrs>;
#[doc = "AXIMC master 2 issuing capability override functionality register"]
pub mod aximc_m2_fn_mod;
#[doc = "AXIMC_M5_FN_MOD2 (rw) register accessor: AXIMC master 5 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m5_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m5_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m5_fn_mod2`]
module"]
pub type AXIMC_M5_FN_MOD2 = crate::Reg<aximc_m5_fn_mod2::AXIMC_M5_FN_MOD2rs>;
#[doc = "AXIMC master 5 packing functionality register"]
pub mod aximc_m5_fn_mod2;
#[doc = "AXIMC_M5_READ_QOS (rw) register accessor: AXIMC master 5 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m5_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m5_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m5_read_qos`]
module"]
pub type AXIMC_M5_READ_QOS = crate::Reg<aximc_m5_read_qos::AXIMC_M5_READ_QOSrs>;
#[doc = "AXIMC master 5 read priority register"]
pub mod aximc_m5_read_qos;
#[doc = "AXIMC_M5_WRITE_QOS (rw) register accessor: AXIMC master 5 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m5_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m5_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m5_write_qos`]
module"]
pub type AXIMC_M5_WRITE_QOS = crate::Reg<aximc_m5_write_qos::AXIMC_M5_WRITE_QOSrs>;
#[doc = "AXIMC master 5 write priority register"]
pub mod aximc_m5_write_qos;
#[doc = "AXIMC_M5_FN_MOD (rw) register accessor: AXIMC master 5 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m5_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m5_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m5_fn_mod`]
module"]
pub type AXIMC_M5_FN_MOD = crate::Reg<aximc_m5_fn_mod::AXIMC_M5_FN_MODrs>;
#[doc = "AXIMC master 5 issuing capability override functionality register"]
pub mod aximc_m5_fn_mod;
#[doc = "AXIMC_M3_READ_QOS (rw) register accessor: AXIMC master 3 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m3_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m3_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m3_read_qos`]
module"]
pub type AXIMC_M3_READ_QOS = crate::Reg<aximc_m3_read_qos::AXIMC_M3_READ_QOSrs>;
#[doc = "AXIMC master 3 read priority register"]
pub mod aximc_m3_read_qos;
#[doc = "AXIMC_M3_WRITE_QOS (rw) register accessor: AXIMC master 3 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m3_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m3_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m3_write_qos`]
module"]
pub type AXIMC_M3_WRITE_QOS = crate::Reg<aximc_m3_write_qos::AXIMC_M3_WRITE_QOSrs>;
#[doc = "AXIMC master 3 write priority register"]
pub mod aximc_m3_write_qos;
#[doc = "AXIMC_M3_FN_MOD (rw) register accessor: AXIMC master 3 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m3_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m3_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m3_fn_mod`]
module"]
pub type AXIMC_M3_FN_MOD = crate::Reg<aximc_m3_fn_mod::AXIMC_M3_FN_MODrs>;
#[doc = "AXIMC master 3 packing functionality register"]
pub mod aximc_m3_fn_mod;
#[doc = "AXIMC_M7_READ_QOS (rw) register accessor: AXIMC master 7 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m7_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m7_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m7_read_qos`]
module"]
pub type AXIMC_M7_READ_QOS = crate::Reg<aximc_m7_read_qos::AXIMC_M7_READ_QOSrs>;
#[doc = "AXIMC master 7 read priority register"]
pub mod aximc_m7_read_qos;
#[doc = "AXIMC_M7_WRITE_QOS (rw) register accessor: AXIMC master 7 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m7_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m7_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m7_write_qos`]
module"]
pub type AXIMC_M7_WRITE_QOS = crate::Reg<aximc_m7_write_qos::AXIMC_M7_WRITE_QOSrs>;
#[doc = "AXIMC master 7 write priority register"]
pub mod aximc_m7_write_qos;
#[doc = "AXIMC_M7_FN_MOD (rw) register accessor: AXIMC master 7 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m7_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m7_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m7_fn_mod`]
module"]
pub type AXIMC_M7_FN_MOD = crate::Reg<aximc_m7_fn_mod::AXIMC_M7_FN_MODrs>;
#[doc = "AXIMC master 7 issuing capability override functionality register"]
pub mod aximc_m7_fn_mod;
#[doc = "AXIMC_M8_READ_QOS (rw) register accessor: AXIMC master 8 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m8_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m8_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m8_read_qos`]
module"]
pub type AXIMC_M8_READ_QOS = crate::Reg<aximc_m8_read_qos::AXIMC_M8_READ_QOSrs>;
#[doc = "AXIMC master 8 read priority register"]
pub mod aximc_m8_read_qos;
#[doc = "AXIMC_M8_WRITE_QOS (rw) register accessor: AXIMC master 8 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m8_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m8_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m8_write_qos`]
module"]
pub type AXIMC_M8_WRITE_QOS = crate::Reg<aximc_m8_write_qos::AXIMC_M8_WRITE_QOSrs>;
#[doc = "AXIMC master 8 write priority register"]
pub mod aximc_m8_write_qos;
#[doc = "AXIMC_M8_FN_MOD (rw) register accessor: AXIMC master 8 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m8_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m8_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m8_fn_mod`]
module"]
pub type AXIMC_M8_FN_MOD = crate::Reg<aximc_m8_fn_mod::AXIMC_M8_FN_MODrs>;
#[doc = "AXIMC master 8 issuing capability override functionality register"]
pub mod aximc_m8_fn_mod;
#[doc = "AXIMC_M4_FN_MOD2 (rw) register accessor: AXIMC master 4 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m4_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m4_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m4_fn_mod2`]
module"]
pub type AXIMC_M4_FN_MOD2 = crate::Reg<aximc_m4_fn_mod2::AXIMC_M4_FN_MOD2rs>;
#[doc = "AXIMC master 4 packing functionality register"]
pub mod aximc_m4_fn_mod2;
#[doc = "AXIMC_M4_READ_QOS (rw) register accessor: AXIMC master 4 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m4_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m4_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m4_read_qos`]
module"]
pub type AXIMC_M4_READ_QOS = crate::Reg<aximc_m4_read_qos::AXIMC_M4_READ_QOSrs>;
#[doc = "AXIMC master 4 read priority register"]
pub mod aximc_m4_read_qos;
#[doc = "AXIMC_M4_WRITE_QOS (rw) register accessor: AXIMC master 4 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m4_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m4_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m4_write_qos`]
module"]
pub type AXIMC_M4_WRITE_QOS = crate::Reg<aximc_m4_write_qos::AXIMC_M4_WRITE_QOSrs>;
#[doc = "AXIMC master 4 write priority register"]
pub mod aximc_m4_write_qos;
#[doc = "AXIMC_M4_FN_MOD (rw) register accessor: AXIMC master 4 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m4_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m4_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m4_fn_mod`]
module"]
pub type AXIMC_M4_FN_MOD = crate::Reg<aximc_m4_fn_mod::AXIMC_M4_FN_MODrs>;
#[doc = "AXIMC master 4 packing functionality register"]
pub mod aximc_m4_fn_mod;
#[doc = "AXIMC_M9_READ_QOS (rw) register accessor: AXIMC master 9 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m9_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m9_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m9_read_qos`]
module"]
pub type AXIMC_M9_READ_QOS = crate::Reg<aximc_m9_read_qos::AXIMC_M9_READ_QOSrs>;
#[doc = "AXIMC master 9 read priority register"]
pub mod aximc_m9_read_qos;
#[doc = "AXIMC_M9_WRITE_QOS (rw) register accessor: AXIMC master 9 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m9_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m9_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m9_write_qos`]
module"]
pub type AXIMC_M9_WRITE_QOS = crate::Reg<aximc_m9_write_qos::AXIMC_M9_WRITE_QOSrs>;
#[doc = "AXIMC master 9 write priority register"]
pub mod aximc_m9_write_qos;
#[doc = "AXIMC_M9_FN_MOD (rw) register accessor: AXIMC master 9 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m9_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m9_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m9_fn_mod`]
module"]
pub type AXIMC_M9_FN_MOD = crate::Reg<aximc_m9_fn_mod::AXIMC_M9_FN_MODrs>;
#[doc = "AXIMC master 9 issuing capability override functionality register"]
pub mod aximc_m9_fn_mod;
#[doc = "AXIMC_M10_READ_QOS (rw) register accessor: AXIMC master 10 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m10_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m10_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m10_read_qos`]
module"]
pub type AXIMC_M10_READ_QOS = crate::Reg<aximc_m10_read_qos::AXIMC_M10_READ_QOSrs>;
#[doc = "AXIMC master 10 read priority register"]
pub mod aximc_m10_read_qos;
#[doc = "AXIMC_M10_WRITE_QOS (rw) register accessor: AXIMC master 10 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m10_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m10_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m10_write_qos`]
module"]
pub type AXIMC_M10_WRITE_QOS = crate::Reg<aximc_m10_write_qos::AXIMC_M10_WRITE_QOSrs>;
#[doc = "AXIMC master 10 write priority register"]
pub mod aximc_m10_write_qos;
#[doc = "AXIMC_M10_FN_MOD (rw) register accessor: AXIMC master 10 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m10_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m10_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m10_fn_mod`]
module"]
pub type AXIMC_M10_FN_MOD = crate::Reg<aximc_m10_fn_mod::AXIMC_M10_FN_MODrs>;
#[doc = "AXIMC master 10 issuing capability override functionality register"]
pub mod aximc_m10_fn_mod;
#[doc = "AXIMC_M6_FN_MOD2 (rw) register accessor: AXIMC master 6 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m6_fn_mod2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m6_fn_mod2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m6_fn_mod2`]
module"]
pub type AXIMC_M6_FN_MOD2 = crate::Reg<aximc_m6_fn_mod2::AXIMC_M6_FN_MOD2rs>;
#[doc = "AXIMC master 6 packing functionality register"]
pub mod aximc_m6_fn_mod2;
#[doc = "AXIMC_M6_READ_QOS (rw) register accessor: AXIMC master 6 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m6_read_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m6_read_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m6_read_qos`]
module"]
pub type AXIMC_M6_READ_QOS = crate::Reg<aximc_m6_read_qos::AXIMC_M6_READ_QOSrs>;
#[doc = "AXIMC master 6 read priority register"]
pub mod aximc_m6_read_qos;
#[doc = "AXIMC_M6_WRITE_QOS (rw) register accessor: AXIMC master 6 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m6_write_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m6_write_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m6_write_qos`]
module"]
pub type AXIMC_M6_WRITE_QOS = crate::Reg<aximc_m6_write_qos::AXIMC_M6_WRITE_QOSrs>;
#[doc = "AXIMC master 6 write priority register"]
pub mod aximc_m6_write_qos;
#[doc = "AXIMC_M6_FN_MOD (rw) register accessor: AXIMC master 6 issuing capability override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m6_fn_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m6_fn_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m6_fn_mod`]
module"]
pub type AXIMC_M6_FN_MOD = crate::Reg<aximc_m6_fn_mod::AXIMC_M6_FN_MODrs>;
#[doc = "AXIMC master 6 issuing capability override functionality register"]
pub mod aximc_m6_fn_mod;
#[doc = "AXIMC_PERIPH_ID_4 (r) register accessor: AXIMC peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_4`]
module"]
pub type AXIMC_PERIPH_ID_4 = crate::Reg<aximc_periph_id_4::AXIMC_PERIPH_ID_4rs>;
#[doc = "AXIMC peripheral ID4 register"]
pub mod aximc_periph_id_4;
#[doc = "AXIMC_PERIPH_ID_5 (r) register accessor: AXIMC peripheral ID5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_5`]
module"]
pub type AXIMC_PERIPH_ID_5 = crate::Reg<aximc_periph_id_5::AXIMC_PERIPH_ID_5rs>;
#[doc = "AXIMC peripheral ID5 register"]
pub mod aximc_periph_id_5;
#[doc = "AXIMC_PERIPH_ID_6 (r) register accessor: AXIMC peripheral ID6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_6`]
module"]
pub type AXIMC_PERIPH_ID_6 = crate::Reg<aximc_periph_id_6::AXIMC_PERIPH_ID_6rs>;
#[doc = "AXIMC peripheral ID6 register"]
pub mod aximc_periph_id_6;
#[doc = "AXIMC_PERIPH_ID_7 (r) register accessor: AXIMC peripheral ID7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_7`]
module"]
pub type AXIMC_PERIPH_ID_7 = crate::Reg<aximc_periph_id_7::AXIMC_PERIPH_ID_7rs>;
#[doc = "AXIMC peripheral ID7 register"]
pub mod aximc_periph_id_7;
#[doc = "AXIMC_PERIPH_ID_0 (r) register accessor: AXIMC peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_0`]
module"]
pub type AXIMC_PERIPH_ID_0 = crate::Reg<aximc_periph_id_0::AXIMC_PERIPH_ID_0rs>;
#[doc = "AXIMC peripheral ID0 register"]
pub mod aximc_periph_id_0;
#[doc = "AXIMC_PERIPH_ID_1 (r) register accessor: AXIMC peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_1`]
module"]
pub type AXIMC_PERIPH_ID_1 = crate::Reg<aximc_periph_id_1::AXIMC_PERIPH_ID_1rs>;
#[doc = "AXIMC peripheral ID1 register"]
pub mod aximc_periph_id_1;
#[doc = "AXIMC_PERIPH_ID_2 (r) register accessor: AXIMC peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_2`]
module"]
pub type AXIMC_PERIPH_ID_2 = crate::Reg<aximc_periph_id_2::AXIMC_PERIPH_ID_2rs>;
#[doc = "AXIMC peripheral ID2 register"]
pub mod aximc_periph_id_2;
#[doc = "AXIMC_PERIPH_ID_3 (r) register accessor: AXIMC peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_periph_id_3`]
module"]
pub type AXIMC_PERIPH_ID_3 = crate::Reg<aximc_periph_id_3::AXIMC_PERIPH_ID_3rs>;
#[doc = "AXIMC peripheral ID3 register"]
pub mod aximc_periph_id_3;
#[doc = "AXIMC_COMP_ID_0 (r) register accessor: AXIMC component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_comp_id_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_comp_id_0`]
module"]
pub type AXIMC_COMP_ID_0 = crate::Reg<aximc_comp_id_0::AXIMC_COMP_ID_0rs>;
#[doc = "AXIMC component ID0 register"]
pub mod aximc_comp_id_0;
#[doc = "AXIMC_COMP_ID_1 (r) register accessor: AXIMC component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_comp_id_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_comp_id_1`]
module"]
pub type AXIMC_COMP_ID_1 = crate::Reg<aximc_comp_id_1::AXIMC_COMP_ID_1rs>;
#[doc = "AXIMC component ID1 register"]
pub mod aximc_comp_id_1;
#[doc = "AXIMC_COMP_ID_2 (r) register accessor: AXIMC component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_comp_id_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_comp_id_2`]
module"]
pub type AXIMC_COMP_ID_2 = crate::Reg<aximc_comp_id_2::AXIMC_COMP_ID_2rs>;
#[doc = "AXIMC component ID2 register"]
pub mod aximc_comp_id_2;
#[doc = "AXIMC_COMP_ID_3 (r) register accessor: AXIMC component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_comp_id_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_comp_id_3`]
module"]
pub type AXIMC_COMP_ID_3 = crate::Reg<aximc_comp_id_3::AXIMC_COMP_ID_3rs>;
#[doc = "AXIMC component ID3 register"]
pub mod aximc_comp_id_3;
#[doc = "AXIMC_M0_FN_MOD_AHB (rw) register accessor: AXIMC master 0 AHB conversion override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m0_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m0_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m0_fn_mod_ahb`]
module"]
pub type AXIMC_M0_FN_MOD_AHB = crate::Reg<aximc_m0_fn_mod_ahb::AXIMC_M0_FN_MOD_AHBrs>;
#[doc = "AXIMC master 0 AHB conversion override functionality register"]
pub mod aximc_m0_fn_mod_ahb;
#[doc = "AXIMC_M1_FN_MOD_AHB (rw) register accessor: AXIMC master 1 AHB conversion override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m1_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m1_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m1_fn_mod_ahb`]
module"]
pub type AXIMC_M1_FN_MOD_AHB = crate::Reg<aximc_m1_fn_mod_ahb::AXIMC_M1_FN_MOD_AHBrs>;
#[doc = "AXIMC master 1 AHB conversion override functionality register"]
pub mod aximc_m1_fn_mod_ahb;
#[doc = "AXIMC_M2_FN_MOD_AHB (rw) register accessor: AXIMC master 2 AHB conversion override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m2_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m2_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m2_fn_mod_ahb`]
module"]
pub type AXIMC_M2_FN_MOD_AHB = crate::Reg<aximc_m2_fn_mod_ahb::AXIMC_M2_FN_MOD_AHBrs>;
#[doc = "AXIMC master 2 AHB conversion override functionality register"]
pub mod aximc_m2_fn_mod_ahb;
#[doc = "AXIMC_M5_FN_MOD_AHB (rw) register accessor: AXIMC master 5 AHB conversion override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m5_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m5_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m5_fn_mod_ahb`]
module"]
pub type AXIMC_M5_FN_MOD_AHB = crate::Reg<aximc_m5_fn_mod_ahb::AXIMC_M5_FN_MOD_AHBrs>;
#[doc = "AXIMC master 5 AHB conversion override functionality register"]
pub mod aximc_m5_fn_mod_ahb;
#[doc = "AXIMC_M6_FN_MOD_AHB (rw) register accessor: AXIMC master 6 AHB conversion override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m6_fn_mod_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m6_fn_mod_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_m6_fn_mod_ahb`]
module"]
pub type AXIMC_M6_FN_MOD_AHB = crate::Reg<aximc_m6_fn_mod_ahb::AXIMC_M6_FN_MOD_AHBrs>;
#[doc = "AXIMC master 6 AHB conversion override functionality register"]
pub mod aximc_m6_fn_mod_ahb;
#[doc = "AXIMC_FN_MOD_LB (rw) register accessor: AXIMC long burst capability inhibition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_fn_mod_lb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_fn_mod_lb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aximc_fn_mod_lb`]
module"]
pub type AXIMC_FN_MOD_LB = crate::Reg<aximc_fn_mod_lb::AXIMC_FN_MOD_LBrs>;
#[doc = "AXIMC long burst capability inhibition register"]
pub mod aximc_fn_mod_lb;
