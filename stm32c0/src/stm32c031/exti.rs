#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    rpr1: RPR1,
    fpr1: FPR1,
    _reserved5: [u8; 0x4c],
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    _reserved9: [u8; 0x10],
    imr1: IMR1,
    emr1: EMR1,
}
impl RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    ///0x04 - EXTI falling trigger selection register 1
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    ///0x08 - EXTI software interrupt event register 1
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    ///0x0c - EXTI rising edge pending register 1
    #[inline(always)]
    pub const fn rpr1(&self) -> &RPR1 {
        &self.rpr1
    }
    ///0x10 - EXTI falling edge pending register 1
    #[inline(always)]
    pub const fn fpr1(&self) -> &FPR1 {
        &self.fpr1
    }
    ///0x60 - EXTI external interrupt selection register
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    ///0x64 - EXTI external interrupt selection register
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    ///0x68 - EXTI external interrupt selection register
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    ///0x6c - EXTI external interrupt selection register
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    ///0x80 - EXTI CPU wakeup with interrupt mask register
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    ///0x84 - EXTI CPU wakeup with event mask register
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
}
/**RTSR1 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:RTSR1)

For information about available fields see [`mod@rtsr1`]
module*/
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
///EXTI rising trigger selection register
pub mod rtsr1;
/**FTSR1 (rw) register accessor: EXTI falling trigger selection register 1

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:FTSR1)

For information about available fields see [`mod@ftsr1`]
module*/
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
///EXTI falling trigger selection register 1
pub mod ftsr1;
/**SWIER1 (rw) register accessor: EXTI software interrupt event register 1

You can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:SWIER1)

For information about available fields see [`mod@swier1`]
module*/
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
///EXTI software interrupt event register 1
pub mod swier1;
/**RPR1 (rw) register accessor: EXTI rising edge pending register 1

You can [`read`](crate::Reg::read) this register and get [`rpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:RPR1)

For information about available fields see [`mod@rpr1`]
module*/
pub type RPR1 = crate::Reg<rpr1::RPR1rs>;
///EXTI rising edge pending register 1
pub mod rpr1;
/**FPR1 (rw) register accessor: EXTI falling edge pending register 1

You can [`read`](crate::Reg::read) this register and get [`fpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:FPR1)

For information about available fields see [`mod@fpr1`]
module*/
pub type FPR1 = crate::Reg<fpr1::FPR1rs>;
///EXTI falling edge pending register 1
pub mod fpr1;
/**EXTICR1 (rw) register accessor: EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:EXTICR1)

For information about available fields see [`mod@exticr1`]
module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///EXTI external interrupt selection register
pub mod exticr1;
/**EXTICR2 (rw) register accessor: EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:EXTICR2)

For information about available fields see [`mod@exticr2`]
module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///EXTI external interrupt selection register
pub mod exticr2;
/**EXTICR3 (rw) register accessor: EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:EXTICR3)

For information about available fields see [`mod@exticr3`]
module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///EXTI external interrupt selection register
pub mod exticr3;
/**EXTICR4 (rw) register accessor: EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:EXTICR4)

For information about available fields see [`mod@exticr4`]
module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///EXTI external interrupt selection register
pub mod exticr4;
/**IMR1 (rw) register accessor: EXTI CPU wakeup with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:IMR1)

For information about available fields see [`mod@imr1`]
module*/
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
///EXTI CPU wakeup with interrupt mask register
pub mod imr1;
/**EMR1 (rw) register accessor: EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:EMR1)

For information about available fields see [`mod@emr1`]
module*/
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
///EXTI CPU wakeup with event mask register
pub mod emr1;
