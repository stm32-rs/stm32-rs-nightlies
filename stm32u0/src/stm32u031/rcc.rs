#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rcc_cr: RCC_CR,
    rcc_icscr: RCC_ICSCR,
    rcc_cfgr: RCC_CFGR,
    rcc_pllcfgr: RCC_PLLCFGR,
    _reserved4: [u8; 0x08],
    rcc_cier: RCC_CIER,
    rcc_cifr: RCC_CIFR,
    rcc_cicr: RCC_CICR,
    _reserved7: [u8; 0x04],
    rcc_ahbrstr: RCC_AHBRSTR,
    rcc_ioprstr: RCC_IOPRSTR,
    _reserved9: [u8; 0x08],
    rcc_apbrstr1: RCC_APBRSTR1,
    _reserved10: [u8; 0x04],
    rcc_apbrstr2: RCC_APBRSTR2,
    _reserved11: [u8; 0x04],
    rcc_ahbenr: RCC_AHBENR,
    rcc_iopenr: RCC_IOPENR,
    rcc_dbgcfgr: RCC_DBGCFGR,
    _reserved14: [u8; 0x04],
    rcc_apbenr1: RCC_APBENR1,
    _reserved15: [u8; 0x04],
    rcc_apbenr2: RCC_APBENR2,
    _reserved16: [u8; 0x04],
    rcc_ahbsmenr: RCC_AHBSMENR,
    rcc_iopsmenr: RCC_IOPSMENR,
    _reserved18: [u8; 0x08],
    rcc_apbsmenr1: RCC_APBSMENR1,
    _reserved19: [u8; 0x04],
    rcc_apbsmenr2: RCC_APBSMENR2,
    _reserved20: [u8; 0x04],
    rcc_ccipr: RCC_CCIPR,
    _reserved21: [u8; 0x04],
    rcc_bdcr: RCC_BDCR,
    rcc_csr: RCC_CSR,
    rcc_crrcr: RCC_CRRCR,
}
impl RegisterBlock {
    ///0x00 - Clock control register
    #[inline(always)]
    pub const fn rcc_cr(&self) -> &RCC_CR {
        &self.rcc_cr
    }
    ///0x04 - Internal clock sources calibration register
    #[inline(always)]
    pub const fn rcc_icscr(&self) -> &RCC_ICSCR {
        &self.rcc_icscr
    }
    ///0x08 - Clock configuration register
    #[inline(always)]
    pub const fn rcc_cfgr(&self) -> &RCC_CFGR {
        &self.rcc_cfgr
    }
    ///0x0c - PLL configuration register
    #[inline(always)]
    pub const fn rcc_pllcfgr(&self) -> &RCC_PLLCFGR {
        &self.rcc_pllcfgr
    }
    ///0x18 - Clock interrupt enable register
    #[inline(always)]
    pub const fn rcc_cier(&self) -> &RCC_CIER {
        &self.rcc_cier
    }
    ///0x1c - Clock interrupt flag register
    #[inline(always)]
    pub const fn rcc_cifr(&self) -> &RCC_CIFR {
        &self.rcc_cifr
    }
    ///0x20 - Clock interrupt clear register
    #[inline(always)]
    pub const fn rcc_cicr(&self) -> &RCC_CICR {
        &self.rcc_cicr
    }
    ///0x28 - AHB peripheral reset register
    #[inline(always)]
    pub const fn rcc_ahbrstr(&self) -> &RCC_AHBRSTR {
        &self.rcc_ahbrstr
    }
    ///0x2c - I/O port reset register
    #[inline(always)]
    pub const fn rcc_ioprstr(&self) -> &RCC_IOPRSTR {
        &self.rcc_ioprstr
    }
    ///0x38 - APB peripheral reset register 1
    #[inline(always)]
    pub const fn rcc_apbrstr1(&self) -> &RCC_APBRSTR1 {
        &self.rcc_apbrstr1
    }
    ///0x40 - APB peripheral reset register 2
    #[inline(always)]
    pub const fn rcc_apbrstr2(&self) -> &RCC_APBRSTR2 {
        &self.rcc_apbrstr2
    }
    ///0x48 - AHB peripheral clock enable register
    #[inline(always)]
    pub const fn rcc_ahbenr(&self) -> &RCC_AHBENR {
        &self.rcc_ahbenr
    }
    ///0x4c - I/O port clock enable register
    #[inline(always)]
    pub const fn rcc_iopenr(&self) -> &RCC_IOPENR {
        &self.rcc_iopenr
    }
    ///0x50 - Debug configuration register
    #[inline(always)]
    pub const fn rcc_dbgcfgr(&self) -> &RCC_DBGCFGR {
        &self.rcc_dbgcfgr
    }
    ///0x58 - APB peripheral clock enable register 1
    #[inline(always)]
    pub const fn rcc_apbenr1(&self) -> &RCC_APBENR1 {
        &self.rcc_apbenr1
    }
    ///0x60 - APB peripheral clock enable register 2
    #[inline(always)]
    pub const fn rcc_apbenr2(&self) -> &RCC_APBENR2 {
        &self.rcc_apbenr2
    }
    ///0x68 - AHB peripheral clock enable in Sleep/Stop mode register
    #[inline(always)]
    pub const fn rcc_ahbsmenr(&self) -> &RCC_AHBSMENR {
        &self.rcc_ahbsmenr
    }
    ///0x6c - I/O port in Sleep mode clock enable register
    #[inline(always)]
    pub const fn rcc_iopsmenr(&self) -> &RCC_IOPSMENR {
        &self.rcc_iopsmenr
    }
    ///0x78 - APB peripheral clock enable in Sleep/Stop mode register 1
    #[inline(always)]
    pub const fn rcc_apbsmenr1(&self) -> &RCC_APBSMENR1 {
        &self.rcc_apbsmenr1
    }
    ///0x80 - APB peripheral clock enable in Sleep/Stop mode register 2
    #[inline(always)]
    pub const fn rcc_apbsmenr2(&self) -> &RCC_APBSMENR2 {
        &self.rcc_apbsmenr2
    }
    ///0x88 - Peripherals independent clock configuration register
    #[inline(always)]
    pub const fn rcc_ccipr(&self) -> &RCC_CCIPR {
        &self.rcc_ccipr
    }
    ///0x90 - RTC domain control register
    #[inline(always)]
    pub const fn rcc_bdcr(&self) -> &RCC_BDCR {
        &self.rcc_bdcr
    }
    ///0x94 - Control/status register
    #[inline(always)]
    pub const fn rcc_csr(&self) -> &RCC_CSR {
        &self.rcc_csr
    }
    ///0x98 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn rcc_crrcr(&self) -> &RCC_CRRCR {
        &self.rcc_crrcr
    }
}
/**RCC_CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`rcc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CR)

For information about available fields see [`mod@rcc_cr`]
module*/
pub type RCC_CR = crate::Reg<rcc_cr::RCC_CRrs>;
///Clock control register
pub mod rcc_cr;
/**RCC_ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`rcc_icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_ICSCR)

For information about available fields see [`mod@rcc_icscr`]
module*/
pub type RCC_ICSCR = crate::Reg<rcc_icscr::RCC_ICSCRrs>;
///Internal clock sources calibration register
pub mod rcc_icscr;
/**RCC_CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CFGR)

For information about available fields see [`mod@rcc_cfgr`]
module*/
pub type RCC_CFGR = crate::Reg<rcc_cfgr::RCC_CFGRrs>;
///Clock configuration register
pub mod rcc_cfgr;
/**RCC_PLLCFGR (rw) register accessor: PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_PLLCFGR)

For information about available fields see [`mod@rcc_pllcfgr`]
module*/
pub type RCC_PLLCFGR = crate::Reg<rcc_pllcfgr::RCC_PLLCFGRrs>;
///PLL configuration register
pub mod rcc_pllcfgr;
/**RCC_CIER (rw) register accessor: Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CIER)

For information about available fields see [`mod@rcc_cier`]
module*/
pub type RCC_CIER = crate::Reg<rcc_cier::RCC_CIERrs>;
///Clock interrupt enable register
pub mod rcc_cier;
/**RCC_CIFR (r) register accessor: Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CIFR)

For information about available fields see [`mod@rcc_cifr`]
module*/
pub type RCC_CIFR = crate::Reg<rcc_cifr::RCC_CIFRrs>;
///Clock interrupt flag register
pub mod rcc_cifr;
/**RCC_CICR (w) register accessor: Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CICR)

For information about available fields see [`mod@rcc_cicr`]
module*/
pub type RCC_CICR = crate::Reg<rcc_cicr::RCC_CICRrs>;
///Clock interrupt clear register
pub mod rcc_cicr;
/**RCC_AHBRSTR (rw) register accessor: AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_AHBRSTR)

For information about available fields see [`mod@rcc_ahbrstr`]
module*/
pub type RCC_AHBRSTR = crate::Reg<rcc_ahbrstr::RCC_AHBRSTRrs>;
///AHB peripheral reset register
pub mod rcc_ahbrstr;
/**RCC_IOPRSTR (rw) register accessor: I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_IOPRSTR)

For information about available fields see [`mod@rcc_ioprstr`]
module*/
pub type RCC_IOPRSTR = crate::Reg<rcc_ioprstr::RCC_IOPRSTRrs>;
///I/O port reset register
pub mod rcc_ioprstr;
/**RCC_APBRSTR1 (rw) register accessor: APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbrstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbrstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_APBRSTR1)

For information about available fields see [`mod@rcc_apbrstr1`]
module*/
pub type RCC_APBRSTR1 = crate::Reg<rcc_apbrstr1::RCC_APBRSTR1rs>;
///APB peripheral reset register 1
pub mod rcc_apbrstr1;
/**RCC_APBRSTR2 (rw) register accessor: APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbrstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbrstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_APBRSTR2)

For information about available fields see [`mod@rcc_apbrstr2`]
module*/
pub type RCC_APBRSTR2 = crate::Reg<rcc_apbrstr2::RCC_APBRSTR2rs>;
///APB peripheral reset register 2
pub mod rcc_apbrstr2;
/**RCC_AHBENR (rw) register accessor: AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_AHBENR)

For information about available fields see [`mod@rcc_ahbenr`]
module*/
pub type RCC_AHBENR = crate::Reg<rcc_ahbenr::RCC_AHBENRrs>;
///AHB peripheral clock enable register
pub mod rcc_ahbenr;
/**RCC_IOPENR (rw) register accessor: I/O port clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_IOPENR)

For information about available fields see [`mod@rcc_iopenr`]
module*/
pub type RCC_IOPENR = crate::Reg<rcc_iopenr::RCC_IOPENRrs>;
///I/O port clock enable register
pub mod rcc_iopenr;
/**RCC_DBGCFGR (rw) register accessor: Debug configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_dbgcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_DBGCFGR)

For information about available fields see [`mod@rcc_dbgcfgr`]
module*/
pub type RCC_DBGCFGR = crate::Reg<rcc_dbgcfgr::RCC_DBGCFGRrs>;
///Debug configuration register
pub mod rcc_dbgcfgr;
/**RCC_APBENR1 (rw) register accessor: APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_APBENR1)

For information about available fields see [`mod@rcc_apbenr1`]
module*/
pub type RCC_APBENR1 = crate::Reg<rcc_apbenr1::RCC_APBENR1rs>;
///APB peripheral clock enable register 1
pub mod rcc_apbenr1;
/**RCC_APBENR2 (rw) register accessor: APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_APBENR2)

For information about available fields see [`mod@rcc_apbenr2`]
module*/
pub type RCC_APBENR2 = crate::Reg<rcc_apbenr2::RCC_APBENR2rs>;
///APB peripheral clock enable register 2
pub mod rcc_apbenr2;
/**RCC_AHBSMENR (rw) register accessor: AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_AHBSMENR)

For information about available fields see [`mod@rcc_ahbsmenr`]
module*/
pub type RCC_AHBSMENR = crate::Reg<rcc_ahbsmenr::RCC_AHBSMENRrs>;
///AHB peripheral clock enable in Sleep/Stop mode register
pub mod rcc_ahbsmenr;
/**RCC_IOPSMENR (rw) register accessor: I/O port in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_IOPSMENR)

For information about available fields see [`mod@rcc_iopsmenr`]
module*/
pub type RCC_IOPSMENR = crate::Reg<rcc_iopsmenr::RCC_IOPSMENRrs>;
///I/O port in Sleep mode clock enable register
pub mod rcc_iopsmenr;
/**RCC_APBSMENR1 (rw) register accessor: APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbsmenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbsmenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_APBSMENR1)

For information about available fields see [`mod@rcc_apbsmenr1`]
module*/
pub type RCC_APBSMENR1 = crate::Reg<rcc_apbsmenr1::RCC_APBSMENR1rs>;
///APB peripheral clock enable in Sleep/Stop mode register 1
pub mod rcc_apbsmenr1;
/**RCC_APBSMENR2 (rw) register accessor: APB peripheral clock enable in Sleep/Stop mode register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbsmenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbsmenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_APBSMENR2)

For information about available fields see [`mod@rcc_apbsmenr2`]
module*/
pub type RCC_APBSMENR2 = crate::Reg<rcc_apbsmenr2::RCC_APBSMENR2rs>;
///APB peripheral clock enable in Sleep/Stop mode register 2
pub mod rcc_apbsmenr2;
/**RCC_CCIPR (rw) register accessor: Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CCIPR)

For information about available fields see [`mod@rcc_ccipr`]
module*/
pub type RCC_CCIPR = crate::Reg<rcc_ccipr::RCC_CCIPRrs>;
///Peripherals independent clock configuration register
pub mod rcc_ccipr;
/**RCC_BDCR (rw) register accessor: RTC domain control register

You can [`read`](crate::Reg::read) this register and get [`rcc_bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_BDCR)

For information about available fields see [`mod@rcc_bdcr`]
module*/
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCRrs>;
///RTC domain control register
pub mod rcc_bdcr;
/**RCC_CSR (rw) register accessor: Control/status register

You can [`read`](crate::Reg::read) this register and get [`rcc_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CSR)

For information about available fields see [`mod@rcc_csr`]
module*/
pub type RCC_CSR = crate::Reg<rcc_csr::RCC_CSRrs>;
///Control/status register
pub mod rcc_csr;
/**RCC_CRRCR (rw) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_crrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CRRCR)

For information about available fields see [`mod@rcc_crrcr`]
module*/
pub type RCC_CRRCR = crate::Reg<rcc_crrcr::RCC_CRRCRrs>;
///RCC clock recovery RC register
pub mod rcc_crrcr;
