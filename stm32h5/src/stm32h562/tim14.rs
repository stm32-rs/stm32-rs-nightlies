#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x0a],
    dier: DIER,
    sr: SR,
    egr: EGR,
    _reserved4: [u8; 0x02],
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    ccer: CCER,
    _reserved6: [u8; 0x02],
    cnt: CNT,
    psc: PSC,
    _reserved8: [u8; 0x02],
    arr: ARR,
    _reserved9: [u8; 0x04],
    ccr: [CCR; 1],
    _reserved10: [u8; 0x24],
    tisel: TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM14 control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x0c - TIM14 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - TIM14 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - TIM14 event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - TIM14 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM14 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM14 capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - TIM14 counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - TIM14 prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - TIM14 auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
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
    ///0x5c - TIM14 timer input selection register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
}
pub use crate::stm32h562::tim13::arr;
pub use crate::stm32h562::tim13::ccer;
pub use crate::stm32h562::tim13::ccmr1_input;
pub use crate::stm32h562::tim13::ccmr1_output;
pub use crate::stm32h562::tim13::ccr;
pub use crate::stm32h562::tim13::cnt;
pub use crate::stm32h562::tim13::cr1;
pub use crate::stm32h562::tim13::dier;
pub use crate::stm32h562::tim13::egr;
pub use crate::stm32h562::tim13::psc;
pub use crate::stm32h562::tim13::sr;
pub use crate::stm32h562::tim13::ARR;
pub use crate::stm32h562::tim13::CCER;
pub use crate::stm32h562::tim13::CCMR1_INPUT;
pub use crate::stm32h562::tim13::CCMR1_OUTPUT;
pub use crate::stm32h562::tim13::CCR;
pub use crate::stm32h562::tim13::CNT;
pub use crate::stm32h562::tim13::CR1;
pub use crate::stm32h562::tim13::DIER;
pub use crate::stm32h562::tim13::EGR;
pub use crate::stm32h562::tim13::PSC;
pub use crate::stm32h562::tim13::SR;
/**TISEL (rw) register accessor: TIM14 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM14:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///TIM14 timer input selection register
pub mod tisel;
