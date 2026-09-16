#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    cfgr: CFGR,
    pllcfgr: PLLCFGR,
    _reserved4: [u8; 0x08],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved7: [u8; 0x04],
    ahbrstr: AHBRSTR,
    ioprstr: IOPRSTR,
    _reserved9: [u8; 0x08],
    apbrstr1: APBRSTR1,
    _reserved10: [u8; 0x04],
    apbrstr2: APBRSTR2,
    _reserved11: [u8; 0x04],
    ahbenr: AHBENR,
    iopenr: IOPENR,
    dbgcfgr: DBGCFGR,
    _reserved14: [u8; 0x04],
    apbenr1: APBENR1,
    _reserved15: [u8; 0x04],
    apbenr2: APBENR2,
    _reserved16: [u8; 0x04],
    ahbsmenr: AHBSMENR,
    iopsmenr: IOPSMENR,
    _reserved18: [u8; 0x08],
    apbsmenr1: APBSMENR1,
    _reserved19: [u8; 0x04],
    apbsmenr2: APBSMENR2,
    _reserved20: [u8; 0x04],
    ccipr: CCIPR,
    _reserved21: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    crrcr: CRRCR,
}
impl RegisterBlock {
    ///0x00 - Clock control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Internal clock sources calibration register
    #[inline(always)]
    pub const fn icscr(&self) -> &ICSCR {
        &self.icscr
    }
    ///0x08 - Clock configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x0c - PLL configuration register
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    ///0x18 - Clock interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x1c - Clock interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x20 - Clock interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x28 - AHB peripheral reset register
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    ///0x2c - I/O port reset register
    #[inline(always)]
    pub const fn ioprstr(&self) -> &IOPRSTR {
        &self.ioprstr
    }
    ///0x38 - APB peripheral reset register 1
    #[inline(always)]
    pub const fn apbrstr1(&self) -> &APBRSTR1 {
        &self.apbrstr1
    }
    ///0x40 - APB peripheral reset register 2
    #[inline(always)]
    pub const fn apbrstr2(&self) -> &APBRSTR2 {
        &self.apbrstr2
    }
    ///0x48 - AHB peripheral clock enable register
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    ///0x4c - I/O port clock enable register
    #[inline(always)]
    pub const fn iopenr(&self) -> &IOPENR {
        &self.iopenr
    }
    ///0x50 - Debug configuration register
    #[inline(always)]
    pub const fn dbgcfgr(&self) -> &DBGCFGR {
        &self.dbgcfgr
    }
    ///0x58 - APB peripheral clock enable register 1
    #[inline(always)]
    pub const fn apbenr1(&self) -> &APBENR1 {
        &self.apbenr1
    }
    ///0x60 - APB peripheral clock enable register 2
    #[inline(always)]
    pub const fn apbenr2(&self) -> &APBENR2 {
        &self.apbenr2
    }
    ///0x68 - AHB peripheral clock enable in Sleep/Stop mode register
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &AHBSMENR {
        &self.ahbsmenr
    }
    ///0x6c - I/O port in Sleep mode clock enable register
    #[inline(always)]
    pub const fn iopsmenr(&self) -> &IOPSMENR {
        &self.iopsmenr
    }
    ///0x78 - APB peripheral clock enable in Sleep/Stop mode register 1
    #[inline(always)]
    pub const fn apbsmenr1(&self) -> &APBSMENR1 {
        &self.apbsmenr1
    }
    ///0x80 - APB peripheral clock enable in Sleep/Stop mode register 2
    #[inline(always)]
    pub const fn apbsmenr2(&self) -> &APBSMENR2 {
        &self.apbsmenr2
    }
    ///0x88 - Peripherals independent clock configuration register
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    ///0x90 - RTC domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x94 - Control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x98 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
}
/**CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Clock control register
pub mod cr;
/**ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`] module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///Internal clock sources calibration register
pub mod icscr;
/**CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Clock configuration register
pub mod cfgr;
/**PLLCFGR (rw) register accessor: PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:PLLCFGR)

For information about available fields see [`mod@pllcfgr`] module*/
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
///PLL configuration register
pub mod pllcfgr;
/**CIER (rw) register accessor: Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///Clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///Clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///Clock interrupt clear register
pub mod cicr;
/**AHBRSTR (rw) register accessor: AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:AHBRSTR)

For information about available fields see [`mod@ahbrstr`] module*/
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
///AHB peripheral reset register
pub mod ahbrstr;
/**IOPRSTR (rw) register accessor: I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:IOPRSTR)

For information about available fields see [`mod@ioprstr`] module*/
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTRrs>;
///I/O port reset register
pub mod ioprstr;
/**APBRSTR1 (rw) register accessor: APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBRSTR1)

For information about available fields see [`mod@apbrstr1`] module*/
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1rs>;
///APB peripheral reset register 1
pub mod apbrstr1;
/**APBRSTR2 (rw) register accessor: APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBRSTR2)

For information about available fields see [`mod@apbrstr2`] module*/
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2rs>;
///APB peripheral reset register 2
pub mod apbrstr2;
/**AHBENR (rw) register accessor: AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:AHBENR)

For information about available fields see [`mod@ahbenr`] module*/
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
///AHB peripheral clock enable register
pub mod ahbenr;
/**IOPENR (rw) register accessor: I/O port clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:IOPENR)

For information about available fields see [`mod@iopenr`] module*/
pub type IOPENR = crate::Reg<iopenr::IOPENRrs>;
///I/O port clock enable register
pub mod iopenr;
/**DBGCFGR (rw) register accessor: Debug configuration register

You can [`read`](crate::Reg::read) this register and get [`dbgcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:DBGCFGR)

For information about available fields see [`mod@dbgcfgr`] module*/
pub type DBGCFGR = crate::Reg<dbgcfgr::DBGCFGRrs>;
///Debug configuration register
pub mod dbgcfgr;
/**APBENR1 (rw) register accessor: APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBENR1)

For information about available fields see [`mod@apbenr1`] module*/
pub type APBENR1 = crate::Reg<apbenr1::APBENR1rs>;
///APB peripheral clock enable register 1
pub mod apbenr1;
/**APBENR2 (rw) register accessor: APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apbenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBENR2)

For information about available fields see [`mod@apbenr2`] module*/
pub type APBENR2 = crate::Reg<apbenr2::APBENR2rs>;
///APB peripheral clock enable register 2
pub mod apbenr2;
/**AHBSMENR (rw) register accessor: AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:AHBSMENR)

For information about available fields see [`mod@ahbsmenr`] module*/
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENRrs>;
///AHB peripheral clock enable in Sleep/Stop mode register
pub mod ahbsmenr;
/**IOPSMENR (rw) register accessor: I/O port in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:IOPSMENR)

For information about available fields see [`mod@iopsmenr`] module*/
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENRrs>;
///I/O port in Sleep mode clock enable register
pub mod iopsmenr;
/**APBSMENR1 (rw) register accessor: APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBSMENR1)

For information about available fields see [`mod@apbsmenr1`] module*/
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1rs>;
///APB peripheral clock enable in Sleep/Stop mode register 1
pub mod apbsmenr1;
/**APBSMENR2 (rw) register accessor: APB peripheral clock enable in Sleep/Stop mode register 2

You can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBSMENR2)

For information about available fields see [`mod@apbsmenr2`] module*/
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2rs>;
///APB peripheral clock enable in Sleep/Stop mode register 2
pub mod apbsmenr2;
/**CCIPR (rw) register accessor: Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CCIPR)

For information about available fields see [`mod@ccipr`] module*/
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
///Peripherals independent clock configuration register
pub mod ccipr;
/**BDCR (rw) register accessor: RTC domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RTC domain control register
pub mod bdcr;
/**CSR (rw) register accessor: Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Control/status register
pub mod csr;
/**CRRCR (rw) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CRRCR)

For information about available fields see [`mod@crrcr`] module*/
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
///RCC clock recovery RC register
pub mod crrcr;
