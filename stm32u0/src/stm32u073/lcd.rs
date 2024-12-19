#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    lcd_cr: LCD_CR,
    lcd_fcr: LCD_FCR,
    lcd_sr: LCD_SR,
    lcd_clr: LCD_CLR,
    _reserved4: [u8; 0x04],
    lcd_ram0: LCD_RAM0,
    lcd_ram1: LCD_RAM1,
    lcd_ram2: LCD_RAM2,
    lcd_ram3: LCD_RAM3,
    lcd_ram4: LCD_RAM4,
    lcd_ram5: LCD_RAM5,
    lcd_ram6: LCD_RAM6,
    lcd_ram7: LCD_RAM7,
    lcd_ram8: LCD_RAM8,
    lcd_ram9: LCD_RAM9,
    lcd_ram10: LCD_RAM10,
    lcd_ram11: LCD_RAM11,
    lcd_ram12: LCD_RAM12,
    lcd_ram13: LCD_RAM13,
    lcd_ram14: LCD_RAM14,
    lcd_ram15: LCD_RAM15,
}
impl RegisterBlock {
    ///0x00 - LCD control register
    #[inline(always)]
    pub const fn lcd_cr(&self) -> &LCD_CR {
        &self.lcd_cr
    }
    ///0x04 - LCD frame control register
    #[inline(always)]
    pub const fn lcd_fcr(&self) -> &LCD_FCR {
        &self.lcd_fcr
    }
    ///0x08 - LCD status register
    #[inline(always)]
    pub const fn lcd_sr(&self) -> &LCD_SR {
        &self.lcd_sr
    }
    ///0x0c - LCD clear register
    #[inline(always)]
    pub const fn lcd_clr(&self) -> &LCD_CLR {
        &self.lcd_clr
    }
    ///0x14 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram0(&self) -> &LCD_RAM0 {
        &self.lcd_ram0
    }
    ///0x18 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram1(&self) -> &LCD_RAM1 {
        &self.lcd_ram1
    }
    ///0x1c - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram2(&self) -> &LCD_RAM2 {
        &self.lcd_ram2
    }
    ///0x20 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram3(&self) -> &LCD_RAM3 {
        &self.lcd_ram3
    }
    ///0x24 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram4(&self) -> &LCD_RAM4 {
        &self.lcd_ram4
    }
    ///0x28 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram5(&self) -> &LCD_RAM5 {
        &self.lcd_ram5
    }
    ///0x2c - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram6(&self) -> &LCD_RAM6 {
        &self.lcd_ram6
    }
    ///0x30 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram7(&self) -> &LCD_RAM7 {
        &self.lcd_ram7
    }
    ///0x34 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram8(&self) -> &LCD_RAM8 {
        &self.lcd_ram8
    }
    ///0x38 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram9(&self) -> &LCD_RAM9 {
        &self.lcd_ram9
    }
    ///0x3c - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram10(&self) -> &LCD_RAM10 {
        &self.lcd_ram10
    }
    ///0x40 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram11(&self) -> &LCD_RAM11 {
        &self.lcd_ram11
    }
    ///0x44 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram12(&self) -> &LCD_RAM12 {
        &self.lcd_ram12
    }
    ///0x48 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram13(&self) -> &LCD_RAM13 {
        &self.lcd_ram13
    }
    ///0x4c - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram14(&self) -> &LCD_RAM14 {
        &self.lcd_ram14
    }
    ///0x50 - LCD display memory
    #[inline(always)]
    pub const fn lcd_ram15(&self) -> &LCD_RAM15 {
        &self.lcd_ram15
    }
}
/**LCD_CR (rw) register accessor: LCD control register

You can [`read`](crate::Reg::read) this register and get [`lcd_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_CR)

For information about available fields see [`mod@lcd_cr`]
module*/
pub type LCD_CR = crate::Reg<lcd_cr::LCD_CRrs>;
///LCD control register
pub mod lcd_cr;
/**LCD_FCR (rw) register accessor: LCD frame control register

You can [`read`](crate::Reg::read) this register and get [`lcd_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_FCR)

For information about available fields see [`mod@lcd_fcr`]
module*/
pub type LCD_FCR = crate::Reg<lcd_fcr::LCD_FCRrs>;
///LCD frame control register
pub mod lcd_fcr;
/**LCD_SR (rw) register accessor: LCD status register

You can [`read`](crate::Reg::read) this register and get [`lcd_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_SR)

For information about available fields see [`mod@lcd_sr`]
module*/
pub type LCD_SR = crate::Reg<lcd_sr::LCD_SRrs>;
///LCD status register
pub mod lcd_sr;
/**LCD_CLR (w) register accessor: LCD clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_CLR)

For information about available fields see [`mod@lcd_clr`]
module*/
pub type LCD_CLR = crate::Reg<lcd_clr::LCD_CLRrs>;
///LCD clear register
pub mod lcd_clr;
/**LCD_RAM0 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM0)

For information about available fields see [`mod@lcd_ram0`]
module*/
pub type LCD_RAM0 = crate::Reg<lcd_ram0::LCD_RAM0rs>;
///LCD display memory
pub mod lcd_ram0;
/**LCD_RAM1 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM1)

For information about available fields see [`mod@lcd_ram1`]
module*/
pub type LCD_RAM1 = crate::Reg<lcd_ram1::LCD_RAM1rs>;
///LCD display memory
pub mod lcd_ram1;
/**LCD_RAM2 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM2)

For information about available fields see [`mod@lcd_ram2`]
module*/
pub type LCD_RAM2 = crate::Reg<lcd_ram2::LCD_RAM2rs>;
///LCD display memory
pub mod lcd_ram2;
/**LCD_RAM3 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM3)

For information about available fields see [`mod@lcd_ram3`]
module*/
pub type LCD_RAM3 = crate::Reg<lcd_ram3::LCD_RAM3rs>;
///LCD display memory
pub mod lcd_ram3;
/**LCD_RAM4 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM4)

For information about available fields see [`mod@lcd_ram4`]
module*/
pub type LCD_RAM4 = crate::Reg<lcd_ram4::LCD_RAM4rs>;
///LCD display memory
pub mod lcd_ram4;
/**LCD_RAM5 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM5)

For information about available fields see [`mod@lcd_ram5`]
module*/
pub type LCD_RAM5 = crate::Reg<lcd_ram5::LCD_RAM5rs>;
///LCD display memory
pub mod lcd_ram5;
/**LCD_RAM6 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM6)

For information about available fields see [`mod@lcd_ram6`]
module*/
pub type LCD_RAM6 = crate::Reg<lcd_ram6::LCD_RAM6rs>;
///LCD display memory
pub mod lcd_ram6;
/**LCD_RAM7 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM7)

For information about available fields see [`mod@lcd_ram7`]
module*/
pub type LCD_RAM7 = crate::Reg<lcd_ram7::LCD_RAM7rs>;
///LCD display memory
pub mod lcd_ram7;
/**LCD_RAM8 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM8)

For information about available fields see [`mod@lcd_ram8`]
module*/
pub type LCD_RAM8 = crate::Reg<lcd_ram8::LCD_RAM8rs>;
///LCD display memory
pub mod lcd_ram8;
/**LCD_RAM9 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM9)

For information about available fields see [`mod@lcd_ram9`]
module*/
pub type LCD_RAM9 = crate::Reg<lcd_ram9::LCD_RAM9rs>;
///LCD display memory
pub mod lcd_ram9;
/**LCD_RAM10 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM10)

For information about available fields see [`mod@lcd_ram10`]
module*/
pub type LCD_RAM10 = crate::Reg<lcd_ram10::LCD_RAM10rs>;
///LCD display memory
pub mod lcd_ram10;
/**LCD_RAM11 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM11)

For information about available fields see [`mod@lcd_ram11`]
module*/
pub type LCD_RAM11 = crate::Reg<lcd_ram11::LCD_RAM11rs>;
///LCD display memory
pub mod lcd_ram11;
/**LCD_RAM12 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM12)

For information about available fields see [`mod@lcd_ram12`]
module*/
pub type LCD_RAM12 = crate::Reg<lcd_ram12::LCD_RAM12rs>;
///LCD display memory
pub mod lcd_ram12;
/**LCD_RAM13 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM13)

For information about available fields see [`mod@lcd_ram13`]
module*/
pub type LCD_RAM13 = crate::Reg<lcd_ram13::LCD_RAM13rs>;
///LCD display memory
pub mod lcd_ram13;
/**LCD_RAM14 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM14)

For information about available fields see [`mod@lcd_ram14`]
module*/
pub type LCD_RAM14 = crate::Reg<lcd_ram14::LCD_RAM14rs>;
///LCD display memory
pub mod lcd_ram14;
/**LCD_RAM15 (rw) register accessor: LCD display memory

You can [`read`](crate::Reg::read) this register and get [`lcd_ram15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LCD:LCD_RAM15)

For information about available fields see [`mod@lcd_ram15`]
module*/
pub type LCD_RAM15 = crate::Reg<lcd_ram15::LCD_RAM15rs>;
///LCD display memory
pub mod lcd_ram15;
