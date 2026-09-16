#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    smcr: SMCR,
    dier: DIER,
    _reserved4: [u8; 0x02],
    sr: SR,
    egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ccer: CCER,
    _reserved_9_cnt: [u8; 0x04],
    psc: PSC,
    _reserved11: [u8; 0x02],
    arr: ARR,
    _reserved12: [u8; 0x02],
    rcr: RCR,
    ccr: (),
    _reserved14: [u8; 0x10],
    bdtr: BDTR,
    dcr: DCR,
    _reserved16: [u8; 0x02],
    dmar: DMAR,
    _reserved17: [u8; 0x04],
    ccmr3_output: CCMR3_OUTPUT,
    ccr5: CCR5,
    ccr6: CCR6,
    _reserved20: [u8; 0x02],
    or: OR,
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
    ///0x08 - slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
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
    ///0x1c - capture/compare mode register 2 (input mode)
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - capture/compare mode register (output mode)
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - 16-bit counter register
    #[inline(always)]
    pub const fn cnt16(&self) -> &CNT16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
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
    ///0x54 - capture/compare mode register 3 (output mode)
    #[inline(always)]
    pub const fn ccmr3_output(&self) -> &CCMR3_OUTPUT {
        &self.ccmr3_output
    }
    ///0x58 - capture/compare register
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    ///0x5c - capture/compare register
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
    ///0x60 - option registers
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
pub use crate::stm32f302::tim1::cr1;
pub use crate::stm32f302::tim1::cr2;
pub use crate::stm32f302::tim1::CR1;
pub use crate::stm32f302::tim1::CR2;
/**SMCR (rw) register accessor: slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#TIM8:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///slave mode control register
pub mod smcr;
pub use crate::stm32f302::tim1::ccer;
pub use crate::stm32f302::tim1::ccmr1_input;
pub use crate::stm32f302::tim1::ccmr1_output;
pub use crate::stm32f302::tim1::ccmr2_input;
pub use crate::stm32f302::tim1::ccmr2_output;
pub use crate::stm32f302::tim1::cnt;
pub use crate::stm32f302::tim1::dier;
pub use crate::stm32f302::tim1::egr;
pub use crate::stm32f302::tim1::sr;
pub use crate::stm32f302::tim1::CCER;
pub use crate::stm32f302::tim1::CCMR1_INPUT;
pub use crate::stm32f302::tim1::CCMR1_OUTPUT;
pub use crate::stm32f302::tim1::CCMR2_INPUT;
pub use crate::stm32f302::tim1::CCMR2_OUTPUT;
pub use crate::stm32f302::tim1::CNT;
pub use crate::stm32f302::tim1::DIER;
pub use crate::stm32f302::tim1::EGR;
pub use crate::stm32f302::tim1::SR;
/**CNT16 (rw) register accessor: 16-bit counter register

You can [`read`](crate::Reg::read) this register and get [`cnt16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#TIM8:CNT16)

For information about available fields see [`mod@cnt16`] module*/
pub type CNT16 = crate::Reg<cnt16::CNT16rs>;
///16-bit counter register
pub mod cnt16;
pub use crate::stm32f302::tim1::arr;
pub use crate::stm32f302::tim1::bdtr;
pub use crate::stm32f302::tim1::ccmr3_output;
pub use crate::stm32f302::tim1::ccr;
pub use crate::stm32f302::tim1::ccr5;
pub use crate::stm32f302::tim1::ccr6;
pub use crate::stm32f302::tim1::dcr;
pub use crate::stm32f302::tim1::dmar;
pub use crate::stm32f302::tim1::psc;
pub use crate::stm32f302::tim1::rcr;
pub use crate::stm32f302::tim1::ARR;
pub use crate::stm32f302::tim1::BDTR;
pub use crate::stm32f302::tim1::CCMR3_OUTPUT;
pub use crate::stm32f302::tim1::CCR;
pub use crate::stm32f302::tim1::CCR5;
pub use crate::stm32f302::tim1::CCR6;
pub use crate::stm32f302::tim1::DCR;
pub use crate::stm32f302::tim1::DMAR;
pub use crate::stm32f302::tim1::PSC;
pub use crate::stm32f302::tim1::RCR;
/**OR (rw) register accessor: option registers

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#TIM8:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///option registers
pub mod or;
