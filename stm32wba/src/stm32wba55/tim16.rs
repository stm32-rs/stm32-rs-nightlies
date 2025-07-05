#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim_cr1: TIM_CR1,
    _reserved1: [u8; 0x02],
    tim_cr2: TIM_CR2,
    _reserved2: [u8; 0x06],
    tim_dier: TIM_DIER,
    _reserved3: [u8; 0x02],
    tim_sr: TIM_SR,
    _reserved4: [u8; 0x02],
    tim_egr: TIM_EGR,
    _reserved5: [u8; 0x02],
    _reserved_5_tim_ccmr: [u8; 0x04],
    _reserved6: [u8; 0x04],
    tim_ccer: TIM_CCER,
    _reserved7: [u8; 0x02],
    tim_cnt: TIM_CNT,
    tim_psc: TIM_PSC,
    _reserved9: [u8; 0x02],
    tim_arr: TIM_ARR,
    tim_rcr: TIM_RCR,
    _reserved11: [u8; 0x02],
    tim_ccr1: TIM_CCR1,
    _reserved12: [u8; 0x0c],
    tim_bdtr: TIM_BDTR,
    _reserved13: [u8; 0x0c],
    tim_dtr2: TIM_DTR2,
    _reserved14: [u8; 0x04],
    tim_tisel: TIM_TISEL,
    tim_af1: TIM_AF1,
    tim_af2: TIM_AF2,
    tim_or1: TIM_OR1,
    _reserved18: [u8; 0x0370],
    tim_dcr: TIM_DCR,
    tim_dmar: TIM_DMAR,
}
impl RegisterBlock {
    ///0x00 - TIM control register 1
    #[inline(always)]
    pub const fn tim_cr1(&self) -> &TIM_CR1 {
        &self.tim_cr1
    }
    ///0x04 - TIM control register 2
    #[inline(always)]
    pub const fn tim_cr2(&self) -> &TIM_CR2 {
        &self.tim_cr2
    }
    ///0x0c - TIM DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim_dier(&self) -> &TIM_DIER {
        &self.tim_dier
    }
    ///0x10 - TIM status register
    #[inline(always)]
    pub const fn tim_sr(&self) -> &TIM_SR {
        &self.tim_sr
    }
    ///0x14 - TIM event generation register
    #[inline(always)]
    pub const fn tim_egr(&self) -> &TIM_EGR {
        &self.tim_egr
    }
    ///0x18 - TIM capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim_ccmr1_alternate1(&self) -> &TIM_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim_ccmr1(&self) -> &TIM_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x20 - TIM capture/compare enable register
    #[inline(always)]
    pub const fn tim_ccer(&self) -> &TIM_CCER {
        &self.tim_ccer
    }
    ///0x24 - TIM counter
    #[inline(always)]
    pub const fn tim_cnt(&self) -> &TIM_CNT {
        &self.tim_cnt
    }
    ///0x28 - TIM prescaler
    #[inline(always)]
    pub const fn tim_psc(&self) -> &TIM_PSC {
        &self.tim_psc
    }
    ///0x2c - TIM auto-reload register
    #[inline(always)]
    pub const fn tim_arr(&self) -> &TIM_ARR {
        &self.tim_arr
    }
    ///0x30 - TIM repetition counter register
    #[inline(always)]
    pub const fn tim_rcr(&self) -> &TIM_RCR {
        &self.tim_rcr
    }
    ///0x34 - TIM capture/compare register 1
    #[inline(always)]
    pub const fn tim_ccr1(&self) -> &TIM_CCR1 {
        &self.tim_ccr1
    }
    ///0x44 - TIM break and dead-time register
    #[inline(always)]
    pub const fn tim_bdtr(&self) -> &TIM_BDTR {
        &self.tim_bdtr
    }
    ///0x54 - TIM timer deadtime register 2
    #[inline(always)]
    pub const fn tim_dtr2(&self) -> &TIM_DTR2 {
        &self.tim_dtr2
    }
    ///0x5c - TIM input selection register
    #[inline(always)]
    pub const fn tim_tisel(&self) -> &TIM_TISEL {
        &self.tim_tisel
    }
    ///0x60 - TIM alternate function register 1
    #[inline(always)]
    pub const fn tim_af1(&self) -> &TIM_AF1 {
        &self.tim_af1
    }
    ///0x64 - TIM alternate function register 2
    #[inline(always)]
    pub const fn tim_af2(&self) -> &TIM_AF2 {
        &self.tim_af2
    }
    ///0x68 - TIM option register 1
    #[inline(always)]
    pub const fn tim_or1(&self) -> &TIM_OR1 {
        &self.tim_or1
    }
    ///0x3dc - TIM DMA control register
    #[inline(always)]
    pub const fn tim_dcr(&self) -> &TIM_DCR {
        &self.tim_dcr
    }
    ///0x3e0 - TIM16/TIM17 DMA address for full transfer
    #[inline(always)]
    pub const fn tim_dmar(&self) -> &TIM_DMAR {
        &self.tim_dmar
    }
}
/**TIM_CR1 (rw) register accessor: TIM control register 1

You can [`read`](crate::Reg::read) this register and get [`tim_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CR1)

For information about available fields see [`mod@tim_cr1`] module*/
pub type TIM_CR1 = crate::Reg<tim_cr1::TIM_CR1rs>;
///TIM control register 1
pub mod tim_cr1;
/**TIM_CR2 (rw) register accessor: TIM control register 2

You can [`read`](crate::Reg::read) this register and get [`tim_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CR2)

For information about available fields see [`mod@tim_cr2`] module*/
pub type TIM_CR2 = crate::Reg<tim_cr2::TIM_CR2rs>;
///TIM control register 2
pub mod tim_cr2;
/**TIM_DIER (rw) register accessor: TIM DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_DIER)

For information about available fields see [`mod@tim_dier`] module*/
pub type TIM_DIER = crate::Reg<tim_dier::TIM_DIERrs>;
///TIM DMA/interrupt enable register
pub mod tim_dier;
/**TIM_SR (rw) register accessor: TIM status register

You can [`read`](crate::Reg::read) this register and get [`tim_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_SR)

For information about available fields see [`mod@tim_sr`] module*/
pub type TIM_SR = crate::Reg<tim_sr::TIM_SRrs>;
///TIM status register
pub mod tim_sr;
/**TIM_EGR (w) register accessor: TIM event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_EGR)

For information about available fields see [`mod@tim_egr`] module*/
pub type TIM_EGR = crate::Reg<tim_egr::TIM_EGRrs>;
///TIM event generation register
pub mod tim_egr;
/**TIM_CCMR1 (rw) register accessor: TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CCMR1)

For information about available fields see [`mod@tim_ccmr1`] module*/
pub type TIM_CCMR1 = crate::Reg<tim_ccmr1::TIM_CCMR1rs>;
///TIM capture/compare mode register 1 \[alternate\]
pub mod tim_ccmr1;
/**TIM_CCMR1_ALTERNATE1 (rw) register accessor: TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim_ccmr1_alternate1`] module*/
pub type TIM_CCMR1_ALTERNATE1 = crate::Reg<tim_ccmr1_alternate1::TIM_CCMR1_ALTERNATE1rs>;
///TIM capture/compare mode register 1 \[alternate\]
pub mod tim_ccmr1_alternate1;
/**TIM_CCER (rw) register accessor: TIM capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CCER)

For information about available fields see [`mod@tim_ccer`] module*/
pub type TIM_CCER = crate::Reg<tim_ccer::TIM_CCERrs>;
///TIM capture/compare enable register
pub mod tim_ccer;
/**TIM_CNT (rw) register accessor: TIM counter

You can [`read`](crate::Reg::read) this register and get [`tim_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CNT)

For information about available fields see [`mod@tim_cnt`] module*/
pub type TIM_CNT = crate::Reg<tim_cnt::TIM_CNTrs>;
///TIM counter
pub mod tim_cnt;
/**TIM_PSC (rw) register accessor: TIM prescaler

You can [`read`](crate::Reg::read) this register and get [`tim_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_PSC)

For information about available fields see [`mod@tim_psc`] module*/
pub type TIM_PSC = crate::Reg<tim_psc::TIM_PSCrs>;
///TIM prescaler
pub mod tim_psc;
/**TIM_ARR (rw) register accessor: TIM auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_ARR)

For information about available fields see [`mod@tim_arr`] module*/
pub type TIM_ARR = crate::Reg<tim_arr::TIM_ARRrs>;
///TIM auto-reload register
pub mod tim_arr;
/**TIM_RCR (rw) register accessor: TIM repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_RCR)

For information about available fields see [`mod@tim_rcr`] module*/
pub type TIM_RCR = crate::Reg<tim_rcr::TIM_RCRrs>;
///TIM repetition counter register
pub mod tim_rcr;
/**TIM_CCR1 (rw) register accessor: TIM capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CCR1)

For information about available fields see [`mod@tim_ccr1`] module*/
pub type TIM_CCR1 = crate::Reg<tim_ccr1::TIM_CCR1rs>;
///TIM capture/compare register 1
pub mod tim_ccr1;
/**TIM_BDTR (rw) register accessor: TIM break and dead-time register

You can [`read`](crate::Reg::read) this register and get [`tim_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_BDTR)

For information about available fields see [`mod@tim_bdtr`] module*/
pub type TIM_BDTR = crate::Reg<tim_bdtr::TIM_BDTRrs>;
///TIM break and dead-time register
pub mod tim_bdtr;
/**TIM_DTR2 (rw) register accessor: TIM timer deadtime register 2

You can [`read`](crate::Reg::read) this register and get [`tim_dtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_DTR2)

For information about available fields see [`mod@tim_dtr2`] module*/
pub type TIM_DTR2 = crate::Reg<tim_dtr2::TIM_DTR2rs>;
///TIM timer deadtime register 2
pub mod tim_dtr2;
/**TIM_TISEL (rw) register accessor: TIM input selection register

You can [`read`](crate::Reg::read) this register and get [`tim_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_TISEL)

For information about available fields see [`mod@tim_tisel`] module*/
pub type TIM_TISEL = crate::Reg<tim_tisel::TIM_TISELrs>;
///TIM input selection register
pub mod tim_tisel;
/**TIM_AF1 (rw) register accessor: TIM alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`tim_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_AF1)

For information about available fields see [`mod@tim_af1`] module*/
pub type TIM_AF1 = crate::Reg<tim_af1::TIM_AF1rs>;
///TIM alternate function register 1
pub mod tim_af1;
/**TIM_AF2 (rw) register accessor: TIM alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`tim_af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_AF2)

For information about available fields see [`mod@tim_af2`] module*/
pub type TIM_AF2 = crate::Reg<tim_af2::TIM_AF2rs>;
///TIM alternate function register 2
pub mod tim_af2;
/**TIM_OR1 (rw) register accessor: TIM option register 1

You can [`read`](crate::Reg::read) this register and get [`tim_or1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_or1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_OR1)

For information about available fields see [`mod@tim_or1`] module*/
pub type TIM_OR1 = crate::Reg<tim_or1::TIM_OR1rs>;
///TIM option register 1
pub mod tim_or1;
/**TIM_DCR (rw) register accessor: TIM DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_DCR)

For information about available fields see [`mod@tim_dcr`] module*/
pub type TIM_DCR = crate::Reg<tim_dcr::TIM_DCRrs>;
///TIM DMA control register
pub mod tim_dcr;
/**TIM_DMAR (rw) register accessor: TIM16/TIM17 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_DMAR)

For information about available fields see [`mod@tim_dmar`] module*/
pub type TIM_DMAR = crate::Reg<tim_dmar::TIM_DMARrs>;
///TIM16/TIM17 DMA address for full transfer
pub mod tim_dmar;
