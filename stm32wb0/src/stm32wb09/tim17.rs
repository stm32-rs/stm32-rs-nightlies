#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x06],
    dier: DIER,
    _reserved3: [u8; 0x02],
    sr: SR,
    _reserved4: [u8; 0x02],
    egr: EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    ccer: CCER,
    _reserved7: [u8; 0x02],
    _reserved_7_cnt: [u8; 0x04],
    psc: PSC,
    _reserved9: [u8; 0x02],
    arr: ARR,
    _reserved10: [u8; 0x02],
    rcr: RCR,
    _reserved11: [u8; 0x02],
    ccr: [CCR; 1],
    _reserved12: [u8; 0x0e],
    bdtr: BDTR,
    dcr: DCR,
    _reserved14: [u8; 0x02],
    dmar: DMAR,
    _reserved15: [u8; 0x02],
    or1: OR1,
    _reserved16: [u8; 0x0c],
    af1: AF1,
}
impl RegisterBlock {
    ///0x00 - CR1 register
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - CR2 register
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x0c - DIER register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - EGR register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 -
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - CCMR1 register
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - CCER register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - 16-bit counter register
    #[inline(always)]
    pub const fn cnt16(&self) -> &CNT16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - CNT register
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 - PSC register
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - ARR register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x30 - RCR register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    ///0x34 - capture/compare register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CCR1` register.</div>
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x34 - capture/compare register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x34 - capture/compare register
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x44 - BDTR register
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    ///0x48 - DCR register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x4c - DMAR register
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
    ///0x50 - OR1 register
    #[inline(always)]
    pub const fn or1(&self) -> &OR1 {
        &self.or1
    }
    ///0x60 - AF1 register
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
}
pub use crate::stm32wb09::tim16::ccer;
pub use crate::stm32wb09::tim16::ccmr1_input;
pub use crate::stm32wb09::tim16::ccmr1_output;
pub use crate::stm32wb09::tim16::cnt;
pub use crate::stm32wb09::tim16::cr1;
pub use crate::stm32wb09::tim16::cr2;
pub use crate::stm32wb09::tim16::dier;
pub use crate::stm32wb09::tim16::egr;
pub use crate::stm32wb09::tim16::sr;
pub use crate::stm32wb09::tim16::CCER;
pub use crate::stm32wb09::tim16::CCMR1_INPUT;
pub use crate::stm32wb09::tim16::CCMR1_OUTPUT;
pub use crate::stm32wb09::tim16::CNT;
pub use crate::stm32wb09::tim16::CR1;
pub use crate::stm32wb09::tim16::CR2;
pub use crate::stm32wb09::tim16::DIER;
pub use crate::stm32wb09::tim16::EGR;
pub use crate::stm32wb09::tim16::SR;
/**CNT16 (rw) register accessor: 16-bit counter register

You can [`read`](crate::Reg::read) this register and get [`cnt16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TIM17:CNT16)

For information about available fields see [`mod@cnt16`] module*/
pub type CNT16 = crate::Reg<cnt16::CNT16rs>;
///16-bit counter register
pub mod cnt16;
pub use crate::stm32wb09::tim16::arr;
pub use crate::stm32wb09::tim16::bdtr;
pub use crate::stm32wb09::tim16::ccr;
pub use crate::stm32wb09::tim16::dcr;
pub use crate::stm32wb09::tim16::dmar;
pub use crate::stm32wb09::tim16::psc;
pub use crate::stm32wb09::tim16::rcr;
pub use crate::stm32wb09::tim16::ARR;
pub use crate::stm32wb09::tim16::BDTR;
pub use crate::stm32wb09::tim16::CCR;
pub use crate::stm32wb09::tim16::DCR;
pub use crate::stm32wb09::tim16::DMAR;
pub use crate::stm32wb09::tim16::PSC;
pub use crate::stm32wb09::tim16::RCR;
/**OR1 (rw) register accessor: OR1 register

You can [`read`](crate::Reg::read) this register and get [`or1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TIM17:OR1)

For information about available fields see [`mod@or1`] module*/
pub type OR1 = crate::Reg<or1::OR1rs>;
///OR1 register
pub mod or1;
/**AF1 (rw) register accessor: AF1 register

You can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TIM17:AF1)

For information about available fields see [`mod@af1`] module*/
pub type AF1 = crate::Reg<af1::AF1rs>;
///AF1 register
pub mod af1;
