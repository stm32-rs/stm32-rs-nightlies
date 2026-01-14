#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    die_id: DIE_ID,
    jtag_id: JTAG_ID,
    i2c_fmp_ctrl: I2C_FMP_CTRL,
    io_dtr: IO_DTR,
    io_iber: IO_IBER,
    io_ievr: IO_IEVR,
    io_ier: IO_IER,
    io_iscr: IO_ISCR,
    pwrc_ier: PWRC_IER,
    pwrc_iscr: PWRC_ISCR,
    _reserved10: [u8; 0x04],
    blerxtx_dtr: BLERXTX_DTR,
    blerxtx_iber: BLERXTX_IBER,
    blerxtx_ievr: BLERXTX_IEVR,
    blerxtx_ier: BLERXTX_IER,
    blerxtx_iscr: BLERXTX_ISCR,
}
impl RegisterBlock {
    ///0x00 - DIE_ID register
    #[inline(always)]
    pub const fn die_id(&self) -> &DIE_ID {
        &self.die_id
    }
    ///0x04 - JTAG_ID register
    #[inline(always)]
    pub const fn jtag_id(&self) -> &JTAG_ID {
        &self.jtag_id
    }
    ///0x08 - I2C_FMP_CTRL register
    #[inline(always)]
    pub const fn i2c_fmp_ctrl(&self) -> &I2C_FMP_CTRL {
        &self.i2c_fmp_ctrl
    }
    ///0x0c - IO_DTR register
    #[inline(always)]
    pub const fn io_dtr(&self) -> &IO_DTR {
        &self.io_dtr
    }
    ///0x10 - IO_IBER register
    #[inline(always)]
    pub const fn io_iber(&self) -> &IO_IBER {
        &self.io_iber
    }
    ///0x14 - IO_IEVR register
    #[inline(always)]
    pub const fn io_ievr(&self) -> &IO_IEVR {
        &self.io_ievr
    }
    ///0x18 - IO_IER register
    #[inline(always)]
    pub const fn io_ier(&self) -> &IO_IER {
        &self.io_ier
    }
    ///0x1c - IO_ISCR register
    #[inline(always)]
    pub const fn io_iscr(&self) -> &IO_ISCR {
        &self.io_iscr
    }
    ///0x20 - PWRC_IER register
    #[inline(always)]
    pub const fn pwrc_ier(&self) -> &PWRC_IER {
        &self.pwrc_ier
    }
    ///0x24 - PWRC_ISCR register
    #[inline(always)]
    pub const fn pwrc_iscr(&self) -> &PWRC_ISCR {
        &self.pwrc_iscr
    }
    ///0x2c - BLERXTX_DTR register
    #[inline(always)]
    pub const fn blerxtx_dtr(&self) -> &BLERXTX_DTR {
        &self.blerxtx_dtr
    }
    ///0x30 - BLERXTX_IBER register
    #[inline(always)]
    pub const fn blerxtx_iber(&self) -> &BLERXTX_IBER {
        &self.blerxtx_iber
    }
    ///0x34 - BLERXTX_IEVR register
    #[inline(always)]
    pub const fn blerxtx_ievr(&self) -> &BLERXTX_IEVR {
        &self.blerxtx_ievr
    }
    ///0x38 - BLERXTX_IER register
    #[inline(always)]
    pub const fn blerxtx_ier(&self) -> &BLERXTX_IER {
        &self.blerxtx_ier
    }
    ///0x3c - BLERXTX_ISCR register
    #[inline(always)]
    pub const fn blerxtx_iscr(&self) -> &BLERXTX_ISCR {
        &self.blerxtx_iscr
    }
}
/**DIE_ID (r) register accessor: DIE_ID register

You can [`read`](crate::Reg::read) this register and get [`die_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:DIE_ID)

For information about available fields see [`mod@die_id`] module*/
pub type DIE_ID = crate::Reg<die_id::DIE_IDrs>;
///DIE_ID register
pub mod die_id;
/**JTAG_ID (r) register accessor: JTAG_ID register

You can [`read`](crate::Reg::read) this register and get [`jtag_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:JTAG_ID)

For information about available fields see [`mod@jtag_id`] module*/
pub type JTAG_ID = crate::Reg<jtag_id::JTAG_IDrs>;
///JTAG_ID register
pub mod jtag_id;
/**I2C_FMP_CTRL (rw) register accessor: I2C_FMP_CTRL register

You can [`read`](crate::Reg::read) this register and get [`i2c_fmp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fmp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:I2C_FMP_CTRL)

For information about available fields see [`mod@i2c_fmp_ctrl`] module*/
pub type I2C_FMP_CTRL = crate::Reg<i2c_fmp_ctrl::I2C_FMP_CTRLrs>;
///I2C_FMP_CTRL register
pub mod i2c_fmp_ctrl;
/**IO_DTR (rw) register accessor: IO_DTR register

You can [`read`](crate::Reg::read) this register and get [`io_dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:IO_DTR)

For information about available fields see [`mod@io_dtr`] module*/
pub type IO_DTR = crate::Reg<io_dtr::IO_DTRrs>;
///IO_DTR register
pub mod io_dtr;
/**IO_IBER (rw) register accessor: IO_IBER register

You can [`read`](crate::Reg::read) this register and get [`io_iber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_iber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:IO_IBER)

For information about available fields see [`mod@io_iber`] module*/
pub type IO_IBER = crate::Reg<io_iber::IO_IBERrs>;
///IO_IBER register
pub mod io_iber;
/**IO_IEVR (rw) register accessor: IO_IEVR register

You can [`read`](crate::Reg::read) this register and get [`io_ievr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ievr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:IO_IEVR)

For information about available fields see [`mod@io_ievr`] module*/
pub type IO_IEVR = crate::Reg<io_ievr::IO_IEVRrs>;
///IO_IEVR register
pub mod io_ievr;
/**IO_IER (rw) register accessor: IO_IER register

You can [`read`](crate::Reg::read) this register and get [`io_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:IO_IER)

For information about available fields see [`mod@io_ier`] module*/
pub type IO_IER = crate::Reg<io_ier::IO_IERrs>;
///IO_IER register
pub mod io_ier;
/**IO_ISCR (rw) register accessor: IO_ISCR register

You can [`read`](crate::Reg::read) this register and get [`io_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:IO_ISCR)

For information about available fields see [`mod@io_iscr`] module*/
pub type IO_ISCR = crate::Reg<io_iscr::IO_ISCRrs>;
///IO_ISCR register
pub mod io_iscr;
/**PWRC_IER (rw) register accessor: PWRC_IER register

You can [`read`](crate::Reg::read) this register and get [`pwrc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:PWRC_IER)

For information about available fields see [`mod@pwrc_ier`] module*/
pub type PWRC_IER = crate::Reg<pwrc_ier::PWRC_IERrs>;
///PWRC_IER register
pub mod pwrc_ier;
/**PWRC_ISCR (rw) register accessor: PWRC_ISCR register

You can [`read`](crate::Reg::read) this register and get [`pwrc_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:PWRC_ISCR)

For information about available fields see [`mod@pwrc_iscr`] module*/
pub type PWRC_ISCR = crate::Reg<pwrc_iscr::PWRC_ISCRrs>;
///PWRC_ISCR register
pub mod pwrc_iscr;
/**BLERXTX_DTR (rw) register accessor: BLERXTX_DTR register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_DTR)

For information about available fields see [`mod@blerxtx_dtr`] module*/
pub type BLERXTX_DTR = crate::Reg<blerxtx_dtr::BLERXTX_DTRrs>;
///BLERXTX_DTR register
pub mod blerxtx_dtr;
/**BLERXTX_IBER (rw) register accessor: BLERXTX_IBER register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_iber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_iber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_IBER)

For information about available fields see [`mod@blerxtx_iber`] module*/
pub type BLERXTX_IBER = crate::Reg<blerxtx_iber::BLERXTX_IBERrs>;
///BLERXTX_IBER register
pub mod blerxtx_iber;
/**BLERXTX_IEVR (rw) register accessor: BLERXTX_IEVR register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_ievr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_ievr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_IEVR)

For information about available fields see [`mod@blerxtx_ievr`] module*/
pub type BLERXTX_IEVR = crate::Reg<blerxtx_ievr::BLERXTX_IEVRrs>;
///BLERXTX_IEVR register
pub mod blerxtx_ievr;
/**BLERXTX_IER (rw) register accessor: BLERXTX_IER register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_IER)

For information about available fields see [`mod@blerxtx_ier`] module*/
pub type BLERXTX_IER = crate::Reg<blerxtx_ier::BLERXTX_IERrs>;
///BLERXTX_IER register
pub mod blerxtx_ier;
/**BLERXTX_ISCR (rw) register accessor: BLERXTX_ISCR register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#SYSTEM_CTRL:BLERXTX_ISCR)

For information about available fields see [`mod@blerxtx_iscr`] module*/
pub type BLERXTX_ISCR = crate::Reg<blerxtx_iscr::BLERXTX_ISCRrs>;
///BLERXTX_ISCR register
pub mod blerxtx_iscr;
