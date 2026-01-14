#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim1_cr1: TIM1_CR1,
    _reserved1: [u8; 0x02],
    tim1_cr2: TIM1_CR2,
    tim1_smcr: TIM1_SMCR,
    tim1_dier: TIM1_DIER,
    _reserved4: [u8; 0x02],
    tim1_sr: TIM1_SR,
    tim1_egr: TIM1_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim1_ccmr: [u8; 0x04],
    _reserved_7_tim1_ccmr: [u8; 0x04],
    tim1_ccer: TIM1_CCER,
    tim1_cnt: TIM1_CNT,
    tim1_psc: TIM1_PSC,
    _reserved11: [u8; 0x02],
    tim1_arr: TIM1_ARR,
    _reserved12: [u8; 0x02],
    tim1_rcr: TIM1_RCR,
    _reserved13: [u8; 0x02],
    tim1_ccr1: TIM1_CCR1,
    _reserved14: [u8; 0x02],
    tim1_ccr2: TIM1_CCR2,
    _reserved15: [u8; 0x02],
    tim1_ccr3: TIM1_CCR3,
    _reserved16: [u8; 0x02],
    tim1_ccr4: TIM1_CCR4,
    _reserved17: [u8; 0x02],
    tim1_bdtr: TIM1_BDTR,
    tim1_dcr: TIM1_DCR,
    _reserved19: [u8; 0x02],
    tim1_dmar: TIM1_DMAR,
    _reserved20: [u8; 0x04],
    tim1_ccmr3: TIM1_CCMR3,
    tim1_ccr5: TIM1_CCR5,
    tim1_ccr6: TIM1_CCR6,
    _reserved23: [u8; 0x02],
    tim1_af1: TIM1_AF1,
    tim1_af2: TIM1_AF2,
    tim1_tisel: TIM1_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM1 control register 1
    #[inline(always)]
    pub const fn tim1_cr1(&self) -> &TIM1_CR1 {
        &self.tim1_cr1
    }
    ///0x04 - TIM1 control register 2
    #[inline(always)]
    pub const fn tim1_cr2(&self) -> &TIM1_CR2 {
        &self.tim1_cr2
    }
    ///0x08 - TIM1 slave mode control register
    #[inline(always)]
    pub const fn tim1_smcr(&self) -> &TIM1_SMCR {
        &self.tim1_smcr
    }
    ///0x0c - TIM1 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim1_dier(&self) -> &TIM1_DIER {
        &self.tim1_dier
    }
    ///0x10 - TIM1 status register
    #[inline(always)]
    pub const fn tim1_sr(&self) -> &TIM1_SR {
        &self.tim1_sr
    }
    ///0x14 - TIM1 event generation register
    #[inline(always)]
    pub const fn tim1_egr(&self) -> &TIM1_EGR {
        &self.tim1_egr
    }
    ///0x18 - TIM1 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim1_ccmr1_alternate1(&self) -> &TIM1_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM1 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim1_ccmr1(&self) -> &TIM1_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM1 capture/compare mode register 2
    #[inline(always)]
    pub const fn tim1_ccmr2_alternate1(&self) -> &TIM1_CCMR2_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM1 capture/compare mode register 2
    #[inline(always)]
    pub const fn tim1_ccmr2(&self) -> &TIM1_CCMR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - TIM1 capture/compare enable register
    #[inline(always)]
    pub const fn tim1_ccer(&self) -> &TIM1_CCER {
        &self.tim1_ccer
    }
    ///0x24 - TIM1 counter
    #[inline(always)]
    pub const fn tim1_cnt(&self) -> &TIM1_CNT {
        &self.tim1_cnt
    }
    ///0x28 - TIM1 prescaler
    #[inline(always)]
    pub const fn tim1_psc(&self) -> &TIM1_PSC {
        &self.tim1_psc
    }
    ///0x2c - TIM1 auto-reload register
    #[inline(always)]
    pub const fn tim1_arr(&self) -> &TIM1_ARR {
        &self.tim1_arr
    }
    ///0x30 - TIM1 repetition counter register
    #[inline(always)]
    pub const fn tim1_rcr(&self) -> &TIM1_RCR {
        &self.tim1_rcr
    }
    ///0x34 - TIM1 capture/compare register 1
    #[inline(always)]
    pub const fn tim1_ccr1(&self) -> &TIM1_CCR1 {
        &self.tim1_ccr1
    }
    ///0x38 - TIM1 capture/compare register 2
    #[inline(always)]
    pub const fn tim1_ccr2(&self) -> &TIM1_CCR2 {
        &self.tim1_ccr2
    }
    ///0x3c - TIM1 capture/compare register 3
    #[inline(always)]
    pub const fn tim1_ccr3(&self) -> &TIM1_CCR3 {
        &self.tim1_ccr3
    }
    ///0x40 - TIM1 capture/compare register 4
    #[inline(always)]
    pub const fn tim1_ccr4(&self) -> &TIM1_CCR4 {
        &self.tim1_ccr4
    }
    ///0x44 - TIM1 break and dead-time register
    #[inline(always)]
    pub const fn tim1_bdtr(&self) -> &TIM1_BDTR {
        &self.tim1_bdtr
    }
    ///0x48 - TIM1 DMA control register
    #[inline(always)]
    pub const fn tim1_dcr(&self) -> &TIM1_DCR {
        &self.tim1_dcr
    }
    ///0x4c - TIM1 DMA address for full transfer
    #[inline(always)]
    pub const fn tim1_dmar(&self) -> &TIM1_DMAR {
        &self.tim1_dmar
    }
    ///0x54 - TIM1 capture/compare mode register 3
    #[inline(always)]
    pub const fn tim1_ccmr3(&self) -> &TIM1_CCMR3 {
        &self.tim1_ccmr3
    }
    ///0x58 - TIM1 capture/compare register 5
    #[inline(always)]
    pub const fn tim1_ccr5(&self) -> &TIM1_CCR5 {
        &self.tim1_ccr5
    }
    ///0x5c - TIM1 capture/compare register 6
    #[inline(always)]
    pub const fn tim1_ccr6(&self) -> &TIM1_CCR6 {
        &self.tim1_ccr6
    }
    ///0x60 - TIM1 alternate function option register 1
    #[inline(always)]
    pub const fn tim1_af1(&self) -> &TIM1_AF1 {
        &self.tim1_af1
    }
    ///0x64 - TIM1 Alternate function register 2
    #[inline(always)]
    pub const fn tim1_af2(&self) -> &TIM1_AF2 {
        &self.tim1_af2
    }
    ///0x68 - TIM1 timer input selection register
    #[inline(always)]
    pub const fn tim1_tisel(&self) -> &TIM1_TISEL {
        &self.tim1_tisel
    }
}
/**TIM1_CR1 (rw) register accessor: TIM1 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CR1)

For information about available fields see [`mod@tim1_cr1`] module*/
pub type TIM1_CR1 = crate::Reg<tim1_cr1::TIM1_CR1rs>;
///TIM1 control register 1
pub mod tim1_cr1;
/**TIM1_CR2 (rw) register accessor: TIM1 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CR2)

For information about available fields see [`mod@tim1_cr2`] module*/
pub type TIM1_CR2 = crate::Reg<tim1_cr2::TIM1_CR2rs>;
///TIM1 control register 2
pub mod tim1_cr2;
/**TIM1_SMCR (rw) register accessor: TIM1 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim1_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_SMCR)

For information about available fields see [`mod@tim1_smcr`] module*/
pub type TIM1_SMCR = crate::Reg<tim1_smcr::TIM1_SMCRrs>;
///TIM1 slave mode control register
pub mod tim1_smcr;
/**TIM1_DIER (rw) register accessor: TIM1 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim1_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_DIER)

For information about available fields see [`mod@tim1_dier`] module*/
pub type TIM1_DIER = crate::Reg<tim1_dier::TIM1_DIERrs>;
///TIM1 DMA/interrupt enable register
pub mod tim1_dier;
/**TIM1_SR (rw) register accessor: TIM1 status register

You can [`read`](crate::Reg::read) this register and get [`tim1_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_SR)

For information about available fields see [`mod@tim1_sr`] module*/
pub type TIM1_SR = crate::Reg<tim1_sr::TIM1_SRrs>;
///TIM1 status register
pub mod tim1_sr;
/**TIM1_EGR (w) register accessor: TIM1 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_EGR)

For information about available fields see [`mod@tim1_egr`] module*/
pub type TIM1_EGR = crate::Reg<tim1_egr::TIM1_EGRrs>;
///TIM1 event generation register
pub mod tim1_egr;
/**TIM1_CCMR1 (rw) register accessor: TIM1 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR1)

For information about available fields see [`mod@tim1_ccmr1`] module*/
pub type TIM1_CCMR1 = crate::Reg<tim1_ccmr1::TIM1_CCMR1rs>;
///TIM1 capture/compare mode register 1
pub mod tim1_ccmr1;
/**TIM1_CCMR1_ALTERNATE1 (rw) register accessor: TIM1 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim1_ccmr1_alternate1`] module*/
pub type TIM1_CCMR1_ALTERNATE1 = crate::Reg<tim1_ccmr1_alternate1::TIM1_CCMR1_ALTERNATE1rs>;
///TIM1 capture/compare mode register 1
pub mod tim1_ccmr1_alternate1;
/**TIM1_CCMR2 (rw) register accessor: TIM1 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR2)

For information about available fields see [`mod@tim1_ccmr2`] module*/
pub type TIM1_CCMR2 = crate::Reg<tim1_ccmr2::TIM1_CCMR2rs>;
///TIM1 capture/compare mode register 2
pub mod tim1_ccmr2;
/**TIM1_CCMR2_ALTERNATE1 (rw) register accessor: TIM1 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr2_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr2_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR2_ALTERNATE1)

For information about available fields see [`mod@tim1_ccmr2_alternate1`] module*/
pub type TIM1_CCMR2_ALTERNATE1 = crate::Reg<tim1_ccmr2_alternate1::TIM1_CCMR2_ALTERNATE1rs>;
///TIM1 capture/compare mode register 2
pub mod tim1_ccmr2_alternate1;
/**TIM1_CCER (rw) register accessor: TIM1 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim1_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCER)

For information about available fields see [`mod@tim1_ccer`] module*/
pub type TIM1_CCER = crate::Reg<tim1_ccer::TIM1_CCERrs>;
///TIM1 capture/compare enable register
pub mod tim1_ccer;
/**TIM1_CNT (rw) register accessor: TIM1 counter

You can [`read`](crate::Reg::read) this register and get [`tim1_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CNT)

For information about available fields see [`mod@tim1_cnt`] module*/
pub type TIM1_CNT = crate::Reg<tim1_cnt::TIM1_CNTrs>;
///TIM1 counter
pub mod tim1_cnt;
/**TIM1_PSC (rw) register accessor: TIM1 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim1_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_PSC)

For information about available fields see [`mod@tim1_psc`] module*/
pub type TIM1_PSC = crate::Reg<tim1_psc::TIM1_PSCrs>;
///TIM1 prescaler
pub mod tim1_psc;
/**TIM1_ARR (rw) register accessor: TIM1 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim1_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_ARR)

For information about available fields see [`mod@tim1_arr`] module*/
pub type TIM1_ARR = crate::Reg<tim1_arr::TIM1_ARRrs>;
///TIM1 auto-reload register
pub mod tim1_arr;
/**TIM1_RCR (rw) register accessor: TIM1 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim1_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_RCR)

For information about available fields see [`mod@tim1_rcr`] module*/
pub type TIM1_RCR = crate::Reg<tim1_rcr::TIM1_RCRrs>;
///TIM1 repetition counter register
pub mod tim1_rcr;
/**TIM1_CCR1 (rw) register accessor: TIM1 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR1)

For information about available fields see [`mod@tim1_ccr1`] module*/
pub type TIM1_CCR1 = crate::Reg<tim1_ccr1::TIM1_CCR1rs>;
///TIM1 capture/compare register 1
pub mod tim1_ccr1;
/**TIM1_CCR2 (rw) register accessor: TIM1 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR2)

For information about available fields see [`mod@tim1_ccr2`] module*/
pub type TIM1_CCR2 = crate::Reg<tim1_ccr2::TIM1_CCR2rs>;
///TIM1 capture/compare register 2
pub mod tim1_ccr2;
/**TIM1_CCR3 (rw) register accessor: TIM1 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR3)

For information about available fields see [`mod@tim1_ccr3`] module*/
pub type TIM1_CCR3 = crate::Reg<tim1_ccr3::TIM1_CCR3rs>;
///TIM1 capture/compare register 3
pub mod tim1_ccr3;
/**TIM1_CCR4 (rw) register accessor: TIM1 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR4)

For information about available fields see [`mod@tim1_ccr4`] module*/
pub type TIM1_CCR4 = crate::Reg<tim1_ccr4::TIM1_CCR4rs>;
///TIM1 capture/compare register 4
pub mod tim1_ccr4;
/**TIM1_BDTR (rw) register accessor: TIM1 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim1_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_BDTR)

For information about available fields see [`mod@tim1_bdtr`] module*/
pub type TIM1_BDTR = crate::Reg<tim1_bdtr::TIM1_BDTRrs>;
///TIM1 break and dead-time register
pub mod tim1_bdtr;
/**TIM1_DCR (rw) register accessor: TIM1 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim1_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_DCR)

For information about available fields see [`mod@tim1_dcr`] module*/
pub type TIM1_DCR = crate::Reg<tim1_dcr::TIM1_DCRrs>;
///TIM1 DMA control register
pub mod tim1_dcr;
/**TIM1_DMAR (rw) register accessor: TIM1 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim1_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_DMAR)

For information about available fields see [`mod@tim1_dmar`] module*/
pub type TIM1_DMAR = crate::Reg<tim1_dmar::TIM1_DMARrs>;
///TIM1 DMA address for full transfer
pub mod tim1_dmar;
/**TIM1_CCMR3 (rw) register accessor: TIM1 capture/compare mode register 3

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR3)

For information about available fields see [`mod@tim1_ccmr3`] module*/
pub type TIM1_CCMR3 = crate::Reg<tim1_ccmr3::TIM1_CCMR3rs>;
///TIM1 capture/compare mode register 3
pub mod tim1_ccmr3;
/**TIM1_CCR5 (rw) register accessor: TIM1 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR5)

For information about available fields see [`mod@tim1_ccr5`] module*/
pub type TIM1_CCR5 = crate::Reg<tim1_ccr5::TIM1_CCR5rs>;
///TIM1 capture/compare register 5
pub mod tim1_ccr5;
/**TIM1_CCR6 (rw) register accessor: TIM1 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR6)

For information about available fields see [`mod@tim1_ccr6`] module*/
pub type TIM1_CCR6 = crate::Reg<tim1_ccr6::TIM1_CCR6rs>;
///TIM1 capture/compare register 6
pub mod tim1_ccr6;
/**TIM1_AF1 (rw) register accessor: TIM1 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_AF1)

For information about available fields see [`mod@tim1_af1`] module*/
pub type TIM1_AF1 = crate::Reg<tim1_af1::TIM1_AF1rs>;
///TIM1 alternate function option register 1
pub mod tim1_af1;
/**TIM1_AF2 (rw) register accessor: TIM1 Alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_AF2)

For information about available fields see [`mod@tim1_af2`] module*/
pub type TIM1_AF2 = crate::Reg<tim1_af2::TIM1_AF2rs>;
///TIM1 Alternate function register 2
pub mod tim1_af2;
/**TIM1_TISEL (rw) register accessor: TIM1 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim1_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_TISEL)

For information about available fields see [`mod@tim1_tisel`] module*/
pub type TIM1_TISEL = crate::Reg<tim1_tisel::TIM1_TISELrs>;
///TIM1 timer input selection register
pub mod tim1_tisel;
