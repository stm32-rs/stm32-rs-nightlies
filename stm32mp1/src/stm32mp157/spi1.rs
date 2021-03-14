#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI/I2S control register 1"]
    pub spi2s_cr1: SPI2S_CR1,
    #[doc = "0x04 - SPI control register 2"]
    pub spi_cr2: SPI_CR2,
    #[doc = "0x08 - Content of this register is write protected when SPI is enabled"]
    pub spi_cfg1: SPI_CFG1,
    #[doc = "0x0c - The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register."]
    pub spi_cfg2: SPI_CFG2,
    #[doc = "0x10 - SPI/I2S interrupt enable register"]
    pub spi2s_ier: SPI2S_IER,
    #[doc = "0x14 - SPI/I2S status register"]
    pub spi2s_sr: SPI2S_SR,
    #[doc = "0x18 - SPI/I2S interrupt/status flags clear register"]
    pub spi2s_ifcr: SPI2S_IFCR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - SPI/I2S transmit data register"]
    pub spi2s_txdr: SPI2S_TXDR,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - SPI/I2S receive data register"]
    pub spi2s_rxdr: SPI2S_RXDR,
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - SPI polynomial register"]
    pub spi_crcpoly: SPI_CRCPOLY,
    #[doc = "0x44 - SPI transmitter CRC register"]
    pub spi_txcrc: SPI_TXCRC,
    #[doc = "0x48 - SPI receiver CRC register"]
    pub spi_rxcrc: SPI_RXCRC,
    #[doc = "0x4c - SPI underrun data register"]
    pub spi_udrdr: SPI_UDRDR,
    #[doc = "0x50 - All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode."]
    pub spi_i2scfgr: SPI_I2SCFGR,
    _reserved14: [u8; 924usize],
    #[doc = "0x3f0 - SPI/I2S hardware configuration register"]
    pub spi_i2s_hwcfgr: SPI_I2S_HWCFGR,
    #[doc = "0x3f4 - SPI/I2S version register"]
    pub spi_verr: SPI_VERR,
    #[doc = "0x3f8 - SPI/I2S identification register"]
    pub spi_ipidr: SPI_IPIDR,
    #[doc = "0x3fc - SPI/I2S size identification register"]
    pub spi_sidr: SPI_SIDR,
}
#[doc = "SPI/I2S control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_cr1](spi2s_cr1) module"]
pub type SPI2S_CR1 = crate::Reg<u32, _SPI2S_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI2S_CR1;
#[doc = "`read()` method returns [spi2s_cr1::R](spi2s_cr1::R) reader structure"]
impl crate::Readable for SPI2S_CR1 {}
#[doc = "`write(|w| ..)` method takes [spi2s_cr1::W](spi2s_cr1::W) writer structure"]
impl crate::Writable for SPI2S_CR1 {}
#[doc = "SPI/I2S control register 1"]
pub mod spi2s_cr1;
#[doc = "SPI/I2S interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_ier](spi2s_ier) module"]
pub type SPI2S_IER = crate::Reg<u32, _SPI2S_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI2S_IER;
#[doc = "`read()` method returns [spi2s_ier::R](spi2s_ier::R) reader structure"]
impl crate::Readable for SPI2S_IER {}
#[doc = "`write(|w| ..)` method takes [spi2s_ier::W](spi2s_ier::W) writer structure"]
impl crate::Writable for SPI2S_IER {}
#[doc = "SPI/I2S interrupt enable register"]
pub mod spi2s_ier;
#[doc = "SPI/I2S status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_sr](spi2s_sr) module"]
pub type SPI2S_SR = crate::Reg<u32, _SPI2S_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI2S_SR;
#[doc = "`read()` method returns [spi2s_sr::R](spi2s_sr::R) reader structure"]
impl crate::Readable for SPI2S_SR {}
#[doc = "SPI/I2S status register"]
pub mod spi2s_sr;
#[doc = "SPI/I2S interrupt/status flags clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_ifcr](spi2s_ifcr) module"]
pub type SPI2S_IFCR = crate::Reg<u32, _SPI2S_IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI2S_IFCR;
#[doc = "`write(|w| ..)` method takes [spi2s_ifcr::W](spi2s_ifcr::W) writer structure"]
impl crate::Writable for SPI2S_IFCR {}
#[doc = "SPI/I2S interrupt/status flags clear register"]
pub mod spi2s_ifcr;
#[doc = "SPI/I2S transmit data register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_txdr](spi2s_txdr) module"]
pub type SPI2S_TXDR = crate::Reg<u32, _SPI2S_TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI2S_TXDR;
#[doc = "`write(|w| ..)` method takes [spi2s_txdr::W](spi2s_txdr::W) writer structure"]
impl crate::Writable for SPI2S_TXDR {}
#[doc = "SPI/I2S transmit data register"]
pub mod spi2s_txdr;
#[doc = "SPI/I2S receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_rxdr](spi2s_rxdr) module"]
pub type SPI2S_RXDR = crate::Reg<u32, _SPI2S_RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI2S_RXDR;
#[doc = "`read()` method returns [spi2s_rxdr::R](spi2s_rxdr::R) reader structure"]
impl crate::Readable for SPI2S_RXDR {}
#[doc = "SPI/I2S receive data register"]
pub mod spi2s_rxdr;
#[doc = "SPI control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cr2](spi_cr2) module"]
pub type SPI_CR2 = crate::Reg<u32, _SPI_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CR2;
#[doc = "`read()` method returns [spi_cr2::R](spi_cr2::R) reader structure"]
impl crate::Readable for SPI_CR2 {}
#[doc = "`write(|w| ..)` method takes [spi_cr2::W](spi_cr2::W) writer structure"]
impl crate::Writable for SPI_CR2 {}
#[doc = "SPI control register 2"]
pub mod spi_cr2;
#[doc = "Content of this register is write protected when SPI is enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg1](spi_cfg1) module"]
pub type SPI_CFG1 = crate::Reg<u32, _SPI_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CFG1;
#[doc = "`read()` method returns [spi_cfg1::R](spi_cfg1::R) reader structure"]
impl crate::Readable for SPI_CFG1 {}
#[doc = "`write(|w| ..)` method takes [spi_cfg1::W](spi_cfg1::W) writer structure"]
impl crate::Writable for SPI_CFG1 {}
#[doc = "Content of this register is write protected when SPI is enabled"]
pub mod spi_cfg1;
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg2](spi_cfg2) module"]
pub type SPI_CFG2 = crate::Reg<u32, _SPI_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CFG2;
#[doc = "`read()` method returns [spi_cfg2::R](spi_cfg2::R) reader structure"]
impl crate::Readable for SPI_CFG2 {}
#[doc = "`write(|w| ..)` method takes [spi_cfg2::W](spi_cfg2::W) writer structure"]
impl crate::Writable for SPI_CFG2 {}
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register."]
pub mod spi_cfg2;
#[doc = "SPI polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_crcpoly](spi_crcpoly) module"]
pub type SPI_CRCPOLY = crate::Reg<u32, _SPI_CRCPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CRCPOLY;
#[doc = "`read()` method returns [spi_crcpoly::R](spi_crcpoly::R) reader structure"]
impl crate::Readable for SPI_CRCPOLY {}
#[doc = "`write(|w| ..)` method takes [spi_crcpoly::W](spi_crcpoly::W) writer structure"]
impl crate::Writable for SPI_CRCPOLY {}
#[doc = "SPI polynomial register"]
pub mod spi_crcpoly;
#[doc = "SPI transmitter CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_txcrc](spi_txcrc) module"]
pub type SPI_TXCRC = crate::Reg<u32, _SPI_TXCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_TXCRC;
#[doc = "`read()` method returns [spi_txcrc::R](spi_txcrc::R) reader structure"]
impl crate::Readable for SPI_TXCRC {}
#[doc = "SPI transmitter CRC register"]
pub mod spi_txcrc;
#[doc = "SPI receiver CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rxcrc](spi_rxcrc) module"]
pub type SPI_RXCRC = crate::Reg<u32, _SPI_RXCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RXCRC;
#[doc = "`read()` method returns [spi_rxcrc::R](spi_rxcrc::R) reader structure"]
impl crate::Readable for SPI_RXCRC {}
#[doc = "SPI receiver CRC register"]
pub mod spi_rxcrc;
#[doc = "SPI underrun data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_udrdr](spi_udrdr) module"]
pub type SPI_UDRDR = crate::Reg<u32, _SPI_UDRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_UDRDR;
#[doc = "`read()` method returns [spi_udrdr::R](spi_udrdr::R) reader structure"]
impl crate::Readable for SPI_UDRDR {}
#[doc = "`write(|w| ..)` method takes [spi_udrdr::W](spi_udrdr::W) writer structure"]
impl crate::Writable for SPI_UDRDR {}
#[doc = "SPI underrun data register"]
pub mod spi_udrdr;
#[doc = "All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2scfgr](spi_i2scfgr) module"]
pub type SPI_I2SCFGR = crate::Reg<u32, _SPI_I2SCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_I2SCFGR;
#[doc = "`read()` method returns [spi_i2scfgr::R](spi_i2scfgr::R) reader structure"]
impl crate::Readable for SPI_I2SCFGR {}
#[doc = "`write(|w| ..)` method takes [spi_i2scfgr::W](spi_i2scfgr::W) writer structure"]
impl crate::Writable for SPI_I2SCFGR {}
#[doc = "All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode."]
pub mod spi_i2scfgr;
#[doc = "SPI/I2S hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2s_hwcfgr](spi_i2s_hwcfgr) module"]
pub type SPI_I2S_HWCFGR = crate::Reg<u32, _SPI_I2S_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_I2S_HWCFGR;
#[doc = "`read()` method returns [spi_i2s_hwcfgr::R](spi_i2s_hwcfgr::R) reader structure"]
impl crate::Readable for SPI_I2S_HWCFGR {}
#[doc = "SPI/I2S hardware configuration register"]
pub mod spi_i2s_hwcfgr;
#[doc = "SPI/I2S version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_verr](spi_verr) module"]
pub type SPI_VERR = crate::Reg<u32, _SPI_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_VERR;
#[doc = "`read()` method returns [spi_verr::R](spi_verr::R) reader structure"]
impl crate::Readable for SPI_VERR {}
#[doc = "SPI/I2S version register"]
pub mod spi_verr;
#[doc = "SPI/I2S identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ipidr](spi_ipidr) module"]
pub type SPI_IPIDR = crate::Reg<u32, _SPI_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IPIDR;
#[doc = "`read()` method returns [spi_ipidr::R](spi_ipidr::R) reader structure"]
impl crate::Readable for SPI_IPIDR {}
#[doc = "SPI/I2S identification register"]
pub mod spi_ipidr;
#[doc = "SPI/I2S size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sidr](spi_sidr) module"]
pub type SPI_SIDR = crate::Reg<u32, _SPI_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SIDR;
#[doc = "`read()` method returns [spi_sidr::R](spi_sidr::R) reader structure"]
impl crate::Readable for SPI_SIDR {}
#[doc = "SPI/I2S size identification register"]
pub mod spi_sidr;
