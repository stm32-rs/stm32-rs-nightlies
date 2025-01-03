#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    exti_rtsr1: EXTI_RTSR1,
    exti_ftsr1: EXTI_FTSR1,
    exti_swier1: EXTI_SWIER1,
    exti_rpr1: EXTI_RPR1,
    exti_fpr1: EXTI_FPR1,
    _reserved5: [u8; 0x4c],
    exti_exticr1: EXTI_EXTICR1,
    exti_exticr2: EXTI_EXTICR2,
    exti_exticr3: EXTI_EXTICR3,
    exti_exticr4: EXTI_EXTICR4,
    _reserved9: [u8; 0x10],
    exti_imr1: EXTI_IMR1,
    exti_emr1: EXTI_EMR1,
    _reserved11: [u8; 0x08],
    exti_imr2: EXTI_IMR2,
    exti_emr2: EXTI_EMR2,
}
impl RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn exti_rtsr1(&self) -> &EXTI_RTSR1 {
        &self.exti_rtsr1
    }
    ///0x04 - EXTI falling trigger selection register 1
    #[inline(always)]
    pub const fn exti_ftsr1(&self) -> &EXTI_FTSR1 {
        &self.exti_ftsr1
    }
    ///0x08 - EXTI software interrupt event register 1
    #[inline(always)]
    pub const fn exti_swier1(&self) -> &EXTI_SWIER1 {
        &self.exti_swier1
    }
    ///0x0c - EXTI rising edge pending register 1
    #[inline(always)]
    pub const fn exti_rpr1(&self) -> &EXTI_RPR1 {
        &self.exti_rpr1
    }
    ///0x10 - EXTI falling edge pending register 1
    #[inline(always)]
    pub const fn exti_fpr1(&self) -> &EXTI_FPR1 {
        &self.exti_fpr1
    }
    ///0x60 - EXTI external interrupt selection register 1
    #[inline(always)]
    pub const fn exti_exticr1(&self) -> &EXTI_EXTICR1 {
        &self.exti_exticr1
    }
    ///0x64 - EXTI external interrupt selection register 2
    #[inline(always)]
    pub const fn exti_exticr2(&self) -> &EXTI_EXTICR2 {
        &self.exti_exticr2
    }
    ///0x68 - EXTI external interrupt selection register 3
    #[inline(always)]
    pub const fn exti_exticr3(&self) -> &EXTI_EXTICR3 {
        &self.exti_exticr3
    }
    ///0x6c - EXTI external interrupt selection register 4
    #[inline(always)]
    pub const fn exti_exticr4(&self) -> &EXTI_EXTICR4 {
        &self.exti_exticr4
    }
    ///0x80 - EXTI CPU wake-up with interrupt mask register
    #[inline(always)]
    pub const fn exti_imr1(&self) -> &EXTI_IMR1 {
        &self.exti_imr1
    }
    ///0x84 - EXTI CPU wake-up with event mask register
    #[inline(always)]
    pub const fn exti_emr1(&self) -> &EXTI_EMR1 {
        &self.exti_emr1
    }
    ///0x90 - EXTI CPU wake-up with interrupt mask register
    #[inline(always)]
    pub const fn exti_imr2(&self) -> &EXTI_IMR2 {
        &self.exti_imr2
    }
    ///0x94 - EXTI CPU wake-up with event mask register
    #[inline(always)]
    pub const fn exti_emr2(&self) -> &EXTI_EMR2 {
        &self.exti_emr2
    }
}
/**EXTI_RTSR1 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`exti_rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_RTSR1)

For information about available fields see [`mod@exti_rtsr1`]
module*/
pub type EXTI_RTSR1 = crate::Reg<exti_rtsr1::EXTI_RTSR1rs>;
///EXTI rising trigger selection register
pub mod exti_rtsr1;
/**EXTI_FTSR1 (rw) register accessor: EXTI falling trigger selection register 1

You can [`read`](crate::Reg::read) this register and get [`exti_ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_FTSR1)

For information about available fields see [`mod@exti_ftsr1`]
module*/
pub type EXTI_FTSR1 = crate::Reg<exti_ftsr1::EXTI_FTSR1rs>;
///EXTI falling trigger selection register 1
pub mod exti_ftsr1;
/**EXTI_SWIER1 (rw) register accessor: EXTI software interrupt event register 1

You can [`read`](crate::Reg::read) this register and get [`exti_swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_SWIER1)

For information about available fields see [`mod@exti_swier1`]
module*/
pub type EXTI_SWIER1 = crate::Reg<exti_swier1::EXTI_SWIER1rs>;
///EXTI software interrupt event register 1
pub mod exti_swier1;
/**EXTI_RPR1 (rw) register accessor: EXTI rising edge pending register 1

You can [`read`](crate::Reg::read) this register and get [`exti_rpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_rpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_RPR1)

For information about available fields see [`mod@exti_rpr1`]
module*/
pub type EXTI_RPR1 = crate::Reg<exti_rpr1::EXTI_RPR1rs>;
///EXTI rising edge pending register 1
pub mod exti_rpr1;
/**EXTI_FPR1 (rw) register accessor: EXTI falling edge pending register 1

You can [`read`](crate::Reg::read) this register and get [`exti_fpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_fpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_FPR1)

For information about available fields see [`mod@exti_fpr1`]
module*/
pub type EXTI_FPR1 = crate::Reg<exti_fpr1::EXTI_FPR1rs>;
///EXTI falling edge pending register 1
pub mod exti_fpr1;
/**EXTI_EXTICR1 (rw) register accessor: EXTI external interrupt selection register 1

You can [`read`](crate::Reg::read) this register and get [`exti_exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_EXTICR1)

For information about available fields see [`mod@exti_exticr1`]
module*/
pub type EXTI_EXTICR1 = crate::Reg<exti_exticr1::EXTI_EXTICR1rs>;
///EXTI external interrupt selection register 1
pub mod exti_exticr1;
/**EXTI_EXTICR2 (rw) register accessor: EXTI external interrupt selection register 2

You can [`read`](crate::Reg::read) this register and get [`exti_exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_EXTICR2)

For information about available fields see [`mod@exti_exticr2`]
module*/
pub type EXTI_EXTICR2 = crate::Reg<exti_exticr2::EXTI_EXTICR2rs>;
///EXTI external interrupt selection register 2
pub mod exti_exticr2;
/**EXTI_EXTICR3 (rw) register accessor: EXTI external interrupt selection register 3

You can [`read`](crate::Reg::read) this register and get [`exti_exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_EXTICR3)

For information about available fields see [`mod@exti_exticr3`]
module*/
pub type EXTI_EXTICR3 = crate::Reg<exti_exticr3::EXTI_EXTICR3rs>;
///EXTI external interrupt selection register 3
pub mod exti_exticr3;
/**EXTI_EXTICR4 (rw) register accessor: EXTI external interrupt selection register 4

You can [`read`](crate::Reg::read) this register and get [`exti_exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_EXTICR4)

For information about available fields see [`mod@exti_exticr4`]
module*/
pub type EXTI_EXTICR4 = crate::Reg<exti_exticr4::EXTI_EXTICR4rs>;
///EXTI external interrupt selection register 4
pub mod exti_exticr4;
/**EXTI_IMR1 (rw) register accessor: EXTI CPU wake-up with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`exti_imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_IMR1)

For information about available fields see [`mod@exti_imr1`]
module*/
pub type EXTI_IMR1 = crate::Reg<exti_imr1::EXTI_IMR1rs>;
///EXTI CPU wake-up with interrupt mask register
pub mod exti_imr1;
/**EXTI_EMR1 (rw) register accessor: EXTI CPU wake-up with event mask register

You can [`read`](crate::Reg::read) this register and get [`exti_emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_EMR1)

For information about available fields see [`mod@exti_emr1`]
module*/
pub type EXTI_EMR1 = crate::Reg<exti_emr1::EXTI_EMR1rs>;
///EXTI CPU wake-up with event mask register
pub mod exti_emr1;
/**EXTI_IMR2 (rw) register accessor: EXTI CPU wake-up with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`exti_imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_IMR2)

For information about available fields see [`mod@exti_imr2`]
module*/
pub type EXTI_IMR2 = crate::Reg<exti_imr2::EXTI_IMR2rs>;
///EXTI CPU wake-up with interrupt mask register
pub mod exti_imr2;
/**EXTI_EMR2 (rw) register accessor: EXTI CPU wake-up with event mask register

You can [`read`](crate::Reg::read) this register and get [`exti_emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#EXTI:EXTI_EMR2)

For information about available fields see [`mod@exti_emr2`]
module*/
pub type EXTI_EMR2 = crate::Reg<exti_emr2::EXTI_EMR2rs>;
///EXTI CPU wake-up with event mask register
pub mod exti_emr2;
