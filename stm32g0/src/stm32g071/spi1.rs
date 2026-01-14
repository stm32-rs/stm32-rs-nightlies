#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x02],
    sr: SR,
    _reserved3: [u8; 0x02],
    _reserved_3_dr: [u8; 0x02],
    _reserved4: [u8; 0x02],
    crcpr: CRCPR,
    _reserved5: [u8; 0x02],
    rxcrcr: RXCRCR,
    _reserved6: [u8; 0x02],
    txcrcr: TXCRCR,
    _reserved7: [u8; 0x02],
    i2scfgr: I2SCFGR,
    _reserved8: [u8; 0x02],
    i2spr: I2SPR,
    _reserved9: [u8; 0x03ce],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - Direct 8-bit access to data register
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0c - data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x10 - CRC polynomial register
    #[inline(always)]
    pub const fn crcpr(&self) -> &CRCPR {
        &self.crcpr
    }
    ///0x14 - RX CRC register
    #[inline(always)]
    pub const fn rxcrcr(&self) -> &RXCRCR {
        &self.rxcrcr
    }
    ///0x18 - TX CRC register
    #[inline(always)]
    pub const fn txcrcr(&self) -> &TXCRCR {
        &self.txcrcr
    }
    ///0x1c - configuration register
    #[inline(always)]
    pub const fn i2scfgr(&self) -> &I2SCFGR {
        &self.i2scfgr
    }
    ///0x20 - prescaler register
    #[inline(always)]
    pub const fn i2spr(&self) -> &I2SPR {
        &self.i2spr
    }
    ///0x3f0 - hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - EXTI IP Version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - EXTI Identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - EXTI Size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///control register 1
pub mod cr1;
/**CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///control register 2
pub mod cr2;
/**SR (rw) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**DR (rw) register accessor: data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///data register
pub mod dr;
/**DR8 (rw) register accessor: Direct 8-bit access to data register

You can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:DR8)

For information about available fields see [`mod@dr8`] module*/
pub type DR8 = crate::Reg<dr8::DR8rs>;
///Direct 8-bit access to data register
pub mod dr8;
/**CRCPR (rw) register accessor: CRC polynomial register

You can [`read`](crate::Reg::read) this register and get [`crcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:CRCPR)

For information about available fields see [`mod@crcpr`] module*/
pub type CRCPR = crate::Reg<crcpr::CRCPRrs>;
///CRC polynomial register
pub mod crcpr;
/**RXCRCR (r) register accessor: RX CRC register

You can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:RXCRCR)

For information about available fields see [`mod@rxcrcr`] module*/
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCRrs>;
///RX CRC register
pub mod rxcrcr;
/**TXCRCR (r) register accessor: TX CRC register

You can [`read`](crate::Reg::read) this register and get [`txcrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:TXCRCR)

For information about available fields see [`mod@txcrcr`] module*/
pub type TXCRCR = crate::Reg<txcrcr::TXCRCRrs>;
///TX CRC register
pub mod txcrcr;
/**I2SCFGR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:I2SCFGR)

For information about available fields see [`mod@i2scfgr`] module*/
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGRrs>;
///configuration register
pub mod i2scfgr;
/**I2SPR (rw) register accessor: prescaler register

You can [`read`](crate::Reg::read) this register and get [`i2spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:I2SPR)

For information about available fields see [`mod@i2spr`] module*/
pub type I2SPR = crate::Reg<i2spr::I2SPRrs>;
///prescaler register
pub mod i2spr;
/**HWCFGR (r) register accessor: hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: EXTI IP Version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///EXTI IP Version register
pub mod verr;
/**IPIDR (r) register accessor: EXTI Identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///EXTI Identification register
pub mod ipidr;
/**SIDR (r) register accessor: EXTI Size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#SPI1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///EXTI Size ID register
pub mod sidr;
