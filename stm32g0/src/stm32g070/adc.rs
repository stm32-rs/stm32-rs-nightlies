#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ier: IER,
    cr: CR,
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    smpr: SMPR,
    _reserved6: [u8; 0x08],
    awd1tr: AWD1TR,
    awd2tr: AWD2TR,
    _reserved_8_chselr0: [u8; 0x04],
    awd3tr: AWD3TR,
    _reserved10: [u8; 0x10],
    dr: DR,
    _reserved11: [u8; 0x5c],
    awd2cr: AWD2CR,
    awd3cr: AWD3CR,
    _reserved13: [u8; 0x0c],
    calfact: CALFACT,
    _reserved14: [u8; 0x0250],
    ccr: CCR,
    _reserved15: [u8; 0xcc],
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    hwcfgr0: HWCFGR0,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - ADC interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - ADC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x08 - ADC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - ADC configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x10 - ADC configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x14 - ADC sampling time register
    #[inline(always)]
    pub const fn smpr(&self) -> &SMPR {
        &self.smpr
    }
    ///0x20 - watchdog threshold register
    #[inline(always)]
    pub const fn awd1tr(&self) -> &AWD1TR {
        &self.awd1tr
    }
    ///0x24 - watchdog threshold register
    #[inline(always)]
    pub const fn awd2tr(&self) -> &AWD2TR {
        &self.awd2tr
    }
    ///0x28 - channel selection register CHSELRMOD = 1 in ADC_CFGR1
    #[inline(always)]
    pub const fn chselr1(&self) -> &CHSELR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - channel selection register
    #[inline(always)]
    pub const fn chselr0(&self) -> &CHSELR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - watchdog threshold register
    #[inline(always)]
    pub const fn awd3tr(&self) -> &AWD3TR {
        &self.awd3tr
    }
    ///0x40 - ADC group regular conversion data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0xa0 - ADC analog watchdog 2 configuration register
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    ///0xa4 - ADC analog watchdog 3 configuration register
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    ///0xb4 - ADC calibration factors register
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
    ///0x308 - ADC common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x3d8 - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    ///0x3dc - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    ///0x3e0 - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    ///0x3e4 - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    ///0x3e8 - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3ec - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f0 - Hardware Configuration Register
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
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
/**ISR (rw) register accessor: ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:ISR)

For information about available fields see [`mod@isr`]
module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///ADC interrupt and status register
pub mod isr;
/**IER (rw) register accessor: ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:IER)

For information about available fields see [`mod@ier`]
module*/
pub type IER = crate::Reg<ier::IERrs>;
///ADC interrupt enable register
pub mod ier;
/**CR (rw) register accessor: ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///ADC control register
pub mod cr;
/**CFGR1 (rw) register accessor: ADC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CFGR1)

For information about available fields see [`mod@cfgr1`]
module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///ADC configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CFGR2)

For information about available fields see [`mod@cfgr2`]
module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///ADC configuration register 2
pub mod cfgr2;
/**SMPR (rw) register accessor: ADC sampling time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:SMPR)

For information about available fields see [`mod@smpr`]
module*/
pub type SMPR = crate::Reg<smpr::SMPRrs>;
///ADC sampling time register
pub mod smpr;
/**AWD1TR (rw) register accessor: watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:AWD1TR)

For information about available fields see [`mod@awd1tr`]
module*/
pub type AWD1TR = crate::Reg<awd1tr::AWD1TRrs>;
///watchdog threshold register
pub mod awd1tr;
/**AWD2TR (rw) register accessor: watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd2tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:AWD2TR)

For information about available fields see [`mod@awd2tr`]
module*/
pub type AWD2TR = crate::Reg<awd2tr::AWD2TRrs>;
///watchdog threshold register
pub mod awd2tr;
/**CHSELR0 (rw) register accessor: channel selection register

You can [`read`](crate::Reg::read) this register and get [`chselr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CHSELR0)

For information about available fields see [`mod@chselr0`]
module*/
pub type CHSELR0 = crate::Reg<chselr0::CHSELR0rs>;
///channel selection register
pub mod chselr0;
/**CHSELR1 (rw) register accessor: channel selection register CHSELRMOD = 1 in ADC_CFGR1

You can [`read`](crate::Reg::read) this register and get [`chselr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CHSELR1)

For information about available fields see [`mod@chselr1`]
module*/
pub type CHSELR1 = crate::Reg<chselr1::CHSELR1rs>;
///channel selection register CHSELRMOD = 1 in ADC_CFGR1
pub mod chselr1;
/**AWD3TR (rw) register accessor: watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd3tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:AWD3TR)

For information about available fields see [`mod@awd3tr`]
module*/
pub type AWD3TR = crate::Reg<awd3tr::AWD3TRrs>;
///watchdog threshold register
pub mod awd3tr;
/**DR (r) register accessor: ADC group regular conversion data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:DR)

For information about available fields see [`mod@dr`]
module*/
pub type DR = crate::Reg<dr::DRrs>;
///ADC group regular conversion data register
pub mod dr;
/**AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:AWD2CR)

For information about available fields see [`mod@awd2cr`]
module*/
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
///ADC analog watchdog 2 configuration register
pub mod awd2cr;
/**AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:AWD3CR)

For information about available fields see [`mod@awd3cr`]
module*/
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
///ADC analog watchdog 3 configuration register
pub mod awd3cr;
/**CALFACT (rw) register accessor: ADC calibration factors register

You can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CALFACT)

For information about available fields see [`mod@calfact`]
module*/
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
///ADC calibration factors register
pub mod calfact;
/**CCR (rw) register accessor: ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:CCR)

For information about available fields see [`mod@ccr`]
module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common control register
pub mod ccr;
/**HWCFGR6 (rw) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR6)

For information about available fields see [`mod@hwcfgr6`]
module*/
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
///Hardware Configuration Register
pub mod hwcfgr6;
/**HWCFGR5 (rw) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR5)

For information about available fields see [`mod@hwcfgr5`]
module*/
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
///Hardware Configuration Register
pub mod hwcfgr5;
/**HWCFGR4 (rw) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR4)

For information about available fields see [`mod@hwcfgr4`]
module*/
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
///Hardware Configuration Register
pub mod hwcfgr4;
/**HWCFGR3 (rw) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR3)

For information about available fields see [`mod@hwcfgr3`]
module*/
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
///Hardware Configuration Register
pub mod hwcfgr3;
/**HWCFGR2 (rw) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`]
module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///Hardware Configuration Register
pub mod hwcfgr2;
/**HWCFGR1 (rw) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`]
module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///Hardware Configuration Register
pub mod hwcfgr1;
/**HWCFGR0 (r) register accessor: Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:HWCFGR0)

For information about available fields see [`mod@hwcfgr0`]
module*/
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
///Hardware Configuration Register
pub mod hwcfgr0;
/**VERR (r) register accessor: EXTI IP Version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:VERR)

For information about available fields see [`mod@verr`]
module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///EXTI IP Version register
pub mod verr;
/**IPIDR (r) register accessor: EXTI Identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:IPIDR)

For information about available fields see [`mod@ipidr`]
module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///EXTI Identification register
pub mod ipidr;
/**SIDR (r) register accessor: EXTI Size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC:SIDR)

For information about available fields see [`mod@sidr`]
module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///EXTI Size ID register
pub mod sidr;
