#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    _reserved2: [u8; 0x02],
    smcr: SMCR,
    dier: DIER,
    _reserved4: [u8; 0x02],
    sr: SR,
    _reserved5: [u8; 0x02],
    egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ccer: CCER,
    _reserved9: [u8; 0x02],
    _reserved_9_cnt: [u8; 0x04],
    psc: PSC,
    _reserved11: [u8; 0x02],
    arr: ARR,
    _reserved12: [u8; 0x06],
    ccr: (),
    _reserved13: [u8; 0x14],
    dcr: DCR,
    _reserved14: [u8; 0x02],
    dmar: DMAR,
}
impl RegisterBlock {
    ///0x00 - TIM4 control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - TIM4 control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - TIM4 slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x0c - TIM4 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - TIM4 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - TIM4 event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - TIM4 capture/compare mode register 1
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM4 capture/compare mode register 1
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM4 capture/compare mode register 2
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM4 capture/compare mode register 2
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - TIM4 capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - 16-bit counter register
    #[inline(always)]
    pub const fn cnt16(&self) -> &CNT16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - TIM4 counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 - TIM4 prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - TIM4 auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x34..0x3c - capture/compare register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CCR1` register.</div>
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(52)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x34..0x3c - capture/compare register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(52)
                .add(4 * n)
                .cast()
        })
    }
    ///0x34 - capture/compare register
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x38 - capture/compare register
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR {
        self.ccr(1)
    }
    ///0x3c - capture/compare register
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR {
        self.ccr(2)
    }
    ///0x40 - capture/compare register
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR {
        self.ccr(3)
    }
    ///0x48 - TIM4 DMA control register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x4c - TIM4 DMA address for full transfer
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
}
pub use crate::stm32l4p5::tim3::cr1;
pub use crate::stm32l4p5::tim3::cr2;
pub use crate::stm32l4p5::tim3::CR1;
pub use crate::stm32l4p5::tim3::CR2;
/**SMCR (rw) register accessor: TIM4 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#TIM4:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///TIM4 slave mode control register
pub mod smcr;
pub use crate::stm32l4p5::tim3::ccer;
pub use crate::stm32l4p5::tim3::ccmr1_input;
pub use crate::stm32l4p5::tim3::ccmr1_output;
pub use crate::stm32l4p5::tim3::ccmr2_input;
pub use crate::stm32l4p5::tim3::ccmr2_output;
pub use crate::stm32l4p5::tim3::cnt;
pub use crate::stm32l4p5::tim3::dier;
pub use crate::stm32l4p5::tim3::egr;
pub use crate::stm32l4p5::tim3::sr;
pub use crate::stm32l4p5::tim3::CCER;
pub use crate::stm32l4p5::tim3::CCMR1_INPUT;
pub use crate::stm32l4p5::tim3::CCMR1_OUTPUT;
pub use crate::stm32l4p5::tim3::CCMR2_INPUT;
pub use crate::stm32l4p5::tim3::CCMR2_OUTPUT;
pub use crate::stm32l4p5::tim3::CNT;
pub use crate::stm32l4p5::tim3::DIER;
pub use crate::stm32l4p5::tim3::EGR;
pub use crate::stm32l4p5::tim3::SR;
/**CNT16 (rw) register accessor: 16-bit counter register

You can [`read`](crate::Reg::read) this register and get [`cnt16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#TIM4:CNT16)

For information about available fields see [`mod@cnt16`] module*/
pub type CNT16 = crate::Reg<cnt16::CNT16rs>;
///16-bit counter register
pub mod cnt16;
pub use crate::stm32l4p5::tim3::arr;
pub use crate::stm32l4p5::tim3::ccr;
pub use crate::stm32l4p5::tim3::dcr;
pub use crate::stm32l4p5::tim3::dmar;
pub use crate::stm32l4p5::tim3::psc;
pub use crate::stm32l4p5::tim3::ARR;
pub use crate::stm32l4p5::tim3::CCR;
pub use crate::stm32l4p5::tim3::DCR;
pub use crate::stm32l4p5::tim3::DMAR;
pub use crate::stm32l4p5::tim3::PSC;
