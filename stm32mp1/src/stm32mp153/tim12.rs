#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim12_cr1: TIM12_CR1,
    _reserved1: [u8; 0x02],
    tim12_cr2: TIM12_CR2,
    tim12_smcr: TIM12_SMCR,
    tim12_dier: TIM12_DIER,
    _reserved4: [u8; 0x02],
    tim12_sr: TIM12_SR,
    tim12_egr: TIM12_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim12_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    tim12_ccer: TIM12_CCER,
    tim12_cnt: TIM12_CNT,
    tim12_psc: TIM12_PSC,
    _reserved10: [u8; 0x02],
    tim12_arr: TIM12_ARR,
    _reserved11: [u8; 0x06],
    tim12_ccr1: TIM12_CCR1,
    _reserved12: [u8; 0x02],
    tim12_ccr2: TIM12_CCR2,
    _reserved13: [u8; 0x2e],
    tim12_tisel: TIM12_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM12 control register 1
    #[inline(always)]
    pub const fn tim12_cr1(&self) -> &TIM12_CR1 {
        &self.tim12_cr1
    }
    ///0x04 - TIM12 control register 2
    #[inline(always)]
    pub const fn tim12_cr2(&self) -> &TIM12_CR2 {
        &self.tim12_cr2
    }
    ///0x08 - TIM12 slave mode control register
    #[inline(always)]
    pub const fn tim12_smcr(&self) -> &TIM12_SMCR {
        &self.tim12_smcr
    }
    ///0x0c - TIM12 interrupt enable register
    #[inline(always)]
    pub const fn tim12_dier(&self) -> &TIM12_DIER {
        &self.tim12_dier
    }
    ///0x10 - TIM12 status register
    #[inline(always)]
    pub const fn tim12_sr(&self) -> &TIM12_SR {
        &self.tim12_sr
    }
    ///0x14 - TIM12 event generation register
    #[inline(always)]
    pub const fn tim12_egr(&self) -> &TIM12_EGR {
        &self.tim12_egr
    }
    ///0x18 - TIM12 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim12_ccmr1_output(&self) -> &TIM12_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM12 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim12_ccmr1_input(&self) -> &TIM12_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM12 capture/compare enable register
    #[inline(always)]
    pub const fn tim12_ccer(&self) -> &TIM12_CCER {
        &self.tim12_ccer
    }
    ///0x24 - TIM12 counter
    #[inline(always)]
    pub const fn tim12_cnt(&self) -> &TIM12_CNT {
        &self.tim12_cnt
    }
    ///0x28 - TIM12 prescaler
    #[inline(always)]
    pub const fn tim12_psc(&self) -> &TIM12_PSC {
        &self.tim12_psc
    }
    ///0x2c - TIM12 auto-reload register
    #[inline(always)]
    pub const fn tim12_arr(&self) -> &TIM12_ARR {
        &self.tim12_arr
    }
    ///0x34 - TIM12 capture/compare register 1
    #[inline(always)]
    pub const fn tim12_ccr1(&self) -> &TIM12_CCR1 {
        &self.tim12_ccr1
    }
    ///0x38 - TIM12 capture/compare register 2
    #[inline(always)]
    pub const fn tim12_ccr2(&self) -> &TIM12_CCR2 {
        &self.tim12_ccr2
    }
    ///0x68 - TIM12 timer input selection register
    #[inline(always)]
    pub const fn tim12_tisel(&self) -> &TIM12_TISEL {
        &self.tim12_tisel
    }
}
/**TIM12_CR1 (rw) register accessor: TIM12 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim12_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CR1)

For information about available fields see [`mod@tim12_cr1`]
module*/
pub type TIM12_CR1 = crate::Reg<tim12_cr1::TIM12_CR1rs>;
///TIM12 control register 1
pub mod tim12_cr1;
/**TIM12_CR2 (rw) register accessor: TIM12 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim12_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CR2)

For information about available fields see [`mod@tim12_cr2`]
module*/
pub type TIM12_CR2 = crate::Reg<tim12_cr2::TIM12_CR2rs>;
///TIM12 control register 2
pub mod tim12_cr2;
/**TIM12_SMCR (rw) register accessor: TIM12 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim12_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_SMCR)

For information about available fields see [`mod@tim12_smcr`]
module*/
pub type TIM12_SMCR = crate::Reg<tim12_smcr::TIM12_SMCRrs>;
///TIM12 slave mode control register
pub mod tim12_smcr;
/**TIM12_DIER (rw) register accessor: TIM12 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim12_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_DIER)

For information about available fields see [`mod@tim12_dier`]
module*/
pub type TIM12_DIER = crate::Reg<tim12_dier::TIM12_DIERrs>;
///TIM12 interrupt enable register
pub mod tim12_dier;
/**TIM12_SR (rw) register accessor: TIM12 status register

You can [`read`](crate::Reg::read) this register and get [`tim12_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_SR)

For information about available fields see [`mod@tim12_sr`]
module*/
pub type TIM12_SR = crate::Reg<tim12_sr::TIM12_SRrs>;
///TIM12 status register
pub mod tim12_sr;
/**TIM12_EGR (w) register accessor: TIM12 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_EGR)

For information about available fields see [`mod@tim12_egr`]
module*/
pub type TIM12_EGR = crate::Reg<tim12_egr::TIM12_EGRrs>;
///TIM12 event generation register
pub mod tim12_egr;
/**TIM12_CCMR1_input (rw) register accessor: TIM12 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim12_ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CCMR1_input)

For information about available fields see [`mod@tim12_ccmr1_input`]
module*/
#[doc(alias = "TIM12_CCMR1_input")]
pub type TIM12_CCMR1_INPUT = crate::Reg<tim12_ccmr1_input::TIM12_CCMR1_INPUTrs>;
///TIM12 capture/compare mode register 1
pub mod tim12_ccmr1_input;
/**TIM12_CCMR1_output (rw) register accessor: TIM12 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim12_ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CCMR1_output)

For information about available fields see [`mod@tim12_ccmr1_output`]
module*/
#[doc(alias = "TIM12_CCMR1_output")]
pub type TIM12_CCMR1_OUTPUT = crate::Reg<tim12_ccmr1_output::TIM12_CCMR1_OUTPUTrs>;
///TIM12 capture/compare mode register 1
pub mod tim12_ccmr1_output;
/**TIM12_CCER (rw) register accessor: TIM12 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim12_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CCER)

For information about available fields see [`mod@tim12_ccer`]
module*/
pub type TIM12_CCER = crate::Reg<tim12_ccer::TIM12_CCERrs>;
///TIM12 capture/compare enable register
pub mod tim12_ccer;
/**TIM12_CNT (rw) register accessor: TIM12 counter

You can [`read`](crate::Reg::read) this register and get [`tim12_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CNT)

For information about available fields see [`mod@tim12_cnt`]
module*/
pub type TIM12_CNT = crate::Reg<tim12_cnt::TIM12_CNTrs>;
///TIM12 counter
pub mod tim12_cnt;
/**TIM12_PSC (rw) register accessor: TIM12 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim12_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_PSC)

For information about available fields see [`mod@tim12_psc`]
module*/
pub type TIM12_PSC = crate::Reg<tim12_psc::TIM12_PSCrs>;
///TIM12 prescaler
pub mod tim12_psc;
/**TIM12_ARR (rw) register accessor: TIM12 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim12_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_ARR)

For information about available fields see [`mod@tim12_arr`]
module*/
pub type TIM12_ARR = crate::Reg<tim12_arr::TIM12_ARRrs>;
///TIM12 auto-reload register
pub mod tim12_arr;
/**TIM12_CCR1 (rw) register accessor: TIM12 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim12_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CCR1)

For information about available fields see [`mod@tim12_ccr1`]
module*/
pub type TIM12_CCR1 = crate::Reg<tim12_ccr1::TIM12_CCR1rs>;
///TIM12 capture/compare register 1
pub mod tim12_ccr1;
/**TIM12_CCR2 (rw) register accessor: TIM12 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim12_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_CCR2)

For information about available fields see [`mod@tim12_ccr2`]
module*/
pub type TIM12_CCR2 = crate::Reg<tim12_ccr2::TIM12_CCR2rs>;
///TIM12 capture/compare register 2
pub mod tim12_ccr2;
/**TIM12_TISEL (rw) register accessor: TIM12 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim12_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:TIM12_TISEL)

For information about available fields see [`mod@tim12_tisel`]
module*/
pub type TIM12_TISEL = crate::Reg<tim12_tisel::TIM12_TISELrs>;
///TIM12 timer input selection register
pub mod tim12_tisel;
