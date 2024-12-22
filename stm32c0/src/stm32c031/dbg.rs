#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idcode: IDCODE,
    cr: CR,
    apb_fz1: APB_FZ1,
    apb_fz2: APB_FZ2,
}
impl RegisterBlock {
    ///0x00 - DBG device ID code register
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    ///0x04 - DBG configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - DBG APB freeze register 1
    #[inline(always)]
    pub const fn apb_fz1(&self) -> &APB_FZ1 {
        &self.apb_fz1
    }
    ///0x0c - DBG APB freeze register 2
    #[inline(always)]
    pub const fn apb_fz2(&self) -> &APB_FZ2 {
        &self.apb_fz2
    }
}
/**IDCODE (r) register accessor: DBG device ID code register

You can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#DBG:IDCODE)

For information about available fields see [`mod@idcode`]
module*/
pub type IDCODE = crate::Reg<idcode::IDCODErs>;
///DBG device ID code register
pub mod idcode;
/**CR (rw) register accessor: DBG configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#DBG:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///DBG configuration register
pub mod cr;
/**APB_FZ1 (rw) register accessor: DBG APB freeze register 1

You can [`read`](crate::Reg::read) this register and get [`apb_fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#DBG:APB_FZ1)

For information about available fields see [`mod@apb_fz1`]
module*/
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1rs>;
///DBG APB freeze register 1
pub mod apb_fz1;
/**APB_FZ2 (rw) register accessor: DBG APB freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb_fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#DBG:APB_FZ2)

For information about available fields see [`mod@apb_fz2`]
module*/
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2rs>;
///DBG APB freeze register 2
pub mod apb_fz2;
