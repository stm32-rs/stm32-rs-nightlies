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
    scsr: SCSR,
    skr: SKR,
    swpr: SWPR,
    swpr2: SWPR2,
    _reserved10: [u8; 0x04],
    rsscmdr: RSSCMDR,
}
impl RegisterBlock {
    ///0x00 - SYSCFG secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x04 - configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x08 - FPU interrupt mask register
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
    ///0x14 - CFGR2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x18 - SCSR
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        &self.scsr
    }
    ///0x1c - SKR
    #[inline(always)]
    pub const fn skr(&self) -> &SKR {
        &self.skr
    }
    ///0x20 - SWPR
    #[inline(always)]
    pub const fn swpr(&self) -> &SWPR {
        &self.swpr
    }
    ///0x24 - SWPR2
    #[inline(always)]
    pub const fn swpr2(&self) -> &SWPR2 {
        &self.swpr2
    }
    ///0x2c - RSSCMDR
    #[inline(always)]
    pub const fn rsscmdr(&self) -> &RSSCMDR {
        &self.rsscmdr
    }
}
/**SECCFGR (rw) register accessor: SYSCFG secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///SYSCFG secure configuration register
pub mod seccfgr;
/**CFGR1 (rw) register accessor: configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///configuration register 1
pub mod cfgr1;
/**FPUIMR (rw) register accessor: FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:FPUIMR)

For information about available fields see [`mod@fpuimr`] module*/
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
///FPU interrupt mask register
pub mod fpuimr;
/**CNSLCKR (rw) register accessor: SYSCFG CPU non-secure lock register

You can [`read`](crate::Reg::read) this register and get [`cnslckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:CNSLCKR)

For information about available fields see [`mod@cnslckr`] module*/
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKRrs>;
///SYSCFG CPU non-secure lock register
pub mod cnslckr;
/**CSLOCKR (rw) register accessor: SYSCFG CPU secure lock register

You can [`read`](crate::Reg::read) this register and get [`cslockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cslockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:CSLOCKR)

For information about available fields see [`mod@cslockr`] module*/
pub type CSLOCKR = crate::Reg<cslockr::CSLOCKRrs>;
///SYSCFG CPU secure lock register
pub mod cslockr;
/**SCSR (rw) register accessor: SCSR

You can [`read`](crate::Reg::read) this register and get [`scsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:SCSR)

For information about available fields see [`mod@scsr`] module*/
pub type SCSR = crate::Reg<scsr::SCSRrs>;
///SCSR
pub mod scsr;
/**CFGR2 (rw) register accessor: CFGR2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///CFGR2
pub mod cfgr2;
/**SWPR (w) register accessor: SWPR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:SWPR)

For information about available fields see [`mod@swpr`] module*/
pub type SWPR = crate::Reg<swpr::SWPRrs>;
///SWPR
pub mod swpr;
/**SKR (w) register accessor: SKR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:SKR)

For information about available fields see [`mod@skr`] module*/
pub type SKR = crate::Reg<skr::SKRrs>;
///SKR
pub mod skr;
/**SWPR2 (w) register accessor: SWPR2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:SWPR2)

For information about available fields see [`mod@swpr2`] module*/
pub type SWPR2 = crate::Reg<swpr2::SWPR2rs>;
///SWPR2
pub mod swpr2;
/**RSSCMDR (rw) register accessor: RSSCMDR

You can [`read`](crate::Reg::read) this register and get [`rsscmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsscmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:RSSCMDR)

For information about available fields see [`mod@rsscmdr`] module*/
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDRrs>;
///RSSCMDR
pub mod rsscmdr;
