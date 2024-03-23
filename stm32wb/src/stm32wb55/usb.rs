#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ep0r: EP0R,
    _reserved1: [u8; 0x02],
    ep1r: EP1R,
    _reserved2: [u8; 0x02],
    ep2r: EP2R,
    _reserved3: [u8; 0x02],
    ep3r: EP3R,
    _reserved4: [u8; 0x02],
    ep4r: EP4R,
    _reserved5: [u8; 0x02],
    ep5r: EP5R,
    _reserved6: [u8; 0x02],
    ep6r: EP6R,
    _reserved7: [u8; 0x02],
    ep7r: EP7R,
    _reserved8: [u8; 0x22],
    cntr: CNTR,
    _reserved9: [u8; 0x02],
    istr: ISTR,
    _reserved10: [u8; 0x02],
    fnr: FNR,
    _reserved11: [u8; 0x02],
    daddr: DADDR,
    _reserved12: [u8; 0x02],
    btable: BTABLE,
    count0_tx: COUNT0_TX,
    _reserved_14_lpmcsr: [u8; 0x02],
    count0_rx: COUNT0_RX,
    bcdr: BCDR,
    count1_tx: COUNT1_TX,
    addr1_rx: ADDR1_RX,
    count1_rx: COUNT1_RX,
    _reserved20: [u8; 0x02],
    count2_tx: COUNT2_TX,
    addr2_rx: ADDR2_RX,
    count2_rx: COUNT2_RX,
    _reserved23: [u8; 0x02],
    count3_tx: COUNT3_TX,
    addr3_rx: ADDR3_RX,
    count3_rx: COUNT3_RX,
    _reserved26: [u8; 0x02],
    count4_tx: COUNT4_TX,
    addr4_rx: ADDR4_RX,
    count4_rx: COUNT4_RX,
    _reserved29: [u8; 0x02],
    count5_tx: COUNT5_TX,
    addr5_rx: ADDR5_RX,
    count5_rx: COUNT5_RX,
    _reserved32: [u8; 0x02],
    count6_tx: COUNT6_TX,
    addr6_rx: ADDR6_RX,
    count6_rx: COUNT6_RX,
    _reserved35: [u8; 0x02],
    count7_tx: COUNT7_TX,
    addr7_rx: ADDR7_RX,
    count7_rx: COUNT7_RX,
}
impl RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn ep0r(&self) -> &EP0R {
        &self.ep0r
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn ep1r(&self) -> &EP1R {
        &self.ep1r
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn ep2r(&self) -> &EP2R {
        &self.ep2r
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn ep3r(&self) -> &EP3R {
        &self.ep3r
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn ep4r(&self) -> &EP4R {
        &self.ep4r
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn ep5r(&self) -> &EP5R {
        &self.ep5r
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn ep6r(&self) -> &EP6R {
        &self.ep6r
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn ep7r(&self) -> &EP7R {
        &self.ep7r
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
    #[doc = "0x52 - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count0_tx(&self) -> &COUNT0_TX {
        &self.count0_tx
    }
    #[doc = "0x54 - control and status register"]
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &LPMCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr0_rx(&self) -> &ADDR0_RX {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x56 - Reception byte count 0"]
    #[inline(always)]
    pub const fn count0_rx(&self) -> &COUNT0_RX {
        &self.count0_rx
    }
    #[doc = "0x58 - Battery charging detector("]
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
    #[doc = "0x5a - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count1_tx(&self) -> &COUNT1_TX {
        &self.count1_tx
    }
    #[doc = "0x5c - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr1_rx(&self) -> &ADDR1_RX {
        &self.addr1_rx
    }
    #[doc = "0x5e - Reception byte count 0"]
    #[inline(always)]
    pub const fn count1_rx(&self) -> &COUNT1_RX {
        &self.count1_rx
    }
    #[doc = "0x62 - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count2_tx(&self) -> &COUNT2_TX {
        &self.count2_tx
    }
    #[doc = "0x64 - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr2_rx(&self) -> &ADDR2_RX {
        &self.addr2_rx
    }
    #[doc = "0x66 - Reception byte count 0"]
    #[inline(always)]
    pub const fn count2_rx(&self) -> &COUNT2_RX {
        &self.count2_rx
    }
    #[doc = "0x6a - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count3_tx(&self) -> &COUNT3_TX {
        &self.count3_tx
    }
    #[doc = "0x6c - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr3_rx(&self) -> &ADDR3_RX {
        &self.addr3_rx
    }
    #[doc = "0x6e - Reception byte count 0"]
    #[inline(always)]
    pub const fn count3_rx(&self) -> &COUNT3_RX {
        &self.count3_rx
    }
    #[doc = "0x72 - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count4_tx(&self) -> &COUNT4_TX {
        &self.count4_tx
    }
    #[doc = "0x74 - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr4_rx(&self) -> &ADDR4_RX {
        &self.addr4_rx
    }
    #[doc = "0x76 - Reception byte count 0"]
    #[inline(always)]
    pub const fn count4_rx(&self) -> &COUNT4_RX {
        &self.count4_rx
    }
    #[doc = "0x7a - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count5_tx(&self) -> &COUNT5_TX {
        &self.count5_tx
    }
    #[doc = "0x7c - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr5_rx(&self) -> &ADDR5_RX {
        &self.addr5_rx
    }
    #[doc = "0x7e - Reception byte count 0"]
    #[inline(always)]
    pub const fn count5_rx(&self) -> &COUNT5_RX {
        &self.count5_rx
    }
    #[doc = "0x82 - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count6_tx(&self) -> &COUNT6_TX {
        &self.count6_tx
    }
    #[doc = "0x84 - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr6_rx(&self) -> &ADDR6_RX {
        &self.addr6_rx
    }
    #[doc = "0x86 - Reception byte count 0"]
    #[inline(always)]
    pub const fn count6_rx(&self) -> &COUNT6_RX {
        &self.count6_rx
    }
    #[doc = "0x8a - Transmission byte count 0"]
    #[inline(always)]
    pub const fn count7_tx(&self) -> &COUNT7_TX {
        &self.count7_tx
    }
    #[doc = "0x8c - Reception buffer address 0"]
    #[inline(always)]
    pub const fn addr7_rx(&self) -> &ADDR7_RX {
        &self.addr7_rx
    }
    #[doc = "0x8e - Reception byte count 0"]
    #[inline(always)]
    pub const fn count7_rx(&self) -> &COUNT7_RX {
        &self.count7_rx
    }
}
#[doc = "EP0R (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0r`]
module"]
pub type EP0R = crate::Reg<ep0r::EP0Rrs>;
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "EP1R (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1r`]
module"]
pub type EP1R = crate::Reg<ep1r::EP1Rrs>;
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "EP2R (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2r`]
module"]
pub type EP2R = crate::Reg<ep2r::EP2Rrs>;
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "EP3R (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3r`]
module"]
pub type EP3R = crate::Reg<ep3r::EP3Rrs>;
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "EP4R (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4r`]
module"]
pub type EP4R = crate::Reg<ep4r::EP4Rrs>;
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "EP5R (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5r`]
module"]
pub type EP5R = crate::Reg<ep5r::EP5Rrs>;
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "EP6R (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6r`]
module"]
pub type EP6R = crate::Reg<ep6r::EP6Rrs>;
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "EP7R (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7r`]
module"]
pub type EP7R = crate::Reg<ep7r::EP7Rrs>;
#[doc = "endpoint 7 register"]
pub mod ep7r;
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
#[doc = "COUNT0_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count0_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count0_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count0_tx`]
module"]
pub type COUNT0_TX = crate::Reg<count0_tx::COUNT0_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count0_tx;
#[doc = "COUNT1_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count1_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count1_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count1_tx`]
module"]
pub type COUNT1_TX = crate::Reg<count1_tx::COUNT1_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count1_tx;
#[doc = "COUNT2_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count2_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count2_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count2_tx`]
module"]
pub type COUNT2_TX = crate::Reg<count2_tx::COUNT2_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count2_tx;
#[doc = "COUNT3_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count3_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count3_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count3_tx`]
module"]
pub type COUNT3_TX = crate::Reg<count3_tx::COUNT3_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count3_tx;
#[doc = "COUNT4_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count4_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count4_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count4_tx`]
module"]
pub type COUNT4_TX = crate::Reg<count4_tx::COUNT4_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count4_tx;
#[doc = "COUNT5_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count5_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count5_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count5_tx`]
module"]
pub type COUNT5_TX = crate::Reg<count5_tx::COUNT5_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count5_tx;
#[doc = "COUNT6_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count6_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count6_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count6_tx`]
module"]
pub type COUNT6_TX = crate::Reg<count6_tx::COUNT6_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count6_tx;
#[doc = "COUNT7_TX (rw) register accessor: Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count7_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count7_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count7_tx`]
module"]
pub type COUNT7_TX = crate::Reg<count7_tx::COUNT7_TXrs>;
#[doc = "Transmission byte count 0"]
pub mod count7_tx;
#[doc = "ADDR0_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr0_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr0_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0_rx`]
module"]
pub type ADDR0_RX = crate::Reg<addr0_rx::ADDR0_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr0_rx;
#[doc = "ADDR1_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr1_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr1_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1_rx`]
module"]
pub type ADDR1_RX = crate::Reg<addr1_rx::ADDR1_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr1_rx;
#[doc = "ADDR2_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr2_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2_rx`]
module"]
pub type ADDR2_RX = crate::Reg<addr2_rx::ADDR2_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr2_rx;
#[doc = "ADDR3_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr3_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr3_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr3_rx`]
module"]
pub type ADDR3_RX = crate::Reg<addr3_rx::ADDR3_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr3_rx;
#[doc = "ADDR4_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr4_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr4_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr4_rx`]
module"]
pub type ADDR4_RX = crate::Reg<addr4_rx::ADDR4_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr4_rx;
#[doc = "ADDR5_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr5_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr5_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr5_rx`]
module"]
pub type ADDR5_RX = crate::Reg<addr5_rx::ADDR5_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr5_rx;
#[doc = "ADDR6_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr6_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr6_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr6_rx`]
module"]
pub type ADDR6_RX = crate::Reg<addr6_rx::ADDR6_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr6_rx;
#[doc = "ADDR7_RX (rw) register accessor: Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr7_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr7_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr7_rx`]
module"]
pub type ADDR7_RX = crate::Reg<addr7_rx::ADDR7_RXrs>;
#[doc = "Reception buffer address 0"]
pub mod addr7_rx;
#[doc = "COUNT0_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count0_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count0_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count0_rx`]
module"]
pub type COUNT0_RX = crate::Reg<count0_rx::COUNT0_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count0_rx;
#[doc = "COUNT1_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count1_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count1_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count1_rx`]
module"]
pub type COUNT1_RX = crate::Reg<count1_rx::COUNT1_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count1_rx;
#[doc = "COUNT2_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count2_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count2_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count2_rx`]
module"]
pub type COUNT2_RX = crate::Reg<count2_rx::COUNT2_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count2_rx;
#[doc = "COUNT3_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count3_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count3_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count3_rx`]
module"]
pub type COUNT3_RX = crate::Reg<count3_rx::COUNT3_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count3_rx;
#[doc = "COUNT4_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count4_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count4_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count4_rx`]
module"]
pub type COUNT4_RX = crate::Reg<count4_rx::COUNT4_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count4_rx;
#[doc = "COUNT5_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count5_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count5_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count5_rx`]
module"]
pub type COUNT5_RX = crate::Reg<count5_rx::COUNT5_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count5_rx;
#[doc = "COUNT6_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count6_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count6_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count6_rx`]
module"]
pub type COUNT6_RX = crate::Reg<count6_rx::COUNT6_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count6_rx;
#[doc = "COUNT7_RX (rw) register accessor: Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count7_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count7_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count7_rx`]
module"]
pub type COUNT7_RX = crate::Reg<count7_rx::COUNT7_RXrs>;
#[doc = "Reception byte count 0"]
pub mod count7_rx;
#[doc = "LPMCSR (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcsr`]
module"]
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSRrs>;
#[doc = "control and status register"]
pub mod lpmcsr;
#[doc = "BCDR (rw) register accessor: Battery charging detector(\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdr`]
module"]
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
#[doc = "Battery charging detector("]
pub mod bcdr;
