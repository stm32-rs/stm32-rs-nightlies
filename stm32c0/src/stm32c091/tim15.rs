#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim15_cr1: TIM15_CR1,
    _reserved1: [u8; 0x02],
    tim15_cr2: TIM15_CR2,
    _reserved2: [u8; 0x02],
    tim15_smcr: TIM15_SMCR,
    tim15_dier: TIM15_DIER,
    _reserved4: [u8; 0x02],
    tim15_sr: TIM15_SR,
    _reserved5: [u8; 0x02],
    tim15_egr: TIM15_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim15_ccmr: [u8; 0x04],
    _reserved7: [u8; 0x04],
    tim15_ccer: TIM15_CCER,
    _reserved8: [u8; 0x02],
    tim15_cnt: TIM15_CNT,
    tim15_psc: TIM15_PSC,
    _reserved10: [u8; 0x02],
    tim15_arr: TIM15_ARR,
    _reserved11: [u8; 0x02],
    tim15_rcr: TIM15_RCR,
    _reserved12: [u8; 0x02],
    tim15_ccr1: TIM15_CCR1,
    _reserved13: [u8; 0x02],
    tim15_ccr2: TIM15_CCR2,
    _reserved14: [u8; 0x0a],
    tim15_bdtr: TIM15_BDTR,
    tim15_dcr: TIM15_DCR,
    _reserved16: [u8; 0x02],
    tim15_dmar: TIM15_DMAR,
    _reserved17: [u8; 0x12],
    tim15_af1: TIM15_AF1,
    _reserved18: [u8; 0x04],
    tim15_tisel: TIM15_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM15 control register 1
    #[inline(always)]
    pub const fn tim15_cr1(&self) -> &TIM15_CR1 {
        &self.tim15_cr1
    }
    ///0x04 - TIM15 control register 2
    #[inline(always)]
    pub const fn tim15_cr2(&self) -> &TIM15_CR2 {
        &self.tim15_cr2
    }
    ///0x08 - TIM15 slave mode control register
    #[inline(always)]
    pub const fn tim15_smcr(&self) -> &TIM15_SMCR {
        &self.tim15_smcr
    }
    ///0x0c - TIM15 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim15_dier(&self) -> &TIM15_DIER {
        &self.tim15_dier
    }
    ///0x10 - TIM15 status register
    #[inline(always)]
    pub const fn tim15_sr(&self) -> &TIM15_SR {
        &self.tim15_sr
    }
    ///0x14 - TIM15 event generation register
    #[inline(always)]
    pub const fn tim15_egr(&self) -> &TIM15_EGR {
        &self.tim15_egr
    }
    ///0x18 - TIM15 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim15_ccmr1_alternate1(&self) -> &TIM15_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM15 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim15_ccmr1(&self) -> &TIM15_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM15 capture/compare enable register
    #[inline(always)]
    pub const fn tim15_ccer(&self) -> &TIM15_CCER {
        &self.tim15_ccer
    }
    ///0x24 - TIM15 counter
    #[inline(always)]
    pub const fn tim15_cnt(&self) -> &TIM15_CNT {
        &self.tim15_cnt
    }
    ///0x28 - TIM15 prescaler
    #[inline(always)]
    pub const fn tim15_psc(&self) -> &TIM15_PSC {
        &self.tim15_psc
    }
    ///0x2c - TIM15 auto-reload register
    #[inline(always)]
    pub const fn tim15_arr(&self) -> &TIM15_ARR {
        &self.tim15_arr
    }
    ///0x30 - TIM15 repetition counter register
    #[inline(always)]
    pub const fn tim15_rcr(&self) -> &TIM15_RCR {
        &self.tim15_rcr
    }
    ///0x34 - TIM15 capture/compare register 1
    #[inline(always)]
    pub const fn tim15_ccr1(&self) -> &TIM15_CCR1 {
        &self.tim15_ccr1
    }
    ///0x38 - TIM15 capture/compare register 2
    #[inline(always)]
    pub const fn tim15_ccr2(&self) -> &TIM15_CCR2 {
        &self.tim15_ccr2
    }
    ///0x44 - TIM15 break and dead-time register
    #[inline(always)]
    pub const fn tim15_bdtr(&self) -> &TIM15_BDTR {
        &self.tim15_bdtr
    }
    ///0x48 - TIM15 DMA control register
    #[inline(always)]
    pub const fn tim15_dcr(&self) -> &TIM15_DCR {
        &self.tim15_dcr
    }
    ///0x4c - TIM15 DMA address for full transfer
    #[inline(always)]
    pub const fn tim15_dmar(&self) -> &TIM15_DMAR {
        &self.tim15_dmar
    }
    ///0x60 - TIM15 alternate register 1
    #[inline(always)]
    pub const fn tim15_af1(&self) -> &TIM15_AF1 {
        &self.tim15_af1
    }
    ///0x68 - TIM15 input selection register
    #[inline(always)]
    pub const fn tim15_tisel(&self) -> &TIM15_TISEL {
        &self.tim15_tisel
    }
}
/**TIM15_CR1 (rw) register accessor: TIM15 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CR1)

For information about available fields see [`mod@tim15_cr1`] module*/
pub type TIM15_CR1 = crate::Reg<tim15_cr1::TIM15_CR1rs>;
///TIM15 control register 1
pub mod tim15_cr1;
/**TIM15_CR2 (rw) register accessor: TIM15 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim15_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CR2)

For information about available fields see [`mod@tim15_cr2`] module*/
pub type TIM15_CR2 = crate::Reg<tim15_cr2::TIM15_CR2rs>;
///TIM15 control register 2
pub mod tim15_cr2;
/**TIM15_SMCR (rw) register accessor: TIM15 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim15_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_SMCR)

For information about available fields see [`mod@tim15_smcr`] module*/
pub type TIM15_SMCR = crate::Reg<tim15_smcr::TIM15_SMCRrs>;
///TIM15 slave mode control register
pub mod tim15_smcr;
/**TIM15_DIER (rw) register accessor: TIM15 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim15_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_DIER)

For information about available fields see [`mod@tim15_dier`] module*/
pub type TIM15_DIER = crate::Reg<tim15_dier::TIM15_DIERrs>;
///TIM15 DMA/interrupt enable register
pub mod tim15_dier;
/**TIM15_SR (rw) register accessor: TIM15 status register

You can [`read`](crate::Reg::read) this register and get [`tim15_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_SR)

For information about available fields see [`mod@tim15_sr`] module*/
pub type TIM15_SR = crate::Reg<tim15_sr::TIM15_SRrs>;
///TIM15 status register
pub mod tim15_sr;
/**TIM15_EGR (rw) register accessor: TIM15 event generation register

You can [`read`](crate::Reg::read) this register and get [`tim15_egr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_egr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_EGR)

For information about available fields see [`mod@tim15_egr`] module*/
pub type TIM15_EGR = crate::Reg<tim15_egr::TIM15_EGRrs>;
///TIM15 event generation register
pub mod tim15_egr;
/**TIM15_CCMR1 (rw) register accessor: TIM15 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CCMR1)

For information about available fields see [`mod@tim15_ccmr1`] module*/
pub type TIM15_CCMR1 = crate::Reg<tim15_ccmr1::TIM15_CCMR1rs>;
///TIM15 capture/compare mode register 1
pub mod tim15_ccmr1;
/**TIM15_CCMR1_ALTERNATE1 (rw) register accessor: TIM15 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim15_ccmr1_alternate1`] module*/
pub type TIM15_CCMR1_ALTERNATE1 = crate::Reg<tim15_ccmr1_alternate1::TIM15_CCMR1_ALTERNATE1rs>;
///TIM15 capture/compare mode register 1
pub mod tim15_ccmr1_alternate1;
/**TIM15_CCER (rw) register accessor: TIM15 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim15_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CCER)

For information about available fields see [`mod@tim15_ccer`] module*/
pub type TIM15_CCER = crate::Reg<tim15_ccer::TIM15_CCERrs>;
///TIM15 capture/compare enable register
pub mod tim15_ccer;
/**TIM15_CNT (rw) register accessor: TIM15 counter

You can [`read`](crate::Reg::read) this register and get [`tim15_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CNT)

For information about available fields see [`mod@tim15_cnt`] module*/
pub type TIM15_CNT = crate::Reg<tim15_cnt::TIM15_CNTrs>;
///TIM15 counter
pub mod tim15_cnt;
/**TIM15_PSC (rw) register accessor: TIM15 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim15_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_PSC)

For information about available fields see [`mod@tim15_psc`] module*/
pub type TIM15_PSC = crate::Reg<tim15_psc::TIM15_PSCrs>;
///TIM15 prescaler
pub mod tim15_psc;
/**TIM15_ARR (rw) register accessor: TIM15 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim15_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_ARR)

For information about available fields see [`mod@tim15_arr`] module*/
pub type TIM15_ARR = crate::Reg<tim15_arr::TIM15_ARRrs>;
///TIM15 auto-reload register
pub mod tim15_arr;
/**TIM15_RCR (rw) register accessor: TIM15 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim15_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_RCR)

For information about available fields see [`mod@tim15_rcr`] module*/
pub type TIM15_RCR = crate::Reg<tim15_rcr::TIM15_RCRrs>;
///TIM15 repetition counter register
pub mod tim15_rcr;
/**TIM15_CCR1 (rw) register accessor: TIM15 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CCR1)

For information about available fields see [`mod@tim15_ccr1`] module*/
pub type TIM15_CCR1 = crate::Reg<tim15_ccr1::TIM15_CCR1rs>;
///TIM15 capture/compare register 1
pub mod tim15_ccr1;
/**TIM15_CCR2 (rw) register accessor: TIM15 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim15_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CCR2)

For information about available fields see [`mod@tim15_ccr2`] module*/
pub type TIM15_CCR2 = crate::Reg<tim15_ccr2::TIM15_CCR2rs>;
///TIM15 capture/compare register 2
pub mod tim15_ccr2;
/**TIM15_BDTR (rw) register accessor: TIM15 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim15_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_BDTR)

For information about available fields see [`mod@tim15_bdtr`] module*/
pub type TIM15_BDTR = crate::Reg<tim15_bdtr::TIM15_BDTRrs>;
///TIM15 break and dead-time register
pub mod tim15_bdtr;
/**TIM15_DCR (rw) register accessor: TIM15 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim15_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_DCR)

For information about available fields see [`mod@tim15_dcr`] module*/
pub type TIM15_DCR = crate::Reg<tim15_dcr::TIM15_DCRrs>;
///TIM15 DMA control register
pub mod tim15_dcr;
/**TIM15_DMAR (rw) register accessor: TIM15 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim15_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_DMAR)

For information about available fields see [`mod@tim15_dmar`] module*/
pub type TIM15_DMAR = crate::Reg<tim15_dmar::TIM15_DMARrs>;
///TIM15 DMA address for full transfer
pub mod tim15_dmar;
/**TIM15_AF1 (rw) register accessor: TIM15 alternate register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_AF1)

For information about available fields see [`mod@tim15_af1`] module*/
pub type TIM15_AF1 = crate::Reg<tim15_af1::TIM15_AF1rs>;
///TIM15 alternate register 1
pub mod tim15_af1;
/**TIM15_TISEL (rw) register accessor: TIM15 input selection register

You can [`read`](crate::Reg::read) this register and get [`tim15_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_TISEL)

For information about available fields see [`mod@tim15_tisel`] module*/
pub type TIM15_TISEL = crate::Reg<tim15_tisel::TIM15_TISELrs>;
///TIM15 input selection register
pub mod tim15_tisel;
