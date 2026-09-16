#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sspcr1: SSPCR1,
    sspcr2: SSPCR2,
    sspsr: SSPSR,
    sspdr: SSPDR,
    sspcrcpr: SSPCRCPR,
    ssprxcrcr: SSPRXCRCR,
    ssptxcrcr: SSPTXCRCR,
}
impl RegisterBlock {
    ///0x00 - SPI_SSPCR1 register
    #[inline(always)]
    pub const fn sspcr1(&self) -> &SSPCR1 {
        &self.sspcr1
    }
    ///0x04 - SPI_SSPCR2 register
    #[inline(always)]
    pub const fn sspcr2(&self) -> &SSPCR2 {
        &self.sspcr2
    }
    ///0x08 - SPI_SSPSR register
    #[inline(always)]
    pub const fn sspsr(&self) -> &SSPSR {
        &self.sspsr
    }
    ///0x0c - SPI_SSPDR register
    #[inline(always)]
    pub const fn sspdr(&self) -> &SSPDR {
        &self.sspdr
    }
    ///0x10 - SPI_SSPCRCPR register
    #[inline(always)]
    pub const fn sspcrcpr(&self) -> &SSPCRCPR {
        &self.sspcrcpr
    }
    ///0x14 - SPI_SSPRXCRCR register
    #[inline(always)]
    pub const fn ssprxcrcr(&self) -> &SSPRXCRCR {
        &self.ssprxcrcr
    }
    ///0x18 - SPI_SSPTXCRCR register
    #[inline(always)]
    pub const fn ssptxcrcr(&self) -> &SSPTXCRCR {
        &self.ssptxcrcr
    }
}
/**SSPCR1 (rw) register accessor: SPI_SSPCR1 register

You can [`read`](crate::Reg::read) this register and get [`sspcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPCR1)

For information about available fields see [`mod@sspcr1`] module*/
pub type SSPCR1 = crate::Reg<sspcr1::SSPCR1rs>;
///SPI_SSPCR1 register
pub mod sspcr1;
/**SSPCR2 (rw) register accessor: SPI_SSPCR2 register

You can [`read`](crate::Reg::read) this register and get [`sspcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPCR2)

For information about available fields see [`mod@sspcr2`] module*/
pub type SSPCR2 = crate::Reg<sspcr2::SSPCR2rs>;
///SPI_SSPCR2 register
pub mod sspcr2;
/**SSPSR (rw) register accessor: SPI_SSPSR register

You can [`read`](crate::Reg::read) this register and get [`sspsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPSR)

For information about available fields see [`mod@sspsr`] module*/
pub type SSPSR = crate::Reg<sspsr::SSPSRrs>;
///SPI_SSPSR register
pub mod sspsr;
/**SSPDR (rw) register accessor: SPI_SSPDR register

You can [`read`](crate::Reg::read) this register and get [`sspdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPDR)

For information about available fields see [`mod@sspdr`] module*/
pub type SSPDR = crate::Reg<sspdr::SSPDRrs>;
///SPI_SSPDR register
pub mod sspdr;
/**SSPCRCPR (rw) register accessor: SPI_SSPCRCPR register

You can [`read`](crate::Reg::read) this register and get [`sspcrcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcrcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPCRCPR)

For information about available fields see [`mod@sspcrcpr`] module*/
pub type SSPCRCPR = crate::Reg<sspcrcpr::SSPCRCPRrs>;
///SPI_SSPCRCPR register
pub mod sspcrcpr;
/**SSPRXCRCR (r) register accessor: SPI_SSPRXCRCR register

You can [`read`](crate::Reg::read) this register and get [`ssprxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPRXCRCR)

For information about available fields see [`mod@ssprxcrcr`] module*/
pub type SSPRXCRCR = crate::Reg<ssprxcrcr::SSPRXCRCRrs>;
///SPI_SSPRXCRCR register
pub mod ssprxcrcr;
/**SSPTXCRCR (r) register accessor: SPI_SSPTXCRCR register

You can [`read`](crate::Reg::read) this register and get [`ssptxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI:SSPTXCRCR)

For information about available fields see [`mod@ssptxcrcr`] module*/
pub type SSPTXCRCR = crate::Reg<ssptxcrcr::SSPTXCRCRrs>;
///SPI_SSPTXCRCR register
pub mod ssptxcrcr;
