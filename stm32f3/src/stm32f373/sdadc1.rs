#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    isr: ISR,
    clrisr: CLRISR,
    _reserved4: [u8; 0x04],
    jchgr: JCHGR,
    _reserved5: [u8; 0x08],
    conf0r: CONF0R,
    conf1r: CONF1R,
    conf2r: CONF2R,
    _reserved8: [u8; 0x14],
    confchr1: CONFCHR1,
    confchr2: CONFCHR2,
    _reserved10: [u8; 0x18],
    jdatar: JDATAR,
    rdatar: RDATAR,
    _reserved12: [u8; 0x08],
    jdata12r: JDATA12R,
    rdata12r: RDATA12R,
    jdata13r: JDATA13R,
    rdata13r: RDATA13R,
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
    ///0x08 - interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x0c - interrupt and status clear register
    #[inline(always)]
    pub const fn clrisr(&self) -> &CLRISR {
        &self.clrisr
    }
    ///0x14 - injected channel group selection register
    #[inline(always)]
    pub const fn jchgr(&self) -> &JCHGR {
        &self.jchgr
    }
    ///0x20 - configuration 0 register
    #[inline(always)]
    pub const fn conf0r(&self) -> &CONF0R {
        &self.conf0r
    }
    ///0x24 - configuration 1 register
    #[inline(always)]
    pub const fn conf1r(&self) -> &CONF1R {
        &self.conf1r
    }
    ///0x28 - configuration 2 register
    #[inline(always)]
    pub const fn conf2r(&self) -> &CONF2R {
        &self.conf2r
    }
    ///0x40 - channel configuration register 1
    #[inline(always)]
    pub const fn confchr1(&self) -> &CONFCHR1 {
        &self.confchr1
    }
    ///0x44 - channel configuration register 2
    #[inline(always)]
    pub const fn confchr2(&self) -> &CONFCHR2 {
        &self.confchr2
    }
    ///0x60 - data register for injected group
    #[inline(always)]
    pub const fn jdatar(&self) -> &JDATAR {
        &self.jdatar
    }
    ///0x64 - data register for the regular channel
    #[inline(always)]
    pub const fn rdatar(&self) -> &RDATAR {
        &self.rdatar
    }
    ///0x70 - SDADC1 and SDADC2 injected data register
    #[inline(always)]
    pub const fn jdata12r(&self) -> &JDATA12R {
        &self.jdata12r
    }
    ///0x74 - SDADC1 and SDADC2 regular data register
    #[inline(always)]
    pub const fn rdata12r(&self) -> &RDATA12R {
        &self.rdata12r
    }
    ///0x78 - SDADC1 and SDADC3 injected data register
    #[inline(always)]
    pub const fn jdata13r(&self) -> &JDATA13R {
        &self.jdata13r
    }
    ///0x7c - SDADC1 and SDADC3 regular data register
    #[inline(always)]
    pub const fn rdata13r(&self) -> &RDATA13R {
        &self.rdata13r
    }
}
/**CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///control register 1
pub mod cr1;
/**CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///control register 2
pub mod cr2;
/**ISR (r) register accessor: interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///interrupt and status register
pub mod isr;
/**CLRISR (rw) register accessor: interrupt and status clear register

You can [`read`](crate::Reg::read) this register and get [`clrisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CLRISR)

For information about available fields see [`mod@clrisr`] module*/
pub type CLRISR = crate::Reg<clrisr::CLRISRrs>;
///interrupt and status clear register
pub mod clrisr;
/**JCHGR (rw) register accessor: injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JCHGR)

For information about available fields see [`mod@jchgr`] module*/
pub type JCHGR = crate::Reg<jchgr::JCHGRrs>;
///injected channel group selection register
pub mod jchgr;
/**CONF0R (rw) register accessor: configuration 0 register

You can [`read`](crate::Reg::read) this register and get [`conf0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONF0R)

For information about available fields see [`mod@conf0r`] module*/
pub type CONF0R = crate::Reg<conf0r::CONF0Rrs>;
///configuration 0 register
pub mod conf0r;
/**CONF1R (rw) register accessor: configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`conf1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONF1R)

For information about available fields see [`mod@conf1r`] module*/
pub type CONF1R = crate::Reg<conf1r::CONF1Rrs>;
///configuration 1 register
pub mod conf1r;
/**CONF2R (rw) register accessor: configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`conf2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONF2R)

For information about available fields see [`mod@conf2r`] module*/
pub type CONF2R = crate::Reg<conf2r::CONF2Rrs>;
///configuration 2 register
pub mod conf2r;
/**CONFCHR1 (rw) register accessor: channel configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confchr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confchr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONFCHR1)

For information about available fields see [`mod@confchr1`] module*/
pub type CONFCHR1 = crate::Reg<confchr1::CONFCHR1rs>;
///channel configuration register 1
pub mod confchr1;
/**CONFCHR2 (rw) register accessor: channel configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confchr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confchr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONFCHR2)

For information about available fields see [`mod@confchr2`] module*/
pub type CONFCHR2 = crate::Reg<confchr2::CONFCHR2rs>;
///channel configuration register 2
pub mod confchr2;
/**JDATAR (r) register accessor: data register for injected group

You can [`read`](crate::Reg::read) this register and get [`jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JDATAR)

For information about available fields see [`mod@jdatar`] module*/
pub type JDATAR = crate::Reg<jdatar::JDATARrs>;
///data register for injected group
pub mod jdatar;
/**RDATAR (r) register accessor: data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:RDATAR)

For information about available fields see [`mod@rdatar`] module*/
pub type RDATAR = crate::Reg<rdatar::RDATARrs>;
///data register for the regular channel
pub mod rdatar;
/**JDATA12R (r) register accessor: SDADC1 and SDADC2 injected data register

You can [`read`](crate::Reg::read) this register and get [`jdata12r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JDATA12R)

For information about available fields see [`mod@jdata12r`] module*/
pub type JDATA12R = crate::Reg<jdata12r::JDATA12Rrs>;
///SDADC1 and SDADC2 injected data register
pub mod jdata12r;
/**RDATA12R (r) register accessor: SDADC1 and SDADC2 regular data register

You can [`read`](crate::Reg::read) this register and get [`rdata12r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:RDATA12R)

For information about available fields see [`mod@rdata12r`] module*/
pub type RDATA12R = crate::Reg<rdata12r::RDATA12Rrs>;
///SDADC1 and SDADC2 regular data register
pub mod rdata12r;
/**JDATA13R (r) register accessor: SDADC1 and SDADC3 injected data register

You can [`read`](crate::Reg::read) this register and get [`jdata13r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JDATA13R)

For information about available fields see [`mod@jdata13r`] module*/
pub type JDATA13R = crate::Reg<jdata13r::JDATA13Rrs>;
///SDADC1 and SDADC3 injected data register
pub mod jdata13r;
/**RDATA13R (r) register accessor: SDADC1 and SDADC3 regular data register

You can [`read`](crate::Reg::read) this register and get [`rdata13r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:RDATA13R)

For information about available fields see [`mod@rdata13r`] module*/
pub type RDATA13R = crate::Reg<rdata13r::RDATA13Rrs>;
///SDADC1 and SDADC3 regular data register
pub mod rdata13r;
