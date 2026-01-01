#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rcc_cr: RCC_CR,
    rcc_icscr: RCC_ICSCR,
    rcc_cfgr: RCC_CFGR,
    _reserved3: [u8; 0x08],
    rcc_crrcr: RCC_CRRCR,
    rcc_cier: RCC_CIER,
    rcc_cifr: RCC_CIFR,
    rcc_cicr: RCC_CICR,
    rcc_ioprstr: RCC_IOPRSTR,
    rcc_ahbrstr: RCC_AHBRSTR,
    rcc_apbrstr1: RCC_APBRSTR1,
    rcc_apbrstr2: RCC_APBRSTR2,
    rcc_iopenr: RCC_IOPENR,
    rcc_ahbenr: RCC_AHBENR,
    rcc_apbenr1: RCC_APBENR1,
    rcc_apbenr2: RCC_APBENR2,
    rcc_iopsmenr: RCC_IOPSMENR,
    rcc_ahbsmenr: RCC_AHBSMENR,
    rcc_apbsmenr1: RCC_APBSMENR1,
    rcc_apbsmenr2: RCC_APBSMENR2,
    rcc_ccipr1: RCC_CCIPR1,
    rcc_ccipr2: RCC_CCIPR2,
    rcc_csr1: RCC_CSR1,
    rcc_csr2: RCC_CSR2,
}
impl RegisterBlock {
    ///0x00 - RCC clock control register
    #[inline(always)]
    pub const fn rcc_cr(&self) -> &RCC_CR {
        &self.rcc_cr
    }
    ///0x04 - RCC internal clock source calibration register
    #[inline(always)]
    pub const fn rcc_icscr(&self) -> &RCC_ICSCR {
        &self.rcc_icscr
    }
    ///0x08 - RCC clock configuration register
    #[inline(always)]
    pub const fn rcc_cfgr(&self) -> &RCC_CFGR {
        &self.rcc_cfgr
    }
    ///0x14 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn rcc_crrcr(&self) -> &RCC_CRRCR {
        &self.rcc_crrcr
    }
    ///0x18 - RCC clock interrupt enable register
    #[inline(always)]
    pub const fn rcc_cier(&self) -> &RCC_CIER {
        &self.rcc_cier
    }
    ///0x1c - RCC clock interrupt flag register
    #[inline(always)]
    pub const fn rcc_cifr(&self) -> &RCC_CIFR {
        &self.rcc_cifr
    }
    ///0x20 - RCC clock interrupt clear register
    #[inline(always)]
    pub const fn rcc_cicr(&self) -> &RCC_CICR {
        &self.rcc_cicr
    }
    ///0x24 - RCC I/O port reset register
    #[inline(always)]
    pub const fn rcc_ioprstr(&self) -> &RCC_IOPRSTR {
        &self.rcc_ioprstr
    }
    ///0x28 - RCC AHB peripheral reset register
    #[inline(always)]
    pub const fn rcc_ahbrstr(&self) -> &RCC_AHBRSTR {
        &self.rcc_ahbrstr
    }
    ///0x2c - RCC APB peripheral reset register 1
    #[inline(always)]
    pub const fn rcc_apbrstr1(&self) -> &RCC_APBRSTR1 {
        &self.rcc_apbrstr1
    }
    ///0x30 - RCC APB peripheral reset register 2
    #[inline(always)]
    pub const fn rcc_apbrstr2(&self) -> &RCC_APBRSTR2 {
        &self.rcc_apbrstr2
    }
    ///0x34 - RCC I/O port clock enable register
    #[inline(always)]
    pub const fn rcc_iopenr(&self) -> &RCC_IOPENR {
        &self.rcc_iopenr
    }
    ///0x38 - RCC AHB peripheral clock enable register
    #[inline(always)]
    pub const fn rcc_ahbenr(&self) -> &RCC_AHBENR {
        &self.rcc_ahbenr
    }
    ///0x3c - RCC APB peripheral clock enable register 1
    #[inline(always)]
    pub const fn rcc_apbenr1(&self) -> &RCC_APBENR1 {
        &self.rcc_apbenr1
    }
    ///0x40 - RCC APB peripheral clock enable register 2
    #[inline(always)]
    pub const fn rcc_apbenr2(&self) -> &RCC_APBENR2 {
        &self.rcc_apbenr2
    }
    ///0x44 - RCC I/O port in Sleep mode clock enable register
    #[inline(always)]
    pub const fn rcc_iopsmenr(&self) -> &RCC_IOPSMENR {
        &self.rcc_iopsmenr
    }
    ///0x48 - RCC AHB peripheral clock enable in Sleep/Stop mode register
    #[inline(always)]
    pub const fn rcc_ahbsmenr(&self) -> &RCC_AHBSMENR {
        &self.rcc_ahbsmenr
    }
    ///0x4c - RCC APB peripheral clock enable in Sleep/Stop mode register 1
    #[inline(always)]
    pub const fn rcc_apbsmenr1(&self) -> &RCC_APBSMENR1 {
        &self.rcc_apbsmenr1
    }
    ///0x50 - RCC APB peripheral clock enable in Sleep/Stop mode register 2
    #[inline(always)]
    pub const fn rcc_apbsmenr2(&self) -> &RCC_APBSMENR2 {
        &self.rcc_apbsmenr2
    }
    ///0x54 - RCC peripherals independent clock configuration register 1
    #[inline(always)]
    pub const fn rcc_ccipr1(&self) -> &RCC_CCIPR1 {
        &self.rcc_ccipr1
    }
    ///0x58 - RCC peripherals independent clock configuration register 2
    #[inline(always)]
    pub const fn rcc_ccipr2(&self) -> &RCC_CCIPR2 {
        &self.rcc_ccipr2
    }
    ///0x5c - RCC control/status register 1
    #[inline(always)]
    pub const fn rcc_csr1(&self) -> &RCC_CSR1 {
        &self.rcc_csr1
    }
    ///0x60 - RCC control/status register 2
    #[inline(always)]
    pub const fn rcc_csr2(&self) -> &RCC_CSR2 {
        &self.rcc_csr2
    }
}
/**RCC_CR (rw) register accessor: RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`rcc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CR)

For information about available fields see [`mod@rcc_cr`] module*/
pub type RCC_CR = crate::Reg<rcc_cr::RCC_CRrs>;
///RCC clock control register
pub mod rcc_cr;
/**RCC_ICSCR (rw) register accessor: RCC internal clock source calibration register

You can [`read`](crate::Reg::read) this register and get [`rcc_icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_ICSCR)

For information about available fields see [`mod@rcc_icscr`] module*/
pub type RCC_ICSCR = crate::Reg<rcc_icscr::RCC_ICSCRrs>;
///RCC internal clock source calibration register
pub mod rcc_icscr;
/**RCC_CFGR (rw) register accessor: RCC clock configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CFGR)

For information about available fields see [`mod@rcc_cfgr`] module*/
pub type RCC_CFGR = crate::Reg<rcc_cfgr::RCC_CFGRrs>;
///RCC clock configuration register
pub mod rcc_cfgr;
/**RCC_CRRCR (r) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CRRCR)

For information about available fields see [`mod@rcc_crrcr`] module*/
pub type RCC_CRRCR = crate::Reg<rcc_crrcr::RCC_CRRCRrs>;
///RCC clock recovery RC register
pub mod rcc_crrcr;
/**RCC_CIER (rw) register accessor: RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CIER)

For information about available fields see [`mod@rcc_cier`] module*/
pub type RCC_CIER = crate::Reg<rcc_cier::RCC_CIERrs>;
///RCC clock interrupt enable register
pub mod rcc_cier;
/**RCC_CIFR (r) register accessor: RCC clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CIFR)

For information about available fields see [`mod@rcc_cifr`] module*/
pub type RCC_CIFR = crate::Reg<rcc_cifr::RCC_CIFRrs>;
///RCC clock interrupt flag register
pub mod rcc_cifr;
/**RCC_CICR (w) register accessor: RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CICR)

For information about available fields see [`mod@rcc_cicr`] module*/
pub type RCC_CICR = crate::Reg<rcc_cicr::RCC_CICRrs>;
///RCC clock interrupt clear register
pub mod rcc_cicr;
/**RCC_IOPRSTR (rw) register accessor: RCC I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_IOPRSTR)

For information about available fields see [`mod@rcc_ioprstr`] module*/
pub type RCC_IOPRSTR = crate::Reg<rcc_ioprstr::RCC_IOPRSTRrs>;
///RCC I/O port reset register
pub mod rcc_ioprstr;
/**RCC_AHBRSTR (rw) register accessor: RCC AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_AHBRSTR)

For information about available fields see [`mod@rcc_ahbrstr`] module*/
pub type RCC_AHBRSTR = crate::Reg<rcc_ahbrstr::RCC_AHBRSTRrs>;
///RCC AHB peripheral reset register
pub mod rcc_ahbrstr;
/**RCC_APBRSTR1 (rw) register accessor: RCC APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbrstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbrstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBRSTR1)

For information about available fields see [`mod@rcc_apbrstr1`] module*/
pub type RCC_APBRSTR1 = crate::Reg<rcc_apbrstr1::RCC_APBRSTR1rs>;
///RCC APB peripheral reset register 1
pub mod rcc_apbrstr1;
/**RCC_APBRSTR2 (rw) register accessor: RCC APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbrstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbrstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBRSTR2)

For information about available fields see [`mod@rcc_apbrstr2`] module*/
pub type RCC_APBRSTR2 = crate::Reg<rcc_apbrstr2::RCC_APBRSTR2rs>;
///RCC APB peripheral reset register 2
pub mod rcc_apbrstr2;
/**RCC_IOPENR (rw) register accessor: RCC I/O port clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_IOPENR)

For information about available fields see [`mod@rcc_iopenr`] module*/
pub type RCC_IOPENR = crate::Reg<rcc_iopenr::RCC_IOPENRrs>;
///RCC I/O port clock enable register
pub mod rcc_iopenr;
/**RCC_AHBENR (rw) register accessor: RCC AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_AHBENR)

For information about available fields see [`mod@rcc_ahbenr`] module*/
pub type RCC_AHBENR = crate::Reg<rcc_ahbenr::RCC_AHBENRrs>;
///RCC AHB peripheral clock enable register
pub mod rcc_ahbenr;
/**RCC_APBENR1 (rw) register accessor: RCC APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBENR1)

For information about available fields see [`mod@rcc_apbenr1`] module*/
pub type RCC_APBENR1 = crate::Reg<rcc_apbenr1::RCC_APBENR1rs>;
///RCC APB peripheral clock enable register 1
pub mod rcc_apbenr1;
/**RCC_APBENR2 (rw) register accessor: RCC APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBENR2)

For information about available fields see [`mod@rcc_apbenr2`] module*/
pub type RCC_APBENR2 = crate::Reg<rcc_apbenr2::RCC_APBENR2rs>;
///RCC APB peripheral clock enable register 2
pub mod rcc_apbenr2;
/**RCC_IOPSMENR (rw) register accessor: RCC I/O port in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_IOPSMENR)

For information about available fields see [`mod@rcc_iopsmenr`] module*/
pub type RCC_IOPSMENR = crate::Reg<rcc_iopsmenr::RCC_IOPSMENRrs>;
///RCC I/O port in Sleep mode clock enable register
pub mod rcc_iopsmenr;
/**RCC_AHBSMENR (rw) register accessor: RCC AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_AHBSMENR)

For information about available fields see [`mod@rcc_ahbsmenr`] module*/
pub type RCC_AHBSMENR = crate::Reg<rcc_ahbsmenr::RCC_AHBSMENRrs>;
///RCC AHB peripheral clock enable in Sleep/Stop mode register
pub mod rcc_ahbsmenr;
/**RCC_APBSMENR1 (rw) register accessor: RCC APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbsmenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbsmenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBSMENR1)

For information about available fields see [`mod@rcc_apbsmenr1`] module*/
pub type RCC_APBSMENR1 = crate::Reg<rcc_apbsmenr1::RCC_APBSMENR1rs>;
///RCC APB peripheral clock enable in Sleep/Stop mode register 1
pub mod rcc_apbsmenr1;
/**RCC_APBSMENR2 (rw) register accessor: RCC APB peripheral clock enable in Sleep/Stop mode register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbsmenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbsmenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBSMENR2)

For information about available fields see [`mod@rcc_apbsmenr2`] module*/
pub type RCC_APBSMENR2 = crate::Reg<rcc_apbsmenr2::RCC_APBSMENR2rs>;
///RCC APB peripheral clock enable in Sleep/Stop mode register 2
pub mod rcc_apbsmenr2;
/**RCC_CCIPR1 (rw) register accessor: RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CCIPR1)

For information about available fields see [`mod@rcc_ccipr1`] module*/
pub type RCC_CCIPR1 = crate::Reg<rcc_ccipr1::RCC_CCIPR1rs>;
///RCC peripherals independent clock configuration register 1
pub mod rcc_ccipr1;
/**RCC_CCIPR2 (rw) register accessor: RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CCIPR2)

For information about available fields see [`mod@rcc_ccipr2`] module*/
pub type RCC_CCIPR2 = crate::Reg<rcc_ccipr2::RCC_CCIPR2rs>;
///RCC peripherals independent clock configuration register 2
pub mod rcc_ccipr2;
/**RCC_CSR1 (rw) register accessor: RCC control/status register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CSR1)

For information about available fields see [`mod@rcc_csr1`] module*/
pub type RCC_CSR1 = crate::Reg<rcc_csr1::RCC_CSR1rs>;
///RCC control/status register 1
pub mod rcc_csr1;
/**RCC_CSR2 (rw) register accessor: RCC control/status register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_csr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CSR2)

For information about available fields see [`mod@rcc_csr2`] module*/
pub type RCC_CSR2 = crate::Reg<rcc_csr2::RCC_CSR2rs>;
///RCC control/status register 2
pub mod rcc_csr2;
