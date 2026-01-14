#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    imr1: IMR1,
    emr1: EMR1,
    pr1: PR1,
    imr2: IMR2,
    emr2: EMR2,
    pr2: PR2,
    imr3: IMR3,
    emr3: EMR3,
}
impl RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    ///0x04 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    ///0x08 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    ///0x0c - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    ///0x10 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    ///0x14 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    ///0x18 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    ///0x1c - EXTI event mask register
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
    ///0x20 - EXTI pending register
    #[inline(always)]
    pub const fn pr1(&self) -> &PR1 {
        &self.pr1
    }
    ///0x24 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    ///0x28 - EXTI event mask register
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
    ///0x2c - EXTI pending register
    #[inline(always)]
    pub const fn pr2(&self) -> &PR2 {
        &self.pr2
    }
    ///0x30 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn imr3(&self) -> &IMR3 {
        &self.imr3
    }
    ///0x34 - EXTI event mask register
    #[inline(always)]
    pub const fn emr3(&self) -> &EMR3 {
        &self.emr3
    }
}
/**RTSR1 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:RTSR1)

For information about available fields see [`mod@rtsr1`] module*/
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
///EXTI rising trigger selection register
pub mod rtsr1;
/**FTSR1 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:FTSR1)

For information about available fields see [`mod@ftsr1`] module*/
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
///EXTI falling trigger selection register
pub mod ftsr1;
/**SWIER1 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:SWIER1)

For information about available fields see [`mod@swier1`] module*/
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
///EXTI software interrupt event register
pub mod swier1;
/**RTSR2 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:RTSR2)

For information about available fields see [`mod@rtsr2`] module*/
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
///EXTI rising trigger selection register
pub mod rtsr2;
/**FTSR2 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:FTSR2)

For information about available fields see [`mod@ftsr2`] module*/
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
///EXTI falling trigger selection register
pub mod ftsr2;
/**SWIER2 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:SWIER2)

For information about available fields see [`mod@swier2`] module*/
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
///EXTI software interrupt event register
pub mod swier2;
/**IMR1 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:IMR1)

For information about available fields see [`mod@imr1`] module*/
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
///EXTI interrupt mask register
pub mod imr1;
/**EMR1 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:EMR1)

For information about available fields see [`mod@emr1`] module*/
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
///EXTI event mask register
pub mod emr1;
/**PR1 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:PR1)

For information about available fields see [`mod@pr1`] module*/
pub type PR1 = crate::Reg<pr1::PR1rs>;
///EXTI pending register
pub mod pr1;
/**IMR2 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:IMR2)

For information about available fields see [`mod@imr2`] module*/
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
///EXTI interrupt mask register
pub mod imr2;
/**EMR2 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:EMR2)

For information about available fields see [`mod@emr2`] module*/
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
///EXTI event mask register
pub mod emr2;
/**PR2 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:PR2)

For information about available fields see [`mod@pr2`] module*/
pub type PR2 = crate::Reg<pr2::PR2rs>;
///EXTI pending register
pub mod pr2;
/**IMR3 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:IMR3)

For information about available fields see [`mod@imr3`] module*/
pub type IMR3 = crate::Reg<imr3::IMR3rs>;
///EXTI interrupt mask register
pub mod imr3;
/**EMR3 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`emr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#EXTI:EMR3)

For information about available fields see [`mod@emr3`] module*/
pub type EMR3 = crate::Reg<emr3::EMR3rs>;
///EXTI event mask register
pub mod emr3;
