#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gicv_ctlr: GICV_CTLR,
    gicv_pmr: GICV_PMR,
    gicv_bpr: GICV_BPR,
    gicv_iar: GICV_IAR,
    gicv_eoir: GICV_EOIR,
    gicv_rpr: GICV_RPR,
    gicv_hppir: GICV_HPPIR,
    gicv_abpr: GICV_ABPR,
    gicv_aiar: GICV_AIAR,
    gicv_aeoir: GICV_AEOIR,
    gicv_ahppir: GICV_AHPPIR,
    _reserved11: [u8; 0xa4],
    gicv_apr0: GICV_APR0,
    _reserved12: [u8; 0x28],
    gicv_iidr: GICV_IIDR,
    _reserved13: [u8; 0x0f00],
    gicv_dir: GICV_DIR,
}
impl RegisterBlock {
    #[doc = "0x00 - GICV virtual machine control register"]
    #[inline(always)]
    pub const fn gicv_ctlr(&self) -> &GICV_CTLR {
        &self.gicv_ctlr
    }
    #[doc = "0x04 - GICV VM priority mask register"]
    #[inline(always)]
    pub const fn gicv_pmr(&self) -> &GICV_PMR {
        &self.gicv_pmr
    }
    #[doc = "0x08 - GICV VM binary point register"]
    #[inline(always)]
    pub const fn gicv_bpr(&self) -> &GICV_BPR {
        &self.gicv_bpr
    }
    #[doc = "0x0c - GICV VM interrupt acknowledge register"]
    #[inline(always)]
    pub const fn gicv_iar(&self) -> &GICV_IAR {
        &self.gicv_iar
    }
    #[doc = "0x10 - GICV VM end of interrupt register"]
    #[inline(always)]
    pub const fn gicv_eoir(&self) -> &GICV_EOIR {
        &self.gicv_eoir
    }
    #[doc = "0x14 - GICV VM running priority register"]
    #[inline(always)]
    pub const fn gicv_rpr(&self) -> &GICV_RPR {
        &self.gicv_rpr
    }
    #[doc = "0x18 - GICV VM highest priority pending interrupt register"]
    #[inline(always)]
    pub const fn gicv_hppir(&self) -> &GICV_HPPIR {
        &self.gicv_hppir
    }
    #[doc = "0x1c - GICV VM aliased binary point register"]
    #[inline(always)]
    pub const fn gicv_abpr(&self) -> &GICV_ABPR {
        &self.gicv_abpr
    }
    #[doc = "0x20 - GICV VM aliased interrupt register"]
    #[inline(always)]
    pub const fn gicv_aiar(&self) -> &GICV_AIAR {
        &self.gicv_aiar
    }
    #[doc = "0x24 - GICV VM aliased end of interrupt register"]
    #[inline(always)]
    pub const fn gicv_aeoir(&self) -> &GICV_AEOIR {
        &self.gicv_aeoir
    }
    #[doc = "0x28 - GICV VM aliased highest priority pending interrupt register"]
    #[inline(always)]
    pub const fn gicv_ahppir(&self) -> &GICV_AHPPIR {
        &self.gicv_ahppir
    }
    #[doc = "0xd0 - The GICV_APR0 is an alias of GICH_APR."]
    #[inline(always)]
    pub const fn gicv_apr0(&self) -> &GICV_APR0 {
        &self.gicv_apr0
    }
    #[doc = "0xfc - The GICV_IIDR is an alias of GICC_IIDR."]
    #[inline(always)]
    pub const fn gicv_iidr(&self) -> &GICV_IIDR {
        &self.gicv_iidr
    }
    #[doc = "0x1000 - GICV VM deactivate interrupt register"]
    #[inline(always)]
    pub const fn gicv_dir(&self) -> &GICV_DIR {
        &self.gicv_dir
    }
}
#[doc = "GICV_CTLR (rw) register accessor: GICV virtual machine control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_ctlr`]
module"]
pub type GICV_CTLR = crate::Reg<gicv_ctlr::GICV_CTLRrs>;
#[doc = "GICV virtual machine control register"]
pub mod gicv_ctlr;
#[doc = "GICV_PMR (rw) register accessor: GICV VM priority mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_pmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_pmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_pmr`]
module"]
pub type GICV_PMR = crate::Reg<gicv_pmr::GICV_PMRrs>;
#[doc = "GICV VM priority mask register"]
pub mod gicv_pmr;
#[doc = "GICV_BPR (rw) register accessor: GICV VM binary point register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_bpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_bpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_bpr`]
module"]
pub type GICV_BPR = crate::Reg<gicv_bpr::GICV_BPRrs>;
#[doc = "GICV VM binary point register"]
pub mod gicv_bpr;
#[doc = "GICV_IAR (r) register accessor: GICV VM interrupt acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_iar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_iar`]
module"]
pub type GICV_IAR = crate::Reg<gicv_iar::GICV_IARrs>;
#[doc = "GICV VM interrupt acknowledge register"]
pub mod gicv_iar;
#[doc = "GICV_EOIR (w) register accessor: GICV VM end of interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_eoir`]
module"]
pub type GICV_EOIR = crate::Reg<gicv_eoir::GICV_EOIRrs>;
#[doc = "GICV VM end of interrupt register"]
pub mod gicv_eoir;
#[doc = "GICV_RPR (r) register accessor: GICV VM running priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_rpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_rpr`]
module"]
pub type GICV_RPR = crate::Reg<gicv_rpr::GICV_RPRrs>;
#[doc = "GICV VM running priority register"]
pub mod gicv_rpr;
#[doc = "GICV_HPPIR (r) register accessor: GICV VM highest priority pending interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_hppir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_hppir`]
module"]
pub type GICV_HPPIR = crate::Reg<gicv_hppir::GICV_HPPIRrs>;
#[doc = "GICV VM highest priority pending interrupt register"]
pub mod gicv_hppir;
#[doc = "GICV_ABPR (rw) register accessor: GICV VM aliased binary point register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_abpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_abpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_abpr`]
module"]
pub type GICV_ABPR = crate::Reg<gicv_abpr::GICV_ABPRrs>;
#[doc = "GICV VM aliased binary point register"]
pub mod gicv_abpr;
#[doc = "GICV_AIAR (r) register accessor: GICV VM aliased interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_aiar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_aiar`]
module"]
pub type GICV_AIAR = crate::Reg<gicv_aiar::GICV_AIARrs>;
#[doc = "GICV VM aliased interrupt register"]
pub mod gicv_aiar;
#[doc = "GICV_AEOIR (w) register accessor: GICV VM aliased end of interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_aeoir`]
module"]
pub type GICV_AEOIR = crate::Reg<gicv_aeoir::GICV_AEOIRrs>;
#[doc = "GICV VM aliased end of interrupt register"]
pub mod gicv_aeoir;
#[doc = "GICV_AHPPIR (r) register accessor: GICV VM aliased highest priority pending interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_ahppir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_ahppir`]
module"]
pub type GICV_AHPPIR = crate::Reg<gicv_ahppir::GICV_AHPPIRrs>;
#[doc = "GICV VM aliased highest priority pending interrupt register"]
pub mod gicv_ahppir;
#[doc = "GICV_APR0 (rw) register accessor: The GICV_APR0 is an alias of GICH_APR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_apr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_apr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_apr0`]
module"]
pub type GICV_APR0 = crate::Reg<gicv_apr0::GICV_APR0rs>;
#[doc = "The GICV_APR0 is an alias of GICH_APR."]
pub mod gicv_apr0;
#[doc = "GICV_IIDR (r) register accessor: The GICV_IIDR is an alias of GICC_IIDR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_iidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_iidr`]
module"]
pub type GICV_IIDR = crate::Reg<gicv_iidr::GICV_IIDRrs>;
#[doc = "The GICV_IIDR is an alias of GICC_IIDR."]
pub mod gicv_iidr;
#[doc = "GICV_DIR (w) register accessor: GICV VM deactivate interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicv_dir`]
module"]
pub type GICV_DIR = crate::Reg<gicv_dir::GICV_DIRrs>;
#[doc = "GICV VM deactivate interrupt register"]
pub mod gicv_dir;
