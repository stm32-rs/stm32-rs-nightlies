#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crel: CREL,
    ccfg: CCFG,
    cstat: CSTAT,
    cwd: CWD,
    ir: IR,
    ie: IE,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Calibration Unit Core Release Register"]
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    #[doc = "0x04 - Calibration Configuration Register"]
    #[inline(always)]
    pub const fn ccfg(&self) -> &CCFG {
        &self.ccfg
    }
    #[doc = "0x08 - Calibration Status Register"]
    #[inline(always)]
    pub const fn cstat(&self) -> &CSTAT {
        &self.cstat
    }
    #[doc = "0x0c - Calibration Watchdog Register"]
    #[inline(always)]
    pub const fn cwd(&self) -> &CWD {
        &self.cwd
    }
    #[doc = "0x10 - Clock Calibration Unit Interrupt Register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x14 - Clock Calibration Unit Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
}
#[doc = "CREL (rw) register accessor: Clock Calibration Unit Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
pub type CREL = crate::Reg<crel::CRELrs>;
#[doc = "Clock Calibration Unit Core Release Register"]
pub mod crel;
#[doc = "CCFG (rw) register accessor: Calibration Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg`]
module"]
pub type CCFG = crate::Reg<ccfg::CCFGrs>;
#[doc = "Calibration Configuration Register"]
pub mod ccfg;
#[doc = "CSTAT (rw) register accessor: Calibration Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstat`]
module"]
pub type CSTAT = crate::Reg<cstat::CSTATrs>;
#[doc = "Calibration Status Register"]
pub mod cstat;
#[doc = "CWD (rw) register accessor: Calibration Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwd`]
module"]
pub type CWD = crate::Reg<cwd::CWDrs>;
#[doc = "Calibration Watchdog Register"]
pub mod cwd;
#[doc = "IR (rw) register accessor: Clock Calibration Unit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IRrs>;
#[doc = "Clock Calibration Unit Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: Clock Calibration Unit Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IErs>;
#[doc = "Clock Calibration Unit Interrupt Enable Register"]
pub mod ie;
