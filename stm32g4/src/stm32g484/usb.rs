#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ep0r: EP0R,
    ep1r: EP1R,
    ep2r: EP2R,
    ep3r: EP3R,
    ep4r: EP4R,
    ep5r: EP5R,
    ep6r: EP6R,
    ep7r: EP7R,
    _reserved8: [u8; 0x20],
    cntr: CNTR,
    istr: ISTR,
    fnr: FNR,
    daddr: DADDR,
    btable: BTABLE,
    _reserved13: [u8; 0x04],
    bcdr: BCDR,
}
impl RegisterBlock {
    ///0x00 - USB endpoint n register
    #[inline(always)]
    pub const fn ep0r(&self) -> &EP0R {
        &self.ep0r
    }
    ///0x04 - USB endpoint n register
    #[inline(always)]
    pub const fn ep1r(&self) -> &EP1R {
        &self.ep1r
    }
    ///0x08 - USB endpoint n register
    #[inline(always)]
    pub const fn ep2r(&self) -> &EP2R {
        &self.ep2r
    }
    ///0x0c - USB endpoint n register
    #[inline(always)]
    pub const fn ep3r(&self) -> &EP3R {
        &self.ep3r
    }
    ///0x10 - USB endpoint n register
    #[inline(always)]
    pub const fn ep4r(&self) -> &EP4R {
        &self.ep4r
    }
    ///0x14 - USB endpoint n register
    #[inline(always)]
    pub const fn ep5r(&self) -> &EP5R {
        &self.ep5r
    }
    ///0x18 - USB endpoint n register
    #[inline(always)]
    pub const fn ep6r(&self) -> &EP6R {
        &self.ep6r
    }
    ///0x1c - USB endpoint n register
    #[inline(always)]
    pub const fn ep7r(&self) -> &EP7R {
        &self.ep7r
    }
    ///0x40 - USB control register
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    ///0x44 - USB interrupt status register
    #[inline(always)]
    pub const fn istr(&self) -> &ISTR {
        &self.istr
    }
    ///0x48 - USB frame number register
    #[inline(always)]
    pub const fn fnr(&self) -> &FNR {
        &self.fnr
    }
    ///0x4c - USB device address
    #[inline(always)]
    pub const fn daddr(&self) -> &DADDR {
        &self.daddr
    }
    ///0x50 - Buffer table address
    #[inline(always)]
    pub const fn btable(&self) -> &BTABLE {
        &self.btable
    }
    ///0x58 - Battery Charging Detector
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
}
/**EP0R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP0R)

For information about available fields see [`mod@ep0r`] module*/
pub type EP0R = crate::Reg<ep0r::EP0Rrs>;
///USB endpoint n register
pub mod ep0r;
/**EP1R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP1R)

For information about available fields see [`mod@ep1r`] module*/
pub type EP1R = crate::Reg<ep1r::EP1Rrs>;
///USB endpoint n register
pub mod ep1r;
/**EP2R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP2R)

For information about available fields see [`mod@ep2r`] module*/
pub type EP2R = crate::Reg<ep2r::EP2Rrs>;
///USB endpoint n register
pub mod ep2r;
/**EP3R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP3R)

For information about available fields see [`mod@ep3r`] module*/
pub type EP3R = crate::Reg<ep3r::EP3Rrs>;
///USB endpoint n register
pub mod ep3r;
/**EP4R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP4R)

For information about available fields see [`mod@ep4r`] module*/
pub type EP4R = crate::Reg<ep4r::EP4Rrs>;
///USB endpoint n register
pub mod ep4r;
/**EP5R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP5R)

For information about available fields see [`mod@ep5r`] module*/
pub type EP5R = crate::Reg<ep5r::EP5Rrs>;
///USB endpoint n register
pub mod ep5r;
/**EP6R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP6R)

For information about available fields see [`mod@ep6r`] module*/
pub type EP6R = crate::Reg<ep6r::EP6Rrs>;
///USB endpoint n register
pub mod ep6r;
/**EP7R (rw) register accessor: USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`ep7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:EP7R)

For information about available fields see [`mod@ep7r`] module*/
pub type EP7R = crate::Reg<ep7r::EP7Rrs>;
///USB endpoint n register
pub mod ep7r;
/**CNTR (rw) register accessor: USB control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:CNTR)

For information about available fields see [`mod@cntr`] module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///USB control register
pub mod cntr;
/**ISTR (rw) register accessor: USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:ISTR)

For information about available fields see [`mod@istr`] module*/
pub type ISTR = crate::Reg<istr::ISTRrs>;
///USB interrupt status register
pub mod istr;
/**FNR (r) register accessor: USB frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:FNR)

For information about available fields see [`mod@fnr`] module*/
pub type FNR = crate::Reg<fnr::FNRrs>;
///USB frame number register
pub mod fnr;
/**DADDR (rw) register accessor: USB device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:DADDR)

For information about available fields see [`mod@daddr`] module*/
pub type DADDR = crate::Reg<daddr::DADDRrs>;
///USB device address
pub mod daddr;
/**BTABLE (rw) register accessor: Buffer table address

You can [`read`](crate::Reg::read) this register and get [`btable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:BTABLE)

For information about available fields see [`mod@btable`] module*/
pub type BTABLE = crate::Reg<btable::BTABLErs>;
///Buffer table address
pub mod btable;
/**BCDR (rw) register accessor: Battery Charging Detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:BCDR)

For information about available fields see [`mod@bcdr`] module*/
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
///Battery Charging Detector
pub mod bcdr;
