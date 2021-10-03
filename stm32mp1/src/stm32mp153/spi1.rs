#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI/I2S control register 1"]
    pub spi2s_cr1: crate::Reg<spi2s_cr1::SPI2S_CR1_SPEC>,
    #[doc = "0x04 - SPI control register 2"]
    pub spi_cr2: crate::Reg<spi_cr2::SPI_CR2_SPEC>,
    #[doc = "0x08 - Content of this register is write protected when SPI is enabled"]
    pub spi_cfg1: crate::Reg<spi_cfg1::SPI_CFG1_SPEC>,
    #[doc = "0x0c - The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register."]
    pub spi_cfg2: crate::Reg<spi_cfg2::SPI_CFG2_SPEC>,
    #[doc = "0x10 - SPI/I2S interrupt enable register"]
    pub spi2s_ier: crate::Reg<spi2s_ier::SPI2S_IER_SPEC>,
    #[doc = "0x14 - SPI/I2S status register"]
    pub spi2s_sr: crate::Reg<spi2s_sr::SPI2S_SR_SPEC>,
    #[doc = "0x18 - SPI/I2S interrupt/status flags clear register"]
    pub spi2s_ifcr: crate::Reg<spi2s_ifcr::SPI2S_IFCR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - SPI/I2S transmit data register"]
    pub spi2s_txdr: crate::Reg<spi2s_txdr::SPI2S_TXDR_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - SPI/I2S receive data register"]
    pub spi2s_rxdr: crate::Reg<spi2s_rxdr::SPI2S_RXDR_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - SPI polynomial register"]
    pub spi_crcpoly: crate::Reg<spi_crcpoly::SPI_CRCPOLY_SPEC>,
    #[doc = "0x44 - SPI transmitter CRC register"]
    pub spi_txcrc: crate::Reg<spi_txcrc::SPI_TXCRC_SPEC>,
    #[doc = "0x48 - SPI receiver CRC register"]
    pub spi_rxcrc: crate::Reg<spi_rxcrc::SPI_RXCRC_SPEC>,
    #[doc = "0x4c - SPI underrun data register"]
    pub spi_udrdr: crate::Reg<spi_udrdr::SPI_UDRDR_SPEC>,
    #[doc = "0x50 - All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode."]
    pub spi_i2scfgr: crate::Reg<spi_i2scfgr::SPI_I2SCFGR_SPEC>,
    _reserved14: [u8; 0x039c],
    #[doc = "0x3f0 - SPI/I2S hardware configuration register"]
    pub spi_i2s_hwcfgr: crate::Reg<spi_i2s_hwcfgr::SPI_I2S_HWCFGR_SPEC>,
    #[doc = "0x3f4 - SPI/I2S version register"]
    pub spi_verr: crate::Reg<spi_verr::SPI_VERR_SPEC>,
    #[doc = "0x3f8 - SPI/I2S identification register"]
    pub spi_ipidr: crate::Reg<spi_ipidr::SPI_IPIDR_SPEC>,
    #[doc = "0x3fc - SPI/I2S size identification register"]
    pub spi_sidr: crate::Reg<spi_sidr::SPI_SIDR_SPEC>,
}
#[doc = "SPI2S_CR1 register accessor: an alias for `Reg<SPI2S_CR1_SPEC>`"]
pub type SPI2S_CR1 = crate::Reg<spi2s_cr1::SPI2S_CR1_SPEC>;
#[doc = "SPI/I2S control register 1"]
pub mod spi2s_cr1;
#[doc = "SPI2S_IER register accessor: an alias for `Reg<SPI2S_IER_SPEC>`"]
pub type SPI2S_IER = crate::Reg<spi2s_ier::SPI2S_IER_SPEC>;
#[doc = "SPI/I2S interrupt enable register"]
pub mod spi2s_ier;
#[doc = "SPI2S_SR register accessor: an alias for `Reg<SPI2S_SR_SPEC>`"]
pub type SPI2S_SR = crate::Reg<spi2s_sr::SPI2S_SR_SPEC>;
#[doc = "SPI/I2S status register"]
pub mod spi2s_sr;
#[doc = "SPI2S_IFCR register accessor: an alias for `Reg<SPI2S_IFCR_SPEC>`"]
pub type SPI2S_IFCR = crate::Reg<spi2s_ifcr::SPI2S_IFCR_SPEC>;
#[doc = "SPI/I2S interrupt/status flags clear register"]
pub mod spi2s_ifcr;
#[doc = "SPI2S_TXDR register accessor: an alias for `Reg<SPI2S_TXDR_SPEC>`"]
pub type SPI2S_TXDR = crate::Reg<spi2s_txdr::SPI2S_TXDR_SPEC>;
#[doc = "SPI/I2S transmit data register"]
pub mod spi2s_txdr;
#[doc = "SPI2S_RXDR register accessor: an alias for `Reg<SPI2S_RXDR_SPEC>`"]
pub type SPI2S_RXDR = crate::Reg<spi2s_rxdr::SPI2S_RXDR_SPEC>;
#[doc = "SPI/I2S receive data register"]
pub mod spi2s_rxdr;
#[doc = "SPI_CR2 register accessor: an alias for `Reg<SPI_CR2_SPEC>`"]
pub type SPI_CR2 = crate::Reg<spi_cr2::SPI_CR2_SPEC>;
#[doc = "SPI control register 2"]
pub mod spi_cr2;
#[doc = "SPI_CFG1 register accessor: an alias for `Reg<SPI_CFG1_SPEC>`"]
pub type SPI_CFG1 = crate::Reg<spi_cfg1::SPI_CFG1_SPEC>;
#[doc = "Content of this register is write protected when SPI is enabled"]
pub mod spi_cfg1;
#[doc = "SPI_CFG2 register accessor: an alias for `Reg<SPI_CFG2_SPEC>`"]
pub type SPI_CFG2 = crate::Reg<spi_cfg2::SPI_CFG2_SPEC>;
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register."]
pub mod spi_cfg2;
#[doc = "SPI_CRCPOLY register accessor: an alias for `Reg<SPI_CRCPOLY_SPEC>`"]
pub type SPI_CRCPOLY = crate::Reg<spi_crcpoly::SPI_CRCPOLY_SPEC>;
#[doc = "SPI polynomial register"]
pub mod spi_crcpoly;
#[doc = "SPI_TXCRC register accessor: an alias for `Reg<SPI_TXCRC_SPEC>`"]
pub type SPI_TXCRC = crate::Reg<spi_txcrc::SPI_TXCRC_SPEC>;
#[doc = "SPI transmitter CRC register"]
pub mod spi_txcrc;
#[doc = "SPI_RXCRC register accessor: an alias for `Reg<SPI_RXCRC_SPEC>`"]
pub type SPI_RXCRC = crate::Reg<spi_rxcrc::SPI_RXCRC_SPEC>;
#[doc = "SPI receiver CRC register"]
pub mod spi_rxcrc;
#[doc = "SPI_UDRDR register accessor: an alias for `Reg<SPI_UDRDR_SPEC>`"]
pub type SPI_UDRDR = crate::Reg<spi_udrdr::SPI_UDRDR_SPEC>;
#[doc = "SPI underrun data register"]
pub mod spi_udrdr;
#[doc = "SPI_I2SCFGR register accessor: an alias for `Reg<SPI_I2SCFGR_SPEC>`"]
pub type SPI_I2SCFGR = crate::Reg<spi_i2scfgr::SPI_I2SCFGR_SPEC>;
#[doc = "All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode."]
pub mod spi_i2scfgr;
#[doc = "SPI_I2S_HWCFGR register accessor: an alias for `Reg<SPI_I2S_HWCFGR_SPEC>`"]
pub type SPI_I2S_HWCFGR = crate::Reg<spi_i2s_hwcfgr::SPI_I2S_HWCFGR_SPEC>;
#[doc = "SPI/I2S hardware configuration register"]
pub mod spi_i2s_hwcfgr;
#[doc = "SPI_VERR register accessor: an alias for `Reg<SPI_VERR_SPEC>`"]
pub type SPI_VERR = crate::Reg<spi_verr::SPI_VERR_SPEC>;
#[doc = "SPI/I2S version register"]
pub mod spi_verr;
#[doc = "SPI_IPIDR register accessor: an alias for `Reg<SPI_IPIDR_SPEC>`"]
pub type SPI_IPIDR = crate::Reg<spi_ipidr::SPI_IPIDR_SPEC>;
#[doc = "SPI/I2S identification register"]
pub mod spi_ipidr;
#[doc = "SPI_SIDR register accessor: an alias for `Reg<SPI_SIDR_SPEC>`"]
pub type SPI_SIDR = crate::Reg<spi_sidr::SPI_SIDR_SPEC>;
#[doc = "SPI/I2S size identification register"]
pub mod spi_sidr;
