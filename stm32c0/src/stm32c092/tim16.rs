#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim16_cr1: TIM16_CR1,
    _reserved1: [u8; 0x02],
    tim16_cr2: TIM16_CR2,
    _reserved2: [u8; 0x06],
    tim16_dier: TIM16_DIER,
    _reserved3: [u8; 0x02],
    tim16_sr: TIM16_SR,
    _reserved4: [u8; 0x02],
    tim16_egr: TIM16_EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_tim16_ccmr: [u8; 0x04],
    _reserved6: [u8; 0x04],
    tim16_ccer: TIM16_CCER,
    _reserved7: [u8; 0x02],
    tim16_cnt: TIM16_CNT,
    tim16_psc: TIM16_PSC,
    _reserved9: [u8; 0x02],
    tim16_arr: TIM16_ARR,
    _reserved10: [u8; 0x02],
    tim16_rcr: TIM16_RCR,
    _reserved11: [u8; 0x02],
    tim16_ccr1: TIM16_CCR1,
    _reserved12: [u8; 0x0e],
    tim16_bdtr: TIM16_BDTR,
    tim16_dcr: TIM16_DCR,
    _reserved14: [u8; 0x02],
    tim16_dmar: TIM16_DMAR,
    _reserved15: [u8; 0x12],
    tim16_af1: TIM16_AF1,
    _reserved16: [u8; 0x04],
    tim16_tisel: TIM16_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM16 control register 1
    #[inline(always)]
    pub const fn tim16_cr1(&self) -> &TIM16_CR1 {
        &self.tim16_cr1
    }
    ///0x04 - TIM16 control register 2
    #[inline(always)]
    pub const fn tim16_cr2(&self) -> &TIM16_CR2 {
        &self.tim16_cr2
    }
    ///0x0c - TIM16 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim16_dier(&self) -> &TIM16_DIER {
        &self.tim16_dier
    }
    ///0x10 - TIM16 status register
    #[inline(always)]
    pub const fn tim16_sr(&self) -> &TIM16_SR {
        &self.tim16_sr
    }
    ///0x14 - TIM16 event generation register
    #[inline(always)]
    pub const fn tim16_egr(&self) -> &TIM16_EGR {
        &self.tim16_egr
    }
    ///0x18 - TIM16 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim16_ccmr1_alternate1(&self) -> &TIM16_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM16 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim16_ccmr1(&self) -> &TIM16_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM16 capture/compare enable register
    #[inline(always)]
    pub const fn tim16_ccer(&self) -> &TIM16_CCER {
        &self.tim16_ccer
    }
    ///0x24 - TIM16 counter
    #[inline(always)]
    pub const fn tim16_cnt(&self) -> &TIM16_CNT {
        &self.tim16_cnt
    }
    ///0x28 - TIM16 prescaler
    #[inline(always)]
    pub const fn tim16_psc(&self) -> &TIM16_PSC {
        &self.tim16_psc
    }
    ///0x2c - TIM16 auto-reload register
    #[inline(always)]
    pub const fn tim16_arr(&self) -> &TIM16_ARR {
        &self.tim16_arr
    }
    ///0x30 - TIM16 repetition counter register
    #[inline(always)]
    pub const fn tim16_rcr(&self) -> &TIM16_RCR {
        &self.tim16_rcr
    }
    ///0x34 - TIM16 capture/compare register 1
    #[inline(always)]
    pub const fn tim16_ccr1(&self) -> &TIM16_CCR1 {
        &self.tim16_ccr1
    }
    ///0x44 - TIM16 break and dead-time register
    #[inline(always)]
    pub const fn tim16_bdtr(&self) -> &TIM16_BDTR {
        &self.tim16_bdtr
    }
    ///0x48 - TIM16 DMA control register
    #[inline(always)]
    pub const fn tim16_dcr(&self) -> &TIM16_DCR {
        &self.tim16_dcr
    }
    ///0x4c - TIM16 DMA address for full transfer
    #[inline(always)]
    pub const fn tim16_dmar(&self) -> &TIM16_DMAR {
        &self.tim16_dmar
    }
    ///0x60 - TIM16 alternate function register 1
    #[inline(always)]
    pub const fn tim16_af1(&self) -> &TIM16_AF1 {
        &self.tim16_af1
    }
    ///0x68 - TIM16 input selection register
    #[inline(always)]
    pub const fn tim16_tisel(&self) -> &TIM16_TISEL {
        &self.tim16_tisel
    }
}
/**TIM16_CR1 (rw) register accessor: TIM16 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CR1)

For information about available fields see [`mod@tim16_cr1`] module*/
pub type TIM16_CR1 = crate::Reg<tim16_cr1::TIM16_CR1rs>;
///TIM16 control register 1
pub mod tim16_cr1;
/**TIM16_CR2 (rw) register accessor: TIM16 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim16_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CR2)

For information about available fields see [`mod@tim16_cr2`] module*/
pub type TIM16_CR2 = crate::Reg<tim16_cr2::TIM16_CR2rs>;
///TIM16 control register 2
pub mod tim16_cr2;
/**TIM16_DIER (rw) register accessor: TIM16 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim16_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_DIER)

For information about available fields see [`mod@tim16_dier`] module*/
pub type TIM16_DIER = crate::Reg<tim16_dier::TIM16_DIERrs>;
///TIM16 DMA/interrupt enable register
pub mod tim16_dier;
/**TIM16_SR (rw) register accessor: TIM16 status register

You can [`read`](crate::Reg::read) this register and get [`tim16_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_SR)

For information about available fields see [`mod@tim16_sr`] module*/
pub type TIM16_SR = crate::Reg<tim16_sr::TIM16_SRrs>;
///TIM16 status register
pub mod tim16_sr;
/**TIM16_EGR (w) register accessor: TIM16 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_EGR)

For information about available fields see [`mod@tim16_egr`] module*/
pub type TIM16_EGR = crate::Reg<tim16_egr::TIM16_EGRrs>;
///TIM16 event generation register
pub mod tim16_egr;
/**TIM16_CCMR1 (rw) register accessor: TIM16 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CCMR1)

For information about available fields see [`mod@tim16_ccmr1`] module*/
pub type TIM16_CCMR1 = crate::Reg<tim16_ccmr1::TIM16_CCMR1rs>;
///TIM16 capture/compare mode register 1
pub mod tim16_ccmr1;
/**TIM16_CCMR1_ALTERNATE1 (rw) register accessor: TIM16 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim16_ccmr1_alternate1`] module*/
pub type TIM16_CCMR1_ALTERNATE1 = crate::Reg<tim16_ccmr1_alternate1::TIM16_CCMR1_ALTERNATE1rs>;
///TIM16 capture/compare mode register 1
pub mod tim16_ccmr1_alternate1;
/**TIM16_CCER (rw) register accessor: TIM16 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim16_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CCER)

For information about available fields see [`mod@tim16_ccer`] module*/
pub type TIM16_CCER = crate::Reg<tim16_ccer::TIM16_CCERrs>;
///TIM16 capture/compare enable register
pub mod tim16_ccer;
/**TIM16_CNT (rw) register accessor: TIM16 counter

You can [`read`](crate::Reg::read) this register and get [`tim16_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CNT)

For information about available fields see [`mod@tim16_cnt`] module*/
pub type TIM16_CNT = crate::Reg<tim16_cnt::TIM16_CNTrs>;
///TIM16 counter
pub mod tim16_cnt;
/**TIM16_PSC (rw) register accessor: TIM16 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim16_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_PSC)

For information about available fields see [`mod@tim16_psc`] module*/
pub type TIM16_PSC = crate::Reg<tim16_psc::TIM16_PSCrs>;
///TIM16 prescaler
pub mod tim16_psc;
/**TIM16_ARR (rw) register accessor: TIM16 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim16_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_ARR)

For information about available fields see [`mod@tim16_arr`] module*/
pub type TIM16_ARR = crate::Reg<tim16_arr::TIM16_ARRrs>;
///TIM16 auto-reload register
pub mod tim16_arr;
/**TIM16_RCR (rw) register accessor: TIM16 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim16_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_RCR)

For information about available fields see [`mod@tim16_rcr`] module*/
pub type TIM16_RCR = crate::Reg<tim16_rcr::TIM16_RCRrs>;
///TIM16 repetition counter register
pub mod tim16_rcr;
/**TIM16_CCR1 (rw) register accessor: TIM16 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_CCR1)

For information about available fields see [`mod@tim16_ccr1`] module*/
pub type TIM16_CCR1 = crate::Reg<tim16_ccr1::TIM16_CCR1rs>;
///TIM16 capture/compare register 1
pub mod tim16_ccr1;
/**TIM16_BDTR (rw) register accessor: TIM16 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim16_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_BDTR)

For information about available fields see [`mod@tim16_bdtr`] module*/
pub type TIM16_BDTR = crate::Reg<tim16_bdtr::TIM16_BDTRrs>;
///TIM16 break and dead-time register
pub mod tim16_bdtr;
/**TIM16_DCR (rw) register accessor: TIM16 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim16_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_DCR)

For information about available fields see [`mod@tim16_dcr`] module*/
pub type TIM16_DCR = crate::Reg<tim16_dcr::TIM16_DCRrs>;
///TIM16 DMA control register
pub mod tim16_dcr;
/**TIM16_DMAR (rw) register accessor: TIM16 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim16_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_DMAR)

For information about available fields see [`mod@tim16_dmar`] module*/
pub type TIM16_DMAR = crate::Reg<tim16_dmar::TIM16_DMARrs>;
///TIM16 DMA address for full transfer
pub mod tim16_dmar;
/**TIM16_AF1 (rw) register accessor: TIM16 alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`tim16_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_AF1)

For information about available fields see [`mod@tim16_af1`] module*/
pub type TIM16_AF1 = crate::Reg<tim16_af1::TIM16_AF1rs>;
///TIM16 alternate function register 1
pub mod tim16_af1;
/**TIM16_TISEL (rw) register accessor: TIM16 input selection register

You can [`read`](crate::Reg::read) this register and get [`tim16_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM16:TIM16_TISEL)

For information about available fields see [`mod@tim16_tisel`] module*/
pub type TIM16_TISEL = crate::Reg<tim16_tisel::TIM16_TISELrs>;
///TIM16 input selection register
pub mod tim16_tisel;
