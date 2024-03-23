#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lpdma_seccfgr: LPDMA_SECCFGR,
    lpdma_privcfgr: LPDMA_PRIVCFGR,
    lpdma_rcfglockr: LPDMA_RCFGLOCKR,
    lpdma_misr: LPDMA_MISR,
    lpdma_smisr: LPDMA_SMISR,
    _reserved5: [u8; 0x3c],
    lpdma_c0lbar: LPDMA_C0LBAR,
    _reserved6: [u8; 0x08],
    lpdma_c0fcr: LPDMA_C0FCR,
    lpdma_c0sr: LPDMA_C0SR,
    lpdma_c0cr: LPDMA_C0CR,
    _reserved9: [u8; 0x28],
    lpdma_c0tr1: LPDMA_C0TR1,
    lpdma_c0tr2: LPDMA_C0TR2,
    lpdma_c0br1: LPDMA_C0BR1,
    lpdma_c0sar: LPDMA_C0SAR,
    lpdma_c0dar: LPDMA_C0DAR,
    _reserved14: [u8; 0x28],
    lpdma_c0llr: LPDMA_C0LLR,
    lpdma_c1lbar: LPDMA_C1LBAR,
    _reserved16: [u8; 0x08],
    lpdma_c1fcr: LPDMA_C1FCR,
    lpdma_c1sr: LPDMA_C1SR,
    lpdma_c1cr: LPDMA_C1CR,
    _reserved19: [u8; 0x28],
    lpdma_c1tr1: LPDMA_C1TR1,
    lpdma_c1tr2: LPDMA_C1TR2,
    lpdma_c1br1: LPDMA_C1BR1,
    lpdma_c1sar: LPDMA_C1SAR,
    lpdma_c1dar: LPDMA_C1DAR,
    _reserved24: [u8; 0x28],
    lpdma_c1llr: LPDMA_C1LLR,
    lpdma_c2lbar: LPDMA_C2LBAR,
    _reserved26: [u8; 0x08],
    lpdma_c2fcr: LPDMA_C2FCR,
    lpdma_c2sr: LPDMA_C2SR,
    lpdma_c2cr: LPDMA_C2CR,
    _reserved29: [u8; 0x28],
    lpdma_c2tr1: LPDMA_C2TR1,
    lpdma_c2tr2: LPDMA_C2TR2,
    lpdma_c2br1: LPDMA_C2BR1,
    lpdma_c2sar: LPDMA_C2SAR,
    lpdma_c2dar: LPDMA_C2DAR,
    _reserved34: [u8; 0x28],
    lpdma_c2llr: LPDMA_C2LLR,
    lpdma_c3lbar: LPDMA_C3LBAR,
    _reserved36: [u8; 0x08],
    lpdma_c3fcr: LPDMA_C3FCR,
    lpdma_c3sr: LPDMA_C3SR,
    lpdma_c3cr: LPDMA_C3CR,
    _reserved39: [u8; 0x28],
    lpdma_c3tr1: LPDMA_C3TR1,
    lpdma_c3tr2: LPDMA_C3TR2,
    lpdma_c3br1: LPDMA_C3BR1,
    lpdma_c3sar: LPDMA_C3SAR,
    lpdma_c3dar: LPDMA_C3DAR,
    _reserved44: [u8; 0x28],
    lpdma_c3llr: LPDMA_C3LLR,
}
impl RegisterBlock {
    #[doc = "0x00 - LPDMA secure configuration register"]
    #[inline(always)]
    pub const fn lpdma_seccfgr(&self) -> &LPDMA_SECCFGR {
        &self.lpdma_seccfgr
    }
    #[doc = "0x04 - LPDMA privileged configuration register"]
    #[inline(always)]
    pub const fn lpdma_privcfgr(&self) -> &LPDMA_PRIVCFGR {
        &self.lpdma_privcfgr
    }
    #[doc = "0x08 - LPDMA configuration lock register"]
    #[inline(always)]
    pub const fn lpdma_rcfglockr(&self) -> &LPDMA_RCFGLOCKR {
        &self.lpdma_rcfglockr
    }
    #[doc = "0x0c - LPDMA non-secure masked interrupt status register"]
    #[inline(always)]
    pub const fn lpdma_misr(&self) -> &LPDMA_MISR {
        &self.lpdma_misr
    }
    #[doc = "0x10 - LPDMA secure masked interrupt status register"]
    #[inline(always)]
    pub const fn lpdma_smisr(&self) -> &LPDMA_SMISR {
        &self.lpdma_smisr
    }
    #[doc = "0x50 - LPDMA channel 0 linked-list base address register"]
    #[inline(always)]
    pub const fn lpdma_c0lbar(&self) -> &LPDMA_C0LBAR {
        &self.lpdma_c0lbar
    }
    #[doc = "0x5c - LPDMA channel 0 flag clear register"]
    #[inline(always)]
    pub const fn lpdma_c0fcr(&self) -> &LPDMA_C0FCR {
        &self.lpdma_c0fcr
    }
    #[doc = "0x60 - LPDMA channel 0 status register"]
    #[inline(always)]
    pub const fn lpdma_c0sr(&self) -> &LPDMA_C0SR {
        &self.lpdma_c0sr
    }
    #[doc = "0x64 - LPDMA channel 0 control register"]
    #[inline(always)]
    pub const fn lpdma_c0cr(&self) -> &LPDMA_C0CR {
        &self.lpdma_c0cr
    }
    #[doc = "0x90 - LPDMA channel 0 transfer register 1"]
    #[inline(always)]
    pub const fn lpdma_c0tr1(&self) -> &LPDMA_C0TR1 {
        &self.lpdma_c0tr1
    }
    #[doc = "0x94 - LPDMA channel 0 transfer register 2"]
    #[inline(always)]
    pub const fn lpdma_c0tr2(&self) -> &LPDMA_C0TR2 {
        &self.lpdma_c0tr2
    }
    #[doc = "0x98 - LPDMA channel 0 block register 1"]
    #[inline(always)]
    pub const fn lpdma_c0br1(&self) -> &LPDMA_C0BR1 {
        &self.lpdma_c0br1
    }
    #[doc = "0x9c - LPDMA channel 0 source address register"]
    #[inline(always)]
    pub const fn lpdma_c0sar(&self) -> &LPDMA_C0SAR {
        &self.lpdma_c0sar
    }
    #[doc = "0xa0 - LPDMA channel 0 destination address register"]
    #[inline(always)]
    pub const fn lpdma_c0dar(&self) -> &LPDMA_C0DAR {
        &self.lpdma_c0dar
    }
    #[doc = "0xcc - LPDMA channel 0 linked-list address register"]
    #[inline(always)]
    pub const fn lpdma_c0llr(&self) -> &LPDMA_C0LLR {
        &self.lpdma_c0llr
    }
    #[doc = "0xd0 - LPDMA channel 1 linked-list base address register"]
    #[inline(always)]
    pub const fn lpdma_c1lbar(&self) -> &LPDMA_C1LBAR {
        &self.lpdma_c1lbar
    }
    #[doc = "0xdc - LPDMA channel 1 flag clear register"]
    #[inline(always)]
    pub const fn lpdma_c1fcr(&self) -> &LPDMA_C1FCR {
        &self.lpdma_c1fcr
    }
    #[doc = "0xe0 - LPDMA channel 1 status register"]
    #[inline(always)]
    pub const fn lpdma_c1sr(&self) -> &LPDMA_C1SR {
        &self.lpdma_c1sr
    }
    #[doc = "0xe4 - LPDMA channel 1 control register"]
    #[inline(always)]
    pub const fn lpdma_c1cr(&self) -> &LPDMA_C1CR {
        &self.lpdma_c1cr
    }
    #[doc = "0x110 - LPDMA channel 1 transfer register 1"]
    #[inline(always)]
    pub const fn lpdma_c1tr1(&self) -> &LPDMA_C1TR1 {
        &self.lpdma_c1tr1
    }
    #[doc = "0x114 - LPDMA channel 1 transfer register 2"]
    #[inline(always)]
    pub const fn lpdma_c1tr2(&self) -> &LPDMA_C1TR2 {
        &self.lpdma_c1tr2
    }
    #[doc = "0x118 - LPDMA channel 1 block register 1"]
    #[inline(always)]
    pub const fn lpdma_c1br1(&self) -> &LPDMA_C1BR1 {
        &self.lpdma_c1br1
    }
    #[doc = "0x11c - LPDMA channel 1 source address register"]
    #[inline(always)]
    pub const fn lpdma_c1sar(&self) -> &LPDMA_C1SAR {
        &self.lpdma_c1sar
    }
    #[doc = "0x120 - LPDMA channel 1 destination address register"]
    #[inline(always)]
    pub const fn lpdma_c1dar(&self) -> &LPDMA_C1DAR {
        &self.lpdma_c1dar
    }
    #[doc = "0x14c - LPDMA channel 1 linked-list address register"]
    #[inline(always)]
    pub const fn lpdma_c1llr(&self) -> &LPDMA_C1LLR {
        &self.lpdma_c1llr
    }
    #[doc = "0x150 - LPDMA channel 2 linked-list base address register"]
    #[inline(always)]
    pub const fn lpdma_c2lbar(&self) -> &LPDMA_C2LBAR {
        &self.lpdma_c2lbar
    }
    #[doc = "0x15c - LPDMA channel 2 flag clear register"]
    #[inline(always)]
    pub const fn lpdma_c2fcr(&self) -> &LPDMA_C2FCR {
        &self.lpdma_c2fcr
    }
    #[doc = "0x160 - LPDMA channel 2 status register"]
    #[inline(always)]
    pub const fn lpdma_c2sr(&self) -> &LPDMA_C2SR {
        &self.lpdma_c2sr
    }
    #[doc = "0x164 - LPDMA channel 2 control register"]
    #[inline(always)]
    pub const fn lpdma_c2cr(&self) -> &LPDMA_C2CR {
        &self.lpdma_c2cr
    }
    #[doc = "0x190 - LPDMA channel 2 transfer register 1"]
    #[inline(always)]
    pub const fn lpdma_c2tr1(&self) -> &LPDMA_C2TR1 {
        &self.lpdma_c2tr1
    }
    #[doc = "0x194 - LPDMA channel 2 transfer register 2"]
    #[inline(always)]
    pub const fn lpdma_c2tr2(&self) -> &LPDMA_C2TR2 {
        &self.lpdma_c2tr2
    }
    #[doc = "0x198 - LPDMA channel 2 block register 1"]
    #[inline(always)]
    pub const fn lpdma_c2br1(&self) -> &LPDMA_C2BR1 {
        &self.lpdma_c2br1
    }
    #[doc = "0x19c - LPDMA channel 2 source address register"]
    #[inline(always)]
    pub const fn lpdma_c2sar(&self) -> &LPDMA_C2SAR {
        &self.lpdma_c2sar
    }
    #[doc = "0x1a0 - LPDMA channel 2 destination address register"]
    #[inline(always)]
    pub const fn lpdma_c2dar(&self) -> &LPDMA_C2DAR {
        &self.lpdma_c2dar
    }
    #[doc = "0x1cc - LPDMA channel 2 linked-list address register"]
    #[inline(always)]
    pub const fn lpdma_c2llr(&self) -> &LPDMA_C2LLR {
        &self.lpdma_c2llr
    }
    #[doc = "0x1d0 - LPDMA channel 3 linked-list base address register"]
    #[inline(always)]
    pub const fn lpdma_c3lbar(&self) -> &LPDMA_C3LBAR {
        &self.lpdma_c3lbar
    }
    #[doc = "0x1dc - LPDMA channel 3 flag clear register"]
    #[inline(always)]
    pub const fn lpdma_c3fcr(&self) -> &LPDMA_C3FCR {
        &self.lpdma_c3fcr
    }
    #[doc = "0x1e0 - LPDMA channel 3 status register"]
    #[inline(always)]
    pub const fn lpdma_c3sr(&self) -> &LPDMA_C3SR {
        &self.lpdma_c3sr
    }
    #[doc = "0x1e4 - LPDMA channel 3 control register"]
    #[inline(always)]
    pub const fn lpdma_c3cr(&self) -> &LPDMA_C3CR {
        &self.lpdma_c3cr
    }
    #[doc = "0x210 - LPDMA channel 3 transfer register 1"]
    #[inline(always)]
    pub const fn lpdma_c3tr1(&self) -> &LPDMA_C3TR1 {
        &self.lpdma_c3tr1
    }
    #[doc = "0x214 - LPDMA channel 3 transfer register 2"]
    #[inline(always)]
    pub const fn lpdma_c3tr2(&self) -> &LPDMA_C3TR2 {
        &self.lpdma_c3tr2
    }
    #[doc = "0x218 - LPDMA channel 3 block register 1"]
    #[inline(always)]
    pub const fn lpdma_c3br1(&self) -> &LPDMA_C3BR1 {
        &self.lpdma_c3br1
    }
    #[doc = "0x21c - LPDMA channel 3 source address register"]
    #[inline(always)]
    pub const fn lpdma_c3sar(&self) -> &LPDMA_C3SAR {
        &self.lpdma_c3sar
    }
    #[doc = "0x220 - LPDMA channel 3 destination address register"]
    #[inline(always)]
    pub const fn lpdma_c3dar(&self) -> &LPDMA_C3DAR {
        &self.lpdma_c3dar
    }
    #[doc = "0x24c - LPDMA channel 3 linked-list address register"]
    #[inline(always)]
    pub const fn lpdma_c3llr(&self) -> &LPDMA_C3LLR {
        &self.lpdma_c3llr
    }
}
#[doc = "LPDMA_SECCFGR (rw) register accessor: LPDMA secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_seccfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_seccfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_seccfgr`]
module"]
pub type LPDMA_SECCFGR = crate::Reg<lpdma_seccfgr::LPDMA_SECCFGRrs>;
#[doc = "LPDMA secure configuration register"]
pub mod lpdma_seccfgr;
#[doc = "LPDMA_PRIVCFGR (rw) register accessor: LPDMA privileged configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_privcfgr`]
module"]
pub type LPDMA_PRIVCFGR = crate::Reg<lpdma_privcfgr::LPDMA_PRIVCFGRrs>;
#[doc = "LPDMA privileged configuration register"]
pub mod lpdma_privcfgr;
#[doc = "LPDMA_RCFGLOCKR (rw) register accessor: LPDMA configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_rcfglockr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_rcfglockr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_rcfglockr`]
module"]
pub type LPDMA_RCFGLOCKR = crate::Reg<lpdma_rcfglockr::LPDMA_RCFGLOCKRrs>;
#[doc = "LPDMA configuration lock register"]
pub mod lpdma_rcfglockr;
#[doc = "LPDMA_MISR (r) register accessor: LPDMA non-secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_misr`]
module"]
pub type LPDMA_MISR = crate::Reg<lpdma_misr::LPDMA_MISRrs>;
#[doc = "LPDMA non-secure masked interrupt status register"]
pub mod lpdma_misr;
#[doc = "LPDMA_SMISR (r) register accessor: LPDMA secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_smisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_smisr`]
module"]
pub type LPDMA_SMISR = crate::Reg<lpdma_smisr::LPDMA_SMISRrs>;
#[doc = "LPDMA secure masked interrupt status register"]
pub mod lpdma_smisr;
#[doc = "LPDMA_C0LBAR (rw) register accessor: LPDMA channel 0 linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0lbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0lbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0lbar`]
module"]
pub type LPDMA_C0LBAR = crate::Reg<lpdma_c0lbar::LPDMA_C0LBARrs>;
#[doc = "LPDMA channel 0 linked-list base address register"]
pub mod lpdma_c0lbar;
#[doc = "LPDMA_C0FCR (w) register accessor: LPDMA channel 0 flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0fcr`]
module"]
pub type LPDMA_C0FCR = crate::Reg<lpdma_c0fcr::LPDMA_C0FCRrs>;
#[doc = "LPDMA channel 0 flag clear register"]
pub mod lpdma_c0fcr;
#[doc = "LPDMA_C0SR (r) register accessor: LPDMA channel 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0sr`]
module"]
pub type LPDMA_C0SR = crate::Reg<lpdma_c0sr::LPDMA_C0SRrs>;
#[doc = "LPDMA channel 0 status register"]
pub mod lpdma_c0sr;
#[doc = "LPDMA_C0CR (rw) register accessor: LPDMA channel 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0cr`]
module"]
pub type LPDMA_C0CR = crate::Reg<lpdma_c0cr::LPDMA_C0CRrs>;
#[doc = "LPDMA channel 0 control register"]
pub mod lpdma_c0cr;
#[doc = "LPDMA_C0TR1 (rw) register accessor: LPDMA channel 0 transfer register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0tr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0tr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0tr1`]
module"]
pub type LPDMA_C0TR1 = crate::Reg<lpdma_c0tr1::LPDMA_C0TR1rs>;
#[doc = "LPDMA channel 0 transfer register 1"]
pub mod lpdma_c0tr1;
#[doc = "LPDMA_C0TR2 (rw) register accessor: LPDMA channel 0 transfer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0tr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0tr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0tr2`]
module"]
pub type LPDMA_C0TR2 = crate::Reg<lpdma_c0tr2::LPDMA_C0TR2rs>;
#[doc = "LPDMA channel 0 transfer register 2"]
pub mod lpdma_c0tr2;
#[doc = "LPDMA_C0BR1 (rw) register accessor: LPDMA channel 0 block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0br1`]
module"]
pub type LPDMA_C0BR1 = crate::Reg<lpdma_c0br1::LPDMA_C0BR1rs>;
#[doc = "LPDMA channel 0 block register 1"]
pub mod lpdma_c0br1;
#[doc = "LPDMA_C0SAR (rw) register accessor: LPDMA channel 0 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0sar`]
module"]
pub type LPDMA_C0SAR = crate::Reg<lpdma_c0sar::LPDMA_C0SARrs>;
#[doc = "LPDMA channel 0 source address register"]
pub mod lpdma_c0sar;
#[doc = "LPDMA_C0DAR (rw) register accessor: LPDMA channel 0 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0dar`]
module"]
pub type LPDMA_C0DAR = crate::Reg<lpdma_c0dar::LPDMA_C0DARrs>;
#[doc = "LPDMA channel 0 destination address register"]
pub mod lpdma_c0dar;
#[doc = "LPDMA_C0LLR (rw) register accessor: LPDMA channel 0 linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0llr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0llr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c0llr`]
module"]
pub type LPDMA_C0LLR = crate::Reg<lpdma_c0llr::LPDMA_C0LLRrs>;
#[doc = "LPDMA channel 0 linked-list address register"]
pub mod lpdma_c0llr;
#[doc = "LPDMA_C1LBAR (rw) register accessor: LPDMA channel 1 linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1lbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1lbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1lbar`]
module"]
pub type LPDMA_C1LBAR = crate::Reg<lpdma_c1lbar::LPDMA_C1LBARrs>;
#[doc = "LPDMA channel 1 linked-list base address register"]
pub mod lpdma_c1lbar;
#[doc = "LPDMA_C1FCR (w) register accessor: LPDMA channel 1 flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1fcr`]
module"]
pub type LPDMA_C1FCR = crate::Reg<lpdma_c1fcr::LPDMA_C1FCRrs>;
#[doc = "LPDMA channel 1 flag clear register"]
pub mod lpdma_c1fcr;
#[doc = "LPDMA_C1SR (r) register accessor: LPDMA channel 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1sr`]
module"]
pub type LPDMA_C1SR = crate::Reg<lpdma_c1sr::LPDMA_C1SRrs>;
#[doc = "LPDMA channel 1 status register"]
pub mod lpdma_c1sr;
#[doc = "LPDMA_C1CR (rw) register accessor: LPDMA channel 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1cr`]
module"]
pub type LPDMA_C1CR = crate::Reg<lpdma_c1cr::LPDMA_C1CRrs>;
#[doc = "LPDMA channel 1 control register"]
pub mod lpdma_c1cr;
#[doc = "LPDMA_C1TR1 (rw) register accessor: LPDMA channel 1 transfer register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1tr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1tr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1tr1`]
module"]
pub type LPDMA_C1TR1 = crate::Reg<lpdma_c1tr1::LPDMA_C1TR1rs>;
#[doc = "LPDMA channel 1 transfer register 1"]
pub mod lpdma_c1tr1;
#[doc = "LPDMA_C1TR2 (rw) register accessor: LPDMA channel 1 transfer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1tr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1tr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1tr2`]
module"]
pub type LPDMA_C1TR2 = crate::Reg<lpdma_c1tr2::LPDMA_C1TR2rs>;
#[doc = "LPDMA channel 1 transfer register 2"]
pub mod lpdma_c1tr2;
#[doc = "LPDMA_C1BR1 (rw) register accessor: LPDMA channel 1 block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1br1`]
module"]
pub type LPDMA_C1BR1 = crate::Reg<lpdma_c1br1::LPDMA_C1BR1rs>;
#[doc = "LPDMA channel 1 block register 1"]
pub mod lpdma_c1br1;
#[doc = "LPDMA_C1SAR (rw) register accessor: LPDMA channel 1 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1sar`]
module"]
pub type LPDMA_C1SAR = crate::Reg<lpdma_c1sar::LPDMA_C1SARrs>;
#[doc = "LPDMA channel 1 source address register"]
pub mod lpdma_c1sar;
#[doc = "LPDMA_C1DAR (rw) register accessor: LPDMA channel 1 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1dar`]
module"]
pub type LPDMA_C1DAR = crate::Reg<lpdma_c1dar::LPDMA_C1DARrs>;
#[doc = "LPDMA channel 1 destination address register"]
pub mod lpdma_c1dar;
#[doc = "LPDMA_C1LLR (rw) register accessor: LPDMA channel 1 linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1llr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1llr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c1llr`]
module"]
pub type LPDMA_C1LLR = crate::Reg<lpdma_c1llr::LPDMA_C1LLRrs>;
#[doc = "LPDMA channel 1 linked-list address register"]
pub mod lpdma_c1llr;
#[doc = "LPDMA_C2LBAR (rw) register accessor: LPDMA channel 2 linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2lbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2lbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2lbar`]
module"]
pub type LPDMA_C2LBAR = crate::Reg<lpdma_c2lbar::LPDMA_C2LBARrs>;
#[doc = "LPDMA channel 2 linked-list base address register"]
pub mod lpdma_c2lbar;
#[doc = "LPDMA_C2FCR (w) register accessor: LPDMA channel 2 flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2fcr`]
module"]
pub type LPDMA_C2FCR = crate::Reg<lpdma_c2fcr::LPDMA_C2FCRrs>;
#[doc = "LPDMA channel 2 flag clear register"]
pub mod lpdma_c2fcr;
#[doc = "LPDMA_C2SR (r) register accessor: LPDMA channel 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2sr`]
module"]
pub type LPDMA_C2SR = crate::Reg<lpdma_c2sr::LPDMA_C2SRrs>;
#[doc = "LPDMA channel 2 status register"]
pub mod lpdma_c2sr;
#[doc = "LPDMA_C2CR (rw) register accessor: LPDMA channel 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2cr`]
module"]
pub type LPDMA_C2CR = crate::Reg<lpdma_c2cr::LPDMA_C2CRrs>;
#[doc = "LPDMA channel 2 control register"]
pub mod lpdma_c2cr;
#[doc = "LPDMA_C2TR1 (rw) register accessor: LPDMA channel 2 transfer register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2tr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2tr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2tr1`]
module"]
pub type LPDMA_C2TR1 = crate::Reg<lpdma_c2tr1::LPDMA_C2TR1rs>;
#[doc = "LPDMA channel 2 transfer register 1"]
pub mod lpdma_c2tr1;
#[doc = "LPDMA_C2TR2 (rw) register accessor: LPDMA channel 2 transfer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2tr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2tr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2tr2`]
module"]
pub type LPDMA_C2TR2 = crate::Reg<lpdma_c2tr2::LPDMA_C2TR2rs>;
#[doc = "LPDMA channel 2 transfer register 2"]
pub mod lpdma_c2tr2;
#[doc = "LPDMA_C2BR1 (rw) register accessor: LPDMA channel 2 block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2br1`]
module"]
pub type LPDMA_C2BR1 = crate::Reg<lpdma_c2br1::LPDMA_C2BR1rs>;
#[doc = "LPDMA channel 2 block register 1"]
pub mod lpdma_c2br1;
#[doc = "LPDMA_C2SAR (rw) register accessor: LPDMA channel 2 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2sar`]
module"]
pub type LPDMA_C2SAR = crate::Reg<lpdma_c2sar::LPDMA_C2SARrs>;
#[doc = "LPDMA channel 2 source address register"]
pub mod lpdma_c2sar;
#[doc = "LPDMA_C2DAR (rw) register accessor: LPDMA channel 2 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2dar`]
module"]
pub type LPDMA_C2DAR = crate::Reg<lpdma_c2dar::LPDMA_C2DARrs>;
#[doc = "LPDMA channel 2 destination address register"]
pub mod lpdma_c2dar;
#[doc = "LPDMA_C2LLR (rw) register accessor: LPDMA channel 2 linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2llr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2llr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c2llr`]
module"]
pub type LPDMA_C2LLR = crate::Reg<lpdma_c2llr::LPDMA_C2LLRrs>;
#[doc = "LPDMA channel 2 linked-list address register"]
pub mod lpdma_c2llr;
#[doc = "LPDMA_C3LBAR (rw) register accessor: LPDMA channel 3 linked-list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3lbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3lbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3lbar`]
module"]
pub type LPDMA_C3LBAR = crate::Reg<lpdma_c3lbar::LPDMA_C3LBARrs>;
#[doc = "LPDMA channel 3 linked-list base address register"]
pub mod lpdma_c3lbar;
#[doc = "LPDMA_C3FCR (w) register accessor: LPDMA channel 3 flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3fcr`]
module"]
pub type LPDMA_C3FCR = crate::Reg<lpdma_c3fcr::LPDMA_C3FCRrs>;
#[doc = "LPDMA channel 3 flag clear register"]
pub mod lpdma_c3fcr;
#[doc = "LPDMA_C3SR (r) register accessor: LPDMA channel 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3sr`]
module"]
pub type LPDMA_C3SR = crate::Reg<lpdma_c3sr::LPDMA_C3SRrs>;
#[doc = "LPDMA channel 3 status register"]
pub mod lpdma_c3sr;
#[doc = "LPDMA_C3CR (rw) register accessor: LPDMA channel 3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3cr`]
module"]
pub type LPDMA_C3CR = crate::Reg<lpdma_c3cr::LPDMA_C3CRrs>;
#[doc = "LPDMA channel 3 control register"]
pub mod lpdma_c3cr;
#[doc = "LPDMA_C3TR1 (rw) register accessor: LPDMA channel 3 transfer register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3tr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3tr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3tr1`]
module"]
pub type LPDMA_C3TR1 = crate::Reg<lpdma_c3tr1::LPDMA_C3TR1rs>;
#[doc = "LPDMA channel 3 transfer register 1"]
pub mod lpdma_c3tr1;
#[doc = "LPDMA_C3TR2 (rw) register accessor: LPDMA channel 3 transfer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3tr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3tr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3tr2`]
module"]
pub type LPDMA_C3TR2 = crate::Reg<lpdma_c3tr2::LPDMA_C3TR2rs>;
#[doc = "LPDMA channel 3 transfer register 2"]
pub mod lpdma_c3tr2;
#[doc = "LPDMA_C3BR1 (rw) register accessor: LPDMA channel 3 block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3br1`]
module"]
pub type LPDMA_C3BR1 = crate::Reg<lpdma_c3br1::LPDMA_C3BR1rs>;
#[doc = "LPDMA channel 3 block register 1"]
pub mod lpdma_c3br1;
#[doc = "LPDMA_C3SAR (rw) register accessor: LPDMA channel 3 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3sar`]
module"]
pub type LPDMA_C3SAR = crate::Reg<lpdma_c3sar::LPDMA_C3SARrs>;
#[doc = "LPDMA channel 3 source address register"]
pub mod lpdma_c3sar;
#[doc = "LPDMA_C3DAR (rw) register accessor: LPDMA channel 3 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3dar`]
module"]
pub type LPDMA_C3DAR = crate::Reg<lpdma_c3dar::LPDMA_C3DARrs>;
#[doc = "LPDMA channel 3 destination address register"]
pub mod lpdma_c3dar;
#[doc = "LPDMA_C3LLR (rw) register accessor: LPDMA channel 3 linked-list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3llr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3llr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdma_c3llr`]
module"]
pub type LPDMA_C3LLR = crate::Reg<lpdma_c3llr::LPDMA_C3LLRrs>;
#[doc = "LPDMA channel 3 linked-list address register"]
pub mod lpdma_c3llr;
