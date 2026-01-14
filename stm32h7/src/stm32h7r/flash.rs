#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    keyr: KEYR,
    _reserved2: [u8; 0x08],
    cr: CR,
    sr: SR,
    fcr: FCR,
    _reserved5: [u8; 0x04],
    ier: IER,
    isr: ISR,
    icr: ICR,
    _reserved8: [u8; 0x04],
    crccr: CRCCR,
    crcsaddr: CRCSADDR,
    crceaddr: CRCEADDR,
    crcdatar: CRCDATAR,
    eccsfaddr: ECCSFADDR,
    eccdfaddr: ECCDFADDR,
    _reserved14: [u8; 0xb8],
    optkeyr: OPTKEYR,
    optcr: OPTCR,
    optisr: OPTISR,
    opticr: OPTICR,
    obkcr: OBKCR,
    _reserved19: [u8; 0x04],
    obkdr0: OBKDR0,
    obkdr1: OBKDR1,
    obkdr2: OBKDR2,
    obkdr3: OBKDR3,
    obkdr4: OBKDR4,
    obkdr5: OBKDR5,
    obkdr6: OBKDR6,
    obkdr7: OBKDR7,
    _reserved27: [u8; 0xc8],
    nvsr: NVSR,
    nvsrp: NVSRP,
    rotsr: ROTSR,
    rotsrp: ROTSRP,
    otplsr: OTPLSR,
    otplsrp: OTPLSRP,
    wrpsr: WRPSR,
    wrpsrp: WRPSRP,
    _reserved35: [u8; 0x10],
    hdpsr: HDPSR,
    hdpsrp: HDPSRP,
    _reserved37: [u8; 0x18],
    epochsr: EPOCHSR,
    epochsrp: EPOCHSRP,
    _reserved39: [u8; 0x08],
    obw1sr: OBW1SR,
    obw1srp: OBW1SRP,
    obw2sr: OBW2SR,
    obw2srp: OBW2SRP,
}
impl RegisterBlock {
    ///0x00 - Access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - FLASH control key register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    ///0x10 - FLASH control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x14 - FLASH status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - FLASH status register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x20 - FLASH interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x24 - FLASH interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x28 - FLASH interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x30 - FLASH CRC control register
    #[inline(always)]
    pub const fn crccr(&self) -> &CRCCR {
        &self.crccr
    }
    ///0x34 - FLASH CRC start address register
    #[inline(always)]
    pub const fn crcsaddr(&self) -> &CRCSADDR {
        &self.crcsaddr
    }
    ///0x38 - FLASH CRC end address register
    #[inline(always)]
    pub const fn crceaddr(&self) -> &CRCEADDR {
        &self.crceaddr
    }
    ///0x3c - FLASH CRC data register
    #[inline(always)]
    pub const fn crcdatar(&self) -> &CRCDATAR {
        &self.crcdatar
    }
    ///0x40 - FLASH ECC single error fail address
    #[inline(always)]
    pub const fn eccsfaddr(&self) -> &ECCSFADDR {
        &self.eccsfaddr
    }
    ///0x44 - FLASH ECC double error fail address
    #[inline(always)]
    pub const fn eccdfaddr(&self) -> &ECCDFADDR {
        &self.eccdfaddr
    }
    ///0x100 - FLASH options key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x104 - FLASH options control register
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        &self.optcr
    }
    ///0x108 - FLASH options interrupt status register
    #[inline(always)]
    pub const fn optisr(&self) -> &OPTISR {
        &self.optisr
    }
    ///0x10c - FLASH options interrupt clear register
    #[inline(always)]
    pub const fn opticr(&self) -> &OPTICR {
        &self.opticr
    }
    ///0x110 - FLASH option byte key control register
    #[inline(always)]
    pub const fn obkcr(&self) -> &OBKCR {
        &self.obkcr
    }
    ///0x118 - FLASH option bytes key data register 0
    #[inline(always)]
    pub const fn obkdr0(&self) -> &OBKDR0 {
        &self.obkdr0
    }
    ///0x11c - FLASH option bytes key data register 1
    #[inline(always)]
    pub const fn obkdr1(&self) -> &OBKDR1 {
        &self.obkdr1
    }
    ///0x120 - FLASH option bytes key data register 2
    #[inline(always)]
    pub const fn obkdr2(&self) -> &OBKDR2 {
        &self.obkdr2
    }
    ///0x124 - FLASH option bytes key data register 3
    #[inline(always)]
    pub const fn obkdr3(&self) -> &OBKDR3 {
        &self.obkdr3
    }
    ///0x128 - FLASH option bytes key data register 4
    #[inline(always)]
    pub const fn obkdr4(&self) -> &OBKDR4 {
        &self.obkdr4
    }
    ///0x12c - FLASH option bytes key data register 5
    #[inline(always)]
    pub const fn obkdr5(&self) -> &OBKDR5 {
        &self.obkdr5
    }
    ///0x130 - FLASH option bytes key data register 6
    #[inline(always)]
    pub const fn obkdr6(&self) -> &OBKDR6 {
        &self.obkdr6
    }
    ///0x134 - FLASH option bytes key data register 7
    #[inline(always)]
    pub const fn obkdr7(&self) -> &OBKDR7 {
        &self.obkdr7
    }
    ///0x200 - FLASH non-volatile status register
    #[inline(always)]
    pub const fn nvsr(&self) -> &NVSR {
        &self.nvsr
    }
    ///0x204 - FLASH security status register programming
    #[inline(always)]
    pub const fn nvsrp(&self) -> &NVSRP {
        &self.nvsrp
    }
    ///0x208 - FLASH RoT status register
    #[inline(always)]
    pub const fn rotsr(&self) -> &ROTSR {
        &self.rotsr
    }
    ///0x20c - FLASH RoT status register programming
    #[inline(always)]
    pub const fn rotsrp(&self) -> &ROTSRP {
        &self.rotsrp
    }
    ///0x210 - FLASH OTP lock status register
    #[inline(always)]
    pub const fn otplsr(&self) -> &OTPLSR {
        &self.otplsr
    }
    ///0x214 - FLASH OTP lock status register programming
    #[inline(always)]
    pub const fn otplsrp(&self) -> &OTPLSRP {
        &self.otplsrp
    }
    ///0x218 - FLASH write protection status register
    #[inline(always)]
    pub const fn wrpsr(&self) -> &WRPSR {
        &self.wrpsr
    }
    ///0x21c - FLASH write protection status register programming
    #[inline(always)]
    pub const fn wrpsrp(&self) -> &WRPSRP {
        &self.wrpsrp
    }
    ///0x230 - FLASH hide protection status register
    #[inline(always)]
    pub const fn hdpsr(&self) -> &HDPSR {
        &self.hdpsr
    }
    ///0x234 - FLASH hide protection status register programming
    #[inline(always)]
    pub const fn hdpsrp(&self) -> &HDPSRP {
        &self.hdpsrp
    }
    ///0x250 - FLASH epoch status register
    #[inline(always)]
    pub const fn epochsr(&self) -> &EPOCHSR {
        &self.epochsr
    }
    ///0x254 - FLASH RoT status register programming
    #[inline(always)]
    pub const fn epochsrp(&self) -> &EPOCHSRP {
        &self.epochsrp
    }
    ///0x260 - FLASH option byte word 1 status register
    #[inline(always)]
    pub const fn obw1sr(&self) -> &OBW1SR {
        &self.obw1sr
    }
    ///0x264 - FLASH option byte word 1 status register programming
    #[inline(always)]
    pub const fn obw1srp(&self) -> &OBW1SRP {
        &self.obw1srp
    }
    ///0x268 - FLASH option byte word 2 status register
    #[inline(always)]
    pub const fn obw2sr(&self) -> &OBW2SR {
        &self.obw2sr
    }
    ///0x26c - FLASH option byte word 2 status register programming
    #[inline(always)]
    pub const fn obw2srp(&self) -> &OBW2SRP {
        &self.obw2srp
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
/**KEYR (w) register accessor: FLASH control key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///FLASH control key register
pub mod keyr;
/**CR (rw) register accessor: FLASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///FLASH control register
pub mod cr;
/**SR (r) register accessor: FLASH status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FLASH status register
pub mod sr;
/**FCR (w) register accessor: FLASH status register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///FLASH status register
pub mod fcr;
/**IER (rw) register accessor: FLASH interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///FLASH interrupt enable register
pub mod ier;
/**ISR (r) register accessor: FLASH interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///FLASH interrupt status register
pub mod isr;
/**ICR (w) register accessor: FLASH interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///FLASH interrupt clear register
pub mod icr;
/**CRCCR (rw) register accessor: FLASH CRC control register

You can [`read`](crate::Reg::read) this register and get [`crccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CRCCR)

For information about available fields see [`mod@crccr`] module*/
pub type CRCCR = crate::Reg<crccr::CRCCRrs>;
///FLASH CRC control register
pub mod crccr;
/**CRCSADDR (rw) register accessor: FLASH CRC start address register

You can [`read`](crate::Reg::read) this register and get [`crcsaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CRCSADDR)

For information about available fields see [`mod@crcsaddr`] module*/
pub type CRCSADDR = crate::Reg<crcsaddr::CRCSADDRrs>;
///FLASH CRC start address register
pub mod crcsaddr;
/**CRCEADDR (rw) register accessor: FLASH CRC end address register

You can [`read`](crate::Reg::read) this register and get [`crceaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CRCEADDR)

For information about available fields see [`mod@crceaddr`] module*/
pub type CRCEADDR = crate::Reg<crceaddr::CRCEADDRrs>;
///FLASH CRC end address register
pub mod crceaddr;
/**CRCDATAR (r) register accessor: FLASH CRC data register

You can [`read`](crate::Reg::read) this register and get [`crcdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CRCDATAR)

For information about available fields see [`mod@crcdatar`] module*/
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATARrs>;
///FLASH CRC data register
pub mod crcdatar;
/**ECCSFADDR (r) register accessor: FLASH ECC single error fail address

You can [`read`](crate::Reg::read) this register and get [`eccsfaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ECCSFADDR)

For information about available fields see [`mod@eccsfaddr`] module*/
pub type ECCSFADDR = crate::Reg<eccsfaddr::ECCSFADDRrs>;
///FLASH ECC single error fail address
pub mod eccsfaddr;
/**ECCDFADDR (r) register accessor: FLASH ECC double error fail address

You can [`read`](crate::Reg::read) this register and get [`eccdfaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ECCDFADDR)

For information about available fields see [`mod@eccdfaddr`] module*/
pub type ECCDFADDR = crate::Reg<eccdfaddr::ECCDFADDRrs>;
///FLASH ECC double error fail address
pub mod eccdfaddr;
/**OPTKEYR (w) register accessor: FLASH options key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH options key register
pub mod optkeyr;
/**OPTCR (rw) register accessor: FLASH options control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OPTCR)

For information about available fields see [`mod@optcr`] module*/
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
///FLASH options control register
pub mod optcr;
/**OPTISR (r) register accessor: FLASH options interrupt status register

You can [`read`](crate::Reg::read) this register and get [`optisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OPTISR)

For information about available fields see [`mod@optisr`] module*/
pub type OPTISR = crate::Reg<optisr::OPTISRrs>;
///FLASH options interrupt status register
pub mod optisr;
/**OPTICR (w) register accessor: FLASH options interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OPTICR)

For information about available fields see [`mod@opticr`] module*/
pub type OPTICR = crate::Reg<opticr::OPTICRrs>;
///FLASH options interrupt clear register
pub mod opticr;
/**OBKCR (rw) register accessor: FLASH option byte key control register

You can [`read`](crate::Reg::read) this register and get [`obkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKCR)

For information about available fields see [`mod@obkcr`] module*/
pub type OBKCR = crate::Reg<obkcr::OBKCRrs>;
///FLASH option byte key control register
pub mod obkcr;
/**OBKDR0 (rw) register accessor: FLASH option bytes key data register 0

You can [`read`](crate::Reg::read) this register and get [`obkdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR0)

For information about available fields see [`mod@obkdr0`] module*/
pub type OBKDR0 = crate::Reg<obkdr0::OBKDR0rs>;
///FLASH option bytes key data register 0
pub mod obkdr0;
/**OBKDR1 (rw) register accessor: FLASH option bytes key data register 1

You can [`read`](crate::Reg::read) this register and get [`obkdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR1)

For information about available fields see [`mod@obkdr1`] module*/
pub type OBKDR1 = crate::Reg<obkdr1::OBKDR1rs>;
///FLASH option bytes key data register 1
pub mod obkdr1;
/**OBKDR2 (rw) register accessor: FLASH option bytes key data register 2

You can [`read`](crate::Reg::read) this register and get [`obkdr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR2)

For information about available fields see [`mod@obkdr2`] module*/
pub type OBKDR2 = crate::Reg<obkdr2::OBKDR2rs>;
///FLASH option bytes key data register 2
pub mod obkdr2;
/**OBKDR3 (rw) register accessor: FLASH option bytes key data register 3

You can [`read`](crate::Reg::read) this register and get [`obkdr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR3)

For information about available fields see [`mod@obkdr3`] module*/
pub type OBKDR3 = crate::Reg<obkdr3::OBKDR3rs>;
///FLASH option bytes key data register 3
pub mod obkdr3;
/**OBKDR4 (rw) register accessor: FLASH option bytes key data register 4

You can [`read`](crate::Reg::read) this register and get [`obkdr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR4)

For information about available fields see [`mod@obkdr4`] module*/
pub type OBKDR4 = crate::Reg<obkdr4::OBKDR4rs>;
///FLASH option bytes key data register 4
pub mod obkdr4;
/**OBKDR5 (rw) register accessor: FLASH option bytes key data register 5

You can [`read`](crate::Reg::read) this register and get [`obkdr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR5)

For information about available fields see [`mod@obkdr5`] module*/
pub type OBKDR5 = crate::Reg<obkdr5::OBKDR5rs>;
///FLASH option bytes key data register 5
pub mod obkdr5;
/**OBKDR6 (rw) register accessor: FLASH option bytes key data register 6

You can [`read`](crate::Reg::read) this register and get [`obkdr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR6)

For information about available fields see [`mod@obkdr6`] module*/
pub type OBKDR6 = crate::Reg<obkdr6::OBKDR6rs>;
///FLASH option bytes key data register 6
pub mod obkdr6;
/**OBKDR7 (rw) register accessor: FLASH option bytes key data register 7

You can [`read`](crate::Reg::read) this register and get [`obkdr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkdr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKDR7)

For information about available fields see [`mod@obkdr7`] module*/
pub type OBKDR7 = crate::Reg<obkdr7::OBKDR7rs>;
///FLASH option bytes key data register 7
pub mod obkdr7;
/**NVSR (r) register accessor: FLASH non-volatile status register

You can [`read`](crate::Reg::read) this register and get [`nvsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:NVSR)

For information about available fields see [`mod@nvsr`] module*/
pub type NVSR = crate::Reg<nvsr::NVSRrs>;
///FLASH non-volatile status register
pub mod nvsr;
/**NVSRP (rw) register accessor: FLASH security status register programming

You can [`read`](crate::Reg::read) this register and get [`nvsrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvsrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:NVSRP)

For information about available fields see [`mod@nvsrp`] module*/
pub type NVSRP = crate::Reg<nvsrp::NVSRPrs>;
///FLASH security status register programming
pub mod nvsrp;
/**ROTSR (r) register accessor: FLASH RoT status register

You can [`read`](crate::Reg::read) this register and get [`rotsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ROTSR)

For information about available fields see [`mod@rotsr`] module*/
pub type ROTSR = crate::Reg<rotsr::ROTSRrs>;
///FLASH RoT status register
pub mod rotsr;
/**ROTSRP (rw) register accessor: FLASH RoT status register programming

You can [`read`](crate::Reg::read) this register and get [`rotsrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rotsrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ROTSRP)

For information about available fields see [`mod@rotsrp`] module*/
pub type ROTSRP = crate::Reg<rotsrp::ROTSRPrs>;
///FLASH RoT status register programming
pub mod rotsrp;
/**OTPLSR (r) register accessor: FLASH OTP lock status register

You can [`read`](crate::Reg::read) this register and get [`otplsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OTPLSR)

For information about available fields see [`mod@otplsr`] module*/
pub type OTPLSR = crate::Reg<otplsr::OTPLSRrs>;
///FLASH OTP lock status register
pub mod otplsr;
/**OTPLSRP (rw) register accessor: FLASH OTP lock status register programming

You can [`read`](crate::Reg::read) this register and get [`otplsrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otplsrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OTPLSRP)

For information about available fields see [`mod@otplsrp`] module*/
pub type OTPLSRP = crate::Reg<otplsrp::OTPLSRPrs>;
///FLASH OTP lock status register programming
pub mod otplsrp;
/**WRPSR (rw) register accessor: FLASH write protection status register

You can [`read`](crate::Reg::read) this register and get [`wrpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:WRPSR)

For information about available fields see [`mod@wrpsr`] module*/
pub type WRPSR = crate::Reg<wrpsr::WRPSRrs>;
///FLASH write protection status register
pub mod wrpsr;
/**WRPSRP (rw) register accessor: FLASH write protection status register programming

You can [`read`](crate::Reg::read) this register and get [`wrpsrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:WRPSRP)

For information about available fields see [`mod@wrpsrp`] module*/
pub type WRPSRP = crate::Reg<wrpsrp::WRPSRPrs>;
///FLASH write protection status register programming
pub mod wrpsrp;
/**HDPSR (r) register accessor: FLASH hide protection status register

You can [`read`](crate::Reg::read) this register and get [`hdpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:HDPSR)

For information about available fields see [`mod@hdpsr`] module*/
pub type HDPSR = crate::Reg<hdpsr::HDPSRrs>;
///FLASH hide protection status register
pub mod hdpsr;
/**HDPSRP (rw) register accessor: FLASH hide protection status register programming

You can [`read`](crate::Reg::read) this register and get [`hdpsrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpsrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:HDPSRP)

For information about available fields see [`mod@hdpsrp`] module*/
pub type HDPSRP = crate::Reg<hdpsrp::HDPSRPrs>;
///FLASH hide protection status register programming
pub mod hdpsrp;
/**EPOCHSR (r) register accessor: FLASH epoch status register

You can [`read`](crate::Reg::read) this register and get [`epochsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:EPOCHSR)

For information about available fields see [`mod@epochsr`] module*/
pub type EPOCHSR = crate::Reg<epochsr::EPOCHSRrs>;
///FLASH epoch status register
pub mod epochsr;
/**EPOCHSRP (rw) register accessor: FLASH RoT status register programming

You can [`read`](crate::Reg::read) this register and get [`epochsrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epochsrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:EPOCHSRP)

For information about available fields see [`mod@epochsrp`] module*/
pub type EPOCHSRP = crate::Reg<epochsrp::EPOCHSRPrs>;
///FLASH RoT status register programming
pub mod epochsrp;
/**OBW1SR (r) register accessor: FLASH option byte word 1 status register

You can [`read`](crate::Reg::read) this register and get [`obw1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBW1SR)

For information about available fields see [`mod@obw1sr`] module*/
pub type OBW1SR = crate::Reg<obw1sr::OBW1SRrs>;
///FLASH option byte word 1 status register
pub mod obw1sr;
/**OBW1SRP (rw) register accessor: FLASH option byte word 1 status register programming

You can [`read`](crate::Reg::read) this register and get [`obw1srp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obw1srp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBW1SRP)

For information about available fields see [`mod@obw1srp`] module*/
pub type OBW1SRP = crate::Reg<obw1srp::OBW1SRPrs>;
///FLASH option byte word 1 status register programming
pub mod obw1srp;
/**OBW2SR (r) register accessor: FLASH option byte word 2 status register

You can [`read`](crate::Reg::read) this register and get [`obw2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBW2SR)

For information about available fields see [`mod@obw2sr`] module*/
pub type OBW2SR = crate::Reg<obw2sr::OBW2SRrs>;
///FLASH option byte word 2 status register
pub mod obw2sr;
/**OBW2SRP (rw) register accessor: FLASH option byte word 2 status register programming

You can [`read`](crate::Reg::read) this register and get [`obw2srp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obw2srp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBW2SRP)

For information about available fields see [`mod@obw2srp`] module*/
pub type OBW2SRP = crate::Reg<obw2srp::OBW2SRPrs>;
///FLASH option byte word 2 status register programming
pub mod obw2srp;
