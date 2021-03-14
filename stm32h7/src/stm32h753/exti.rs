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
    pub cpuimr1: CPUIMR1,
    #[doc = "0x84 - EXTI event mask register"]
    pub cpuemr1: CPUEMR1,
    #[doc = "0x88 - EXTI pending register"]
    pub cpupr1: CPUPR1,
    _reserved20: [u8; 4usize],
    #[doc = "0x90 - EXTI interrupt mask register"]
    pub cpuimr2: CPUIMR2,
    #[doc = "0x94 - EXTI event mask register"]
    pub cpuemr2: CPUEMR2,
    #[doc = "0x98 - EXTI pending register"]
    pub cpupr2: CPUPR2,
    _reserved23: [u8; 4usize],
    #[doc = "0xa0 - EXTI interrupt mask register"]
    pub cpuimr3: CPUIMR3,
    #[doc = "0xa4 - EXTI event mask register"]
    pub cpuemr3: CPUEMR3,
    #[doc = "0xa8 - EXTI pending register"]
    pub cpupr3: CPUPR3,
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
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuimr1](cpuimr1) module"]
pub type CPUIMR1 = crate::Reg<u32, _CPUIMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIMR1;
#[doc = "`read()` method returns [cpuimr1::R](cpuimr1::R) reader structure"]
impl crate::Readable for CPUIMR1 {}
#[doc = "`write(|w| ..)` method takes [cpuimr1::W](cpuimr1::W) writer structure"]
impl crate::Writable for CPUIMR1 {}
#[doc = "EXTI interrupt mask register"]
pub mod cpuimr1;
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuemr1](cpuemr1) module"]
pub type CPUEMR1 = crate::Reg<u32, _CPUEMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUEMR1;
#[doc = "`read()` method returns [cpuemr1::R](cpuemr1::R) reader structure"]
impl crate::Readable for CPUEMR1 {}
#[doc = "`write(|w| ..)` method takes [cpuemr1::W](cpuemr1::W) writer structure"]
impl crate::Writable for CPUEMR1 {}
#[doc = "EXTI event mask register"]
pub mod cpuemr1;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpupr1](cpupr1) module"]
pub type CPUPR1 = crate::Reg<u32, _CPUPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUPR1;
#[doc = "`read()` method returns [cpupr1::R](cpupr1::R) reader structure"]
impl crate::Readable for CPUPR1 {}
#[doc = "`write(|w| ..)` method takes [cpupr1::W](cpupr1::W) writer structure"]
impl crate::Writable for CPUPR1 {}
#[doc = "EXTI pending register"]
pub mod cpupr1;
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuimr2](cpuimr2) module"]
pub type CPUIMR2 = crate::Reg<u32, _CPUIMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIMR2;
#[doc = "`read()` method returns [cpuimr2::R](cpuimr2::R) reader structure"]
impl crate::Readable for CPUIMR2 {}
#[doc = "`write(|w| ..)` method takes [cpuimr2::W](cpuimr2::W) writer structure"]
impl crate::Writable for CPUIMR2 {}
#[doc = "EXTI interrupt mask register"]
pub mod cpuimr2;
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuemr2](cpuemr2) module"]
pub type CPUEMR2 = crate::Reg<u32, _CPUEMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUEMR2;
#[doc = "`read()` method returns [cpuemr2::R](cpuemr2::R) reader structure"]
impl crate::Readable for CPUEMR2 {}
#[doc = "`write(|w| ..)` method takes [cpuemr2::W](cpuemr2::W) writer structure"]
impl crate::Writable for CPUEMR2 {}
#[doc = "EXTI event mask register"]
pub mod cpuemr2;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpupr2](cpupr2) module"]
pub type CPUPR2 = crate::Reg<u32, _CPUPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUPR2;
#[doc = "`read()` method returns [cpupr2::R](cpupr2::R) reader structure"]
impl crate::Readable for CPUPR2 {}
#[doc = "`write(|w| ..)` method takes [cpupr2::W](cpupr2::W) writer structure"]
impl crate::Writable for CPUPR2 {}
#[doc = "EXTI pending register"]
pub mod cpupr2;
#[doc = "EXTI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuimr3](cpuimr3) module"]
pub type CPUIMR3 = crate::Reg<u32, _CPUIMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIMR3;
#[doc = "`read()` method returns [cpuimr3::R](cpuimr3::R) reader structure"]
impl crate::Readable for CPUIMR3 {}
#[doc = "`write(|w| ..)` method takes [cpuimr3::W](cpuimr3::W) writer structure"]
impl crate::Writable for CPUIMR3 {}
#[doc = "EXTI interrupt mask register"]
pub mod cpuimr3;
#[doc = "EXTI event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuemr3](cpuemr3) module"]
pub type CPUEMR3 = crate::Reg<u32, _CPUEMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUEMR3;
#[doc = "`read()` method returns [cpuemr3::R](cpuemr3::R) reader structure"]
impl crate::Readable for CPUEMR3 {}
#[doc = "`write(|w| ..)` method takes [cpuemr3::W](cpuemr3::W) writer structure"]
impl crate::Writable for CPUEMR3 {}
#[doc = "EXTI event mask register"]
pub mod cpuemr3;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpupr3](cpupr3) module"]
pub type CPUPR3 = crate::Reg<u32, _CPUPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUPR3;
#[doc = "`read()` method returns [cpupr3::R](cpupr3::R) reader structure"]
impl crate::Readable for CPUPR3 {}
#[doc = "`write(|w| ..)` method takes [cpupr3::W](cpupr3::W) writer structure"]
impl crate::Writable for CPUPR3 {}
#[doc = "EXTI pending register"]
pub mod cpupr3;
