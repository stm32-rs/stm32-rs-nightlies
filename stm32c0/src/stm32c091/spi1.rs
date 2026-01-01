#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    spi_cr1: SPI_CR1,
    _reserved1: [u8; 0x02],
    spi_cr2: SPI_CR2,
    _reserved2: [u8; 0x02],
    spi_sr: SPI_SR,
    _reserved3: [u8; 0x02],
    spi_dr: SPI_DR,
    _reserved4: [u8; 0x02],
    spi_crcpr: SPI_CRCPR,
    _reserved5: [u8; 0x02],
    spi_rxcrcr: SPI_RXCRCR,
    _reserved6: [u8; 0x02],
    spi_txcrcr: SPI_TXCRCR,
    _reserved7: [u8; 0x02],
    spi_i2scfgr: SPI_I2SCFGR,
    _reserved8: [u8; 0x02],
    spi_i2spr: SPI_I2SPR,
}
impl RegisterBlock {
    ///0x00 - SPI control register 1
    #[inline(always)]
    pub const fn spi_cr1(&self) -> &SPI_CR1 {
        &self.spi_cr1
    }
    ///0x04 - SPI control register 2
    #[inline(always)]
    pub const fn spi_cr2(&self) -> &SPI_CR2 {
        &self.spi_cr2
    }
    ///0x08 - SPI status register
    #[inline(always)]
    pub const fn spi_sr(&self) -> &SPI_SR {
        &self.spi_sr
    }
    ///0x0c - SPI data register
    #[inline(always)]
    pub const fn spi_dr(&self) -> &SPI_DR {
        &self.spi_dr
    }
    ///0x10 - SPI CRC polynomial register
    #[inline(always)]
    pub const fn spi_crcpr(&self) -> &SPI_CRCPR {
        &self.spi_crcpr
    }
    ///0x14 - SPI Rx CRC register
    #[inline(always)]
    pub const fn spi_rxcrcr(&self) -> &SPI_RXCRCR {
        &self.spi_rxcrcr
    }
    ///0x18 - SPI Tx CRC register
    #[inline(always)]
    pub const fn spi_txcrcr(&self) -> &SPI_TXCRCR {
        &self.spi_txcrcr
    }
    ///0x1c - SPIx_I2S configuration register
    #[inline(always)]
    pub const fn spi_i2scfgr(&self) -> &SPI_I2SCFGR {
        &self.spi_i2scfgr
    }
    ///0x20 - SPIx_I2S prescaler register
    #[inline(always)]
    pub const fn spi_i2spr(&self) -> &SPI_I2SPR {
        &self.spi_i2spr
    }
}
/**SPI_CR1 (rw) register accessor: SPI control register 1

You can [`read`](crate::Reg::read) this register and get [`spi_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_CR1)

For information about available fields see [`mod@spi_cr1`] module*/
pub type SPI_CR1 = crate::Reg<spi_cr1::SPI_CR1rs>;
///SPI control register 1
pub mod spi_cr1;
/**SPI_CR2 (rw) register accessor: SPI control register 2

You can [`read`](crate::Reg::read) this register and get [`spi_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_CR2)

For information about available fields see [`mod@spi_cr2`] module*/
pub type SPI_CR2 = crate::Reg<spi_cr2::SPI_CR2rs>;
///SPI control register 2
pub mod spi_cr2;
/**SPI_SR (rw) register accessor: SPI status register

You can [`read`](crate::Reg::read) this register and get [`spi_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_SR)

For information about available fields see [`mod@spi_sr`] module*/
pub type SPI_SR = crate::Reg<spi_sr::SPI_SRrs>;
///SPI status register
pub mod spi_sr;
/**SPI_DR (rw) register accessor: SPI data register

You can [`read`](crate::Reg::read) this register and get [`spi_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_DR)

For information about available fields see [`mod@spi_dr`] module*/
pub type SPI_DR = crate::Reg<spi_dr::SPI_DRrs>;
///SPI data register
pub mod spi_dr;
/**SPI_CRCPR (rw) register accessor: SPI CRC polynomial register

You can [`read`](crate::Reg::read) this register and get [`spi_crcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_crcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_CRCPR)

For information about available fields see [`mod@spi_crcpr`] module*/
pub type SPI_CRCPR = crate::Reg<spi_crcpr::SPI_CRCPRrs>;
///SPI CRC polynomial register
pub mod spi_crcpr;
/**SPI_RXCRCR (r) register accessor: SPI Rx CRC register

You can [`read`](crate::Reg::read) this register and get [`spi_rxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_RXCRCR)

For information about available fields see [`mod@spi_rxcrcr`] module*/
pub type SPI_RXCRCR = crate::Reg<spi_rxcrcr::SPI_RXCRCRrs>;
///SPI Rx CRC register
pub mod spi_rxcrcr;
/**SPI_TXCRCR (r) register accessor: SPI Tx CRC register

You can [`read`](crate::Reg::read) this register and get [`spi_txcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_TXCRCR)

For information about available fields see [`mod@spi_txcrcr`] module*/
pub type SPI_TXCRCR = crate::Reg<spi_txcrcr::SPI_TXCRCRrs>;
///SPI Tx CRC register
pub mod spi_txcrcr;
/**SPI_I2SCFGR (rw) register accessor: SPIx_I2S configuration register

You can [`read`](crate::Reg::read) this register and get [`spi_i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_I2SCFGR)

For information about available fields see [`mod@spi_i2scfgr`] module*/
pub type SPI_I2SCFGR = crate::Reg<spi_i2scfgr::SPI_I2SCFGRrs>;
///SPIx_I2S configuration register
pub mod spi_i2scfgr;
/**SPI_I2SPR (rw) register accessor: SPIx_I2S prescaler register

You can [`read`](crate::Reg::read) this register and get [`spi_i2spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_i2spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_I2SPR)

For information about available fields see [`mod@spi_i2spr`] module*/
pub type SPI_I2SPR = crate::Reg<spi_i2spr::SPI_I2SPRrs>;
///SPIx_I2S prescaler register
pub mod spi_i2spr;
