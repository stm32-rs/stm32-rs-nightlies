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
}
impl RegisterBlock {
    #[doc = "0x00 - Remap Memory register"]
    #[inline(always)]
    pub const fn memrmp(&self) -> &MEMRMP {
        &self.memrmp
    }
    #[doc = "0x04 - peripheral mode configuration register"]
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
    #[doc = "0x18 - CCM SRAM control and status register"]
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        &self.scsr
    }
    #[doc = "0x1c - configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x20 - SRAM Write protection register 1"]
    #[inline(always)]
    pub const fn swpr(&self) -> &SWPR {
        &self.swpr
    }
    #[doc = "0x24 - SRAM2 Key Register"]
    #[inline(always)]
    pub const fn skr(&self) -> &SKR {
        &self.skr
    }
}
#[doc = "MEMRMP (rw) register accessor: Remap Memory register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memrmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memrmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memrmp`]
module"]
pub type MEMRMP = crate::Reg<memrmp::MEMRMPrs>;
#[doc = "Remap Memory register"]
pub mod memrmp;
#[doc = "CFGR1 (rw) register accessor: peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "peripheral mode configuration register"]
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
#[doc = "SCSR (rw) register accessor: CCM SRAM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`]
module"]
pub type SCSR = crate::Reg<scsr::SCSRrs>;
#[doc = "CCM SRAM control and status register"]
pub mod scsr;
#[doc = "CFGR2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "SWPR (rw) register accessor: SRAM Write protection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpr`]
module"]
pub type SWPR = crate::Reg<swpr::SWPRrs>;
#[doc = "SRAM Write protection register 1"]
pub mod swpr;
#[doc = "SKR (w) register accessor: SRAM2 Key Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@skr`]
module"]
pub type SKR = crate::Reg<skr::SKRrs>;
#[doc = "SRAM2 Key Register"]
pub mod skr;
