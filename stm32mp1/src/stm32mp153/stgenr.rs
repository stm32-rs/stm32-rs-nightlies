#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stgenr_cntcvl: STGENR_CNTCVL,
    stgenr_cntcvu: STGENR_CNTCVU,
    _reserved2: [u8; 0x0fc8],
    stgenr_pidr4: STGENR_PIDR4,
    stgenr_pidr5: STGENR_PIDR5,
    stgenr_pidr6: STGENR_PIDR6,
    stgenr_pidr7: STGENR_PIDR7,
    stgenr_pidr0: STGENR_PIDR0,
    stgenr_pidr1: STGENR_PIDR1,
    stgenr_pidr2: STGENR_PIDR2,
    stgenr_pidr3: STGENR_PIDR3,
    stgenr_cidr0: STGENR_CIDR0,
    stgenr_cidr1: STGENR_CIDR1,
    stgenr_cidr2: STGENR_CIDR2,
    stgenr_cidr3: STGENR_CIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    #[inline(always)]
    pub const fn stgenr_cntcvl(&self) -> &STGENR_CNTCVL {
        &self.stgenr_cntcvl
    }
    #[doc = "0x04 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
    #[inline(always)]
    pub const fn stgenr_cntcvu(&self) -> &STGENR_CNTCVU {
        &self.stgenr_cntcvu
    }
    #[doc = "0xfd0 - STGENR peripheral ID4 register"]
    #[inline(always)]
    pub const fn stgenr_pidr4(&self) -> &STGENR_PIDR4 {
        &self.stgenr_pidr4
    }
    #[doc = "0xfd4 - STGENR peripheral ID5 register"]
    #[inline(always)]
    pub const fn stgenr_pidr5(&self) -> &STGENR_PIDR5 {
        &self.stgenr_pidr5
    }
    #[doc = "0xfd8 - STGENR peripheral ID6 register"]
    #[inline(always)]
    pub const fn stgenr_pidr6(&self) -> &STGENR_PIDR6 {
        &self.stgenr_pidr6
    }
    #[doc = "0xfdc - STGENR peripheral ID7 register"]
    #[inline(always)]
    pub const fn stgenr_pidr7(&self) -> &STGENR_PIDR7 {
        &self.stgenr_pidr7
    }
    #[doc = "0xfe0 - STGENR peripheral ID0 register"]
    #[inline(always)]
    pub const fn stgenr_pidr0(&self) -> &STGENR_PIDR0 {
        &self.stgenr_pidr0
    }
    #[doc = "0xfe4 - STGENR peripheral ID1 register"]
    #[inline(always)]
    pub const fn stgenr_pidr1(&self) -> &STGENR_PIDR1 {
        &self.stgenr_pidr1
    }
    #[doc = "0xfe8 - STGENR peripheral ID2 register"]
    #[inline(always)]
    pub const fn stgenr_pidr2(&self) -> &STGENR_PIDR2 {
        &self.stgenr_pidr2
    }
    #[doc = "0xfec - STGENR peripheral ID3 register"]
    #[inline(always)]
    pub const fn stgenr_pidr3(&self) -> &STGENR_PIDR3 {
        &self.stgenr_pidr3
    }
    #[doc = "0xff0 - STGENR component ID0 register"]
    #[inline(always)]
    pub const fn stgenr_cidr0(&self) -> &STGENR_CIDR0 {
        &self.stgenr_cidr0
    }
    #[doc = "0xff4 - STGENR component ID1 register"]
    #[inline(always)]
    pub const fn stgenr_cidr1(&self) -> &STGENR_CIDR1 {
        &self.stgenr_cidr1
    }
    #[doc = "0xff8 - STGENR component ID2 register"]
    #[inline(always)]
    pub const fn stgenr_cidr2(&self) -> &STGENR_CIDR2 {
        &self.stgenr_cidr2
    }
    #[doc = "0xffc - STGENR component ID3 register"]
    #[inline(always)]
    pub const fn stgenr_cidr3(&self) -> &STGENR_CIDR3 {
        &self.stgenr_cidr3
    }
}
#[doc = "STGENR_CNTCVL (r) register accessor: the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cntcvl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_cntcvl`]
module"]
pub type STGENR_CNTCVL = crate::Reg<stgenr_cntcvl::STGENR_CNTCVLrs>;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenr_cntcvl;
#[doc = "STGENR_CNTCVU (r) register accessor: the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cntcvu::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_cntcvu`]
module"]
pub type STGENR_CNTCVU = crate::Reg<stgenr_cntcvu::STGENR_CNTCVUrs>;
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register."]
pub mod stgenr_cntcvu;
#[doc = "STGENR_PIDR4 (r) register accessor: STGENR peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr4`]
module"]
pub type STGENR_PIDR4 = crate::Reg<stgenr_pidr4::STGENR_PIDR4rs>;
#[doc = "STGENR peripheral ID4 register"]
pub mod stgenr_pidr4;
#[doc = "STGENR_PIDR5 (r) register accessor: STGENR peripheral ID5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr5`]
module"]
pub type STGENR_PIDR5 = crate::Reg<stgenr_pidr5::STGENR_PIDR5rs>;
#[doc = "STGENR peripheral ID5 register"]
pub mod stgenr_pidr5;
#[doc = "STGENR_PIDR6 (r) register accessor: STGENR peripheral ID6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr6`]
module"]
pub type STGENR_PIDR6 = crate::Reg<stgenr_pidr6::STGENR_PIDR6rs>;
#[doc = "STGENR peripheral ID6 register"]
pub mod stgenr_pidr6;
#[doc = "STGENR_PIDR7 (r) register accessor: STGENR peripheral ID7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr7`]
module"]
pub type STGENR_PIDR7 = crate::Reg<stgenr_pidr7::STGENR_PIDR7rs>;
#[doc = "STGENR peripheral ID7 register"]
pub mod stgenr_pidr7;
#[doc = "STGENR_PIDR0 (r) register accessor: STGENR peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr0`]
module"]
pub type STGENR_PIDR0 = crate::Reg<stgenr_pidr0::STGENR_PIDR0rs>;
#[doc = "STGENR peripheral ID0 register"]
pub mod stgenr_pidr0;
#[doc = "STGENR_PIDR1 (r) register accessor: STGENR peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr1`]
module"]
pub type STGENR_PIDR1 = crate::Reg<stgenr_pidr1::STGENR_PIDR1rs>;
#[doc = "STGENR peripheral ID1 register"]
pub mod stgenr_pidr1;
#[doc = "STGENR_PIDR2 (r) register accessor: STGENR peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr2`]
module"]
pub type STGENR_PIDR2 = crate::Reg<stgenr_pidr2::STGENR_PIDR2rs>;
#[doc = "STGENR peripheral ID2 register"]
pub mod stgenr_pidr2;
#[doc = "STGENR_PIDR3 (r) register accessor: STGENR peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_pidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_pidr3`]
module"]
pub type STGENR_PIDR3 = crate::Reg<stgenr_pidr3::STGENR_PIDR3rs>;
#[doc = "STGENR peripheral ID3 register"]
pub mod stgenr_pidr3;
#[doc = "STGENR_CIDR0 (r) register accessor: STGENR component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_cidr0`]
module"]
pub type STGENR_CIDR0 = crate::Reg<stgenr_cidr0::STGENR_CIDR0rs>;
#[doc = "STGENR component ID0 register"]
pub mod stgenr_cidr0;
#[doc = "STGENR_CIDR1 (r) register accessor: STGENR component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_cidr1`]
module"]
pub type STGENR_CIDR1 = crate::Reg<stgenr_cidr1::STGENR_CIDR1rs>;
#[doc = "STGENR component ID1 register"]
pub mod stgenr_cidr1;
#[doc = "STGENR_CIDR2 (r) register accessor: STGENR component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_cidr2`]
module"]
pub type STGENR_CIDR2 = crate::Reg<stgenr_cidr2::STGENR_CIDR2rs>;
#[doc = "STGENR component ID2 register"]
pub mod stgenr_cidr2;
#[doc = "STGENR_CIDR3 (r) register accessor: STGENR component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stgenr_cidr3`]
module"]
pub type STGENR_CIDR3 = crate::Reg<stgenr_cidr3::STGENR_CIDR3rs>;
#[doc = "STGENR component ID3 register"]
pub mod stgenr_cidr3;
