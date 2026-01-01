#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    spi_sspcr1: SPI_SSPCR1,
    spi_sspcr2: SPI_SSPCR2,
    spi_sspsr: SPI_SSPSR,
    spi_sspdr: SPI_SSPDR,
    spi_sspcrcpr: SPI_SSPCRCPR,
    spi_ssprxcrcr: SPI_SSPRXCRCR,
    spi_ssptxcrcr: SPI_SSPTXCRCR,
    spi2s_i2scfgr: SPI2S_I2SCFGR,
    spi2s_i2spr: SPI2S_I2SPR,
}
impl RegisterBlock {
    ///0x00 - SPI_SSPCR1 register
    #[inline(always)]
    pub const fn spi_sspcr1(&self) -> &SPI_SSPCR1 {
        &self.spi_sspcr1
    }
    ///0x04 - SPI_SSPCR2 register
    #[inline(always)]
    pub const fn spi_sspcr2(&self) -> &SPI_SSPCR2 {
        &self.spi_sspcr2
    }
    ///0x08 - SPI_SSPSR register
    #[inline(always)]
    pub const fn spi_sspsr(&self) -> &SPI_SSPSR {
        &self.spi_sspsr
    }
    ///0x0c - SPI_SSPDR register
    #[inline(always)]
    pub const fn spi_sspdr(&self) -> &SPI_SSPDR {
        &self.spi_sspdr
    }
    ///0x10 - SPI_SSPCRCPR register
    #[inline(always)]
    pub const fn spi_sspcrcpr(&self) -> &SPI_SSPCRCPR {
        &self.spi_sspcrcpr
    }
    ///0x14 - SPI_SSPRXCRCR register
    #[inline(always)]
    pub const fn spi_ssprxcrcr(&self) -> &SPI_SSPRXCRCR {
        &self.spi_ssprxcrcr
    }
    ///0x18 - SPI_SSPTXCRCR register
    #[inline(always)]
    pub const fn spi_ssptxcrcr(&self) -> &SPI_SSPTXCRCR {
        &self.spi_ssptxcrcr
    }
    ///0x1c - SPI2S_I2SCFGR register
    #[inline(always)]
    pub const fn spi2s_i2scfgr(&self) -> &SPI2S_I2SCFGR {
        &self.spi2s_i2scfgr
    }
    ///0x20 - SPI2S_I2SPR register
    #[inline(always)]
    pub const fn spi2s_i2spr(&self) -> &SPI2S_I2SPR {
        &self.spi2s_i2spr
    }
}
/**SPI_SSPCR1 (rw) register accessor: SPI_SSPCR1 register

You can [`read`](crate::Reg::read) this register and get [`spi_sspcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPCR1)

For information about available fields see [`mod@spi_sspcr1`] module*/
pub type SPI_SSPCR1 = crate::Reg<spi_sspcr1::SPI_SSPCR1rs>;
///SPI_SSPCR1 register
pub mod spi_sspcr1;
/**SPI_SSPCR2 (rw) register accessor: SPI_SSPCR2 register

You can [`read`](crate::Reg::read) this register and get [`spi_sspcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPCR2)

For information about available fields see [`mod@spi_sspcr2`] module*/
pub type SPI_SSPCR2 = crate::Reg<spi_sspcr2::SPI_SSPCR2rs>;
///SPI_SSPCR2 register
pub mod spi_sspcr2;
/**SPI_SSPSR (rw) register accessor: SPI_SSPSR register

You can [`read`](crate::Reg::read) this register and get [`spi_sspsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPSR)

For information about available fields see [`mod@spi_sspsr`] module*/
pub type SPI_SSPSR = crate::Reg<spi_sspsr::SPI_SSPSRrs>;
///SPI_SSPSR register
pub mod spi_sspsr;
/**SPI_SSPDR (rw) register accessor: SPI_SSPDR register

You can [`read`](crate::Reg::read) this register and get [`spi_sspdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPDR)

For information about available fields see [`mod@spi_sspdr`] module*/
pub type SPI_SSPDR = crate::Reg<spi_sspdr::SPI_SSPDRrs>;
///SPI_SSPDR register
pub mod spi_sspdr;
/**SPI_SSPCRCPR (rw) register accessor: SPI_SSPCRCPR register

You can [`read`](crate::Reg::read) this register and get [`spi_sspcrcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcrcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPCRCPR)

For information about available fields see [`mod@spi_sspcrcpr`] module*/
pub type SPI_SSPCRCPR = crate::Reg<spi_sspcrcpr::SPI_SSPCRCPRrs>;
///SPI_SSPCRCPR register
pub mod spi_sspcrcpr;
/**SPI_SSPRXCRCR (r) register accessor: SPI_SSPRXCRCR register

You can [`read`](crate::Reg::read) this register and get [`spi_ssprxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPRXCRCR)

For information about available fields see [`mod@spi_ssprxcrcr`] module*/
pub type SPI_SSPRXCRCR = crate::Reg<spi_ssprxcrcr::SPI_SSPRXCRCRrs>;
///SPI_SSPRXCRCR register
pub mod spi_ssprxcrcr;
/**SPI_SSPTXCRCR (r) register accessor: SPI_SSPTXCRCR register

You can [`read`](crate::Reg::read) this register and get [`spi_ssptxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPTXCRCR)

For information about available fields see [`mod@spi_ssptxcrcr`] module*/
pub type SPI_SSPTXCRCR = crate::Reg<spi_ssptxcrcr::SPI_SSPTXCRCRrs>;
///SPI_SSPTXCRCR register
pub mod spi_ssptxcrcr;
/**SPI2S_I2SCFGR (rw) register accessor: SPI2S_I2SCFGR register

You can [`read`](crate::Reg::read) this register and get [`spi2s_i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI2S_I2SCFGR)

For information about available fields see [`mod@spi2s_i2scfgr`] module*/
pub type SPI2S_I2SCFGR = crate::Reg<spi2s_i2scfgr::SPI2S_I2SCFGRrs>;
///SPI2S_I2SCFGR register
pub mod spi2s_i2scfgr;
/**SPI2S_I2SPR (rw) register accessor: SPI2S_I2SPR register

You can [`read`](crate::Reg::read) this register and get [`spi2s_i2spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI2S_I2SPR)

For information about available fields see [`mod@spi2s_i2spr`] module*/
pub type SPI2S_I2SPR = crate::Reg<spi2s_i2spr::SPI2S_I2SPRrs>;
///SPI2S_I2SPR register
pub mod spi2s_i2spr;
