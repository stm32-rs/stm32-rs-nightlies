#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    cfgr1: CFGR1,
    fpuimr: FPUIMR,
    cnslckr: CNSLCKR,
    cslockr: CSLOCKR,
    cfgr2: CFGR2,
    mesr: MESR,
    cccsr: CCCSR,
    ccvr: CCVR,
    cccr: CCCR,
    _reserved10: [u8; 0x04],
    rsscmdr: RSSCMDR,
}
impl RegisterBlock {
    ///0x00 - SYSCFG secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x04 - SYSCFG configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x08 - SYSCFG FPU interrupt mask register
    #[inline(always)]
    pub const fn fpuimr(&self) -> &FPUIMR {
        &self.fpuimr
    }
    ///0x0c - SYSCFG CPU non-secure lock register
    #[inline(always)]
    pub const fn cnslckr(&self) -> &CNSLCKR {
        &self.cnslckr
    }
    ///0x10 - SYSCFG CPU secure lock register
    #[inline(always)]
    pub const fn cslockr(&self) -> &CSLOCKR {
        &self.cslockr
    }
    ///0x14 - SYSCFG configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x18 - SYSCFG memory erase status register
    #[inline(always)]
    pub const fn mesr(&self) -> &MESR {
        &self.mesr
    }
    ///0x1c - SYSCFG compensation cell control/status register
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    ///0x20 - SYSCFG compensation cell value register
    #[inline(always)]
    pub const fn ccvr(&self) -> &CCVR {
        &self.ccvr
    }
    ///0x24 - SYSCFG compensation cell code register
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    ///0x2c - SYSCFG RSS command register
    #[inline(always)]
    pub const fn rsscmdr(&self) -> &RSSCMDR {
        &self.rsscmdr
    }
}
/**SECCFGR (rw) register accessor: SYSCFG secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///SYSCFG secure configuration register
pub mod seccfgr;
/**CFGR1 (rw) register accessor: SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///SYSCFG configuration register 1
pub mod cfgr1;
/**FPUIMR (rw) register accessor: SYSCFG FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:FPUIMR)

For information about available fields see [`mod@fpuimr`] module*/
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
///SYSCFG FPU interrupt mask register
pub mod fpuimr;
/**CNSLCKR (rw) register accessor: SYSCFG CPU non-secure lock register

You can [`read`](crate::Reg::read) this register and get [`cnslckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CNSLCKR)

For information about available fields see [`mod@cnslckr`] module*/
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKRrs>;
///SYSCFG CPU non-secure lock register
pub mod cnslckr;
/**CSLOCKR (rw) register accessor: SYSCFG CPU secure lock register

You can [`read`](crate::Reg::read) this register and get [`cslockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cslockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CSLOCKR)

For information about available fields see [`mod@cslockr`] module*/
pub type CSLOCKR = crate::Reg<cslockr::CSLOCKRrs>;
///SYSCFG CPU secure lock register
pub mod cslockr;
/**CFGR2 (rw) register accessor: SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///SYSCFG configuration register 2
pub mod cfgr2;
/**MESR (rw) register accessor: SYSCFG memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:MESR)

For information about available fields see [`mod@mesr`] module*/
pub type MESR = crate::Reg<mesr::MESRrs>;
///SYSCFG memory erase status register
pub mod mesr;
/**CCCSR (rw) register accessor: SYSCFG compensation cell control/status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CCCSR)

For information about available fields see [`mod@cccsr`] module*/
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
///SYSCFG compensation cell control/status register
pub mod cccsr;
/**CCVR (r) register accessor: SYSCFG compensation cell value register

You can [`read`](crate::Reg::read) this register and get [`ccvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CCVR)

For information about available fields see [`mod@ccvr`] module*/
pub type CCVR = crate::Reg<ccvr::CCVRrs>;
///SYSCFG compensation cell value register
pub mod ccvr;
/**CCCR (rw) register accessor: SYSCFG compensation cell code register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:CCCR)

For information about available fields see [`mod@cccr`] module*/
pub type CCCR = crate::Reg<cccr::CCCRrs>;
///SYSCFG compensation cell code register
pub mod cccr;
/**RSSCMDR (rw) register accessor: SYSCFG RSS command register

You can [`read`](crate::Reg::read) this register and get [`rsscmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsscmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#SYSCFG:RSSCMDR)

For information about available fields see [`mod@rsscmdr`] module*/
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDRrs>;
///SYSCFG RSS command register
pub mod rsscmdr;
