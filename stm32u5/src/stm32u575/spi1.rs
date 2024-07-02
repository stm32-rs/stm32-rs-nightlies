#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    spi_cr1: SPI_CR1,
    spi_cr2: SPI_CR2,
    spi_cfg1: SPI_CFG1,
    spi_cfg2: SPI_CFG2,
    spi_ier: SPI_IER,
    spi_sr: SPI_SR,
    spi_ifcr: SPI_IFCR,
    spi_autocr: SPI_AUTOCR,
    spi_txdr: SPI_TXDR,
    _reserved9: [u8; 0x0c],
    spi_rxdr: SPI_RXDR,
    _reserved10: [u8; 0x0c],
    spi_crcpoly: SPI_CRCPOLY,
    spi_txcrc: SPI_TXCRC,
    spi_rxcrc: SPI_RXCRC,
    spi_udrdr: SPI_UDRDR,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn spi_cr1(&self) -> &SPI_CR1 {
        &self.spi_cr1
    }
    ///0x04 -
    #[inline(always)]
    pub const fn spi_cr2(&self) -> &SPI_CR2 {
        &self.spi_cr2
    }
    ///0x08 - SPI configuration register 1
    #[inline(always)]
    pub const fn spi_cfg1(&self) -> &SPI_CFG1 {
        &self.spi_cfg1
    }
    ///0x0c - SPI configuration register 2
    #[inline(always)]
    pub const fn spi_cfg2(&self) -> &SPI_CFG2 {
        &self.spi_cfg2
    }
    ///0x10 -
    #[inline(always)]
    pub const fn spi_ier(&self) -> &SPI_IER {
        &self.spi_ier
    }
    ///0x14 -
    #[inline(always)]
    pub const fn spi_sr(&self) -> &SPI_SR {
        &self.spi_sr
    }
    ///0x18 -
    #[inline(always)]
    pub const fn spi_ifcr(&self) -> &SPI_IFCR {
        &self.spi_ifcr
    }
    ///0x1c -
    #[inline(always)]
    pub const fn spi_autocr(&self) -> &SPI_AUTOCR {
        &self.spi_autocr
    }
    ///0x20 -
    #[inline(always)]
    pub const fn spi_txdr(&self) -> &SPI_TXDR {
        &self.spi_txdr
    }
    ///0x30 -
    #[inline(always)]
    pub const fn spi_rxdr(&self) -> &SPI_RXDR {
        &self.spi_rxdr
    }
    ///0x40 - SPI polynomial register
    #[inline(always)]
    pub const fn spi_crcpoly(&self) -> &SPI_CRCPOLY {
        &self.spi_crcpoly
    }
    ///0x44 -
    #[inline(always)]
    pub const fn spi_txcrc(&self) -> &SPI_TXCRC {
        &self.spi_txcrc
    }
    ///0x48 -
    #[inline(always)]
    pub const fn spi_rxcrc(&self) -> &SPI_RXCRC {
        &self.spi_rxcrc
    }
    ///0x4c - SPI underrun data register
    #[inline(always)]
    pub const fn spi_udrdr(&self) -> &SPI_UDRDR {
        &self.spi_udrdr
    }
}
/**SPI_CR1 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_CR1)

For information about available fields see [`mod@spi_cr1`]
module*/
pub type SPI_CR1 = crate::Reg<spi_cr1::SPI_CR1rs>;
///
pub mod spi_cr1;
/**SPI_CR2 (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_CR2)

For information about available fields see [`mod@spi_cr2`]
module*/
pub type SPI_CR2 = crate::Reg<spi_cr2::SPI_CR2rs>;
///
pub mod spi_cr2;
/**SPI_CFG1 (rw) register accessor: SPI configuration register 1

You can [`read`](crate::Reg::read) this register and get [`spi_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_CFG1)

For information about available fields see [`mod@spi_cfg1`]
module*/
pub type SPI_CFG1 = crate::Reg<spi_cfg1::SPI_CFG1rs>;
///SPI configuration register 1
pub mod spi_cfg1;
/**SPI_CFG2 (rw) register accessor: SPI configuration register 2

You can [`read`](crate::Reg::read) this register and get [`spi_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_CFG2)

For information about available fields see [`mod@spi_cfg2`]
module*/
pub type SPI_CFG2 = crate::Reg<spi_cfg2::SPI_CFG2rs>;
///SPI configuration register 2
pub mod spi_cfg2;
/**SPI_IER (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_IER)

For information about available fields see [`mod@spi_ier`]
module*/
pub type SPI_IER = crate::Reg<spi_ier::SPI_IERrs>;
///
pub mod spi_ier;
/**SPI_SR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_SR)

For information about available fields see [`mod@spi_sr`]
module*/
pub type SPI_SR = crate::Reg<spi_sr::SPI_SRrs>;
///
pub mod spi_sr;
/**SPI_IFCR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_IFCR)

For information about available fields see [`mod@spi_ifcr`]
module*/
pub type SPI_IFCR = crate::Reg<spi_ifcr::SPI_IFCRrs>;
///
pub mod spi_ifcr;
/**SPI_AUTOCR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_autocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_autocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_AUTOCR)

For information about available fields see [`mod@spi_autocr`]
module*/
pub type SPI_AUTOCR = crate::Reg<spi_autocr::SPI_AUTOCRrs>;
///
pub mod spi_autocr;
/**SPI_TXDR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_TXDR)

For information about available fields see [`mod@spi_txdr`]
module*/
pub type SPI_TXDR = crate::Reg<spi_txdr::SPI_TXDRrs>;
///
pub mod spi_txdr;
/**SPI_RXDR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_RXDR)

For information about available fields see [`mod@spi_rxdr`]
module*/
pub type SPI_RXDR = crate::Reg<spi_rxdr::SPI_RXDRrs>;
///
pub mod spi_rxdr;
/**SPI_CRCPOLY (rw) register accessor: SPI polynomial register

You can [`read`](crate::Reg::read) this register and get [`spi_crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_CRCPOLY)

For information about available fields see [`mod@spi_crcpoly`]
module*/
pub type SPI_CRCPOLY = crate::Reg<spi_crcpoly::SPI_CRCPOLYrs>;
///SPI polynomial register
pub mod spi_crcpoly;
/**SPI_TXCRC (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_txcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_TXCRC)

For information about available fields see [`mod@spi_txcrc`]
module*/
pub type SPI_TXCRC = crate::Reg<spi_txcrc::SPI_TXCRCrs>;
///
pub mod spi_txcrc;
/**SPI_RXCRC (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`spi_rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_RXCRC)

For information about available fields see [`mod@spi_rxcrc`]
module*/
pub type SPI_RXCRC = crate::Reg<spi_rxcrc::SPI_RXCRCrs>;
///
pub mod spi_rxcrc;
/**SPI_UDRDR (rw) register accessor: SPI underrun data register

You can [`read`](crate::Reg::read) this register and get [`spi_udrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_udrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_UDRDR)

For information about available fields see [`mod@spi_udrdr`]
module*/
pub type SPI_UDRDR = crate::Reg<spi_udrdr::SPI_UDRDRrs>;
///SPI underrun data register
pub mod spi_udrdr;
