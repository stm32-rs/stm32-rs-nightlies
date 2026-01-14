#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    spi2s_cr1: SPI2S_CR1,
    cr2: CR2,
    cfg1: CFG1,
    cfg2: CFG2,
    spi2s_ier: SPI2S_IER,
    spi2s_sr: SPI2S_SR,
    spi2s_ifcr: SPI2S_IFCR,
    _reserved7: [u8; 0x04],
    _reserved_7_txdr8: [u8; 0x04],
    _reserved8: [u8; 0x0c],
    _reserved_8_rxdr8: [u8; 0x04],
    _reserved9: [u8; 0x0c],
    crcpoly: CRCPOLY,
    txcrc: TXCRC,
    rxcrc: RXCRC,
    udrdr: UDRDR,
    i2scfgr: I2SCFGR,
    _reserved14: [u8; 0x039c],
    i2s_hwcfgr: I2S_HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - SPI/I2S control register 1
    #[inline(always)]
    pub const fn spi2s_cr1(&self) -> &SPI2S_CR1 {
        &self.spi2s_cr1
    }
    ///0x04 - SPI control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - Content of this register is write protected when SPI is enabled
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    ///0x0c - The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    ///0x10 - SPI/I2S interrupt enable register
    #[inline(always)]
    pub const fn spi2s_ier(&self) -> &SPI2S_IER {
        &self.spi2s_ier
    }
    ///0x14 - SPI/I2S status register
    #[inline(always)]
    pub const fn spi2s_sr(&self) -> &SPI2S_SR {
        &self.spi2s_sr
    }
    ///0x18 - SPI/I2S interrupt/status flags clear register
    #[inline(always)]
    pub const fn spi2s_ifcr(&self) -> &SPI2S_IFCR {
        &self.spi2s_ifcr
    }
    ///0x20 - Direct 8-bit access to transmit data register
    #[inline(always)]
    pub const fn txdr8(&self) -> &TXDR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - Direct 16-bit access to transmit data register
    #[inline(always)]
    pub const fn txdr16(&self) -> &TXDR16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - SPI/I2S transmit data register
    #[inline(always)]
    pub const fn spi2s_txdr(&self) -> &SPI2S_TXDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x30 - Direct 8-bit access to receive data register
    #[inline(always)]
    pub const fn rxdr8(&self) -> &RXDR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x30 - Direct 16-bit access to receive data register
    #[inline(always)]
    pub const fn rxdr16(&self) -> &RXDR16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x30 - SPI/I2S receive data register
    #[inline(always)]
    pub const fn spi2s_rxdr(&self) -> &SPI2S_RXDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    ///0x40 - SPI polynomial register
    #[inline(always)]
    pub const fn crcpoly(&self) -> &CRCPOLY {
        &self.crcpoly
    }
    ///0x44 - SPI transmitter CRC register
    #[inline(always)]
    pub const fn txcrc(&self) -> &TXCRC {
        &self.txcrc
    }
    ///0x48 - SPI receiver CRC register
    #[inline(always)]
    pub const fn rxcrc(&self) -> &RXCRC {
        &self.rxcrc
    }
    ///0x4c - SPI underrun data register
    #[inline(always)]
    pub const fn udrdr(&self) -> &UDRDR {
        &self.udrdr
    }
    ///0x50 - All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.
    #[inline(always)]
    pub const fn i2scfgr(&self) -> &I2SCFGR {
        &self.i2scfgr
    }
    ///0x3f0 - SPI/I2S hardware configuration register
    #[inline(always)]
    pub const fn i2s_hwcfgr(&self) -> &I2S_HWCFGR {
        &self.i2s_hwcfgr
    }
    ///0x3f4 - SPI/I2S version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - SPI/I2S identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - SPI/I2S size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**SPI2S_CR1 (rw) register accessor: SPI/I2S control register 1

You can [`read`](crate::Reg::read) this register and get [`spi2s_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_CR1)

For information about available fields see [`mod@spi2s_cr1`] module*/
pub type SPI2S_CR1 = crate::Reg<spi2s_cr1::SPI2S_CR1rs>;
///SPI/I2S control register 1
pub mod spi2s_cr1;
/**SPI2S_IER (rw) register accessor: SPI/I2S interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`spi2s_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_IER)

