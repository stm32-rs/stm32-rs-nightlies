#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    memrmp: MEMRMP,
    cfgr1: CFGR1,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    scsr: SCSR,
    cfgr2: CFGR2,
    swpr: SWPR,
    skr: SKR,
    _reserved10: [u8; 0xd8],
    imr1: IMR1,
    imr2: IMR2,
    c2imr1: C2IMR1,
    c2imr2: C2IMR2,
    _reserved14: [u8; 0xf8],
    rfdcr: RFDCR,
}
impl RegisterBlock {
    #[doc = "0x00 - memory remap register"]
    #[inline(always)]
    pub const fn memrmp(&self) -> &MEMRMP {
        &self.memrmp
    }
    #[doc = "0x04 - configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    #[doc = "0x18 - SCSR"]
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        &self.scsr
    }
    #[doc = "0x1c - CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x20 - SWPR"]
    #[inline(always)]
    pub const fn swpr(&self) -> &SWPR {
        &self.swpr
    }
    #[doc = "0x24 - SKR"]
    #[inline(always)]
    pub const fn skr(&self) -> &SKR {
        &self.skr
    }
    #[doc = "0x100 - SYSCFG CPU1 interrupt mask register 1"]
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    #[doc = "0x104 - SYSCFG CPU1 interrupt mask register 2"]
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    #[doc = "0x108 - SYSCFG CPU2 interrupt mask register 1"]
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    #[doc = "0x10c - SYSCFG CPU2 interrupt mask register 2"]
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    #[doc = "0x208 - radio debug control register"]
    #[inline(always)]
    pub const fn rfdcr(&self) -> &RFDCR {
        &self.rfdcr
    }
}
#[doc = "MEMRMP (rw) register accessor: memory remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memrmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memrmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memrmp`]
module"]
pub type MEMRMP = crate::Reg<memrmp::MEMRMPrs>;
#[doc = "memory remap register"]
pub mod memrmp;
#[doc = "CFGR1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "EXTICR1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`]
module"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`]
module"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`]
module"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`]
module"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SCSR (rw) register accessor: SCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`]
module"]
pub type SCSR = crate::Reg<scsr::SCSRrs>;
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2 (rw) register accessor: CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SWPR (rw) register accessor: SWPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpr`]
module"]
pub type SWPR = crate::Reg<swpr::SWPRrs>;
#[doc = "SWPR"]
pub mod swpr;
#[doc = "SKR (w) register accessor: SKR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@skr`]
module"]
pub type SKR = crate::Reg<skr::SKRrs>;
#[doc = "SKR"]
pub mod skr;
#[doc = "IMR1 (rw) register accessor: SYSCFG CPU1 interrupt mask register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
#[doc = "SYSCFG CPU1 interrupt mask register 1"]
pub mod imr1;
#[doc = "IMR2 (rw) register accessor: SYSCFG CPU1 interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
#[doc = "SYSCFG CPU1 interrupt mask register 2"]
pub mod imr2;
#[doc = "C2IMR1 (rw) register accessor: SYSCFG CPU2 interrupt mask register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr1`]
module"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
#[doc = "SYSCFG CPU2 interrupt mask register 1"]
pub mod c2imr1;
#[doc = "C2IMR2 (rw) register accessor: SYSCFG CPU2 interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr2`]
module"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
#[doc = "SYSCFG CPU2 interrupt mask register 2"]
pub mod c2imr2;
#[doc = "RFDCR (rw) register accessor: radio debug control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdcr`]
module"]
pub type RFDCR = crate::Reg<rfdcr::RFDCRrs>;
#[doc = "radio debug control register"]
pub mod rfdcr;
