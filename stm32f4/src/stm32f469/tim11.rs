#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x0a],
    dier: DIER,
    _reserved2: [u8; 0x02],
    sr: SR,
    _reserved3: [u8; 0x02],
    egr: EGR,
    _reserved4: [u8; 0x02],
    _reserved_4_ccmr1: [u8; 0x02],
    _reserved5: [u8; 0x06],
    ccer: CCER,
    _reserved6: [u8; 0x02],
    cnt: CNT,
    _reserved7: [u8; 0x02],
    psc: PSC,
    _reserved8: [u8; 0x02],
    arr: ARR,
    _reserved9: [u8; 0x06],
    ccr: [CCR; 1],
    _reserved10: [u8; 0x1a],
    or: OR,
}
impl RegisterBlock {
    ///0x00 - control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
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
    ///0x18 - capture/compare mode register 1 (output mode)
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
    ///0x50 - option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
pub use crate::stm32f469::tim10::arr;
pub use crate::stm32f469::tim10::ccer;
pub use crate::stm32f469::tim10::ccmr1_input;
pub use crate::stm32f469::tim10::ccmr1_output;
pub use crate::stm32f469::tim10::ccr;
pub use crate::stm32f469::tim10::cnt;
pub use crate::stm32f469::tim10::cr1;
pub use crate::stm32f469::tim10::dier;
pub use crate::stm32f469::tim10::egr;
pub use crate::stm32f469::tim10::psc;
pub use crate::stm32f469::tim10::sr;
pub use crate::stm32f469::tim10::ARR;
pub use crate::stm32f469::tim10::CCER;
pub use crate::stm32f469::tim10::CCMR1_INPUT;
pub use crate::stm32f469::tim10::CCMR1_OUTPUT;
pub use crate::stm32f469::tim10::CCR;
pub use crate::stm32f469::tim10::CNT;
pub use crate::stm32f469::tim10::CR1;
pub use crate::stm32f469::tim10::DIER;
pub use crate::stm32f469::tim10::EGR;
pub use crate::stm32f469::tim10::PSC;
pub use crate::stm32f469::tim10::SR;
/**OR (rw) register accessor: option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#TIM11:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///option register
pub mod or;
