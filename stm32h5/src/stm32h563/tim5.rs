#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    smcr: SMCR,
    dier: DIER,
    sr: SR,
    egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ccer: CCER,
    _reserved9: [u8; 0x02],
    cnt: CNT,
    psc: PSC,
    _reserved11: [u8; 0x02],
    arr: ARR,
    _reserved12: [u8; 0x04],
    ccr: [CCR; 4],
    _reserved13: [u8; 0x14],
    ecr: ECR,
    tisel: TISEL,
    af1: AF1,
    af2: AF2,
    _reserved17: [u8; 0x0374],
    dcr: DCR,
    dmar: DMAR,
}
impl RegisterBlock {
    ///0x00 - TIM5 control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - TIM5 control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - TIM5 slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x0c - TIM5 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - TIM5 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - TIM5 event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - TIM5 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM5 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM5 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM5 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - TIM5 capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - TIM5 counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - TIM5 prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - TIM5 auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x34..0x44 - capture/compare register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CCR1` register.</div>
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x34..0x44 - capture/compare register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
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
    ///0x58 - TIM5 timer encoder control register
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    ///0x5c - TIM5 timer input selection register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
    ///0x60 - TIM5 alternate function register 1
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
    ///0x64 - TIM5 alternate function register 2
    #[inline(always)]
    pub const fn af2(&self) -> &AF2 {
        &self.af2
    }
    ///0x3dc - TIM5 DMA control register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x3e0 - TIM5 DMA address for full transfer
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
}
pub use crate::stm32h563::tim2::cr1;
pub use crate::stm32h563::tim2::cr2;
pub use crate::stm32h563::tim2::CR1;
pub use crate::stm32h563::tim2::CR2;
/**SMCR (rw) register accessor: TIM5 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#TIM5:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///TIM5 slave mode control register
pub mod smcr;
pub use crate::stm32h563::tim2::arr;
pub use crate::stm32h563::tim2::ccer;
pub use crate::stm32h563::tim2::ccmr1_input;
pub use crate::stm32h563::tim2::ccmr1_output;
pub use crate::stm32h563::tim2::ccmr2_input;
pub use crate::stm32h563::tim2::ccmr2_output;
pub use crate::stm32h563::tim2::ccr;
pub use crate::stm32h563::tim2::cnt;
pub use crate::stm32h563::tim2::dier;
pub use crate::stm32h563::tim2::ecr;
pub use crate::stm32h563::tim2::egr;
pub use crate::stm32h563::tim2::psc;
pub use crate::stm32h563::tim2::sr;
pub use crate::stm32h563::tim2::ARR;
pub use crate::stm32h563::tim2::CCER;
pub use crate::stm32h563::tim2::CCMR1_INPUT;
pub use crate::stm32h563::tim2::CCMR1_OUTPUT;
pub use crate::stm32h563::tim2::CCMR2_INPUT;
pub use crate::stm32h563::tim2::CCMR2_OUTPUT;
pub use crate::stm32h563::tim2::CCR;
pub use crate::stm32h563::tim2::CNT;
pub use crate::stm32h563::tim2::DIER;
pub use crate::stm32h563::tim2::ECR;
pub use crate::stm32h563::tim2::EGR;
pub use crate::stm32h563::tim2::PSC;
pub use crate::stm32h563::tim2::SR;
/**TISEL (rw) register accessor: TIM5 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#TIM5:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///TIM5 timer input selection register
pub mod tisel;
/**AF1 (rw) register accessor: TIM5 alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#TIM5:AF1)

For information about available fields see [`mod@af1`] module*/
pub type AF1 = crate::Reg<af1::AF1rs>;
///TIM5 alternate function register 1
pub mod af1;
/**AF2 (rw) register accessor: TIM5 alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#TIM5:AF2)

For information about available fields see [`mod@af2`] module*/
pub type AF2 = crate::Reg<af2::AF2rs>;
///TIM5 alternate function register 2
pub mod af2;
pub use crate::stm32h563::tim2::dcr;
pub use crate::stm32h563::tim2::dmar;
pub use crate::stm32h563::tim2::DCR;
pub use crate::stm32h563::tim2::DMAR;