For information about available fields see [`mod@spi2s_ier`] module*/
pub type SPI2S_IER = crate::Reg<spi2s_ier::SPI2S_IERrs>;
///SPI/I2S interrupt enable register
pub mod spi2s_ier;
/**SPI2S_SR (r) register accessor: SPI/I2S status register

You can [`read`](crate::Reg::read) this register and get [`spi2s_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_SR)

For information about available fields see [`mod@spi2s_sr`] module*/
pub type SPI2S_SR = crate::Reg<spi2s_sr::SPI2S_SRrs>;
///SPI/I2S status register
pub mod spi2s_sr;
/**SPI2S_IFCR (w) register accessor: SPI/I2S interrupt/status flags clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_IFCR)

For information about available fields see [`mod@spi2s_ifcr`] module*/
pub type SPI2S_IFCR = crate::Reg<spi2s_ifcr::SPI2S_IFCRrs>;
///SPI/I2S interrupt/status flags clear register
pub mod spi2s_ifcr;
/**SPI2S_TXDR (w) register accessor: SPI/I2S transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_TXDR)

For information about available fields see [`mod@spi2s_txdr`] module*/
pub type SPI2S_TXDR = crate::Reg<spi2s_txdr::SPI2S_TXDRrs>;
///SPI/I2S transmit data register
pub mod spi2s_txdr;
/**TXDR16 (w) register accessor: Direct 16-bit access to transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:TXDR16)

For information about available fields see [`mod@txdr16`] module*/
pub type TXDR16 = crate::Reg<txdr16::TXDR16rs>;
///Direct 16-bit access to transmit data register
pub mod txdr16;
/**TXDR8 (w) register accessor: Direct 8-bit access to transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:TXDR8)

For information about available fields see [`mod@txdr8`] module*/
pub type TXDR8 = crate::Reg<txdr8::TXDR8rs>;
///Direct 8-bit access to transmit data register
pub mod txdr8;
/**SPI2S_RXDR (r) register accessor: SPI/I2S receive data register

You can [`read`](crate::Reg::read) this register and get [`spi2s_rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_RXDR)

For information about available fields see [`mod@spi2s_rxdr`] module*/
pub type SPI2S_RXDR = crate::Reg<spi2s_rxdr::SPI2S_RXDRrs>;
///SPI/I2S receive data register
pub mod spi2s_rxdr;
/**RXDR16 (r) register accessor: Direct 16-bit access to receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:RXDR16)

For information about available fields see [`mod@rxdr16`] module*/
pub type RXDR16 = crate::Reg<rxdr16::RXDR16rs>;
///Direct 16-bit access to receive data register
pub mod rxdr16;
/**RXDR8 (r) register accessor: Direct 8-bit access to receive data register

You can [`read`](crate::Reg::read) this register and get [`rxdr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:RXDR8)

For information about available fields see [`mod@rxdr8`] module*/
pub type RXDR8 = crate::Reg<rxdr8::RXDR8rs>;
///Direct 8-bit access to receive data register
pub mod rxdr8;
/**CR2 (rw) register accessor: SPI control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///SPI control register 2
pub mod cr2;
/**CFG1 (rw) register accessor: Content of this register is write protected when SPI is enabled

You can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:CFG1)

For information about available fields see [`mod@cfg1`] module*/
pub type CFG1 = crate::Reg<cfg1::CFG1rs>;
///Content of this register is write protected when SPI is enabled
pub mod cfg1;
/**CFG2 (rw) register accessor: The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:CFG2)

For information about available fields see [`mod@cfg2`] module*/
pub type CFG2 = crate::Reg<cfg2::CFG2rs>;
///The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.
pub mod cfg2;
/**CRCPOLY (rw) register accessor: SPI polynomial register

You can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:CRCPOLY)

For information about available fields see [`mod@crcpoly`] module*/
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLYrs>;
///SPI polynomial register
pub mod crcpoly;
/**TXCRC (r) register accessor: SPI transmitter CRC register

You can [`read`](crate::Reg::read) this register and get [`txcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:TXCRC)

For information about available fields see [`mod@txcrc`] module*/
pub type TXCRC = crate::Reg<txcrc::TXCRCrs>;
///SPI transmitter CRC register
pub mod txcrc;
/**RXCRC (r) register accessor: SPI receiver CRC register

You can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:RXCRC)

For information about available fields see [`mod@rxcrc`] module*/
pub type RXCRC = crate::Reg<rxcrc::RXCRCrs>;
///SPI receiver CRC register
pub mod rxcrc;
/**UDRDR (rw) register accessor: SPI underrun data register

You can [`read`](crate::Reg::read) this register and get [`udrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:UDRDR)

For information about available fields see [`mod@udrdr`] module*/
pub type UDRDR = crate::Reg<udrdr::UDRDRrs>;
///SPI underrun data register
pub mod udrdr;
/**I2SCFGR (rw) register accessor: All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:I2SCFGR)

For information about available fields see [`mod@i2scfgr`] module*/
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGRrs>;
///All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.
pub mod i2scfgr;
/**I2S_HWCFGR (r) register accessor: SPI/I2S hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`i2s_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:I2S_HWCFGR)

For information about available fields see [`mod@i2s_hwcfgr`] module*/
pub type I2S_HWCFGR = crate::Reg<i2s_hwcfgr::I2S_HWCFGRrs>;
///SPI/I2S hardware configuration register
pub mod i2s_hwcfgr;
/**VERR (r) register accessor: SPI/I2S version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///SPI/I2S version register
pub mod verr;
/**IPIDR (r) register accessor: SPI/I2S identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///SPI/I2S identification register
pub mod ipidr;
/**SIDR (r) register accessor: SPI/I2S size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///SPI/I2S size identification register
pub mod sidr;
