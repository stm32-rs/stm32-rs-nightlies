#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0r: EP0R,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1r: EP1R,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2r: EP2R,
    _reserved3: [u8; 2usize],
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3r: EP3R,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4r: EP4R,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5r: EP5R,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6r: EP6R,
    _reserved7: [u8; 2usize],
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7r: EP7R,
    _reserved8: [u8; 34usize],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    _reserved9: [u8; 2usize],
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    _reserved10: [u8; 2usize],
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    _reserved11: [u8; 2usize],
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    _reserved12: [u8; 2usize],
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
    #[doc = "0x52 - Transmission byte count 0"]
    pub count0_tx: COUNT0_TX,
    _reserved_14_lpmcsr: [u8; 2usize],
    #[doc = "0x56 - Reception byte count 0"]
    pub count0_rx: COUNT0_RX,
    #[doc = "0x58 - Battery charging detector("]
    pub bcdr: BCDR,
    #[doc = "0x5a - Transmission byte count 0"]
    pub count1_tx: COUNT1_TX,
    #[doc = "0x5c - Reception buffer address 0"]
    pub addr1_rx: ADDR1_RX,
    #[doc = "0x5e - Reception byte count 0"]
    pub count1_rx: COUNT1_RX,
    _reserved20: [u8; 2usize],
    #[doc = "0x62 - Transmission byte count 0"]
    pub count2_tx: COUNT2_TX,
    #[doc = "0x64 - Reception buffer address 0"]
    pub addr2_rx: ADDR2_RX,
    #[doc = "0x66 - Reception byte count 0"]
    pub count2_rx: COUNT2_RX,
    _reserved23: [u8; 2usize],
    #[doc = "0x6a - Transmission byte count 0"]
    pub count3_tx: COUNT3_TX,
    #[doc = "0x6c - Reception buffer address 0"]
    pub addr3_rx: ADDR3_RX,
    #[doc = "0x6e - Reception byte count 0"]
    pub count3_rx: COUNT3_RX,
    _reserved26: [u8; 2usize],
    #[doc = "0x72 - Transmission byte count 0"]
    pub count4_tx: COUNT4_TX,
    #[doc = "0x74 - Reception buffer address 0"]
    pub addr4_rx: ADDR4_RX,
    #[doc = "0x76 - Reception byte count 0"]
    pub count4_rx: COUNT4_RX,
    _reserved29: [u8; 2usize],
    #[doc = "0x7a - Transmission byte count 0"]
    pub count5_tx: COUNT5_TX,
    #[doc = "0x7c - Reception buffer address 0"]
    pub addr5_rx: ADDR5_RX,
    #[doc = "0x7e - Reception byte count 0"]
    pub count5_rx: COUNT5_RX,
    _reserved32: [u8; 2usize],
    #[doc = "0x82 - Transmission byte count 0"]
    pub count6_tx: COUNT6_TX,
    #[doc = "0x84 - Reception buffer address 0"]
    pub addr6_rx: ADDR6_RX,
    #[doc = "0x86 - Reception byte count 0"]
    pub count6_rx: COUNT6_RX,
    _reserved35: [u8; 2usize],
    #[doc = "0x8a - Transmission byte count 0"]
    pub count7_tx: COUNT7_TX,
    #[doc = "0x8c - Reception buffer address 0"]
    pub addr7_rx: ADDR7_RX,
    #[doc = "0x8e - Reception byte count 0"]
    pub count7_rx: COUNT7_RX,
}
impl RegisterBlock {
    #[doc = "0x54 - control and status register"]
    #[inline(always)]
    pub fn lpmcsr(&self) -> &LPMCSR {
        unsafe { &*(((self as *const Self) as *const u8).add(84usize) as *const LPMCSR) }
    }
    #[doc = "0x54 - control and status register"]
    #[inline(always)]
    pub fn lpmcsr_mut(&self) -> &mut LPMCSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut LPMCSR) }
    }
    #[doc = "0x54 - Reception buffer address 0"]
    #[inline(always)]
    pub fn addr0_rx(&self) -> &ADDR0_RX {
        unsafe { &*(((self as *const Self) as *const u8).add(84usize) as *const ADDR0_RX) }
    }
    #[doc = "0x54 - Reception buffer address 0"]
    #[inline(always)]
    pub fn addr0_rx_mut(&self) -> &mut ADDR0_RX {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut ADDR0_RX) }
    }
}
#[doc = "endpoint 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0r](ep0r) module"]
pub type EP0R = crate::Reg<u16, _EP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0R;
#[doc = "`read()` method returns [ep0r::R](ep0r::R) reader structure"]
impl crate::Readable for EP0R {}
#[doc = "`write(|w| ..)` method takes [ep0r::W](ep0r::W) writer structure"]
impl crate::Writable for EP0R {}
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "endpoint 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1r](ep1r) module"]
pub type EP1R = crate::Reg<u16, _EP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1R;
#[doc = "`read()` method returns [ep1r::R](ep1r::R) reader structure"]
impl crate::Readable for EP1R {}
#[doc = "`write(|w| ..)` method takes [ep1r::W](ep1r::W) writer structure"]
impl crate::Writable for EP1R {}
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "endpoint 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2r](ep2r) module"]
pub type EP2R = crate::Reg<u16, _EP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2R;
#[doc = "`read()` method returns [ep2r::R](ep2r::R) reader structure"]
impl crate::Readable for EP2R {}
#[doc = "`write(|w| ..)` method takes [ep2r::W](ep2r::W) writer structure"]
impl crate::Writable for EP2R {}
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "endpoint 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3r](ep3r) module"]
pub type EP3R = crate::Reg<u16, _EP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3R;
#[doc = "`read()` method returns [ep3r::R](ep3r::R) reader structure"]
impl crate::Readable for EP3R {}
#[doc = "`write(|w| ..)` method takes [ep3r::W](ep3r::W) writer structure"]
impl crate::Writable for EP3R {}
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "endpoint 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4r](ep4r) module"]
pub type EP4R = crate::Reg<u16, _EP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4R;
#[doc = "`read()` method returns [ep4r::R](ep4r::R) reader structure"]
impl crate::Readable for EP4R {}
#[doc = "`write(|w| ..)` method takes [ep4r::W](ep4r::W) writer structure"]
impl crate::Writable for EP4R {}
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "endpoint 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5r](ep5r) module"]
pub type EP5R = crate::Reg<u16, _EP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5R;
#[doc = "`read()` method returns [ep5r::R](ep5r::R) reader structure"]
impl crate::Readable for EP5R {}
#[doc = "`write(|w| ..)` method takes [ep5r::W](ep5r::W) writer structure"]
impl crate::Writable for EP5R {}
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "endpoint 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6r](ep6r) module"]
pub type EP6R = crate::Reg<u16, _EP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6R;
#[doc = "`read()` method returns [ep6r::R](ep6r::R) reader structure"]
impl crate::Readable for EP6R {}
#[doc = "`write(|w| ..)` method takes [ep6r::W](ep6r::W) writer structure"]
impl crate::Writable for EP6R {}
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "endpoint 7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7r](ep7r) module"]
pub type EP7R = crate::Reg<u16, _EP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7R;
#[doc = "`read()` method returns [ep7r::R](ep7r::R) reader structure"]
impl crate::Readable for EP7R {}
#[doc = "`write(|w| ..)` method takes [ep7r::W](ep7r::W) writer structure"]
impl crate::Writable for EP7R {}
#[doc = "endpoint 7 register"]
pub mod ep7r;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u16, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "control register"]
pub mod cntr;
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](istr) module"]
pub type ISTR = crate::Reg<u16, _ISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTR;
#[doc = "`read()` method returns [istr::R](istr::R) reader structure"]
impl crate::Readable for ISTR {}
#[doc = "`write(|w| ..)` method takes [istr::W](istr::W) writer structure"]
impl crate::Writable for ISTR {}
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnr](fnr) module"]
pub type FNR = crate::Reg<u16, _FNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNR;
#[doc = "`read()` method returns [fnr::R](fnr::R) reader structure"]
impl crate::Readable for FNR {}
#[doc = "frame number register"]
pub mod fnr;
#[doc = "device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](daddr) module"]
pub type DADDR = crate::Reg<u16, _DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR;
#[doc = "`read()` method returns [daddr::R](daddr::R) reader structure"]
impl crate::Readable for DADDR {}
#[doc = "`write(|w| ..)` method takes [daddr::W](daddr::W) writer structure"]
impl crate::Writable for DADDR {}
#[doc = "device address"]
pub mod daddr;
#[doc = "Buffer table address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btable](btable) module"]
pub type BTABLE = crate::Reg<u16, _BTABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTABLE;
#[doc = "`read()` method returns [btable::R](btable::R) reader structure"]
impl crate::Readable for BTABLE {}
#[doc = "`write(|w| ..)` method takes [btable::W](btable::W) writer structure"]
impl crate::Writable for BTABLE {}
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count0_tx](count0_tx) module"]
pub type COUNT0_TX = crate::Reg<u16, _COUNT0_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT0_TX;
#[doc = "`read()` method returns [count0_tx::R](count0_tx::R) reader structure"]
impl crate::Readable for COUNT0_TX {}
#[doc = "`write(|w| ..)` method takes [count0_tx::W](count0_tx::W) writer structure"]
impl crate::Writable for COUNT0_TX {}
#[doc = "Transmission byte count 0"]
pub mod count0_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count1_tx](count1_tx) module"]
pub type COUNT1_TX = crate::Reg<u16, _COUNT1_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT1_TX;
#[doc = "`read()` method returns [count1_tx::R](count1_tx::R) reader structure"]
impl crate::Readable for COUNT1_TX {}
#[doc = "`write(|w| ..)` method takes [count1_tx::W](count1_tx::W) writer structure"]
impl crate::Writable for COUNT1_TX {}
#[doc = "Transmission byte count 0"]
pub mod count1_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count2_tx](count2_tx) module"]
pub type COUNT2_TX = crate::Reg<u16, _COUNT2_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT2_TX;
#[doc = "`read()` method returns [count2_tx::R](count2_tx::R) reader structure"]
impl crate::Readable for COUNT2_TX {}
#[doc = "`write(|w| ..)` method takes [count2_tx::W](count2_tx::W) writer structure"]
impl crate::Writable for COUNT2_TX {}
#[doc = "Transmission byte count 0"]
pub mod count2_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count3_tx](count3_tx) module"]
pub type COUNT3_TX = crate::Reg<u16, _COUNT3_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT3_TX;
#[doc = "`read()` method returns [count3_tx::R](count3_tx::R) reader structure"]
impl crate::Readable for COUNT3_TX {}
#[doc = "`write(|w| ..)` method takes [count3_tx::W](count3_tx::W) writer structure"]
impl crate::Writable for COUNT3_TX {}
#[doc = "Transmission byte count 0"]
pub mod count3_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count4_tx](count4_tx) module"]
pub type COUNT4_TX = crate::Reg<u16, _COUNT4_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT4_TX;
#[doc = "`read()` method returns [count4_tx::R](count4_tx::R) reader structure"]
impl crate::Readable for COUNT4_TX {}
#[doc = "`write(|w| ..)` method takes [count4_tx::W](count4_tx::W) writer structure"]
impl crate::Writable for COUNT4_TX {}
#[doc = "Transmission byte count 0"]
pub mod count4_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count5_tx](count5_tx) module"]
pub type COUNT5_TX = crate::Reg<u16, _COUNT5_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT5_TX;
#[doc = "`read()` method returns [count5_tx::R](count5_tx::R) reader structure"]
impl crate::Readable for COUNT5_TX {}
#[doc = "`write(|w| ..)` method takes [count5_tx::W](count5_tx::W) writer structure"]
impl crate::Writable for COUNT5_TX {}
#[doc = "Transmission byte count 0"]
pub mod count5_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count6_tx](count6_tx) module"]
pub type COUNT6_TX = crate::Reg<u16, _COUNT6_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT6_TX;
#[doc = "`read()` method returns [count6_tx::R](count6_tx::R) reader structure"]
impl crate::Readable for COUNT6_TX {}
#[doc = "`write(|w| ..)` method takes [count6_tx::W](count6_tx::W) writer structure"]
impl crate::Writable for COUNT6_TX {}
#[doc = "Transmission byte count 0"]
pub mod count6_tx;
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count7_tx](count7_tx) module"]
pub type COUNT7_TX = crate::Reg<u16, _COUNT7_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT7_TX;
#[doc = "`read()` method returns [count7_tx::R](count7_tx::R) reader structure"]
impl crate::Readable for COUNT7_TX {}
#[doc = "`write(|w| ..)` method takes [count7_tx::W](count7_tx::W) writer structure"]
impl crate::Writable for COUNT7_TX {}
#[doc = "Transmission byte count 0"]
pub mod count7_tx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0_rx](addr0_rx) module"]
pub type ADDR0_RX = crate::Reg<u16, _ADDR0_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0_RX;
#[doc = "`read()` method returns [addr0_rx::R](addr0_rx::R) reader structure"]
impl crate::Readable for ADDR0_RX {}
#[doc = "`write(|w| ..)` method takes [addr0_rx::W](addr0_rx::W) writer structure"]
impl crate::Writable for ADDR0_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr0_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1_rx](addr1_rx) module"]
pub type ADDR1_RX = crate::Reg<u16, _ADDR1_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1_RX;
#[doc = "`read()` method returns [addr1_rx::R](addr1_rx::R) reader structure"]
impl crate::Readable for ADDR1_RX {}
#[doc = "`write(|w| ..)` method takes [addr1_rx::W](addr1_rx::W) writer structure"]
impl crate::Writable for ADDR1_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr1_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr2_rx](addr2_rx) module"]
pub type ADDR2_RX = crate::Reg<u16, _ADDR2_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR2_RX;
#[doc = "`read()` method returns [addr2_rx::R](addr2_rx::R) reader structure"]
impl crate::Readable for ADDR2_RX {}
#[doc = "`write(|w| ..)` method takes [addr2_rx::W](addr2_rx::W) writer structure"]
impl crate::Writable for ADDR2_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr2_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr3_rx](addr3_rx) module"]
pub type ADDR3_RX = crate::Reg<u16, _ADDR3_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR3_RX;
#[doc = "`read()` method returns [addr3_rx::R](addr3_rx::R) reader structure"]
impl crate::Readable for ADDR3_RX {}
#[doc = "`write(|w| ..)` method takes [addr3_rx::W](addr3_rx::W) writer structure"]
impl crate::Writable for ADDR3_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr3_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr4_rx](addr4_rx) module"]
pub type ADDR4_RX = crate::Reg<u16, _ADDR4_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR4_RX;
#[doc = "`read()` method returns [addr4_rx::R](addr4_rx::R) reader structure"]
impl crate::Readable for ADDR4_RX {}
#[doc = "`write(|w| ..)` method takes [addr4_rx::W](addr4_rx::W) writer structure"]
impl crate::Writable for ADDR4_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr4_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr5_rx](addr5_rx) module"]
pub type ADDR5_RX = crate::Reg<u16, _ADDR5_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR5_RX;
#[doc = "`read()` method returns [addr5_rx::R](addr5_rx::R) reader structure"]
impl crate::Readable for ADDR5_RX {}
#[doc = "`write(|w| ..)` method takes [addr5_rx::W](addr5_rx::W) writer structure"]
impl crate::Writable for ADDR5_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr5_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr6_rx](addr6_rx) module"]
pub type ADDR6_RX = crate::Reg<u16, _ADDR6_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR6_RX;
#[doc = "`read()` method returns [addr6_rx::R](addr6_rx::R) reader structure"]
impl crate::Readable for ADDR6_RX {}
#[doc = "`write(|w| ..)` method takes [addr6_rx::W](addr6_rx::W) writer structure"]
impl crate::Writable for ADDR6_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr6_rx;
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr7_rx](addr7_rx) module"]
pub type ADDR7_RX = crate::Reg<u16, _ADDR7_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR7_RX;
#[doc = "`read()` method returns [addr7_rx::R](addr7_rx::R) reader structure"]
impl crate::Readable for ADDR7_RX {}
#[doc = "`write(|w| ..)` method takes [addr7_rx::W](addr7_rx::W) writer structure"]
impl crate::Writable for ADDR7_RX {}
#[doc = "Reception buffer address 0"]
pub mod addr7_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count0_rx](count0_rx) module"]
pub type COUNT0_RX = crate::Reg<u16, _COUNT0_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT0_RX;
#[doc = "`read()` method returns [count0_rx::R](count0_rx::R) reader structure"]
impl crate::Readable for COUNT0_RX {}
#[doc = "`write(|w| ..)` method takes [count0_rx::W](count0_rx::W) writer structure"]
impl crate::Writable for COUNT0_RX {}
#[doc = "Reception byte count 0"]
pub mod count0_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count1_rx](count1_rx) module"]
pub type COUNT1_RX = crate::Reg<u16, _COUNT1_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT1_RX;
#[doc = "`read()` method returns [count1_rx::R](count1_rx::R) reader structure"]
impl crate::Readable for COUNT1_RX {}
#[doc = "`write(|w| ..)` method takes [count1_rx::W](count1_rx::W) writer structure"]
impl crate::Writable for COUNT1_RX {}
#[doc = "Reception byte count 0"]
pub mod count1_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count2_rx](count2_rx) module"]
pub type COUNT2_RX = crate::Reg<u16, _COUNT2_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT2_RX;
#[doc = "`read()` method returns [count2_rx::R](count2_rx::R) reader structure"]
impl crate::Readable for COUNT2_RX {}
#[doc = "`write(|w| ..)` method takes [count2_rx::W](count2_rx::W) writer structure"]
impl crate::Writable for COUNT2_RX {}
#[doc = "Reception byte count 0"]
pub mod count2_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count3_rx](count3_rx) module"]
pub type COUNT3_RX = crate::Reg<u16, _COUNT3_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT3_RX;
#[doc = "`read()` method returns [count3_rx::R](count3_rx::R) reader structure"]
impl crate::Readable for COUNT3_RX {}
#[doc = "`write(|w| ..)` method takes [count3_rx::W](count3_rx::W) writer structure"]
impl crate::Writable for COUNT3_RX {}
#[doc = "Reception byte count 0"]
pub mod count3_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count4_rx](count4_rx) module"]
pub type COUNT4_RX = crate::Reg<u16, _COUNT4_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT4_RX;
#[doc = "`read()` method returns [count4_rx::R](count4_rx::R) reader structure"]
impl crate::Readable for COUNT4_RX {}
#[doc = "`write(|w| ..)` method takes [count4_rx::W](count4_rx::W) writer structure"]
impl crate::Writable for COUNT4_RX {}
#[doc = "Reception byte count 0"]
pub mod count4_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count5_rx](count5_rx) module"]
pub type COUNT5_RX = crate::Reg<u16, _COUNT5_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT5_RX;
#[doc = "`read()` method returns [count5_rx::R](count5_rx::R) reader structure"]
impl crate::Readable for COUNT5_RX {}
#[doc = "`write(|w| ..)` method takes [count5_rx::W](count5_rx::W) writer structure"]
impl crate::Writable for COUNT5_RX {}
#[doc = "Reception byte count 0"]
pub mod count5_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count6_rx](count6_rx) module"]
pub type COUNT6_RX = crate::Reg<u16, _COUNT6_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT6_RX;
#[doc = "`read()` method returns [count6_rx::R](count6_rx::R) reader structure"]
impl crate::Readable for COUNT6_RX {}
#[doc = "`write(|w| ..)` method takes [count6_rx::W](count6_rx::W) writer structure"]
impl crate::Writable for COUNT6_RX {}
#[doc = "Reception byte count 0"]
pub mod count6_rx;
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count7_rx](count7_rx) module"]
pub type COUNT7_RX = crate::Reg<u16, _COUNT7_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT7_RX;
#[doc = "`read()` method returns [count7_rx::R](count7_rx::R) reader structure"]
impl crate::Readable for COUNT7_RX {}
#[doc = "`write(|w| ..)` method takes [count7_rx::W](count7_rx::W) writer structure"]
impl crate::Writable for COUNT7_RX {}
#[doc = "Reception byte count 0"]
pub mod count7_rx;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcsr](lpmcsr) module"]
pub type LPMCSR = crate::Reg<u16, _LPMCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCSR;
#[doc = "`read()` method returns [lpmcsr::R](lpmcsr::R) reader structure"]
impl crate::Readable for LPMCSR {}
#[doc = "`write(|w| ..)` method takes [lpmcsr::W](lpmcsr::W) writer structure"]
impl crate::Writable for LPMCSR {}
#[doc = "control and status register"]
pub mod lpmcsr;
#[doc = "Battery charging detector(\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdr](bcdr) module"]
pub type BCDR = crate::Reg<u16, _BCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCDR;
#[doc = "`read()` method returns [bcdr::R](bcdr::R) reader structure"]
impl crate::Readable for BCDR {}
#[doc = "`write(|w| ..)` method takes [bcdr::W](bcdr::W) writer structure"]
impl crate::Writable for BCDR {}
#[doc = "Battery charging detector("]
pub mod bcdr;
