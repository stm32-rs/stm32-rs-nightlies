#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - EXTI falling trigger selection register"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - EXTI software interrupt event register"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI D3 pending mask register"]
    pub d3pmr1: D3PMR1,
    #[doc = "0x10 - EXTI D3 pending clear selection register low"]
    pub d3pcr1l: D3PCR1L,
    #[doc = "0x14 - EXTI D3 pending clear selection register high"]
    pub d3pcr1h: D3PCR1H,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - EXTI rising trigger selection register"]
    pub rtsr2: RTSR2,
    #[doc = "0x24 - EXTI falling trigger selection register"]
    pub ftsr2: FTSR2,
    #[doc = "0x28 - EXTI software interrupt event register"]
    pub swier2: SWIER2,
    #[doc = "0x2c - EXTI D3 pending mask register"]
    pub d3pmr2: D3PMR2,
    #[doc = "0x30 - EXTI D3 pending clear selection register low"]
    pub d3pcr2l: D3PCR2L,
    #[doc = "0x34 - EXTI D3 pending clear selection register high"]
    pub d3pcr2h: D3PCR2H,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - EXTI rising trigger selection register"]
    pub rtsr3: RTSR3,
    #[doc = "0x44 - EXTI falling trigger selection register"]
    pub ftsr3: FTSR3,
    #[doc = "0x48 - EXTI software interrupt event register"]
    pub swier3: SWIER3,
    #[doc = "0x4c - EXTI D3 pending mask register"]
    pub d3pmr3: D3PMR3,
    _reserved16: [u8; 4usize],
    #[doc = "0x54 - EXTI D3 pending clear selection register high"]
    pub d3pcr3h: D3PCR3H,
    _reserved17: [u8; 40usize],
    #[doc = "0x80 - EXTI interrupt mask register"]
    pub c1imr1: C1IMR1,
    #[doc = "0x84 - EXTI event mask register"]
    pub c1emr1: C1EMR1,
    #[doc = "0x88 - EXTI pending register"]
    pub c1pr1: C1PR1,
    _reserved20: [u8; 4usize],
    #[doc = "0x90 - EXTI interrupt mask register"]
    pub c1imr2: C1IMR2,
    #[doc = "0x94 - EXTI event mask register"]
    pub c1emr2: C1EMR2,
    #[doc = "0x98 - EXTI pending register"]
    pub c1pr2: C1PR2,
    _reserved23: [u8; 4usize],
    #[doc = "0xa0 - EXTI interrupt mask register"]
    pub c1imr3: C1IMR3,
    #[doc = "0xa4 - EXTI event mask register"]
    pub c1emr3: C1EMR3,
    #[doc = "0xa8 - EXTI pending register"]
    pub c1pr3: C1PR3,
    _reserved26: [u8; 20usize],
    #[doc = "0xc0 - CPU2 EXTI interrupt mask register"]
    pub c2imr1: C2IMR1,
    #[doc = "0xc4 - CPU2 EXTI event mask register"]
    pub c2emr1: C2EMR1,
    #[doc = "0xc8 - CPU2 EXTI pending register"]
    pub c2pr1: C2PR1,
    _reserved29: [u8; 4usize],
    #[doc = "0xd0 - CPU2 EXTI interrupt mask register"]
    pub c2imr2: C2IMR2,
    #[doc = "0xd4 - CPU2 EXTI event mask register"]
    pub c2emr2: C2EMR2,
    #[doc = "0xd8 - CPU2 EXTI pending register"]
    pub c2pr2: C2PR2,
    _reserved32: [u8; 4usize],
    #[doc = "0xe0 - CPU2 EXTI interrupt mask register"]
    pub c2imr3: C2IMR3,
    #[doc = "0xe4 - CPU2 EXTI event mask register"]
    pub c2emr3: C2EMR3,
    #[doc = "0xe8 - CPU2 EXTI pending register"]
    pub c2pr3: C2PR3,
}
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr1](rtsr1) module"]
pub type RTSR1 = crate::Reg<u32, _RTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR1;
#[doc = "`read()` method returns [rtsr1::R](rtsr1::R) reader structure"]
impl crate::Readable for RTSR1 {}
#[doc = "`write(|w| ..)` method takes [rtsr1::W](rtsr1::W) writer structure"]
impl crate::Writable for RTSR1 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr1](ftsr1) module"]
pub type FTSR1 = crate::Reg<u32, _FTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR1;
#[doc = "`read()` method returns [ftsr1::R](ftsr1::R) reader structure"]
impl crate::Readable for FTSR1 {}
#[doc = "`write(|w| ..)` method takes [ftsr1::W](ftsr1::W) writer structure"]
impl crate::Writable for FTSR1 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier1](swier1) module"]
pub type SWIER1 = crate::Reg<u32, _SWIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER1;
#[doc = "`read()` method returns [swier1::R](swier1::R) reader structure"]
impl crate::Readable for SWIER1 {}
#[doc = "`write(|w| ..)` method takes [swier1::W](swier1::W) writer structure"]
impl crate::Writable for SWIER1 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "EXTI D3 pending mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pmr1](d3pmr1) module"]
pub type D3PMR1 = crate::Reg<u32, _D3PMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PMR1;
#[doc = "`read()` method returns [d3pmr1::R](d3pmr1::R) reader structure"]
impl crate::Readable for D3PMR1 {}
#[doc = "`write(|w| ..)` method takes [d3pmr1::W](d3pmr1::W) writer structure"]
impl crate::Writable for D3PMR1 {}
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr1;
#[doc = "EXTI D3 pending clear selection register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr1l](d3pcr1l) module"]
pub type D3PCR1L = crate::Reg<u32, _D3PCR1L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PCR1L;
#[doc = "`read()` method returns [d3pcr1l::R](d3pcr1l::R) reader structure"]
impl crate::Readable for D3PCR1L {}
#[doc = "`write(|w| ..)` method takes [d3pcr1l::W](d3pcr1l::W) writer structure"]
impl crate::Writable for D3PCR1L {}
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr1l;
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr1h](d3pcr1h) module"]
pub type D3PCR1H = crate::Reg<u32, _D3PCR1H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PCR1H;
#[doc = "`read()` method returns [d3pcr1h::R](d3pcr1h::R) reader structure"]
impl crate::Readable for D3PCR1H {}
#[doc = "`write(|w| ..)` method takes [d3pcr1h::W](d3pcr1h::W) writer structure"]
impl crate::Writable for D3PCR1H {}
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr1h;
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](rtsr2) module"]
pub type RTSR2 = crate::Reg<u32, _RTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR2;
#[doc = "`read()` method returns [rtsr2::R](rtsr2::R) reader structure"]
impl crate::Readable for RTSR2 {}
#[doc = "`write(|w| ..)` method takes [rtsr2::W](rtsr2::W) writer structure"]
impl crate::Writable for RTSR2 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr2;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](ftsr2) module"]
pub type FTSR2 = crate::Reg<u32, _FTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR2;
#[doc = "`read()` method returns [ftsr2::R](ftsr2::R) reader structure"]
impl crate::Readable for FTSR2 {}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](ftsr2::W) writer structure"]
impl crate::Writable for FTSR2 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr2;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](swier2) module"]
pub type SWIER2 = crate::Reg<u32, _SWIER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER2;
#[doc = "`read()` method returns [swier2::R](swier2::R) reader structure"]
impl crate::Readable for SWIER2 {}
#[doc = "`write(|w| ..)` method takes [swier2::W](swier2::W) writer structure"]
impl crate::Writable for SWIER2 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier2;
#[doc = "EXTI D3 pending mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pmr2](d3pmr2) module"]
pub type D3PMR2 = crate::Reg<u32, _D3PMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PMR2;
#[doc = "`read()` method returns [d3pmr2::R](d3pmr2::R) reader structure"]
impl crate::Readable for D3PMR2 {}
#[doc = "`write(|w| ..)` method takes [d3pmr2::W](d3pmr2::W) writer structure"]
impl crate::Writable for D3PMR2 {}
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr2;
#[doc = "EXTI D3 pending clear selection register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr2l](d3pcr2l) module"]
pub type D3PCR2L = crate::Reg<u32, _D3PCR2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PCR2L;
#[doc = "`read()` method returns [d3pcr2l::R](d3pcr2l::R) reader structure"]
impl crate::Readable for D3PCR2L {}
#[doc = "`write(|w| ..)` method takes [d3pcr2l::W](d3pcr2l::W) writer structure"]
impl crate::Writable for D3PCR2L {}
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr2l;
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr2h](d3pcr2h) module"]
pub type D3PCR2H = crate::Reg<u32, _D3PCR2H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PCR2H;
#[doc = "`read()` method returns [d3pcr2h::R](d3pcr2h::R) reader structure"]
impl crate::Readable for D3PCR2H {}
#[doc = "`write(|w| ..)` method takes [d3pcr2h::W](d3pcr2h::W) writer structure"]
impl crate::Writable for D3PCR2H {}
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr2h;
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr3](rtsr3) module"]
pub type RTSR3 = crate::Reg<u32, _RTSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR3;
#[doc = "`read()` method returns [rtsr3::R](rtsr3::R) reader structure"]
impl crate::Readable for RTSR3 {}
#[doc = "`write(|w| ..)` method takes [rtsr3::W](rtsr3::W) writer structure"]
impl crate::Writable for RTSR3 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr3;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr3](ftsr3) module"]
pub type FTSR3 = crate::Reg<u32, _FTSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR3;
#[doc = "`read()` method returns [ftsr3::R](ftsr3::R) reader structure"]
impl crate::Readable for FTSR3 {}
#[doc = "`write(|w| ..)` method takes [ftsr3::W](ftsr3::W) writer structure"]
impl crate::Writable for FTSR3 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr3;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier3](swier3) module"]
pub type SWIER3 = crate::Reg<u32, _SWIER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER3;
#[doc = "`read()` method returns [swier3::R](swier3::R) reader structure"]
impl crate::Readable for SWIER3 {}
#[doc = "`write(|w| ..)` method takes [swier3::W](swier3::W) writer structure"]
impl crate::Writable for SWIER3 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier3;
#[doc = "EXTI D3 pending mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pmr3](d3pmr3) module"]
pub type D3PMR3 = crate::Reg<u32, _D3PMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PMR3;
#[doc = "`read()` method returns [d3pmr3::R](d3pmr3::R) reader structure"]
impl crate::Readable for D3PMR3 {}
#[doc = "`write(|w| ..)` method takes [d3pmr3::W](d3pmr3::W) writer structure"]
impl crate::Writable for D3PMR3 {}
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr3;
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr3h](d3pcr3h) module"]
pub type D3PCR3H = crate::Reg<u32, _D3PCR3H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3PCR3H;
#[doc = "`read()` method returns [d3pcr3h::R](d3pcr3h::R) reader structure"]
impl crate::Readable for D3PCR3H {}
#[doc = "`write(|w| ..)` method takes [d3pcr3h::W](d3pcr3h::W) writer structure"]
impl crate::Writable for D3PCR3H {}
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr3h;
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr1](c1imr1) module"]
pub type C1IMR1 = crate::Reg<u32, _C1IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IMR1;
#[doc = "`read()` method returns [c1imr1::R](c1imr1::R) reader structure"]
impl crate::Readable for C1IMR1 {}
#[doc = "`write(|w| ..)` method takes [c1imr1::W](c1imr1::W) writer structure"]
impl crate::Writable for C1IMR1 {}
#[doc = "EXTI interrupt mask register"]
pub mod c1imr1;
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1emr1](c1emr1) module"]
pub type C1EMR1 = crate::Reg<u32, _C1EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1EMR1;
#[doc = "`read()` method returns [c1emr1::R](c1emr1::R) reader structure"]
impl crate::Readable for C1EMR1 {}
#[doc = "`write(|w| ..)` method takes [c1emr1::W](c1emr1::W) writer structure"]
impl crate::Writable for C1EMR1 {}
#[doc = "EXTI event mask register"]
pub mod c1emr1;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1pr1](c1pr1) module"]
pub type C1PR1 = crate::Reg<u32, _C1PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1PR1;
#[doc = "`read()` method returns [c1pr1::R](c1pr1::R) reader structure"]
impl crate::Readable for C1PR1 {}
#[doc = "`write(|w| ..)` method takes [c1pr1::W](c1pr1::W) writer structure"]
impl crate::Writable for C1PR1 {}
#[doc = "EXTI pending register"]
pub mod c1pr1;
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr2](c1imr2) module"]
pub type C1IMR2 = crate::Reg<u32, _C1IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IMR2;
#[doc = "`read()` method returns [c1imr2::R](c1imr2::R) reader structure"]
impl crate::Readable for C1IMR2 {}
#[doc = "`write(|w| ..)` method takes [c1imr2::W](c1imr2::W) writer structure"]
impl crate::Writable for C1IMR2 {}
#[doc = "EXTI interrupt mask register"]
pub mod c1imr2;
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1emr2](c1emr2) module"]
pub type C1EMR2 = crate::Reg<u32, _C1EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1EMR2;
#[doc = "`read()` method returns [c1emr2::R](c1emr2::R) reader structure"]
impl crate::Readable for C1EMR2 {}
#[doc = "`write(|w| ..)` method takes [c1emr2::W](c1emr2::W) writer structure"]
impl crate::Writable for C1EMR2 {}
#[doc = "EXTI event mask register"]
pub mod c1emr2;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1pr2](c1pr2) module"]
pub type C1PR2 = crate::Reg<u32, _C1PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1PR2;
#[doc = "`read()` method returns [c1pr2::R](c1pr2::R) reader structure"]
impl crate::Readable for C1PR2 {}
#[doc = "`write(|w| ..)` method takes [c1pr2::W](c1pr2::W) writer structure"]
impl crate::Writable for C1PR2 {}
#[doc = "EXTI pending register"]
pub mod c1pr2;
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr3](c1imr3) module"]
pub type C1IMR3 = crate::Reg<u32, _C1IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IMR3;
#[doc = "`read()` method returns [c1imr3::R](c1imr3::R) reader structure"]
impl crate::Readable for C1IMR3 {}
#[doc = "`write(|w| ..)` method takes [c1imr3::W](c1imr3::W) writer structure"]
impl crate::Writable for C1IMR3 {}
#[doc = "EXTI interrupt mask register"]
pub mod c1imr3;
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1emr3](c1emr3) module"]
pub type C1EMR3 = crate::Reg<u32, _C1EMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1EMR3;
#[doc = "`read()` method returns [c1emr3::R](c1emr3::R) reader structure"]
impl crate::Readable for C1EMR3 {}
#[doc = "`write(|w| ..)` method takes [c1emr3::W](c1emr3::W) writer structure"]
impl crate::Writable for C1EMR3 {}
#[doc = "EXTI event mask register"]
pub mod c1emr3;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1pr3](c1pr3) module"]
pub type C1PR3 = crate::Reg<u32, _C1PR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1PR3;
#[doc = "`read()` method returns [c1pr3::R](c1pr3::R) reader structure"]
impl crate::Readable for C1PR3 {}
#[doc = "`write(|w| ..)` method takes [c1pr3::W](c1pr3::W) writer structure"]
impl crate::Writable for C1PR3 {}
#[doc = "EXTI pending register"]
pub mod c1pr3;
#[doc = "CPU2 EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](c2imr1) module"]
pub type C2IMR1 = crate::Reg<u32, _C2IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR1;
#[doc = "`read()` method returns [c2imr1::R](c2imr1::R) reader structure"]
impl crate::Readable for C2IMR1 {}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](c2imr1::W) writer structure"]
impl crate::Writable for C2IMR1 {}
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr1;
#[doc = "CPU2 EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr1](c2emr1) module"]
pub type C2EMR1 = crate::Reg<u32, _C2EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR1;
#[doc = "`read()` method returns [c2emr1::R](c2emr1::R) reader structure"]
impl crate::Readable for C2EMR1 {}
#[doc = "`write(|w| ..)` method takes [c2emr1::W](c2emr1::W) writer structure"]
impl crate::Writable for C2EMR1 {}
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr1;
#[doc = "CPU2 EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2pr1](c2pr1) module"]
pub type C2PR1 = crate::Reg<u32, _C2PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2PR1;
#[doc = "`read()` method returns [c2pr1::R](c2pr1::R) reader structure"]
impl crate::Readable for C2PR1 {}
#[doc = "`write(|w| ..)` method takes [c2pr1::W](c2pr1::W) writer structure"]
impl crate::Writable for C2PR1 {}
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr1;
#[doc = "CPU2 EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](c2imr2) module"]
pub type C2IMR2 = crate::Reg<u32, _C2IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR2;
#[doc = "`read()` method returns [c2imr2::R](c2imr2::R) reader structure"]
impl crate::Readable for C2IMR2 {}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](c2imr2::W) writer structure"]
impl crate::Writable for C2IMR2 {}
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr2;
#[doc = "CPU2 EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr2](c2emr2) module"]
pub type C2EMR2 = crate::Reg<u32, _C2EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR2;
#[doc = "`read()` method returns [c2emr2::R](c2emr2::R) reader structure"]
impl crate::Readable for C2EMR2 {}
#[doc = "`write(|w| ..)` method takes [c2emr2::W](c2emr2::W) writer structure"]
impl crate::Writable for C2EMR2 {}
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr2;
#[doc = "CPU2 EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2pr2](c2pr2) module"]
pub type C2PR2 = crate::Reg<u32, _C2PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2PR2;
#[doc = "`read()` method returns [c2pr2::R](c2pr2::R) reader structure"]
impl crate::Readable for C2PR2 {}
#[doc = "`write(|w| ..)` method takes [c2pr2::W](c2pr2::W) writer structure"]
impl crate::Writable for C2PR2 {}
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr2;
#[doc = "CPU2 EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr3](c2imr3) module"]
pub type C2IMR3 = crate::Reg<u32, _C2IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR3;
#[doc = "`read()` method returns [c2imr3::R](c2imr3::R) reader structure"]
impl crate::Readable for C2IMR3 {}
#[doc = "`write(|w| ..)` method takes [c2imr3::W](c2imr3::W) writer structure"]
impl crate::Writable for C2IMR3 {}
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr3;
#[doc = "CPU2 EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr3](c2emr3) module"]
pub type C2EMR3 = crate::Reg<u32, _C2EMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR3;
#[doc = "`read()` method returns [c2emr3::R](c2emr3::R) reader structure"]
impl crate::Readable for C2EMR3 {}
#[doc = "`write(|w| ..)` method takes [c2emr3::W](c2emr3::W) writer structure"]
impl crate::Writable for C2EMR3 {}
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr3;
#[doc = "CPU2 EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2pr3](c2pr3) module"]
pub type C2PR3 = crate::Reg<u32, _C2PR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2PR3;
#[doc = "`read()` method returns [c2pr3::R](c2pr3::R) reader structure"]
impl crate::Readable for C2PR3 {}
#[doc = "`write(|w| ..)` method takes [c2pr3::W](c2pr3::W) writer structure"]
impl crate::Writable for C2PR3 {}
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr3;
