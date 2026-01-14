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
    cnt: CNT,
    psc: PSC,
    _reserved11: [u8; 0x02],
    arr: ARR,
    rcr: RCR,
    _reserved13: [u8; 0x02],
    ccr1: CCR1,
    ccr2: CCR2,
    ccr3: CCR3,
    ccr4: CCR4,
    bdtr: BDTR,
    ccr5: CCR5,
    ccr6: CCR6,
    ccmr3: CCMR3,
    dtr2: DTR2,
    ecr: ECR,
    tisel: TISEL,
    af1: AF1,
    af2: AF2,
    _reserved26: [u8; 0x0374],
    dcr: DCR,
    dmar: DMAR,
}
impl RegisterBlock {
    ///0x00 - TIM8 control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - TIM8 control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - TIM8 slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x0c - TIM8 DMA/interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - TIM8 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - TIM8 event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - TIM8 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM8 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM8 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM8 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - TIM8 capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - TIM8 counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - TIM8 prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - TIM8 auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x30 - TIM8 repetition counter register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    ///0x34 - TIM8 capture/compare register 1
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    ///0x38 - TIM8 capture/compare register 2
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    ///0x3c - TIM8 capture/compare register 3
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    ///0x40 - TIM8 capture/compare register 4
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
    ///0x44 - TIM8 break and dead-time register
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    ///0x48 - TIM8 capture/compare register 5
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    ///0x4c - TIM8 capture/compare register 6
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
    ///0x50 - TIM8 capture/compare mode register 3
    #[inline(always)]
    pub const fn ccmr3(&self) -> &CCMR3 {
        &self.ccmr3
    }
    ///0x54 - TIM8 timer deadtime register 2
    #[inline(always)]
    pub const fn dtr2(&self) -> &DTR2 {
        &self.dtr2
    }
    ///0x58 - TIM8 timer encoder control register
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    ///0x5c - TIM8 timer input selection register
    #[inline(always)]
    pub const fn tisel(&self) -> &TISEL {
        &self.tisel
    }
    ///0x60 - TIM8 alternate function option register 1
    #[inline(always)]
    pub const fn af1(&self) -> &AF1 {
        &self.af1
    }
    ///0x64 - TIM8 alternate function register 2
    #[inline(always)]
    pub const fn af2(&self) -> &AF2 {
        &self.af2
    }
    ///0x3dc - TIM8 DMA control register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x3e0 - TIM8 DMA address for full transfer
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
}
/**CR1 (rw) register accessor: TIM8 control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///TIM8 control register 1
pub mod cr1;
/**CR2 (rw) register accessor: TIM8 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///TIM8 control register 2
pub mod cr2;
/**SMCR (rw) register accessor: TIM8 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///TIM8 slave mode control register
pub mod smcr;
/**DIER (rw) register accessor: TIM8 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///TIM8 DMA/interrupt enable register
pub mod dier;
/**SR (rw) register accessor: TIM8 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///TIM8 status register
pub mod sr;
/**EGR (w) register accessor: TIM8 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:EGR)

For information about available fields see [`mod@egr`] module*/
pub type EGR = crate::Reg<egr::EGRrs>;
///TIM8 event generation register
pub mod egr;
/**CCMR1_Input (rw) register accessor: TIM8 capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCMR1_Input)

For information about available fields see [`mod@ccmr1_input`] module*/
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUTrs>;
///TIM8 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_input;
/**CCMR1_Output (rw) register accessor: TIM8 capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCMR1_Output)

For information about available fields see [`mod@ccmr1_output`] module*/
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUTrs>;
///TIM8 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_output;
/**CCMR2_Input (rw) register accessor: TIM8 capture/compare mode register 2 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCMR2_Input)

For information about available fields see [`mod@ccmr2_input`] module*/
#[doc(alias = "CCMR2_Input")]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUTrs>;
///TIM8 capture/compare mode register 2 \[alternate\]
pub mod ccmr2_input;
/**CCMR2_Output (rw) register accessor: TIM8 capture/compare mode register 2 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCMR2_Output)

For information about available fields see [`mod@ccmr2_output`] module*/
#[doc(alias = "CCMR2_Output")]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUTrs>;
///TIM8 capture/compare mode register 2 \[alternate\]
pub mod ccmr2_output;
/**CCER (rw) register accessor: TIM8 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCER)

For information about available fields see [`mod@ccer`] module*/
pub type CCER = crate::Reg<ccer::CCERrs>;
///TIM8 capture/compare enable register
pub mod ccer;
/**CNT (rw) register accessor: TIM8 counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///TIM8 counter
pub mod cnt;
/**PSC (rw) register accessor: TIM8 prescaler

You can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:PSC)

For information about available fields see [`mod@psc`] module*/
pub type PSC = crate::Reg<psc::PSCrs>;
///TIM8 prescaler
pub mod psc;
/**ARR (rw) register accessor: TIM8 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///TIM8 auto-reload register
pub mod arr;
/**RCR (rw) register accessor: TIM8 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///TIM8 repetition counter register
pub mod rcr;
/**CCR1 (rw) register accessor: TIM8 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCR1)

For information about available fields see [`mod@ccr1`] module*/
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
///TIM8 capture/compare register 1
pub mod ccr1;
/**CCR2 (rw) register accessor: TIM8 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCR2)

For information about available fields see [`mod@ccr2`] module*/
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
///TIM8 capture/compare register 2
pub mod ccr2;
/**CCR3 (rw) register accessor: TIM8 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCR3)

For information about available fields see [`mod@ccr3`] module*/
pub type CCR3 = crate::Reg<ccr3::CCR3rs>;
///TIM8 capture/compare register 3
pub mod ccr3;
/**CCR4 (rw) register accessor: TIM8 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCR4)

For information about available fields see [`mod@ccr4`] module*/
pub type CCR4 = crate::Reg<ccr4::CCR4rs>;
///TIM8 capture/compare register 4
pub mod ccr4;
/**BDTR (rw) register accessor: TIM8 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:BDTR)

For information about available fields see [`mod@bdtr`] module*/
pub type BDTR = crate::Reg<bdtr::BDTRrs>;
///TIM8 break and dead-time register
pub mod bdtr;
/**CCR5 (rw) register accessor: TIM8 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCR5)

For information about available fields see [`mod@ccr5`] module*/
pub type CCR5 = crate::Reg<ccr5::CCR5rs>;
///TIM8 capture/compare register 5
pub mod ccr5;
/**CCR6 (rw) register accessor: TIM8 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCR6)

For information about available fields see [`mod@ccr6`] module*/
pub type CCR6 = crate::Reg<ccr6::CCR6rs>;
///TIM8 capture/compare register 6
pub mod ccr6;
/**CCMR3 (rw) register accessor: TIM8 capture/compare mode register 3

You can [`read`](crate::Reg::read) this register and get [`ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:CCMR3)

For information about available fields see [`mod@ccmr3`] module*/
pub type CCMR3 = crate::Reg<ccmr3::CCMR3rs>;
///TIM8 capture/compare mode register 3
pub mod ccmr3;
/**DTR2 (rw) register accessor: TIM8 timer deadtime register 2

You can [`read`](crate::Reg::read) this register and get [`dtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:DTR2)

For information about available fields see [`mod@dtr2`] module*/
pub type DTR2 = crate::Reg<dtr2::DTR2rs>;
///TIM8 timer deadtime register 2
pub mod dtr2;
/**ECR (rw) register accessor: TIM8 timer encoder control register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:ECR)

For information about available fields see [`mod@ecr`] module*/
pub type ECR = crate::Reg<ecr::ECRrs>;
///TIM8 timer encoder control register
pub mod ecr;
/**TISEL (rw) register accessor: TIM8 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:TISEL)

For information about available fields see [`mod@tisel`] module*/
pub type TISEL = crate::Reg<tisel::TISELrs>;
///TIM8 timer input selection register
pub mod tisel;
/**AF1 (rw) register accessor: TIM8 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:AF1)

For information about available fields see [`mod@af1`] module*/
pub type AF1 = crate::Reg<af1::AF1rs>;
///TIM8 alternate function option register 1
pub mod af1;
/**AF2 (rw) register accessor: TIM8 alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:AF2)

For information about available fields see [`mod@af2`] module*/
pub type AF2 = crate::Reg<af2::AF2rs>;
///TIM8 alternate function register 2
pub mod af2;
/**DCR (rw) register accessor: TIM8 DMA control register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///TIM8 DMA control register
pub mod dcr;
/**DMAR (rw) register accessor: TIM8 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM8:DMAR)

For information about available fields see [`mod@dmar`] module*/
pub type DMAR = crate::Reg<dmar::DMARrs>;
///TIM8 DMA address for full transfer
pub mod dmar;
