#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    comp1_csr: COMP1_CSR,
    comp2_csr: COMP2_CSR,
    comp3_csr: COMP3_CSR,
    comp4_csr: COMP4_CSR,
    comp5_csr: COMP5_CSR,
    comp6_csr: COMP6_CSR,
    comp7_csr: COMP7_CSR,
}
impl RegisterBlock {
    #[doc = "0x1c - control and status register"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &COMP1_CSR {
        &self.comp1_csr
    }
    #[doc = "0x20 - control and status register"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &COMP2_CSR {
        &self.comp2_csr
    }
    #[doc = "0x24 - control and status register"]
    #[inline(always)]
    pub const fn comp3_csr(&self) -> &COMP3_CSR {
        &self.comp3_csr
    }
    #[doc = "0x28 - control and status register"]
    #[inline(always)]
    pub const fn comp4_csr(&self) -> &COMP4_CSR {
        &self.comp4_csr
    }
    #[doc = "0x2c - control and status register"]
    #[inline(always)]
    pub const fn comp5_csr(&self) -> &COMP5_CSR {
        &self.comp5_csr
    }
    #[doc = "0x30 - control and status register"]
    #[inline(always)]
    pub const fn comp6_csr(&self) -> &COMP6_CSR {
        &self.comp6_csr
    }
    #[doc = "0x34 - control and status register"]
    #[inline(always)]
    pub const fn comp7_csr(&self) -> &COMP7_CSR {
        &self.comp7_csr
    }
}
#[doc = "COMP2_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`]
module"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSRrs>;
#[doc = "control and status register"]
pub mod comp2_csr;
#[doc = "COMP4_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp4_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp4_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp4_csr`]
module"]
pub type COMP4_CSR = crate::Reg<comp4_csr::COMP4_CSRrs>;
#[doc = "control and status register"]
pub mod comp4_csr;
#[doc = "COMP6_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp6_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp6_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp6_csr`]
module"]
pub type COMP6_CSR = crate::Reg<comp6_csr::COMP6_CSRrs>;
#[doc = "control and status register"]
pub mod comp6_csr;
#[doc = "COMP3_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp3_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp3_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3_csr`]
module"]
pub type COMP3_CSR = crate::Reg<comp3_csr::COMP3_CSRrs>;
#[doc = "control and status register"]
pub mod comp3_csr;
#[doc = "COMP5_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp5_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp5_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp5_csr`]
module"]
pub type COMP5_CSR = crate::Reg<comp5_csr::COMP5_CSRrs>;
#[doc = "control and status register"]
pub mod comp5_csr;
#[doc = "COMP7_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp7_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp7_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp7_csr`]
module"]
pub type COMP7_CSR = crate::Reg<comp7_csr::COMP7_CSRrs>;
#[doc = "control and status register"]
pub mod comp7_csr;
#[doc = "COMP1_CSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`]
module"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSRrs>;
#[doc = "control and status register"]
pub mod comp1_csr;
