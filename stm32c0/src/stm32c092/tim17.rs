#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim17_cr1: TIM17_CR1,
    _reserved1: [u8; 0x02],
    tim17_cr2: TIM17_CR2,
    _reserved2: [u8; 0x06],
    tim17_dier: TIM17_DIER,
    _reserved3: [u8; 0x02],
    tim17_sr: TIM17_SR,
    _reserved4: [u8; 0x02],
    tim17_egr: TIM17_EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_tim17_ccmr: [u8; 0x04],
    _reserved6: [u8; 0x04],
    tim17_ccer: TIM17_CCER,
    _reserved7: [u8; 0x02],
    tim17_cnt: TIM17_CNT,
    tim17_psc: TIM17_PSC,
    _reserved9: [u8; 0x02],
    tim17_arr: TIM17_ARR,
    _reserved10: [u8; 0x02],
    tim17_rcr: TIM17_RCR,
    _reserved11: [u8; 0x02],
    tim17_ccr1: TIM17_CCR1,
    _reserved12: [u8; 0x0e],
    tim17_bdtr: TIM17_BDTR,
    tim17_dcr: TIM17_DCR,
    _reserved14: [u8; 0x02],
    tim17_dmar: TIM17_DMAR,
    _reserved15: [u8; 0x12],
    tim17_af1: TIM17_AF1,
    _reserved16: [u8; 0x04],
    tim17_tisel: TIM17_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM17 control register 1
    #[inline(always)]
    pub const fn tim17_cr1(&self) -> &TIM17_CR1 {
        &self.tim17_cr1
    }
    ///0x04 - TIM17 control register 2
    #[inline(always)]
    pub const fn tim17_cr2(&self) -> &TIM17_CR2 {
        &self.tim17_cr2
    }
    ///0x0c - TIM17 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim17_dier(&self) -> &TIM17_DIER {
        &self.tim17_dier
    }
    ///0x10 - TIM17 status register
    #[inline(always)]
    pub const fn tim17_sr(&self) -> &TIM17_SR {
        &self.tim17_sr
    }
    ///0x14 - TIM17 event generation register
    #[inline(always)]
    pub const fn tim17_egr(&self) -> &TIM17_EGR {
        &self.tim17_egr
    }
    ///0x18 - TIM17 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim17_ccmr1_alternate1(&self) -> &TIM17_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM17 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim17_ccmr1(&self) -> &TIM17_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM17 capture/compare enable register
    #[inline(always)]
    pub const fn tim17_ccer(&self) -> &TIM17_CCER {
        &self.tim17_ccer
    }
    ///0x24 - TIM17 counter
    #[inline(always)]
    pub const fn tim17_cnt(&self) -> &TIM17_CNT {
        &self.tim17_cnt
    }
    ///0x28 - TIM17 prescaler
    #[inline(always)]
    pub const fn tim17_psc(&self) -> &TIM17_PSC {
        &self.tim17_psc
    }
    ///0x2c - TIM17 auto-reload register
    #[inline(always)]
    pub const fn tim17_arr(&self) -> &TIM17_ARR {
        &self.tim17_arr
    }
    ///0x30 - TIM17 repetition counter register
    #[inline(always)]
    pub const fn tim17_rcr(&self) -> &TIM17_RCR {
        &self.tim17_rcr
    }
    ///0x34 - TIM17 capture/compare register 1
    #[inline(always)]
    pub const fn tim17_ccr1(&self) -> &TIM17_CCR1 {
        &self.tim17_ccr1
    }
    ///0x44 - TIM17 break and dead-time register
    #[inline(always)]
    pub const fn tim17_bdtr(&self) -> &TIM17_BDTR {
        &self.tim17_bdtr
    }
    ///0x48 - TIM17 DMA control register
    #[inline(always)]
    pub const fn tim17_dcr(&self) -> &TIM17_DCR {
        &self.tim17_dcr
    }
    ///0x4c - TIM17 DMA address for full transfer
    #[inline(always)]
    pub const fn tim17_dmar(&self) -> &TIM17_DMAR {
        &self.tim17_dmar
    }
    ///0x60 - TIM17 alternate function register 1
    #[inline(always)]
    pub const fn tim17_af1(&self) -> &TIM17_AF1 {
        &self.tim17_af1
    }
    ///0x68 - TIM17 input selection register
    #[inline(always)]
    pub const fn tim17_tisel(&self) -> &TIM17_TISEL {
        &self.tim17_tisel
    }
}
/**TIM17_CR1 (rw) register accessor: TIM17 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim17_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CR1)

For information about available fields see [`mod@tim17_cr1`] module*/
pub type TIM17_CR1 = crate::Reg<tim17_cr1::TIM17_CR1rs>;
///TIM17 control register 1
pub mod tim17_cr1;
/**TIM17_CR2 (rw) register accessor: TIM17 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim17_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CR2)

For information about available fields see [`mod@tim17_cr2`] module*/
pub type TIM17_CR2 = crate::Reg<tim17_cr2::TIM17_CR2rs>;
///TIM17 control register 2
pub mod tim17_cr2;
/**TIM17_DIER (rw) register accessor: TIM17 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim17_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_DIER)

For information about available fields see [`mod@tim17_dier`] module*/
pub type TIM17_DIER = crate::Reg<tim17_dier::TIM17_DIERrs>;
///TIM17 DMA/interrupt enable register
pub mod tim17_dier;
/**TIM17_SR (rw) register accessor: TIM17 status register

You can [`read`](crate::Reg::read) this register and get [`tim17_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_SR)

For information about available fields see [`mod@tim17_sr`] module*/
pub type TIM17_SR = crate::Reg<tim17_sr::TIM17_SRrs>;
///TIM17 status register
pub mod tim17_sr;
/**TIM17_EGR (w) register accessor: TIM17 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_EGR)

For information about available fields see [`mod@tim17_egr`] module*/
pub type TIM17_EGR = crate::Reg<tim17_egr::TIM17_EGRrs>;
///TIM17 event generation register
pub mod tim17_egr;
/**TIM17_CCMR1 (rw) register accessor: TIM17 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim17_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CCMR1)

For information about available fields see [`mod@tim17_ccmr1`] module*/
pub type TIM17_CCMR1 = crate::Reg<tim17_ccmr1::TIM17_CCMR1rs>;
///TIM17 capture/compare mode register 1
pub mod tim17_ccmr1;
/**TIM17_CCMR1_ALTERNATE1 (rw) register accessor: TIM17 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim17_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim17_ccmr1_alternate1`] module*/
pub type TIM17_CCMR1_ALTERNATE1 = crate::Reg<tim17_ccmr1_alternate1::TIM17_CCMR1_ALTERNATE1rs>;
///TIM17 capture/compare mode register 1
pub mod tim17_ccmr1_alternate1;
/**TIM17_CCER (rw) register accessor: TIM17 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim17_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CCER)

For information about available fields see [`mod@tim17_ccer`] module*/
pub type TIM17_CCER = crate::Reg<tim17_ccer::TIM17_CCERrs>;
///TIM17 capture/compare enable register
pub mod tim17_ccer;
/**TIM17_CNT (rw) register accessor: TIM17 counter

You can [`read`](crate::Reg::read) this register and get [`tim17_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CNT)

For information about available fields see [`mod@tim17_cnt`] module*/
pub type TIM17_CNT = crate::Reg<tim17_cnt::TIM17_CNTrs>;
///TIM17 counter
pub mod tim17_cnt;
/**TIM17_PSC (rw) register accessor: TIM17 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim17_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_PSC)

For information about available fields see [`mod@tim17_psc`] module*/
pub type TIM17_PSC = crate::Reg<tim17_psc::TIM17_PSCrs>;
///TIM17 prescaler
pub mod tim17_psc;
/**TIM17_ARR (rw) register accessor: TIM17 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim17_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_ARR)

For information about available fields see [`mod@tim17_arr`] module*/
pub type TIM17_ARR = crate::Reg<tim17_arr::TIM17_ARRrs>;
///TIM17 auto-reload register
pub mod tim17_arr;
/**TIM17_RCR (rw) register accessor: TIM17 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim17_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_RCR)

For information about available fields see [`mod@tim17_rcr`] module*/
pub type TIM17_RCR = crate::Reg<tim17_rcr::TIM17_RCRrs>;
///TIM17 repetition counter register
pub mod tim17_rcr;
/**TIM17_CCR1 (rw) register accessor: TIM17 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim17_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_CCR1)

For information about available fields see [`mod@tim17_ccr1`] module*/
pub type TIM17_CCR1 = crate::Reg<tim17_ccr1::TIM17_CCR1rs>;
///TIM17 capture/compare register 1
pub mod tim17_ccr1;
/**TIM17_BDTR (rw) register accessor: TIM17 break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim17_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_BDTR)

For information about available fields see [`mod@tim17_bdtr`] module*/
pub type TIM17_BDTR = crate::Reg<tim17_bdtr::TIM17_BDTRrs>;
///TIM17 break and dead-time register
pub mod tim17_bdtr;
/**TIM17_DCR (rw) register accessor: TIM17 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim17_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_DCR)

For information about available fields see [`mod@tim17_dcr`] module*/
pub type TIM17_DCR = crate::Reg<tim17_dcr::TIM17_DCRrs>;
///TIM17 DMA control register
pub mod tim17_dcr;
/**TIM17_DMAR (rw) register accessor: TIM17 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim17_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_DMAR)

For information about available fields see [`mod@tim17_dmar`] module*/
pub type TIM17_DMAR = crate::Reg<tim17_dmar::TIM17_DMARrs>;
///TIM17 DMA address for full transfer
pub mod tim17_dmar;
/**TIM17_AF1 (rw) register accessor: TIM17 alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`tim17_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_AF1)

For information about available fields see [`mod@tim17_af1`] module*/
pub type TIM17_AF1 = crate::Reg<tim17_af1::TIM17_AF1rs>;
///TIM17 alternate function register 1
pub mod tim17_af1;
/**TIM17_TISEL (rw) register accessor: TIM17 input selection register

You can [`read`](crate::Reg::read) this register and get [`tim17_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_TISEL)

For information about available fields see [`mod@tim17_tisel`] module*/
pub type TIM17_TISEL = crate::Reg<tim17_tisel::TIM17_TISELrs>;
///TIM17 input selection register
pub mod tim17_tisel;
