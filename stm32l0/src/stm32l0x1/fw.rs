#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cssa: CSSA,
    csl: CSL,
    nvdssa: NVDSSA,
    nvdsl: NVDSL,
    vdssa: VDSSA,
    vdsl: VDSL,
    _reserved6: [u8; 0x08],
    cr: CR,
}
impl RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    #[inline(always)]
    pub const fn cssa(&self) -> &CSSA {
        &self.cssa
    }
    #[doc = "0x04 - Code segment length"]
    #[inline(always)]
    pub const fn csl(&self) -> &CSL {
        &self.csl
    }
    #[doc = "0x08 - Non-volatile data segment start address"]
    #[inline(always)]
    pub const fn nvdssa(&self) -> &NVDSSA {
        &self.nvdssa
    }
    #[doc = "0x0c - Non-volatile data segment length"]
    #[inline(always)]
    pub const fn nvdsl(&self) -> &NVDSL {
        &self.nvdsl
    }
    #[doc = "0x10 - Volatile data segment start address"]
    #[inline(always)]
    pub const fn vdssa(&self) -> &VDSSA {
        &self.vdssa
    }
    #[doc = "0x14 - Volatile data segment length"]
    #[inline(always)]
    pub const fn vdsl(&self) -> &VDSL {
        &self.vdsl
    }
    #[doc = "0x20 - Configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
}
#[doc = "CSSA (rw) register accessor: Code segment start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cssa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cssa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cssa`]
module"]
pub type CSSA = crate::Reg<cssa::CSSArs>;
#[doc = "Code segment start address"]
pub mod cssa;
#[doc = "CSL (rw) register accessor: Code segment length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csl`]
module"]
pub type CSL = crate::Reg<csl::CSLrs>;
#[doc = "Code segment length"]
pub mod csl;
#[doc = "NVDSSA (rw) register accessor: Non-volatile data segment start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvdssa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvdssa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvdssa`]
module"]
pub type NVDSSA = crate::Reg<nvdssa::NVDSSArs>;
#[doc = "Non-volatile data segment start address"]
pub mod nvdssa;
#[doc = "NVDSL (rw) register accessor: Non-volatile data segment length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvdsl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvdsl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvdsl`]
module"]
pub type NVDSL = crate::Reg<nvdsl::NVDSLrs>;
#[doc = "Non-volatile data segment length"]
pub mod nvdsl;
#[doc = "VDSSA (rw) register accessor: Volatile data segment start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdssa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdssa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdssa`]
module"]
pub type VDSSA = crate::Reg<vdssa::VDSSArs>;
#[doc = "Volatile data segment start address"]
pub mod vdssa;
#[doc = "VDSL (rw) register accessor: Volatile data segment length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdsl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdsl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdsl`]
module"]
pub type VDSL = crate::Reg<vdsl::VDSLrs>;
#[doc = "Volatile data segment length"]
pub mod vdsl;
#[doc = "CR (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Configuration register"]
pub mod cr;
