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
    ioprstr: IOPRSTR,
    ahbrstr: AHBRSTR,
    apbrstr1: APBRSTR1,
    apbrstr2: APBRSTR2,
    iopenr: IOPENR,
    ahbenr: AHBENR,
    apbenr1: APBENR1,
    apbenr2: APBENR2,
    iopsmenr: IOPSMENR,
    ahbsmenr: AHBSMENR,
    apbsmenr1: APBSMENR1,
    apbsmenr2: APBSMENR2,
    ccipr: CCIPR,
    ccipr2: CCIPR2,
    bdcr: BDCR,
    csr: CSR,
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
    ///0x24 - I/O port reset register
    #[inline(always)]
    pub const fn ioprstr(&self) -> &IOPRSTR {
        &self.ioprstr
    }
    ///0x28 - AHB peripheral reset register
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    ///0x2c - APB peripheral reset register 1
    #[inline(always)]
    pub const fn apbrstr1(&self) -> &APBRSTR1 {
        &self.apbrstr1
    }
    ///0x30 - APB peripheral reset register 2
    #[inline(always)]
    pub const fn apbrstr2(&self) -> &APBRSTR2 {
        &self.apbrstr2
    }
    ///0x34 - GPIO clock enable register
    #[inline(always)]
    pub const fn iopenr(&self) -> &IOPENR {
        &self.iopenr
    }
    ///0x38 - AHB peripheral clock enable register
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    ///0x3c - APB peripheral clock enable register 1
    #[inline(always)]
    pub const fn apbenr1(&self) -> &APBENR1 {
        &self.apbenr1
    }
    ///0x40 - APB peripheral clock enable register 2
    #[inline(always)]
    pub const fn apbenr2(&self) -> &APBENR2 {
        &self.apbenr2
    }
    ///0x44 - GPIO in Sleep mode clock enable register
    #[inline(always)]
    pub const fn iopsmenr(&self) -> &IOPSMENR {
        &self.iopsmenr
    }
    ///0x48 - AHB peripheral clock enable in Sleep mode register
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &AHBSMENR {
        &self.ahbsmenr
    }
    ///0x4c - APB peripheral clock enable in Sleep mode register 1
    #[inline(always)]
    pub const fn apbsmenr1(&self) -> &APBSMENR1 {
        &self.apbsmenr1
    }
    ///0x50 - APB peripheral clock enable in Sleep mode register 2
    #[inline(always)]
    pub const fn apbsmenr2(&self) -> &APBSMENR2 {
        &self.apbsmenr2
    }
    ///0x54 - Peripherals independent clock configuration register
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    ///0x58 - Peripherals independent clock configuration register 2
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0x5c - RTC domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x60 - Control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
}
/**CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Clock control register
pub mod cr;
/**ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`] module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///Internal clock sources calibration register
pub mod icscr;
/**CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Clock configuration register
pub mod cfgr;
/**PLLCFGR (rw) register accessor: PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:PLLCFGR)

For information about available fields see [`mod@pllcfgr`] module*/
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
///PLL configuration register
pub mod pllcfgr;
/**CIER (rw) register accessor: Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///Clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///Clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///Clock interrupt clear register
pub mod cicr;
/**IOPRSTR (rw) register accessor: I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:IOPRSTR)

For information about available fields see [`mod@ioprstr`] module*/
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTRrs>;
///I/O port reset register
pub mod ioprstr;
/**AHBRSTR (rw) register accessor: AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:AHBRSTR)

For information about available fields see [`mod@ahbrstr`] module*/
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
///AHB peripheral reset register
pub mod ahbrstr;
/**APBRSTR1 (rw) register accessor: APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBRSTR1)

For information about available fields see [`mod@apbrstr1`] module*/
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1rs>;
///APB peripheral reset register 1
pub mod apbrstr1;
/**APBRSTR2 (rw) register accessor: APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBRSTR2)

For information about available fields see [`mod@apbrstr2`] module*/
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2rs>;
///APB peripheral reset register 2
pub mod apbrstr2;
/**IOPENR (rw) register accessor: GPIO clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:IOPENR)

For information about available fields see [`mod@iopenr`] module*/
pub type IOPENR = crate::Reg<iopenr::IOPENRrs>;
///GPIO clock enable register
pub mod iopenr;
/**AHBENR (rw) register accessor: AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:AHBENR)

For information about available fields see [`mod@ahbenr`] module*/
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
///AHB peripheral clock enable register
pub mod ahbenr;
/**APBENR1 (rw) register accessor: APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBENR1)

For information about available fields see [`mod@apbenr1`] module*/
pub type APBENR1 = crate::Reg<apbenr1::APBENR1rs>;
///APB peripheral clock enable register 1
pub mod apbenr1;
/**APBENR2 (rw) register accessor: APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apbenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBENR2)

For information about available fields see [`mod@apbenr2`] module*/
pub type APBENR2 = crate::Reg<apbenr2::APBENR2rs>;
///APB peripheral clock enable register 2
pub mod apbenr2;
/**IOPSMENR (rw) register accessor: GPIO in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:IOPSMENR)

For information about available fields see [`mod@iopsmenr`] module*/
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENRrs>;
///GPIO in Sleep mode clock enable register
pub mod iopsmenr;
/**AHBSMENR (rw) register accessor: AHB peripheral clock enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:AHBSMENR)

For information about available fields see [`mod@ahbsmenr`] module*/
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENRrs>;
///AHB peripheral clock enable in Sleep mode register
pub mod ahbsmenr;
/**APBSMENR1 (rw) register accessor: APB peripheral clock enable in Sleep mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBSMENR1)

For information about available fields see [`mod@apbsmenr1`] module*/
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1rs>;
///APB peripheral clock enable in Sleep mode register 1
pub mod apbsmenr1;
/**APBSMENR2 (rw) register accessor: APB peripheral clock enable in Sleep mode register 2

You can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBSMENR2)

For information about available fields see [`mod@apbsmenr2`] module*/
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2rs>;
///APB peripheral clock enable in Sleep mode register 2
pub mod apbsmenr2;
/**CCIPR (rw) register accessor: Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CCIPR)

For information about available fields see [`mod@ccipr`] module*/
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
///Peripherals independent clock configuration register
pub mod ccipr;
/**CCIPR2 (rw) register accessor: Peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///Peripherals independent clock configuration register 2
pub mod ccipr2;
/**BDCR (rw) register accessor: RTC domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RTC domain control register
pub mod bdcr;
/**CSR (rw) register accessor: Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Control/status register
pub mod csr;
