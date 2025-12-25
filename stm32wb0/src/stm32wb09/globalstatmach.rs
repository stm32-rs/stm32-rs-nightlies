#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    word0: WORD0,
    word1: WORD1,
    word2: WORD2,
    word3: WORD3,
    word4: WORD4,
    word5: WORD5,
    word6: WORD6,
}
impl RegisterBlock {
    ///0x00 - WORD0 register
    #[inline(always)]
    pub const fn word0(&self) -> &WORD0 {
        &self.word0
    }
    ///0x04 - WORD1 register
    #[inline(always)]
    pub const fn word1(&self) -> &WORD1 {
        &self.word1
    }
    ///0x08 - WORD2 register
    #[inline(always)]
    pub const fn word2(&self) -> &WORD2 {
        &self.word2
    }
    ///0x0c - WORD3 register
    #[inline(always)]
    pub const fn word3(&self) -> &WORD3 {
        &self.word3
    }
    ///0x10 - WORD4 register
    #[inline(always)]
    pub const fn word4(&self) -> &WORD4 {
        &self.word4
    }
    ///0x14 - WORD5 register
    #[inline(always)]
    pub const fn word5(&self) -> &WORD5 {
        &self.word5
    }
    ///0x18 - WORD6 register
    #[inline(always)]
    pub const fn word6(&self) -> &WORD6 {
        &self.word6
    }
}
/**WORD0 (rw) register accessor: WORD0 register

You can [`read`](crate::Reg::read) this register and get [`word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD0)

For information about available fields see [`mod@word0`] module*/
pub type WORD0 = crate::Reg<word0::WORD0rs>;
///WORD0 register
pub mod word0;
/**WORD1 (rw) register accessor: WORD1 register

You can [`read`](crate::Reg::read) this register and get [`word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD1)

For information about available fields see [`mod@word1`] module*/
pub type WORD1 = crate::Reg<word1::WORD1rs>;
///WORD1 register
pub mod word1;
/**WORD2 (rw) register accessor: WORD2 register

You can [`read`](crate::Reg::read) this register and get [`word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD2)

For information about available fields see [`mod@word2`] module*/
pub type WORD2 = crate::Reg<word2::WORD2rs>;
///WORD2 register
pub mod word2;
/**WORD3 (rw) register accessor: WORD3 register

You can [`read`](crate::Reg::read) this register and get [`word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD3)

For information about available fields see [`mod@word3`] module*/
pub type WORD3 = crate::Reg<word3::WORD3rs>;
///WORD3 register
pub mod word3;
/**WORD4 (rw) register accessor: WORD4 register

You can [`read`](crate::Reg::read) this register and get [`word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD4)

For information about available fields see [`mod@word4`] module*/
pub type WORD4 = crate::Reg<word4::WORD4rs>;
///WORD4 register
pub mod word4;
/**WORD5 (rw) register accessor: WORD5 register

You can [`read`](crate::Reg::read) this register and get [`word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD5)

For information about available fields see [`mod@word5`] module*/
pub type WORD5 = crate::Reg<word5::WORD5rs>;
///WORD5 register
pub mod word5;
/**WORD6 (rw) register accessor: WORD6 register

You can [`read`](crate::Reg::read) this register and get [`word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD6)

For information about available fields see [`mod@word6`] module*/
pub type WORD6 = crate::Reg<word6::WORD6rs>;
///WORD6 register
pub mod word6;
