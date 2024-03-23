#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi2s_cr1: SPI2S_CR1,
    spi_cr2: SPI_CR2,
    spi_cfg1: SPI_CFG1,
    spi_cfg2: SPI_CFG2,
    spi2s_ier: SPI2S_IER,
    spi2s_sr: SPI2S_SR,
    spi2s_ifcr: SPI2S_IFCR,
    _reserved7: [u8; 0x04],
    spi2s_txdr: SPI2S_TXDR,
    _reserved8: [u8; 0x0c],
    spi2s_rxdr: SPI2S_RXDR,
    _reserved9: [u8; 0x0c],
    spi_crcpoly: SPI_CRCPOLY,
    spi_txcrc: SPI_TXCRC,
    spi_rxcrc: SPI_RXCRC,
    spi_udrdr: SPI_UDRDR,
    spi_i2scfgr: SPI_I2SCFGR,
    _reserved14: [u8; 0x039c],
    spi_i2s_hwcfgr: SPI_I2S_HWCFGR,
    spi_verr: SPI_VERR,
    spi_ipidr: SPI_IPIDR,
    spi_sidr: SPI_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI/I2S control register 1"]
    #[inline(always)]
    pub const fn spi2s_cr1(&self) -> &SPI2S_CR1 {
        &self.spi2s_cr1
    }
    #[doc = "0x04 - SPI control register 2"]
    #[inline(always)]
    pub const fn spi_cr2(&self) -> &SPI_CR2 {
        &self.spi_cr2
    }
    #[doc = "0x08 - Content of this register is write protected when SPI is enabled"]
    #[inline(always)]
    pub const fn spi_cfg1(&self) -> &SPI_CFG1 {
        &self.spi_cfg1
    }
    #[doc = "0x0c - The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register."]
    #[inline(always)]
    pub const fn spi_cfg2(&self) -> &SPI_CFG2 {
        &self.spi_cfg2
    }
    #[doc = "0x10 - SPI/I2S interrupt enable register"]
    #[inline(always)]
    pub const fn spi2s_ier(&self) -> &SPI2S_IER {
        &self.spi2s_ier
    }
    #[doc = "0x14 - SPI/I2S status register"]
    #[inline(always)]
    pub const fn spi2s_sr(&self) -> &SPI2S_SR {
        &self.spi2s_sr
    }
    #[doc = "0x18 - SPI/I2S interrupt/status flags clear register"]
    #[inline(always)]
    pub const fn spi2s_ifcr(&self) -> &SPI2S_IFCR {
        &self.spi2s_ifcr
    }
    #[doc = "0x20 - SPI/I2S transmit data register"]
    #[inline(always)]
    pub const fn spi2s_txdr(&self) -> &SPI2S_TXDR {
        &self.spi2s_txdr
    }
    #[doc = "0x30 - SPI/I2S receive data register"]
    #[inline(always)]
    pub const fn spi2s_rxdr(&self) -> &SPI2S_RXDR {
        &self.spi2s_rxdr
    }
    #[doc = "0x40 - SPI polynomial register"]
    #[inline(always)]
    pub const fn spi_crcpoly(&self) -> &SPI_CRCPOLY {
        &self.spi_crcpoly
    }
    #[doc = "0x44 - SPI transmitter CRC register"]
    #[inline(always)]
    pub const fn spi_txcrc(&self) -> &SPI_TXCRC {
        &self.spi_txcrc
    }
    #[doc = "0x48 - SPI receiver CRC register"]
    #[inline(always)]
    pub const fn spi_rxcrc(&self) -> &SPI_RXCRC {
        &self.spi_rxcrc
    }
    #[doc = "0x4c - SPI underrun data register"]
    #[inline(always)]
    pub const fn spi_udrdr(&self) -> &SPI_UDRDR {
        &self.spi_udrdr
    }
    #[doc = "0x50 - All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode."]
    #[inline(always)]
    pub const fn spi_i2scfgr(&self) -> &SPI_I2SCFGR {
        &self.spi_i2scfgr
    }
    #[doc = "0x3f0 - SPI/I2S hardware configuration register"]
    #[inline(always)]
    pub const fn spi_i2s_hwcfgr(&self) -> &SPI_I2S_HWCFGR {
        &self.spi_i2s_hwcfgr
    }
    #[doc = "0x3f4 - SPI/I2S version register"]
    #[inline(always)]
    pub const fn spi_verr(&self) -> &SPI_VERR {
        &self.spi_verr
    }
    #[doc = "0x3f8 - SPI/I2S identification register"]
    #[inline(always)]
    pub const fn spi_ipidr(&self) -> &SPI_IPIDR {
        &self.spi_ipidr
    }
    #[doc = "0x3fc - SPI/I2S size identification register"]
    #[inline(always)]
    pub const fn spi_sidr(&self) -> &SPI_SIDR {
        &self.spi_sidr
    }
}
#[doc = "SPI2S_CR1 (rw) register accessor: SPI/I2S control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_cr1`]
module"]
pub type SPI2S_CR1 = crate::Reg<spi2s_cr1::SPI2S_CR1rs>;
#[doc = "SPI/I2S control register 1"]
pub mod spi2s_cr1;
#[doc = "SPI2S_IER (rw) register accessor: SPI/I2S interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_ier`]
module"]
pub type SPI2S_IER = crate::Reg<spi2s_ier::SPI2S_IERrs>;
#[doc = "SPI/I2S interrupt enable register"]
pub mod spi2s_ier;
#[doc = "SPI2S_SR (r) register accessor: SPI/I2S status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_sr`]
module"]
pub type SPI2S_SR = crate::Reg<spi2s_sr::SPI2S_SRrs>;
#[doc = "SPI/I2S status register"]
pub mod spi2s_sr;
#[doc = "SPI2S_IFCR (w) register accessor: SPI/I2S interrupt/status flags clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_ifcr`]
module"]
pub type SPI2S_IFCR = crate::Reg<spi2s_ifcr::SPI2S_IFCRrs>;
#[doc = "SPI/I2S interrupt/status flags clear register"]
pub mod spi2s_ifcr;
#[doc = "SPI2S_TXDR (w) register accessor: SPI/I2S transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_txdr`]
module"]
pub type SPI2S_TXDR = crate::Reg<spi2s_txdr::SPI2S_TXDRrs>;
#[doc = "SPI/I2S transmit data register"]
pub mod spi2s_txdr;
#[doc = "SPI2S_RXDR (r) register accessor: SPI/I2S receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2s_rxdr`]
module"]
pub type SPI2S_RXDR = crate::Reg<spi2s_rxdr::SPI2S_RXDRrs>;
#[doc = "SPI/I2S receive data register"]
pub mod spi2s_rxdr;
#[doc = "SPI_CR2 (rw) register accessor: SPI control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cr2`]
module"]
pub type SPI_CR2 = crate::Reg<spi_cr2::SPI_CR2rs>;
#[doc = "SPI control register 2"]
pub mod spi_cr2;
#[doc = "SPI_CFG1 (rw) register accessor: Content of this register is write protected when SPI is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cfg1`]
module"]
pub type SPI_CFG1 = crate::Reg<spi_cfg1::SPI_CFG1rs>;
#[doc = "Content of this register is write protected when SPI is enabled"]
pub mod spi_cfg1;
#[doc = "SPI_CFG2 (rw) register accessor: The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cfg2`]
module"]
pub type SPI_CFG2 = crate::Reg<spi_cfg2::SPI_CFG2rs>;
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register."]
pub mod spi_cfg2;
#[doc = "SPI_CRCPOLY (rw) register accessor: SPI polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_crcpoly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_crcpoly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_crcpoly`]
module"]
pub type SPI_CRCPOLY = crate::Reg<spi_crcpoly::SPI_CRCPOLYrs>;
#[doc = "SPI polynomial register"]
pub mod spi_crcpoly;
#[doc = "SPI_TXCRC (r) register accessor: SPI transmitter CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txcrc`]
module"]
pub type SPI_TXCRC = crate::Reg<spi_txcrc::SPI_TXCRCrs>;
#[doc = "SPI transmitter CRC register"]
pub mod spi_txcrc;
#[doc = "SPI_RXCRC (r) register accessor: SPI receiver CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxcrc`]
module"]
pub type SPI_RXCRC = crate::Reg<spi_rxcrc::SPI_RXCRCrs>;
#[doc = "SPI receiver CRC register"]
pub mod spi_rxcrc;
#[doc = "SPI_UDRDR (rw) register accessor: SPI underrun data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_udrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_udrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_udrdr`]
module"]
pub type SPI_UDRDR = crate::Reg<spi_udrdr::SPI_UDRDRrs>;
#[doc = "SPI underrun data register"]
pub mod spi_udrdr;
#[doc = "SPI_I2SCFGR (rw) register accessor: All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2scfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2scfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2scfgr`]
module"]
pub type SPI_I2SCFGR = crate::Reg<spi_i2scfgr::SPI_I2SCFGRrs>;
#[doc = "All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode."]
pub mod spi_i2scfgr;
#[doc = "SPI_I2S_HWCFGR (r) register accessor: SPI/I2S hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2s_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2s_hwcfgr`]
module"]
pub type SPI_I2S_HWCFGR = crate::Reg<spi_i2s_hwcfgr::SPI_I2S_HWCFGRrs>;
#[doc = "SPI/I2S hardware configuration register"]
pub mod spi_i2s_hwcfgr;
#[doc = "SPI_VERR (r) register accessor: SPI/I2S version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_verr`]
module"]
pub type SPI_VERR = crate::Reg<spi_verr::SPI_VERRrs>;
#[doc = "SPI/I2S version register"]
pub mod spi_verr;
#[doc = "SPI_IPIDR (r) register accessor: SPI/I2S identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ipidr`]
module"]
pub type SPI_IPIDR = crate::Reg<spi_ipidr::SPI_IPIDRrs>;
#[doc = "SPI/I2S identification register"]
pub mod spi_ipidr;
#[doc = "SPI_SIDR (r) register accessor: SPI/I2S size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sidr`]
module"]
pub type SPI_SIDR = crate::Reg<spi_sidr::SPI_SIDRrs>;
#[doc = "SPI/I2S size identification register"]
pub mod spi_sidr;
