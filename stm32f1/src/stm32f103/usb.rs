#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    epr: [EPR; 8],
    _reserved1: [u8; 0x20],
    cntr: CNTR,
    istr: ISTR,
    fnr: FNR,
    daddr: DADDR,
    btable: BTABLE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - endpoint %s register"]
    #[inline(always)]
    pub const fn epr(&self, n: usize) -> &EPR {
        &self.epr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - endpoint %s register"]
    #[inline(always)]
    pub fn epr_iter(&self) -> impl Iterator<Item = &EPR> {
        self.epr.iter()
    }
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn ep0r(&self) -> &EPR {
        self.epr(0)
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn ep1r(&self) -> &EPR {
        self.epr(1)
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn ep2r(&self) -> &EPR {
        self.epr(2)
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn ep3r(&self) -> &EPR {
        self.epr(3)
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn ep4r(&self) -> &EPR {
        self.epr(4)
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn ep5r(&self) -> &EPR {
        self.epr(5)
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn ep6r(&self) -> &EPR {
        self.epr(6)
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn ep7r(&self) -> &EPR {
        self.epr(7)
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    #[doc = "0x44 - interrupt status register"]
    #[inline(always)]
    pub const fn istr(&self) -> &ISTR {
        &self.istr
    }
    #[doc = "0x48 - frame number register"]
    #[inline(always)]
    pub const fn fnr(&self) -> &FNR {
        &self.fnr
    }
    #[doc = "0x4c - device address"]
    #[inline(always)]
    pub const fn daddr(&self) -> &DADDR {
        &self.daddr
    }
    #[doc = "0x50 - Buffer table address"]
    #[inline(always)]
    pub const fn btable(&self) -> &BTABLE {
        &self.btable
    }
}
#[doc = "EPR (rw) register accessor: endpoint %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epr`]
module"]
pub type EPR = crate::Reg<epr::EPRrs>;
#[doc = "endpoint %s register"]
pub mod epr;
#[doc = "CNTR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
pub type CNTR = crate::Reg<cntr::CNTRrs>;
#[doc = "control register"]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`]
module"]
pub type ISTR = crate::Reg<istr::ISTRrs>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`]
module"]
pub type FNR = crate::Reg<fnr::FNRrs>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
pub type DADDR = crate::Reg<daddr::DADDRrs>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btable`]
module"]
pub type BTABLE = crate::Reg<btable::BTABLErs>;
#[doc = "Buffer table address"]
pub mod btable;
