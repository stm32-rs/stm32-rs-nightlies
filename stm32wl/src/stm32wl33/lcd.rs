#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    fcr: FCR,
    sr: SR,
    clr: CLR,
    _reserved4: [u8; 0x04],
    ram_com0: RAM_COM0,
    _reserved5: [u8; 0x04],
    ram_com1: RAM_COM1,
    _reserved6: [u8; 0x04],
    ram_com2: RAM_COM2,
    _reserved7: [u8; 0x04],
    ram_com3: RAM_COM3,
    _reserved8: [u8; 0x04],
    ram_com4: RAM_COM4,
    _reserved9: [u8; 0x04],
    ram_com5: RAM_COM5,
    _reserved10: [u8; 0x04],
    ram_com6: RAM_COM6,
    _reserved11: [u8; 0x04],
    ram_com7: RAM_COM7,
}
impl RegisterBlock {
    ///0x00 - LCD_CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - LCD_FCR register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x08 - LCD_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - LCD_CLR register
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    ///0x14 - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com0(&self) -> &RAM_COM0 {
        &self.ram_com0
    }
    ///0x1c - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com1(&self) -> &RAM_COM1 {
        &self.ram_com1
    }
    ///0x24 - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com2(&self) -> &RAM_COM2 {
        &self.ram_com2
    }
    ///0x2c - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com3(&self) -> &RAM_COM3 {
        &self.ram_com3
    }
    ///0x34 - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com4(&self) -> &RAM_COM4 {
        &self.ram_com4
    }
    ///0x3c - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com5(&self) -> &RAM_COM5 {
        &self.ram_com5
    }
    ///0x44 - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com6(&self) -> &RAM_COM6 {
        &self.ram_com6
    }
    ///0x4c - LCD_RAM_COMx register
    #[inline(always)]
    pub const fn ram_com7(&self) -> &RAM_COM7 {
        &self.ram_com7
    }
}
/**CR (rw) register accessor: LCD_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///LCD_CR register
pub mod cr;
/**FCR (rw) register accessor: LCD_FCR register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///LCD_FCR register
pub mod fcr;
/**SR (r) register accessor: LCD_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///LCD_SR register
pub mod sr;
/**CLR (rw) register accessor: LCD_CLR register

You can [`read`](crate::Reg::read) this register and get [`clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:CLR)

For information about available fields see [`mod@clr`] module*/
pub type CLR = crate::Reg<clr::CLRrs>;
///LCD_CLR register
pub mod clr;
/**RAM_COM0 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM0)

For information about available fields see [`mod@ram_com0`] module*/
pub type RAM_COM0 = crate::Reg<ram_com0::RAM_COM0rs>;
///LCD_RAM_COMx register
pub mod ram_com0;
/**RAM_COM1 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM1)

For information about available fields see [`mod@ram_com1`] module*/
pub type RAM_COM1 = crate::Reg<ram_com1::RAM_COM1rs>;
///LCD_RAM_COMx register
pub mod ram_com1;
/**RAM_COM2 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM2)

For information about available fields see [`mod@ram_com2`] module*/
pub type RAM_COM2 = crate::Reg<ram_com2::RAM_COM2rs>;
///LCD_RAM_COMx register
pub mod ram_com2;
/**RAM_COM3 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM3)

For information about available fields see [`mod@ram_com3`] module*/
pub type RAM_COM3 = crate::Reg<ram_com3::RAM_COM3rs>;
///LCD_RAM_COMx register
pub mod ram_com3;
/**RAM_COM4 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM4)

For information about available fields see [`mod@ram_com4`] module*/
pub type RAM_COM4 = crate::Reg<ram_com4::RAM_COM4rs>;
///LCD_RAM_COMx register
pub mod ram_com4;
/**RAM_COM5 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM5)

For information about available fields see [`mod@ram_com5`] module*/
pub type RAM_COM5 = crate::Reg<ram_com5::RAM_COM5rs>;
///LCD_RAM_COMx register
pub mod ram_com5;
/**RAM_COM6 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM6)

For information about available fields see [`mod@ram_com6`] module*/
pub type RAM_COM6 = crate::Reg<ram_com6::RAM_COM6rs>;
///LCD_RAM_COMx register
pub mod ram_com6;
/**RAM_COM7 (rw) register accessor: LCD_RAM_COMx register

You can [`read`](crate::Reg::read) this register and get [`ram_com7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_com7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:RAM_COM7)

For information about available fields see [`mod@ram_com7`] module*/
pub type RAM_COM7 = crate::Reg<ram_com7::RAM_COM7rs>;
///LCD_RAM_COMx register
pub mod ram_com7;
