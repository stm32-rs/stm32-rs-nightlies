#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    _reserved2: [u8; 0x04],
    dier: DIER,
    sr: SR,
    egr: EGR,
    _reserved_5_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    ccer: CCER,
    cnt: CNT,
    psc: PSC,
    arr: ARR,
    rcr: RCR,
    ccr: [CCR; 1],
    _reserved12: [u8; 0x0c],
    bdtr: BDTR,
    dcr: DCR,
    dmar: DMAR,
    or1: OR1,
    _reserved16: [u8; 0x0c],
    af1: AF1,
    _reserved17: [u8; 0x04],
    tisel: TISEL,
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
    ///0x0c - DMA/Interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - capture/compare mode register (output mode)
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x30 - repetition counter register
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
    ///0x44 - break and dead-time register
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    ///0x48 - DMA control register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x4c - DMA address for full transfer
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
    ///0x50 - TIM option register 1
    #[inline(always)]
    pub const fn or1(&self) -> &OR1 {
        &self.or1
    }
    ///0x60 - alternate function register 1
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
    ///0x68 - input selection register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
pub use crate::stm32wb55::tim16::arr;
pub use crate::stm32wb55::tim16::bdtr;
pub use crate::stm32wb55::tim16::ccer;
pub use crate::stm32wb55::tim16::ccmr1_input;
pub use crate::stm32wb55::tim16::ccmr1_output;
pub use crate::stm32wb55::tim16::ccr;
pub use crate::stm32wb55::tim16::cnt;
pub use crate::stm32wb55::tim16::cr1;
pub use crate::stm32wb55::tim16::cr2;
pub use crate::stm32wb55::tim16::dcr;
pub use crate::stm32wb55::tim16::dier;
pub use crate::stm32wb55::tim16::dmar;
pub use crate::stm32wb55::tim16::egr;
pub use crate::stm32wb55::tim16::psc;
pub use crate::stm32wb55::tim16::rcr;
pub use crate::stm32wb55::tim16::sr;
pub use crate::stm32wb55::tim16::ARR;
pub use crate::stm32wb55::tim16::BDTR;
pub use crate::stm32wb55::tim16::CCER;
pub use crate::stm32wb55::tim16::CCMR1_INPUT;
pub use crate::stm32wb55::tim16::CCMR1_OUTPUT;
pub use crate::stm32wb55::tim16::CCR;
pub use crate::stm32wb55::tim16::CNT;
pub use crate::stm32wb55::tim16::CR1;
pub use crate::stm32wb55::tim16::CR2;
pub use crate::stm32wb55::tim16::DCR;
pub use crate::stm32wb55::tim16::DIER;
pub use crate::stm32wb55::tim16::DMAR;
pub use crate::stm32wb55::tim16::EGR;
pub use crate::stm32wb55::tim16::PSC;
pub use crate::stm32wb55::tim16::RCR;
pub use crate::stm32wb55::tim16::SR;
/**OR1 (rw) register accessor: TIM option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM17:OR1)

For information about available fields see [`mod@or1`] module*/
pub type OR1 = crate::Reg<or1::OR1rs>;
///TIM option register 1
pub mod or1;
/**AF1 (rw) register accessor: alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM17:AF1)

For information about available fields see [`mod@af1`] module*/
pub type AF1 = crate::Reg<af1::AF1rs>;
///alternate function register 1
pub mod af1;
/**TISEL (rw) register accessor: input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM17:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///input selection register
pub mod tisel;
