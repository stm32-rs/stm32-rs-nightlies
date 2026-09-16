#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    _reserved1: [u8; 0x04],
    ccr: CCR,
    cdr: CDR,
    _reserved3: [u8; 0xe0],
    hwcfgr0: HWCFGR0,
    verr: VERR,
    ipdr: IPDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - ADC common status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x08 - ADC common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x0c - ADC common regular data register for dual mode
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        &self.cdr
    }
    ///0xf0 - ADC hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
    }
    ///0xf4 - ADC version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0xf8 - ADC identification register
    #[inline(always)]
    pub const fn ipdr(&self) -> &IPDR {
        &self.ipdr
    }
    ///0xfc - ADC size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CSR (r) register accessor: ADC common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///ADC common status register
pub mod csr;
/**CCR (rw) register accessor: ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common control register
pub mod ccr;
/**CDR (r) register accessor: ADC common regular data register for dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:CDR)

For information about available fields see [`mod@cdr`] module*/
pub type CDR = crate::Reg<cdr::CDRrs>;
///ADC common regular data register for dual mode
pub mod cdr;
/**HWCFGR0 (r) register accessor: ADC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:HWCFGR0)

For information about available fields see [`mod@hwcfgr0`] module*/
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
///ADC hardware configuration register
pub mod hwcfgr0;
/**VERR (r) register accessor: ADC version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///ADC version register
pub mod verr;
/**IPDR (r) register accessor: ADC identification register

You can [`read`](crate::Reg::read) this register and get [`ipdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:IPDR)

For information about available fields see [`mod@ipdr`] module*/
pub type IPDR = crate::Reg<ipdr::IPDRrs>;
///ADC identification register
pub mod ipdr;
/**SIDR (r) register accessor: ADC size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADCC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///ADC size identification register
pub mod sidr;
