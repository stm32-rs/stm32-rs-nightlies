#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim_cr1: TIM_CR1,
    _reserved1: [u8; 0x02],
    tim_cr2: TIM_CR2,
    tim_smcr: TIM_SMCR,
    tim_dier: TIM_DIER,
    tim_sr: TIM_SR,
    tim_egr: TIM_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim_ccmr: [u8; 0x04],
    _reserved_7_tim_ccmr: [u8; 0x04],
    tim_ccer: TIM_CCER,
    _reserved9: [u8; 0x02],
    tim_cnt: TIM_CNT,
    tim_psc: TIM_PSC,
    _reserved11: [u8; 0x02],
    tim_arr: TIM_ARR,
    _reserved12: [u8; 0x04],
    tim_ccr1: TIM_CCR1,
    tim_ccr2: TIM_CCR2,
    tim_ccr3: TIM_CCR3,
    tim_ccr4: TIM_CCR4,
    _reserved16: [u8; 0x14],
    tim_ecr: TIM_ECR,
    tim_tisel: TIM_TISEL,
    tim_af1: TIM_AF1,
    tim_af2: TIM_AF2,
    _reserved20: [u8; 0x0374],
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
    ///0x08 - TIM slave mode control register
    #[inline(always)]
    pub const fn tim_smcr(&self) -> &TIM_SMCR {
        &self.tim_smcr
    }
    ///0x0c - TIM DMA/Interrupt enable register
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
    pub const fn tim_ccmr1_alternate(&self) -> &TIM_CCMR1_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - TIM capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim_ccmr1(&self) -> &TIM_CCMR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - TIM capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim_ccmr2_alternate(&self) -> &TIM_CCMR2_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - TIM capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim_ccmr2(&self) -> &TIM_CCMR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
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
    ///0x34 - TIM capture/compare register 1
    #[inline(always)]
    pub const fn tim_ccr1(&self) -> &TIM_CCR1 {
        &self.tim_ccr1
    }
    ///0x38 - TIM capture/compare register 2
    #[inline(always)]
    pub const fn tim_ccr2(&self) -> &TIM_CCR2 {
        &self.tim_ccr2
    }
    ///0x3c - TIM capture/compare register 3
    #[inline(always)]
    pub const fn tim_ccr3(&self) -> &TIM_CCR3 {
        &self.tim_ccr3
    }
    ///0x40 - TIM capture/compare register 4
    #[inline(always)]
    pub const fn tim_ccr4(&self) -> &TIM_CCR4 {
        &self.tim_ccr4
    }
    ///0x58 - TIM timer encoder control register
    #[inline(always)]
    pub const fn tim_ecr(&self) -> &TIM_ECR {
        &self.tim_ecr
    }
    ///0x5c - TIM timer input selection register
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
    ///0x3dc - TIM DMA control register
    #[inline(always)]
    pub const fn tim_dcr(&self) -> &TIM_DCR {
        &self.tim_dcr
    }
    ///0x3e0 - TIM DMA address for full transfer
    #[inline(always)]
    pub const fn tim_dmar(&self) -> &TIM_DMAR {
        &self.tim_dmar
    }
}
/**TIM_CR1 (rw) register accessor: TIM control register 1

You can [`read`](crate::Reg::read) this register and get [`tim_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CR1)

For information about available fields see [`mod@tim_cr1`] module*/
pub type TIM_CR1 = crate::Reg<tim_cr1::TIM_CR1rs>;
///TIM control register 1
pub mod tim_cr1;
/**TIM_CR2 (rw) register accessor: TIM control register 2

You can [`read`](crate::Reg::read) this register and get [`tim_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CR2)

For information about available fields see [`mod@tim_cr2`] module*/
pub type TIM_CR2 = crate::Reg<tim_cr2::TIM_CR2rs>;
///TIM control register 2
pub mod tim_cr2;
/**TIM_SMCR (rw) register accessor: TIM slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_SMCR)

For information about available fields see [`mod@tim_smcr`] module*/
pub type TIM_SMCR = crate::Reg<tim_smcr::TIM_SMCRrs>;
///TIM slave mode control register
pub mod tim_smcr;
/**TIM_DIER (rw) register accessor: TIM DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_DIER)

For information about available fields see [`mod@tim_dier`] module*/
pub type TIM_DIER = crate::Reg<tim_dier::TIM_DIERrs>;
///TIM DMA/Interrupt enable register
pub mod tim_dier;
/**TIM_SR (rw) register accessor: TIM status register

You can [`read`](crate::Reg::read) this register and get [`tim_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_SR)

For information about available fields see [`mod@tim_sr`] module*/
pub type TIM_SR = crate::Reg<tim_sr::TIM_SRrs>;
///TIM status register
pub mod tim_sr;
/**TIM_EGR (w) register accessor: TIM event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_EGR)

For information about available fields see [`mod@tim_egr`] module*/
pub type TIM_EGR = crate::Reg<tim_egr::TIM_EGRrs>;
///TIM event generation register
pub mod tim_egr;
/**TIM_CCMR1 (rw) register accessor: TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCMR1)

For information about available fields see [`mod@tim_ccmr1`] module*/
pub type TIM_CCMR1 = crate::Reg<tim_ccmr1::TIM_CCMR1rs>;
///TIM capture/compare mode register 1 \[alternate\]
pub mod tim_ccmr1;
/**TIM_CCMR1_ALTERNATE (rw) register accessor: TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr1_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr1_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCMR1_ALTERNATE)

For information about available fields see [`mod@tim_ccmr1_alternate`] module*/
pub type TIM_CCMR1_ALTERNATE = crate::Reg<tim_ccmr1_alternate::TIM_CCMR1_ALTERNATErs>;
///TIM capture/compare mode register 1 \[alternate\]
pub mod tim_ccmr1_alternate;
/**TIM_CCMR2 (rw) register accessor: TIM capture/compare mode register 2 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCMR2)

For information about available fields see [`mod@tim_ccmr2`] module*/
pub type TIM_CCMR2 = crate::Reg<tim_ccmr2::TIM_CCMR2rs>;
///TIM capture/compare mode register 2 \[alternate\]
pub mod tim_ccmr2;
/**TIM_CCMR2_ALTERNATE (rw) register accessor: TIM capture/compare mode register 2 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr2_alternate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr2_alternate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCMR2_ALTERNATE)

For information about available fields see [`mod@tim_ccmr2_alternate`] module*/
pub type TIM_CCMR2_ALTERNATE = crate::Reg<tim_ccmr2_alternate::TIM_CCMR2_ALTERNATErs>;
///TIM capture/compare mode register 2 \[alternate\]
pub mod tim_ccmr2_alternate;
/**TIM_CCER (rw) register accessor: TIM capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCER)

For information about available fields see [`mod@tim_ccer`] module*/
pub type TIM_CCER = crate::Reg<tim_ccer::TIM_CCERrs>;
///TIM capture/compare enable register
pub mod tim_ccer;
/**TIM_CNT (rw) register accessor: TIM counter

You can [`read`](crate::Reg::read) this register and get [`tim_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CNT)

For information about available fields see [`mod@tim_cnt`] module*/
pub type TIM_CNT = crate::Reg<tim_cnt::TIM_CNTrs>;
///TIM counter
pub mod tim_cnt;
/**TIM_PSC (rw) register accessor: TIM prescaler

You can [`read`](crate::Reg::read) this register and get [`tim_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_PSC)

For information about available fields see [`mod@tim_psc`] module*/
pub type TIM_PSC = crate::Reg<tim_psc::TIM_PSCrs>;
///TIM prescaler
pub mod tim_psc;
/**TIM_ARR (rw) register accessor: TIM auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_ARR)

For information about available fields see [`mod@tim_arr`] module*/
pub type TIM_ARR = crate::Reg<tim_arr::TIM_ARRrs>;
///TIM auto-reload register
pub mod tim_arr;
/**TIM_CCR1 (rw) register accessor: TIM capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCR1)

For information about available fields see [`mod@tim_ccr1`] module*/
pub type TIM_CCR1 = crate::Reg<tim_ccr1::TIM_CCR1rs>;
///TIM capture/compare register 1
pub mod tim_ccr1;
/**TIM_CCR2 (rw) register accessor: TIM capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCR2)

For information about available fields see [`mod@tim_ccr2`] module*/
pub type TIM_CCR2 = crate::Reg<tim_ccr2::TIM_CCR2rs>;
///TIM capture/compare register 2
pub mod tim_ccr2;
/**TIM_CCR3 (rw) register accessor: TIM capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCR3)

For information about available fields see [`mod@tim_ccr3`] module*/
pub type TIM_CCR3 = crate::Reg<tim_ccr3::TIM_CCR3rs>;
///TIM capture/compare register 3
pub mod tim_ccr3;
/**TIM_CCR4 (rw) register accessor: TIM capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CCR4)

For information about available fields see [`mod@tim_ccr4`] module*/
pub type TIM_CCR4 = crate::Reg<tim_ccr4::TIM_CCR4rs>;
///TIM capture/compare register 4
pub mod tim_ccr4;
/**TIM_ECR (rw) register accessor: TIM timer encoder control register

You can [`read`](crate::Reg::read) this register and get [`tim_ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_ECR)

For information about available fields see [`mod@tim_ecr`] module*/
pub type TIM_ECR = crate::Reg<tim_ecr::TIM_ECRrs>;
///TIM timer encoder control register
pub mod tim_ecr;
/**TIM_TISEL (rw) register accessor: TIM timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_TISEL)

For information about available fields see [`mod@tim_tisel`] module*/
pub type TIM_TISEL = crate::Reg<tim_tisel::TIM_TISELrs>;
///TIM timer input selection register
pub mod tim_tisel;
/**TIM_AF1 (rw) register accessor: TIM alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`tim_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_AF1)

For information about available fields see [`mod@tim_af1`] module*/
pub type TIM_AF1 = crate::Reg<tim_af1::TIM_AF1rs>;
///TIM alternate function register 1
pub mod tim_af1;
/**TIM_AF2 (rw) register accessor: TIM alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`tim_af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_AF2)

For information about available fields see [`mod@tim_af2`] module*/
pub type TIM_AF2 = crate::Reg<tim_af2::TIM_AF2rs>;
///TIM alternate function register 2
pub mod tim_af2;
/**TIM_DCR (rw) register accessor: TIM DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_DCR)

For information about available fields see [`mod@tim_dcr`] module*/
pub type TIM_DCR = crate::Reg<tim_dcr::TIM_DCRrs>;
///TIM DMA control register
pub mod tim_dcr;
/**TIM_DMAR (rw) register accessor: TIM DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_DMAR)

For information about available fields see [`mod@tim_dmar`] module*/
pub type TIM_DMAR = crate::Reg<tim_dmar::TIM_DMARrs>;
///TIM DMA address for full transfer
pub mod tim_dmar;
