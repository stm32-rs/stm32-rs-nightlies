#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    d3pmr1: D3PMR1,
    d3pcr1l: D3PCR1L,
    d3pcr1h: D3PCR1H,
    _reserved6: [u8; 0x08],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    d3pmr2: D3PMR2,
    d3pcr2l: D3PCR2L,
    d3pcr2h: D3PCR2H,
    _reserved12: [u8; 0x08],
    rtsr3: RTSR3,
    ftsr3: FTSR3,
    swier3: SWIER3,
    d3pmr3: D3PMR3,
    _reserved16: [u8; 0x04],
    d3pcr3h: D3PCR3H,
    _reserved17: [u8; 0x28],
    cpuimr1: CPUIMR1,
    cpuemr1: CPUEMR1,
    cpupr1: CPUPR1,
    _reserved20: [u8; 0x04],
    cpuimr2: CPUIMR2,
    cpuemr2: CPUEMR2,
    cpupr2: CPUPR2,
    _reserved23: [u8; 0x04],
    cpuimr3: CPUIMR3,
    cpuemr3: CPUEMR3,
    cpupr3: CPUPR3,
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
    ///0x0c - EXTI D3 pending mask register
    #[inline(always)]
    pub const fn d3pmr1(&self) -> &D3PMR1 {
        &self.d3pmr1
    }
    ///0x10 - EXTI D3 pending clear selection register low
    #[inline(always)]
    pub const fn d3pcr1l(&self) -> &D3PCR1L {
        &self.d3pcr1l
    }
    ///0x14 - EXTI D3 pending clear selection register high
    #[inline(always)]
    pub const fn d3pcr1h(&self) -> &D3PCR1H {
        &self.d3pcr1h
    }
    ///0x20 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    ///0x24 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    ///0x28 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    ///0x2c - EXTI D3 pending mask register
    #[inline(always)]
    pub const fn d3pmr2(&self) -> &D3PMR2 {
        &self.d3pmr2
    }
    ///0x30 - EXTI D3 pending clear selection register low
    #[inline(always)]
    pub const fn d3pcr2l(&self) -> &D3PCR2L {
        &self.d3pcr2l
    }
    ///0x34 - EXTI D3 pending clear selection register high
    #[inline(always)]
    pub const fn d3pcr2h(&self) -> &D3PCR2H {
        &self.d3pcr2h
    }
    ///0x40 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr3(&self) -> &RTSR3 {
        &self.rtsr3
    }
    ///0x44 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr3(&self) -> &FTSR3 {
        &self.ftsr3
    }
    ///0x48 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier3(&self) -> &SWIER3 {
        &self.swier3
    }
    ///0x4c - EXTI D3 pending mask register
    #[inline(always)]
    pub const fn d3pmr3(&self) -> &D3PMR3 {
        &self.d3pmr3
    }
    ///0x54 - EXTI D3 pending clear selection register high
    #[inline(always)]
    pub const fn d3pcr3h(&self) -> &D3PCR3H {
        &self.d3pcr3h
    }
    ///0x80 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn cpuimr1(&self) -> &CPUIMR1 {
        &self.cpuimr1
    }
    ///0x84 - EXTI event mask register
    #[inline(always)]
    pub const fn cpuemr1(&self) -> &CPUEMR1 {
        &self.cpuemr1
    }
    ///0x88 - EXTI pending register
    #[inline(always)]
    pub const fn cpupr1(&self) -> &CPUPR1 {
        &self.cpupr1
    }
    ///0x90 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn cpuimr2(&self) -> &CPUIMR2 {
        &self.cpuimr2
    }
    ///0x94 - EXTI event mask register
    #[inline(always)]
    pub const fn cpuemr2(&self) -> &CPUEMR2 {
        &self.cpuemr2
    }
    ///0x98 - EXTI pending register
    #[inline(always)]
    pub const fn cpupr2(&self) -> &CPUPR2 {
        &self.cpupr2
    }
    ///0xa0 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn cpuimr3(&self) -> &CPUIMR3 {
        &self.cpuimr3
    }
    ///0xa4 - EXTI event mask register
    #[inline(always)]
    pub const fn cpuemr3(&self) -> &CPUEMR3 {
        &self.cpuemr3
    }
    ///0xa8 - EXTI pending register
    #[inline(always)]
    pub const fn cpupr3(&self) -> &CPUPR3 {
        &self.cpupr3
    }
}
/**RTSR1 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:RTSR1)

For information about available fields see [`mod@rtsr1`] module*/
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
///EXTI rising trigger selection register
pub mod rtsr1;
/**FTSR1 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:FTSR1)

For information about available fields see [`mod@ftsr1`] module*/
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
///EXTI falling trigger selection register
pub mod ftsr1;
/**SWIER1 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:SWIER1)

For information about available fields see [`mod@swier1`] module*/
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
///EXTI software interrupt event register
pub mod swier1;
/**D3PMR1 (rw) register accessor: EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PMR1)

For information about available fields see [`mod@d3pmr1`] module*/
pub type D3PMR1 = crate::Reg<d3pmr1::D3PMR1rs>;
///EXTI D3 pending mask register
pub mod d3pmr1;
/**D3PCR1L (rw) register accessor: EXTI D3 pending clear selection register low

You can [`read`](crate::Reg::read) this register and get [`d3pcr1l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PCR1L)

For information about available fields see [`mod@d3pcr1l`] module*/
pub type D3PCR1L = crate::Reg<d3pcr1l::D3PCR1Lrs>;
///EXTI D3 pending clear selection register low
pub mod d3pcr1l;
/**D3PCR1H (rw) register accessor: EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr1h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PCR1H)

For information about available fields see [`mod@d3pcr1h`] module*/
pub type D3PCR1H = crate::Reg<d3pcr1h::D3PCR1Hrs>;
///EXTI D3 pending clear selection register high
pub mod d3pcr1h;
/**RTSR2 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:RTSR2)

For information about available fields see [`mod@rtsr2`] module*/
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
///EXTI rising trigger selection register
pub mod rtsr2;
/**FTSR2 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:FTSR2)

For information about available fields see [`mod@ftsr2`] module*/
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
///EXTI falling trigger selection register
pub mod ftsr2;
/**SWIER2 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:SWIER2)

For information about available fields see [`mod@swier2`] module*/
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
///EXTI software interrupt event register
pub mod swier2;
/**D3PMR2 (rw) register accessor: EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PMR2)

For information about available fields see [`mod@d3pmr2`] module*/
pub type D3PMR2 = crate::Reg<d3pmr2::D3PMR2rs>;
///EXTI D3 pending mask register
pub mod d3pmr2;
/**D3PCR2L (rw) register accessor: EXTI D3 pending clear selection register low

You can [`read`](crate::Reg::read) this register and get [`d3pcr2l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PCR2L)

For information about available fields see [`mod@d3pcr2l`] module*/
pub type D3PCR2L = crate::Reg<d3pcr2l::D3PCR2Lrs>;
///EXTI D3 pending clear selection register low
pub mod d3pcr2l;
/**D3PCR2H (rw) register accessor: EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr2h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PCR2H)

For information about available fields see [`mod@d3pcr2h`] module*/
pub type D3PCR2H = crate::Reg<d3pcr2h::D3PCR2Hrs>;
///EXTI D3 pending clear selection register high
pub mod d3pcr2h;
/**RTSR3 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:RTSR3)

For information about available fields see [`mod@rtsr3`] module*/
pub type RTSR3 = crate::Reg<rtsr3::RTSR3rs>;
///EXTI rising trigger selection register
pub mod rtsr3;
/**FTSR3 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:FTSR3)

For information about available fields see [`mod@ftsr3`] module*/
pub type FTSR3 = crate::Reg<ftsr3::FTSR3rs>;
///EXTI falling trigger selection register
pub mod ftsr3;
/**SWIER3 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:SWIER3)

For information about available fields see [`mod@swier3`] module*/
pub type SWIER3 = crate::Reg<swier3::SWIER3rs>;
///EXTI software interrupt event register
pub mod swier3;
/**D3PMR3 (rw) register accessor: EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PMR3)

For information about available fields see [`mod@d3pmr3`] module*/
pub type D3PMR3 = crate::Reg<d3pmr3::D3PMR3rs>;
///EXTI D3 pending mask register
pub mod d3pmr3;
/**D3PCR3H (rw) register accessor: EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr3h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr3h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:D3PCR3H)

For information about available fields see [`mod@d3pcr3h`] module*/
pub type D3PCR3H = crate::Reg<d3pcr3h::D3PCR3Hrs>;
///EXTI D3 pending clear selection register high
pub mod d3pcr3h;
/**CPUIMR1 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`cpuimr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuimr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUIMR1)

For information about available fields see [`mod@cpuimr1`] module*/
pub type CPUIMR1 = crate::Reg<cpuimr1::CPUIMR1rs>;
///EXTI interrupt mask register
pub mod cpuimr1;
/**CPUEMR1 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`cpuemr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUEMR1)

For information about available fields see [`mod@cpuemr1`] module*/
pub type CPUEMR1 = crate::Reg<cpuemr1::CPUEMR1rs>;
///EXTI event mask register
pub mod cpuemr1;
/**CPUPR1 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`cpupr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUPR1)

For information about available fields see [`mod@cpupr1`] module*/
pub type CPUPR1 = crate::Reg<cpupr1::CPUPR1rs>;
///EXTI pending register
pub mod cpupr1;
/**CPUIMR2 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`cpuimr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuimr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUIMR2)

For information about available fields see [`mod@cpuimr2`] module*/
pub type CPUIMR2 = crate::Reg<cpuimr2::CPUIMR2rs>;
///EXTI interrupt mask register
pub mod cpuimr2;
/**CPUEMR2 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`cpuemr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUEMR2)

For information about available fields see [`mod@cpuemr2`] module*/
pub type CPUEMR2 = crate::Reg<cpuemr2::CPUEMR2rs>;
///EXTI event mask register
pub mod cpuemr2;
/**CPUPR2 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`cpupr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUPR2)

For information about available fields see [`mod@cpupr2`] module*/
pub type CPUPR2 = crate::Reg<cpupr2::CPUPR2rs>;
///EXTI pending register
pub mod cpupr2;
/**CPUIMR3 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`cpuimr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuimr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUIMR3)

For information about available fields see [`mod@cpuimr3`] module*/
pub type CPUIMR3 = crate::Reg<cpuimr3::CPUIMR3rs>;
///EXTI interrupt mask register
pub mod cpuimr3;
/**CPUEMR3 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`cpuemr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUEMR3)

For information about available fields see [`mod@cpuemr3`] module*/
pub type CPUEMR3 = crate::Reg<cpuemr3::CPUEMR3rs>;
///EXTI event mask register
pub mod cpuemr3;
/**CPUPR3 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`cpupr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#EXTI:CPUPR3)

For information about available fields see [`mod@cpupr3`] module*/
pub type CPUPR3 = crate::Reg<cpupr3::CPUPR3rs>;
///EXTI pending register
pub mod cpupr3;
