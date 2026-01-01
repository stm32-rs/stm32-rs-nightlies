#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim3_cr1: TIM3_CR1,
    _reserved1: [u8; 0x02],
    tim3_cr2: TIM3_CR2,
    _reserved2: [u8; 0x02],
    tim3_smcr: TIM3_SMCR,
    tim3_dier: TIM3_DIER,
    _reserved4: [u8; 0x02],
    tim3_sr: TIM3_SR,
    _reserved5: [u8; 0x02],
    tim3_egr: TIM3_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim3_ccmr: [u8; 0x04],
    _reserved_7_tim3_ccmr: [u8; 0x04],
    tim3_ccer: TIM3_CCER,
    _reserved9: [u8; 0x02],
    _reserved_9_tim3_: [u8; 0x04],
    tim3_psc: TIM3_PSC,
    _reserved11: [u8; 0x02],
    tim3_arr: TIM3_ARR,
    _reserved12: [u8; 0x04],
    tim3_ccr1: TIM3_CCR1,
    tim3_ccr2: TIM3_CCR2,
    tim3_ccr3: TIM3_CCR3,
    tim3_ccr4: TIM3_CCR4,
    _reserved16: [u8; 0x04],
    tim3_dcr: TIM3_DCR,
    _reserved17: [u8; 0x02],
    tim3_dmar: TIM3_DMAR,
    _reserved18: [u8; 0x12],
    tim3_af1: TIM3_AF1,
    _reserved19: [u8; 0x04],
    tim3_tisel: TIM3_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM3 control register 1
    #[inline(always)]
    pub const fn tim3_cr1(&self) -> &TIM3_CR1 {
        &self.tim3_cr1
    }
    ///0x04 - TIM3 control register 2
    #[inline(always)]
    pub const fn tim3_cr2(&self) -> &TIM3_CR2 {
        &self.tim3_cr2
    }
    ///0x08 - TIM3 slave mode control register
    #[inline(always)]
    pub const fn tim3_smcr(&self) -> &TIM3_SMCR {
        &self.tim3_smcr
    }
    ///0x0c - TIM3 DMA/Interrupt enable register
    #[inline(always)]
    pub const fn tim3_dier(&self) -> &TIM3_DIER {
        &self.tim3_dier
    }
    ///0x10 - TIM3 status register
    #[inline(always)]
    pub const fn tim3_sr(&self) -> &TIM3_SR {
        &self.tim3_sr
    }
    ///0x14 - TIM3 event generation register
    #[inline(always)]
    pub const fn tim3_egr(&self) -> &TIM3_EGR {
        &self.tim3_egr
    }
    ///0x18 - TIM3 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim3_ccmr1_alternate1(&self) -> &TIM3_CCMR1_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM3 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim3_ccmr1(&self) -> &TIM3_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM3 capture/compare mode register 2
    #[inline(always)]
    pub const fn tim3_ccmr2_alternate1(&self) -> &TIM3_CCMR2_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM3 capture/compare mode register 2
    #[inline(always)]
    pub const fn tim3_ccmr2(&self) -> &TIM3_CCMR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - TIM3 capture/compare enable register
    #[inline(always)]
    pub const fn tim3_ccer(&self) -> &TIM3_CCER {
        &self.tim3_ccer
    }
    ///0x24 - TIM3 counter
    #[inline(always)]
    pub const fn tim3_cnt_alternate1(&self) -> &TIM3_CNT_ALTERNATE1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x24 - TIM3 counter
    #[inline(always)]
    pub const fn tim3_cnt(&self) -> &TIM3_CNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x28 - TIM3 prescaler
    #[inline(always)]
    pub const fn tim3_psc(&self) -> &TIM3_PSC {
        &self.tim3_psc
    }
    ///0x2c - TIM3 auto-reload register
    #[inline(always)]
    pub const fn tim3_arr(&self) -> &TIM3_ARR {
        &self.tim3_arr
    }
    ///0x34 - TIM3 capture/compare register 1
    #[inline(always)]
    pub const fn tim3_ccr1(&self) -> &TIM3_CCR1 {
        &self.tim3_ccr1
    }
    ///0x38 - TIM3 capture/compare register 2
    #[inline(always)]
    pub const fn tim3_ccr2(&self) -> &TIM3_CCR2 {
        &self.tim3_ccr2
    }
    ///0x3c - TIM3 capture/compare register 3
    #[inline(always)]
    pub const fn tim3_ccr3(&self) -> &TIM3_CCR3 {
        &self.tim3_ccr3
    }
    ///0x40 - TIM3 capture/compare register 4
    #[inline(always)]
    pub const fn tim3_ccr4(&self) -> &TIM3_CCR4 {
        &self.tim3_ccr4
    }
    ///0x48 - TIM3 DMA control register
    #[inline(always)]
    pub const fn tim3_dcr(&self) -> &TIM3_DCR {
        &self.tim3_dcr
    }
    ///0x4c - TIM3 DMA address for full transfer
    #[inline(always)]
    pub const fn tim3_dmar(&self) -> &TIM3_DMAR {
        &self.tim3_dmar
    }
    ///0x60 - TIM3 alternate function option register 1
    #[inline(always)]
    pub const fn tim3_af1(&self) -> &TIM3_AF1 {
        &self.tim3_af1
    }
    ///0x68 - TIM3 timer input selection register
    #[inline(always)]
    pub const fn tim3_tisel(&self) -> &TIM3_TISEL {
        &self.tim3_tisel
    }
}
/**TIM3_CR1 (rw) register accessor: TIM3 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CR1)

For information about available fields see [`mod@tim3_cr1`] module*/
pub type TIM3_CR1 = crate::Reg<tim3_cr1::TIM3_CR1rs>;
///TIM3 control register 1
pub mod tim3_cr1;
/**TIM3_CR2 (rw) register accessor: TIM3 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim3_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CR2)

For information about available fields see [`mod@tim3_cr2`] module*/
pub type TIM3_CR2 = crate::Reg<tim3_cr2::TIM3_CR2rs>;
///TIM3 control register 2
pub mod tim3_cr2;
/**TIM3_SMCR (rw) register accessor: TIM3 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim3_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_SMCR)

For information about available fields see [`mod@tim3_smcr`] module*/
pub type TIM3_SMCR = crate::Reg<tim3_smcr::TIM3_SMCRrs>;
///TIM3 slave mode control register
pub mod tim3_smcr;
/**TIM3_DIER (rw) register accessor: TIM3 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim3_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_DIER)

For information about available fields see [`mod@tim3_dier`] module*/
pub type TIM3_DIER = crate::Reg<tim3_dier::TIM3_DIERrs>;
///TIM3 DMA/Interrupt enable register
pub mod tim3_dier;
/**TIM3_SR (rw) register accessor: TIM3 status register

You can [`read`](crate::Reg::read) this register and get [`tim3_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_SR)

For information about available fields see [`mod@tim3_sr`] module*/
pub type TIM3_SR = crate::Reg<tim3_sr::TIM3_SRrs>;
///TIM3 status register
pub mod tim3_sr;
/**TIM3_EGR (w) register accessor: TIM3 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_EGR)

For information about available fields see [`mod@tim3_egr`] module*/
pub type TIM3_EGR = crate::Reg<tim3_egr::TIM3_EGRrs>;
///TIM3 event generation register
pub mod tim3_egr;
/**TIM3_CCMR1 (rw) register accessor: TIM3 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCMR1)

For information about available fields see [`mod@tim3_ccmr1`] module*/
pub type TIM3_CCMR1 = crate::Reg<tim3_ccmr1::TIM3_CCMR1rs>;
///TIM3 capture/compare mode register 1
pub mod tim3_ccmr1;
/**TIM3_CCMR1_ALTERNATE1 (rw) register accessor: TIM3 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_ccmr1_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccmr1_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCMR1_ALTERNATE1)

For information about available fields see [`mod@tim3_ccmr1_alternate1`] module*/
pub type TIM3_CCMR1_ALTERNATE1 = crate::Reg<tim3_ccmr1_alternate1::TIM3_CCMR1_ALTERNATE1rs>;
///TIM3 capture/compare mode register 1
pub mod tim3_ccmr1_alternate1;
/**TIM3_CCMR2 (rw) register accessor: TIM3 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim3_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCMR2)

For information about available fields see [`mod@tim3_ccmr2`] module*/
pub type TIM3_CCMR2 = crate::Reg<tim3_ccmr2::TIM3_CCMR2rs>;
///TIM3 capture/compare mode register 2
pub mod tim3_ccmr2;
/**TIM3_CCMR2_ALTERNATE1 (rw) register accessor: TIM3 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim3_ccmr2_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccmr2_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCMR2_ALTERNATE1)

For information about available fields see [`mod@tim3_ccmr2_alternate1`] module*/
pub type TIM3_CCMR2_ALTERNATE1 = crate::Reg<tim3_ccmr2_alternate1::TIM3_CCMR2_ALTERNATE1rs>;
///TIM3 capture/compare mode register 2
pub mod tim3_ccmr2_alternate1;
/**TIM3_CCER (rw) register accessor: TIM3 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim3_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCER)

For information about available fields see [`mod@tim3_ccer`] module*/
pub type TIM3_CCER = crate::Reg<tim3_ccer::TIM3_CCERrs>;
///TIM3 capture/compare enable register
pub mod tim3_ccer;
/**TIM3_CNT (rw) register accessor: TIM3 counter

You can [`read`](crate::Reg::read) this register and get [`tim3_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CNT)

For information about available fields see [`mod@tim3_cnt`] module*/
pub type TIM3_CNT = crate::Reg<tim3_cnt::TIM3_CNTrs>;
///TIM3 counter
pub mod tim3_cnt;
/**TIM3_CNT_ALTERNATE1 (rw) register accessor: TIM3 counter

You can [`read`](crate::Reg::read) this register and get [`tim3_cnt_alternate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_cnt_alternate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CNT_ALTERNATE1)

For information about available fields see [`mod@tim3_cnt_alternate1`] module*/
pub type TIM3_CNT_ALTERNATE1 = crate::Reg<tim3_cnt_alternate1::TIM3_CNT_ALTERNATE1rs>;
///TIM3 counter
pub mod tim3_cnt_alternate1;
/**TIM3_PSC (rw) register accessor: TIM3 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim3_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_PSC)

For information about available fields see [`mod@tim3_psc`] module*/
pub type TIM3_PSC = crate::Reg<tim3_psc::TIM3_PSCrs>;
///TIM3 prescaler
pub mod tim3_psc;
/**TIM3_ARR (rw) register accessor: TIM3 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim3_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_ARR)

For information about available fields see [`mod@tim3_arr`] module*/
pub type TIM3_ARR = crate::Reg<tim3_arr::TIM3_ARRrs>;
///TIM3 auto-reload register
pub mod tim3_arr;
/**TIM3_CCR1 (rw) register accessor: TIM3 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCR1)

For information about available fields see [`mod@tim3_ccr1`] module*/
pub type TIM3_CCR1 = crate::Reg<tim3_ccr1::TIM3_CCR1rs>;
///TIM3 capture/compare register 1
pub mod tim3_ccr1;
/**TIM3_CCR2 (rw) register accessor: TIM3 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim3_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCR2)

For information about available fields see [`mod@tim3_ccr2`] module*/
pub type TIM3_CCR2 = crate::Reg<tim3_ccr2::TIM3_CCR2rs>;
///TIM3 capture/compare register 2
pub mod tim3_ccr2;
/**TIM3_CCR3 (rw) register accessor: TIM3 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim3_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCR3)

For information about available fields see [`mod@tim3_ccr3`] module*/
pub type TIM3_CCR3 = crate::Reg<tim3_ccr3::TIM3_CCR3rs>;
///TIM3 capture/compare register 3
pub mod tim3_ccr3;
/**TIM3_CCR4 (rw) register accessor: TIM3 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim3_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_CCR4)

For information about available fields see [`mod@tim3_ccr4`] module*/
pub type TIM3_CCR4 = crate::Reg<tim3_ccr4::TIM3_CCR4rs>;
///TIM3 capture/compare register 4
pub mod tim3_ccr4;
/**TIM3_DCR (rw) register accessor: TIM3 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim3_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_DCR)

For information about available fields see [`mod@tim3_dcr`] module*/
pub type TIM3_DCR = crate::Reg<tim3_dcr::TIM3_DCRrs>;
///TIM3 DMA control register
pub mod tim3_dcr;
/**TIM3_DMAR (rw) register accessor: TIM3 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim3_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_DMAR)

For information about available fields see [`mod@tim3_dmar`] module*/
pub type TIM3_DMAR = crate::Reg<tim3_dmar::TIM3_DMARrs>;
///TIM3 DMA address for full transfer
pub mod tim3_dmar;
/**TIM3_AF1 (rw) register accessor: TIM3 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_AF1)

For information about available fields see [`mod@tim3_af1`] module*/
pub type TIM3_AF1 = crate::Reg<tim3_af1::TIM3_AF1rs>;
///TIM3 alternate function option register 1
pub mod tim3_af1;
/**TIM3_TISEL (rw) register accessor: TIM3 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim3_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM3:TIM3_TISEL)

For information about available fields see [`mod@tim3_tisel`] module*/
pub type TIM3_TISEL = crate::Reg<tim3_tisel::TIM3_TISELrs>;
///TIM3 timer input selection register
pub mod tim3_tisel;
