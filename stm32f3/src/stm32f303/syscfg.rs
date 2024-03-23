#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr1: CFGR1,
    rcr: RCR,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    cfgr2: CFGR2,
    _reserved7: [u8; 0x2c],
    cfgr4: CFGR4,
    _reserved8: [u8; 0x04],
    cfgr3: CFGR3,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x04 - CCM SRAM protection register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
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
    #[doc = "0x18 - configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x48 - configuration register 4"]
    #[inline(always)]
    pub const fn cfgr4(&self) -> &CFGR4 {
        &self.cfgr4
    }
    #[doc = "0x50 - configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
}
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
#[doc = "CFGR2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "RCR (rw) register accessor: CCM SRAM protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCRrs>;
#[doc = "CCM SRAM protection register"]
pub mod rcr;
#[doc = "CFGR3 (rw) register accessor: configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`]
module"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
#[doc = "configuration register 3"]
pub mod cfgr3;
#[doc = "CFGR4 (rw) register accessor: configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr4`]
module"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4rs>;
#[doc = "configuration register 4"]
pub mod cfgr4;
