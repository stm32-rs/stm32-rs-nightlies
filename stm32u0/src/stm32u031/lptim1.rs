#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_lptim1_isr: [u8; 0x04],
    _reserved_1_lptim1_icr: [u8; 0x04],
    _reserved_2_lptim1_dier: [u8; 0x04],
    lptim1_cfgr: LPTIM1_CFGR,
    lptim1_cr: LPTIM1_CR,
    lptim1_ccr1: LPTIM1_CCR1,
    lptim1_arr: LPTIM1_ARR,
    lptim1_cnt: LPTIM1_CNT,
    _reserved8: [u8; 0x04],
    lptim1_cfgr2: LPTIM1_CFGR2,
    lptim1_rcr: LPTIM1_RCR,
    lptim1_ccmr1: LPTIM1_CCMR1,
    lptim1_ccmr2: LPTIM1_CCMR2,
    lptim1_ccr2: LPTIM1_CCR2,
    lptim1_ccr3: LPTIM1_CCR3,
    lptim1_ccr4: LPTIM1_CCR4,
}
impl RegisterBlock {
    ///0x00 - LPTIM1 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn lptim1_isr_input(&self) -> &LPTIM1_ISR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - LPTIM1 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn lptim1_isr_output(&self) -> &LPTIM1_ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - LPTIM1 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn lptim1_icr_input(&self) -> &LPTIM1_ICR_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - LPTIM1 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn lptim1_icr_output(&self) -> &LPTIM1_ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - LPTIM1 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn lptim1_dier_input(&self) -> &LPTIM1_DIER_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - LPTIM1 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn lptim1_dier_output(&self) -> &LPTIM1_DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - LPTIM configuration register
    #[inline(always)]
    pub const fn lptim1_cfgr(&self) -> &LPTIM1_CFGR {
        &self.lptim1_cfgr
    }
    ///0x10 - LPTIM control register
    #[inline(always)]
    pub const fn lptim1_cr(&self) -> &LPTIM1_CR {
        &self.lptim1_cr
    }
    ///0x14 - LPTIM compare register 1
    #[inline(always)]
    pub const fn lptim1_ccr1(&self) -> &LPTIM1_CCR1 {
        &self.lptim1_ccr1
    }
    ///0x18 - LPTIM autoreload register
    #[inline(always)]
    pub const fn lptim1_arr(&self) -> &LPTIM1_ARR {
        &self.lptim1_arr
    }
    ///0x1c - LPTIM counter register
    #[inline(always)]
    pub const fn lptim1_cnt(&self) -> &LPTIM1_CNT {
        &self.lptim1_cnt
    }
    ///0x24 - LPTIM configuration register 2
    #[inline(always)]
    pub const fn lptim1_cfgr2(&self) -> &LPTIM1_CFGR2 {
        &self.lptim1_cfgr2
    }
    ///0x28 - LPTIM repetition register
    #[inline(always)]
    pub const fn lptim1_rcr(&self) -> &LPTIM1_RCR {
        &self.lptim1_rcr
    }
    ///0x2c - LPTIM capture/compare mode register 1
    #[inline(always)]
    pub const fn lptim1_ccmr1(&self) -> &LPTIM1_CCMR1 {
        &self.lptim1_ccmr1
    }
    ///0x30 - LPTIM capture/compare mode register 2
    #[inline(always)]
    pub const fn lptim1_ccmr2(&self) -> &LPTIM1_CCMR2 {
        &self.lptim1_ccmr2
    }
    ///0x34 - LPTIM compare register 2
    #[inline(always)]
    pub const fn lptim1_ccr2(&self) -> &LPTIM1_CCR2 {
        &self.lptim1_ccr2
    }
    ///0x38 - LPTIM compare register 3
    #[inline(always)]
    pub const fn lptim1_ccr3(&self) -> &LPTIM1_CCR3 {
        &self.lptim1_ccr3
    }
    ///0x3c - LPTIM compare register 4
    #[inline(always)]
    pub const fn lptim1_ccr4(&self) -> &LPTIM1_CCR4 {
        &self.lptim1_ccr4
    }
}
/**LPTIM1_ISR_OUTPUT (r) register accessor: LPTIM1 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim1_isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_ISR_OUTPUT)

For information about available fields see [`mod@lptim1_isr_output`]
module*/
pub type LPTIM1_ISR_OUTPUT = crate::Reg<lptim1_isr_output::LPTIM1_ISR_OUTPUTrs>;
///LPTIM1 interrupt and status register \[alternate\]
pub mod lptim1_isr_output;
/**LPTIM1_ISR_INPUT (r) register accessor: LPTIM1 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim1_isr_input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_ISR_INPUT)

For information about available fields see [`mod@lptim1_isr_input`]
module*/
pub type LPTIM1_ISR_INPUT = crate::Reg<lptim1_isr_input::LPTIM1_ISR_INPUTrs>;
///LPTIM1 interrupt and status register \[alternate\]
pub mod lptim1_isr_input;
/**LPTIM1_ICR_OUTPUT (w) register accessor: LPTIM1 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_ICR_OUTPUT)

For information about available fields see [`mod@lptim1_icr_output`]
module*/
pub type LPTIM1_ICR_OUTPUT = crate::Reg<lptim1_icr_output::LPTIM1_ICR_OUTPUTrs>;
///LPTIM1 interrupt clear register \[alternate\]
pub mod lptim1_icr_output;
/**LPTIM1_ICR_INPUT (w) register accessor: LPTIM1 interrupt clear register \[alternate\]

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_icr_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_ICR_INPUT)

For information about available fields see [`mod@lptim1_icr_input`]
module*/
pub type LPTIM1_ICR_INPUT = crate::Reg<lptim1_icr_input::LPTIM1_ICR_INPUTrs>;
///LPTIM1 interrupt clear register \[alternate\]
pub mod lptim1_icr_input;
/**LPTIM1_DIER_OUTPUT (rw) register accessor: LPTIM1 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim1_dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_DIER_OUTPUT)

For information about available fields see [`mod@lptim1_dier_output`]
module*/
pub type LPTIM1_DIER_OUTPUT = crate::Reg<lptim1_dier_output::LPTIM1_DIER_OUTPUTrs>;
///LPTIM1 interrupt enable register \[alternate\]
pub mod lptim1_dier_output;
/**LPTIM1_DIER_INPUT (rw) register accessor: LPTIM1 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`lptim1_dier_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_dier_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_DIER_INPUT)

For information about available fields see [`mod@lptim1_dier_input`]
module*/
pub type LPTIM1_DIER_INPUT = crate::Reg<lptim1_dier_input::LPTIM1_DIER_INPUTrs>;
///LPTIM1 interrupt enable register \[alternate\]
pub mod lptim1_dier_input;
/**LPTIM1_CFGR (rw) register accessor: LPTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`lptim1_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CFGR)

For information about available fields see [`mod@lptim1_cfgr`]
module*/
pub type LPTIM1_CFGR = crate::Reg<lptim1_cfgr::LPTIM1_CFGRrs>;
///LPTIM configuration register
pub mod lptim1_cfgr;
/**LPTIM1_CR (rw) register accessor: LPTIM control register

You can [`read`](crate::Reg::read) this register and get [`lptim1_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CR)

For information about available fields see [`mod@lptim1_cr`]
module*/
pub type LPTIM1_CR = crate::Reg<lptim1_cr::LPTIM1_CRrs>;
///LPTIM control register
pub mod lptim1_cr;
/**LPTIM1_CCR1 (rw) register accessor: LPTIM compare register 1

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CCR1)

For information about available fields see [`mod@lptim1_ccr1`]
module*/
pub type LPTIM1_CCR1 = crate::Reg<lptim1_ccr1::LPTIM1_CCR1rs>;
///LPTIM compare register 1
pub mod lptim1_ccr1;
/**LPTIM1_ARR (rw) register accessor: LPTIM autoreload register

You can [`read`](crate::Reg::read) this register and get [`lptim1_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_ARR)

For information about available fields see [`mod@lptim1_arr`]
module*/
pub type LPTIM1_ARR = crate::Reg<lptim1_arr::LPTIM1_ARRrs>;
///LPTIM autoreload register
pub mod lptim1_arr;
/**LPTIM1_CNT (r) register accessor: LPTIM counter register

You can [`read`](crate::Reg::read) this register and get [`lptim1_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CNT)

For information about available fields see [`mod@lptim1_cnt`]
module*/
pub type LPTIM1_CNT = crate::Reg<lptim1_cnt::LPTIM1_CNTrs>;
///LPTIM counter register
pub mod lptim1_cnt;
/**LPTIM1_CFGR2 (rw) register accessor: LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lptim1_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CFGR2)

For information about available fields see [`mod@lptim1_cfgr2`]
module*/
pub type LPTIM1_CFGR2 = crate::Reg<lptim1_cfgr2::LPTIM1_CFGR2rs>;
///LPTIM configuration register 2
pub mod lptim1_cfgr2;
/**LPTIM1_RCR (rw) register accessor: LPTIM repetition register

You can [`read`](crate::Reg::read) this register and get [`lptim1_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_RCR)

For information about available fields see [`mod@lptim1_rcr`]
module*/
pub type LPTIM1_RCR = crate::Reg<lptim1_rcr::LPTIM1_RCRrs>;
///LPTIM repetition register
pub mod lptim1_rcr;
/**LPTIM1_CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CCMR1)

For information about available fields see [`mod@lptim1_ccmr1`]
module*/
pub type LPTIM1_CCMR1 = crate::Reg<lptim1_ccmr1::LPTIM1_CCMR1rs>;
///LPTIM capture/compare mode register 1
pub mod lptim1_ccmr1;
/**LPTIM1_CCMR2 (rw) register accessor: LPTIM capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CCMR2)

For information about available fields see [`mod@lptim1_ccmr2`]
module*/
pub type LPTIM1_CCMR2 = crate::Reg<lptim1_ccmr2::LPTIM1_CCMR2rs>;
///LPTIM capture/compare mode register 2
pub mod lptim1_ccmr2;
/**LPTIM1_CCR2 (rw) register accessor: LPTIM compare register 2

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CCR2)

For information about available fields see [`mod@lptim1_ccr2`]
module*/
pub type LPTIM1_CCR2 = crate::Reg<lptim1_ccr2::LPTIM1_CCR2rs>;
///LPTIM compare register 2
pub mod lptim1_ccr2;
/**LPTIM1_CCR3 (rw) register accessor: LPTIM compare register 3

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CCR3)

For information about available fields see [`mod@lptim1_ccr3`]
module*/
pub type LPTIM1_CCR3 = crate::Reg<lptim1_ccr3::LPTIM1_CCR3rs>;
///LPTIM compare register 3
pub mod lptim1_ccr3;
/**LPTIM1_CCR4 (rw) register accessor: LPTIM compare register 4

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM1:LPTIM1_CCR4)

For information about available fields see [`mod@lptim1_ccr4`]
module*/
pub type LPTIM1_CCR4 = crate::Reg<lptim1_ccr4::LPTIM1_CCR4rs>;
///LPTIM compare register 4
pub mod lptim1_ccr4;
