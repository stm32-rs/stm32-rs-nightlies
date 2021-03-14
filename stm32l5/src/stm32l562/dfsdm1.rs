#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    pub ch0cfgr1: CH0CFGR1,
    #[doc = "0x04 - channel configuration y register"]
    pub ch0cfgr2: CH0CFGR2,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub ch0awscdr: CH0AWSCDR,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub ch0wdatr: CH0WDATR,
    #[doc = "0x10 - channel data input register"]
    pub ch0datinr: CH0DATINR,
    #[doc = "0x14 - DFSDM channel y delay register"]
    pub ch0dlyr: CH0DLYR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - CHCFG1R1"]
    pub ch1cfgr1: CH1CFGR1,
    #[doc = "0x24 - CHCFG1R2"]
    pub ch1cfgr2: CH1CFGR2,
    #[doc = "0x28 - AWSCD1R"]
    pub ch1awscdr: CH1AWSCDR,
    #[doc = "0x2c - CHWDAT1R"]
    pub ch1wdatr: CH1WDATR,
    #[doc = "0x30 - CHDATIN1R"]
    pub ch1datinr: CH1DATINR,
    #[doc = "0x34 - DFSDM channel y delay register"]
    pub ch1dlyr: CH1DLYR,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - CHCFG2R1"]
    pub ch2cfgr1: CH2CFGR1,
    #[doc = "0x44 - CHCFG2R2"]
    pub ch2cfgr2: CH2CFGR2,
    #[doc = "0x48 - AWSCD2R"]
    pub ch2awscdr: CH2AWSCDR,
    #[doc = "0x4c - CHWDAT2R"]
    pub ch2wdatr: CH2WDATR,
    #[doc = "0x50 - CHDATIN2R"]
    pub ch2datinr: CH2DATINR,
    #[doc = "0x54 - DFSDM channel y delay register"]
    pub ch2dlyr: CH2DLYR,
    _reserved18: [u8; 8usize],
    #[doc = "0x60 - CHCFG3R1"]
    pub ch3cfgr1: CH3CFGR1,
    #[doc = "0x64 - CHCFG3R2"]
    pub ch3cfgr2: CH3CFGR2,
    #[doc = "0x68 - AWSCD3R"]
    pub ch3awscdr: CH3AWSCDR,
    #[doc = "0x6c - CHWDAT3R"]
    pub ch3wdatr: CH3WDATR,
    #[doc = "0x70 - CHDATIN3R"]
    pub ch3datinr: CH3DATINR,
    #[doc = "0x74 - DFSDM channel y delay register"]
    pub ch3dlyr: CH3DLYR,
    _reserved24: [u8; 8usize],
    #[doc = "0x80 - CHCFG4R1"]
    pub ch4cfgr1: CH4CFGR1,
    #[doc = "0x84 - CHCFG4R2"]
    pub ch4cfgr2: CH4CFGR2,
    #[doc = "0x88 - AWSCD4R"]
    pub ch4awscdr: CH4AWSCDR,
    #[doc = "0x8c - CHWDAT4R"]
    pub ch4wdatr: CH4WDATR,
    #[doc = "0x90 - CHDATIN4R"]
    pub ch4datinr: CH4DATINR,
    #[doc = "0x94 - DFSDM channel y delay register"]
    pub ch4dlyr: CH4DLYR,
    _reserved30: [u8; 8usize],
    #[doc = "0xa0 - CHCFG5R1"]
    pub ch5cfgr1: CH5CFGR1,
    #[doc = "0xa4 - CHCFG5R2"]
    pub ch5cfgr2: CH5CFGR2,
    #[doc = "0xa8 - AWSCD5R"]
    pub ch5awscdr: CH5AWSCDR,
    #[doc = "0xac - CHWDAT5R"]
    pub ch5wdatr: CH5WDATR,
    #[doc = "0xb0 - CHDATIN5R"]
    pub ch5datinr: CH5DATINR,
    #[doc = "0xb4 - DFSDM channel y delay register"]
    pub ch5dlyr: CH5DLYR,
    _reserved36: [u8; 8usize],
    #[doc = "0xc0 - CHCFG6R1"]
    pub ch6cfgr1: CH6CFGR1,
    #[doc = "0xc4 - CH6CFGR2"]
    pub ch6cfgr2: CH6CFGR2,
    #[doc = "0xc8 - AWSCD6R"]
    pub ch6awscdr: CH6AWSCDR,
    #[doc = "0xcc - CHWDAT6R"]
    pub ch6wdatr: CH6WDATR,
    #[doc = "0xd0 - CHDATIN6R"]
    pub ch6datinr: CH6DATINR,
    #[doc = "0xd4 - DFSDM channel y delay register"]
    pub ch6dlyr: CH6DLYR,
    _reserved42: [u8; 8usize],
    #[doc = "0xe0 - CHCFG7R1"]
    pub ch7cfgr1: CH7CFGR1,
    #[doc = "0xe4 - CHCFG7R2"]
    pub ch7cfgr2: CH7CFGR2,
    #[doc = "0xe8 - AWSCD7R"]
    pub ch7awscdr: CH7AWSCDR,
    #[doc = "0xec - CHWDAT7R"]
    pub ch7wdatr: CH7WDATR,
    #[doc = "0xf0 - CHDATIN7R"]
    pub ch7datinr: CH7DATINR,
    #[doc = "0xf4 - DFSDM channel y delay register"]
    pub ch7dlyr: CH7DLYR,
    _reserved48: [u8; 8usize],
    #[doc = "0x100 - control register 1"]
    pub flt0cr1: FLT0CR1,
    #[doc = "0x104 - control register 2"]
    pub flt0cr2: FLT0CR2,
    #[doc = "0x108 - interrupt and status register"]
    pub flt0isr: FLT0ISR,
    #[doc = "0x10c - interrupt flag clear register"]
    pub flt0icr: FLT0ICR,
    #[doc = "0x110 - injected channel group selection register"]
    pub flt0jchgr: FLT0JCHGR,
    #[doc = "0x114 - filter control register"]
    pub flt0fcr: FLT0FCR,
    #[doc = "0x118 - data register for injected group"]
    pub flt0jdatar: FLT0JDATAR,
    #[doc = "0x11c - data register for the regular channel"]
    pub flt0rdatar: FLT0RDATAR,
    #[doc = "0x120 - analog watchdog high threshold register"]
    pub flt0awhtr: FLT0AWHTR,
    #[doc = "0x124 - analog watchdog low threshold register"]
    pub flt0awltr: FLT0AWLTR,
    #[doc = "0x128 - analog watchdog status register"]
    pub flt0awsr: FLT0AWSR,
    #[doc = "0x12c - analog watchdog clear flag register"]
    pub flt0awcfr: FLT0AWCFR,
    #[doc = "0x130 - Extremes detector maximum register"]
    pub flt0exmax: FLT0EXMAX,
    #[doc = "0x134 - Extremes detector minimum register"]
    pub flt0exmin: FLT0EXMIN,
    #[doc = "0x138 - conversion timer register"]
    pub flt0cnvtimr: FLT0CNVTIMR,
    _reserved63: [u8; 68usize],
    #[doc = "0x180 - control register 1"]
    pub flt1cr1: FLT1CR1,
    #[doc = "0x184 - control register 2"]
    pub flt1cr2: FLT1CR2,
    #[doc = "0x188 - interrupt and status register"]
    pub flt1isr: FLT1ISR,
    #[doc = "0x18c - interrupt flag clear register"]
    pub flt1icr: FLT1ICR,
    #[doc = "0x190 - injected channel group selection register"]
    pub flt1jchgr: FLT1JCHGR,
    #[doc = "0x194 - filter control register"]
    pub flt1fcr: FLT1FCR,
    #[doc = "0x198 - data register for injected group"]
    pub flt1jdatar: FLT1JDATAR,
    #[doc = "0x19c - data register for the regular channel"]
    pub flt1rdatar: FLT1RDATAR,
    _reserved71: [u8; 4usize],
    #[doc = "0x1a4 - analog watchdog low threshold register"]
    pub flt1awltr: FLT1AWLTR,
    #[doc = "0x1a8 - analog watchdog status register"]
    pub flt1awsr: FLT1AWSR,
    _reserved_73_flt: [u8; 4usize],
    #[doc = "0x1b0 - Extremes detector maximum register"]
    pub flt1exmax: FLT1EXMAX,
    #[doc = "0x1b4 - Extremes detector minimum register"]
    pub flt1exmin: FLT1EXMIN,
    #[doc = "0x1b8 - conversion timer register"]
    pub flt1cnvtimr: FLT1CNVTIMR,
    _reserved77: [u8; 68usize],
    #[doc = "0x200 - control register 1"]
    pub flt2cr1: FLT2CR1,
    #[doc = "0x204 - control register 2"]
    pub flt2cr2: FLT2CR2,
    #[doc = "0x208 - interrupt and status register"]
    pub flt2isr: FLT2ISR,
    #[doc = "0x20c - interrupt flag clear register"]
    pub flt2icr: FLT2ICR,
    #[doc = "0x210 - injected channel group selection register"]
    pub flt2jchgr: FLT2JCHGR,
    #[doc = "0x214 - filter control register"]
    pub flt2fcr: FLT2FCR,
    #[doc = "0x218 - data register for injected group"]
    pub flt2jdatar: FLT2JDATAR,
    #[doc = "0x21c - data register for the regular channel"]
    pub flt2rdatar: FLT2RDATAR,
    #[doc = "0x220 - analog watchdog high threshold register"]
    pub flt2awhtr: FLT2AWHTR,
    #[doc = "0x224 - analog watchdog low threshold register"]
    pub flt2awltr: FLT2AWLTR,
    #[doc = "0x228 - analog watchdog status register"]
    pub flt2awsr: FLT2AWSR,
    #[doc = "0x22c - analog watchdog clear flag register"]
    pub flt2awcfr: FLT2AWCFR,
    #[doc = "0x230 - Extremes detector maximum register"]
    pub flt2exmax: FLT2EXMAX,
    #[doc = "0x234 - Extremes detector minimum register"]
    pub flt2exmin: FLT2EXMIN,
    #[doc = "0x238 - conversion timer register"]
    pub flt2cnvtimr: FLT2CNVTIMR,
    _reserved92: [u8; 68usize],
    #[doc = "0x280 - control register 1"]
    pub flt3cr1: FLT3CR1,
    #[doc = "0x284 - control register 2"]
    pub flt3cr2: FLT3CR2,
    #[doc = "0x288 - interrupt and status register"]
    pub flt3isr: FLT3ISR,
    #[doc = "0x28c - interrupt flag clear register"]
    pub flt3icr: FLT3ICR,
    #[doc = "0x290 - injected channel group selection register"]
    pub flt3jchgr: FLT3JCHGR,
    #[doc = "0x294 - filter control register"]
    pub flt3fcr: FLT3FCR,
    #[doc = "0x298 - data register for injected group"]
    pub flt3jdatar: FLT3JDATAR,
    #[doc = "0x29c - data register for the regular channel"]
    pub flt3rdatar: FLT3RDATAR,
    #[doc = "0x2a0 - analog watchdog high threshold register"]
    pub flt3awhtr: FLT3AWHTR,
    #[doc = "0x2a4 - analog watchdog low threshold register"]
    pub flt3awltr: FLT3AWLTR,
    #[doc = "0x2a8 - analog watchdog status register"]
    pub flt3awsr: FLT3AWSR,
    #[doc = "0x2ac - analog watchdog clear flag register"]
    pub flt3awcfr: FLT3AWCFR,
    #[doc = "0x2b0 - Extremes detector maximum register"]
    pub flt3exmax: FLT3EXMAX,
    #[doc = "0x2b4 - Extremes detector minimum register"]
    pub flt3exmin: FLT3EXMIN,
    #[doc = "0x2b8 - conversion timer register"]
    pub flt3cnvtimr: FLT3CNVTIMR,
}
impl RegisterBlock {
    #[doc = "0x1ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub fn flt1awcfr(&self) -> &FLT1AWCFR {
        unsafe { &*(((self as *const Self) as *const u8).add(428usize) as *const FLT1AWCFR) }
    }
    #[doc = "0x1ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub fn flt1awcfr_mut(&self) -> &mut FLT1AWCFR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(428usize) as *mut FLT1AWCFR) }
    }
    #[doc = "0x1ac - analog watchdog high threshold register"]
    #[inline(always)]
    pub fn flt1awhtr(&self) -> &FLT1AWHTR {
        unsafe { &*(((self as *const Self) as *const u8).add(428usize) as *const FLT1AWHTR) }
    }
    #[doc = "0x1ac - analog watchdog high threshold register"]
    #[inline(always)]
    pub fn flt1awhtr_mut(&self) -> &mut FLT1AWHTR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(428usize) as *mut FLT1AWHTR) }
    }
}
#[doc = "channel configuration y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cfgr1](ch0cfgr1) module"]
pub type CH0CFGR1 = crate::Reg<u32, _CH0CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CFGR1;
#[doc = "`read()` method returns [ch0cfgr1::R](ch0cfgr1::R) reader structure"]
impl crate::Readable for CH0CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch0cfgr1::W](ch0cfgr1::W) writer structure"]
impl crate::Writable for CH0CFGR1 {}
#[doc = "channel configuration y register"]
pub mod ch0cfgr1;
#[doc = "channel configuration y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cfgr2](ch0cfgr2) module"]
pub type CH0CFGR2 = crate::Reg<u32, _CH0CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CFGR2;
#[doc = "`read()` method returns [ch0cfgr2::R](ch0cfgr2::R) reader structure"]
impl crate::Readable for CH0CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch0cfgr2::W](ch0cfgr2::W) writer structure"]
impl crate::Writable for CH0CFGR2 {}
#[doc = "channel configuration y register"]
pub mod ch0cfgr2;
#[doc = "analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0awscdr](ch0awscdr) module"]
pub type CH0AWSCDR = crate::Reg<u32, _CH0AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0AWSCDR;
#[doc = "`read()` method returns [ch0awscdr::R](ch0awscdr::R) reader structure"]
impl crate::Readable for CH0AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch0awscdr::W](ch0awscdr::W) writer structure"]
impl crate::Writable for CH0AWSCDR {}
#[doc = "analog watchdog and short-circuit detector register"]
pub mod ch0awscdr;
#[doc = "channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0wdatr](ch0wdatr) module"]
pub type CH0WDATR = crate::Reg<u32, _CH0WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0WDATR;
#[doc = "`read()` method returns [ch0wdatr::R](ch0wdatr::R) reader structure"]
impl crate::Readable for CH0WDATR {}
#[doc = "`write(|w| ..)` method takes [ch0wdatr::W](ch0wdatr::W) writer structure"]
impl crate::Writable for CH0WDATR {}
#[doc = "channel watchdog filter data register"]
pub mod ch0wdatr;
#[doc = "channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0datinr](ch0datinr) module"]
pub type CH0DATINR = crate::Reg<u32, _CH0DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0DATINR;
#[doc = "`read()` method returns [ch0datinr::R](ch0datinr::R) reader structure"]
impl crate::Readable for CH0DATINR {}
#[doc = "`write(|w| ..)` method takes [ch0datinr::W](ch0datinr::W) writer structure"]
impl crate::Writable for CH0DATINR {}
#[doc = "channel data input register"]
pub mod ch0datinr;
#[doc = "CHCFG1R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cfgr1](ch1cfgr1) module"]
pub type CH1CFGR1 = crate::Reg<u32, _CH1CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CFGR1;
#[doc = "`read()` method returns [ch1cfgr1::R](ch1cfgr1::R) reader structure"]
impl crate::Readable for CH1CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch1cfgr1::W](ch1cfgr1::W) writer structure"]
impl crate::Writable for CH1CFGR1 {}
#[doc = "CHCFG1R1"]
pub mod ch1cfgr1;
#[doc = "CHCFG1R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cfgr2](ch1cfgr2) module"]
pub type CH1CFGR2 = crate::Reg<u32, _CH1CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CFGR2;
#[doc = "`read()` method returns [ch1cfgr2::R](ch1cfgr2::R) reader structure"]
impl crate::Readable for CH1CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch1cfgr2::W](ch1cfgr2::W) writer structure"]
impl crate::Writable for CH1CFGR2 {}
#[doc = "CHCFG1R2"]
pub mod ch1cfgr2;
#[doc = "AWSCD1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1awscdr](ch1awscdr) module"]
pub type CH1AWSCDR = crate::Reg<u32, _CH1AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1AWSCDR;
#[doc = "`read()` method returns [ch1awscdr::R](ch1awscdr::R) reader structure"]
impl crate::Readable for CH1AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch1awscdr::W](ch1awscdr::W) writer structure"]
impl crate::Writable for CH1AWSCDR {}
#[doc = "AWSCD1R"]
pub mod ch1awscdr;
#[doc = "CHWDAT1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1wdatr](ch1wdatr) module"]
pub type CH1WDATR = crate::Reg<u32, _CH1WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1WDATR;
#[doc = "`read()` method returns [ch1wdatr::R](ch1wdatr::R) reader structure"]
impl crate::Readable for CH1WDATR {}
#[doc = "`write(|w| ..)` method takes [ch1wdatr::W](ch1wdatr::W) writer structure"]
impl crate::Writable for CH1WDATR {}
#[doc = "CHWDAT1R"]
pub mod ch1wdatr;
#[doc = "CHDATIN1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1datinr](ch1datinr) module"]
pub type CH1DATINR = crate::Reg<u32, _CH1DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1DATINR;
#[doc = "`read()` method returns [ch1datinr::R](ch1datinr::R) reader structure"]
impl crate::Readable for CH1DATINR {}
#[doc = "`write(|w| ..)` method takes [ch1datinr::W](ch1datinr::W) writer structure"]
impl crate::Writable for CH1DATINR {}
#[doc = "CHDATIN1R"]
pub mod ch1datinr;
#[doc = "CHCFG2R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cfgr1](ch2cfgr1) module"]
pub type CH2CFGR1 = crate::Reg<u32, _CH2CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CFGR1;
#[doc = "`read()` method returns [ch2cfgr1::R](ch2cfgr1::R) reader structure"]
impl crate::Readable for CH2CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch2cfgr1::W](ch2cfgr1::W) writer structure"]
impl crate::Writable for CH2CFGR1 {}
#[doc = "CHCFG2R1"]
pub mod ch2cfgr1;
#[doc = "CHCFG2R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cfgr2](ch2cfgr2) module"]
pub type CH2CFGR2 = crate::Reg<u32, _CH2CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CFGR2;
#[doc = "`read()` method returns [ch2cfgr2::R](ch2cfgr2::R) reader structure"]
impl crate::Readable for CH2CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch2cfgr2::W](ch2cfgr2::W) writer structure"]
impl crate::Writable for CH2CFGR2 {}
#[doc = "CHCFG2R2"]
pub mod ch2cfgr2;
#[doc = "AWSCD2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2awscdr](ch2awscdr) module"]
pub type CH2AWSCDR = crate::Reg<u32, _CH2AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2AWSCDR;
#[doc = "`read()` method returns [ch2awscdr::R](ch2awscdr::R) reader structure"]
impl crate::Readable for CH2AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch2awscdr::W](ch2awscdr::W) writer structure"]
impl crate::Writable for CH2AWSCDR {}
#[doc = "AWSCD2R"]
pub mod ch2awscdr;
#[doc = "CHWDAT2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2wdatr](ch2wdatr) module"]
pub type CH2WDATR = crate::Reg<u32, _CH2WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2WDATR;
#[doc = "`read()` method returns [ch2wdatr::R](ch2wdatr::R) reader structure"]
impl crate::Readable for CH2WDATR {}
#[doc = "`write(|w| ..)` method takes [ch2wdatr::W](ch2wdatr::W) writer structure"]
impl crate::Writable for CH2WDATR {}
#[doc = "CHWDAT2R"]
pub mod ch2wdatr;
#[doc = "CHDATIN2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2datinr](ch2datinr) module"]
pub type CH2DATINR = crate::Reg<u32, _CH2DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2DATINR;
#[doc = "`read()` method returns [ch2datinr::R](ch2datinr::R) reader structure"]
impl crate::Readable for CH2DATINR {}
#[doc = "`write(|w| ..)` method takes [ch2datinr::W](ch2datinr::W) writer structure"]
impl crate::Writable for CH2DATINR {}
#[doc = "CHDATIN2R"]
pub mod ch2datinr;
#[doc = "CHCFG3R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cfgr1](ch3cfgr1) module"]
pub type CH3CFGR1 = crate::Reg<u32, _CH3CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CFGR1;
#[doc = "`read()` method returns [ch3cfgr1::R](ch3cfgr1::R) reader structure"]
impl crate::Readable for CH3CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch3cfgr1::W](ch3cfgr1::W) writer structure"]
impl crate::Writable for CH3CFGR1 {}
#[doc = "CHCFG3R1"]
pub mod ch3cfgr1;
#[doc = "CHCFG3R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cfgr2](ch3cfgr2) module"]
pub type CH3CFGR2 = crate::Reg<u32, _CH3CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CFGR2;
#[doc = "`read()` method returns [ch3cfgr2::R](ch3cfgr2::R) reader structure"]
impl crate::Readable for CH3CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch3cfgr2::W](ch3cfgr2::W) writer structure"]
impl crate::Writable for CH3CFGR2 {}
#[doc = "CHCFG3R2"]
pub mod ch3cfgr2;
#[doc = "AWSCD3R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3awscdr](ch3awscdr) module"]
pub type CH3AWSCDR = crate::Reg<u32, _CH3AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3AWSCDR;
#[doc = "`read()` method returns [ch3awscdr::R](ch3awscdr::R) reader structure"]
impl crate::Readable for CH3AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch3awscdr::W](ch3awscdr::W) writer structure"]
impl crate::Writable for CH3AWSCDR {}
#[doc = "AWSCD3R"]
pub mod ch3awscdr;
#[doc = "CHWDAT3R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3wdatr](ch3wdatr) module"]
pub type CH3WDATR = crate::Reg<u32, _CH3WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3WDATR;
#[doc = "`read()` method returns [ch3wdatr::R](ch3wdatr::R) reader structure"]
impl crate::Readable for CH3WDATR {}
#[doc = "`write(|w| ..)` method takes [ch3wdatr::W](ch3wdatr::W) writer structure"]
impl crate::Writable for CH3WDATR {}
#[doc = "CHWDAT3R"]
pub mod ch3wdatr;
#[doc = "CHDATIN3R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3datinr](ch3datinr) module"]
pub type CH3DATINR = crate::Reg<u32, _CH3DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3DATINR;
#[doc = "`read()` method returns [ch3datinr::R](ch3datinr::R) reader structure"]
impl crate::Readable for CH3DATINR {}
#[doc = "`write(|w| ..)` method takes [ch3datinr::W](ch3datinr::W) writer structure"]
impl crate::Writable for CH3DATINR {}
#[doc = "CHDATIN3R"]
pub mod ch3datinr;
#[doc = "CHCFG4R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4cfgr1](ch4cfgr1) module"]
pub type CH4CFGR1 = crate::Reg<u32, _CH4CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CFGR1;
#[doc = "`read()` method returns [ch4cfgr1::R](ch4cfgr1::R) reader structure"]
impl crate::Readable for CH4CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch4cfgr1::W](ch4cfgr1::W) writer structure"]
impl crate::Writable for CH4CFGR1 {}
#[doc = "CHCFG4R1"]
pub mod ch4cfgr1;
#[doc = "CHCFG4R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4cfgr2](ch4cfgr2) module"]
pub type CH4CFGR2 = crate::Reg<u32, _CH4CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CFGR2;
#[doc = "`read()` method returns [ch4cfgr2::R](ch4cfgr2::R) reader structure"]
impl crate::Readable for CH4CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch4cfgr2::W](ch4cfgr2::W) writer structure"]
impl crate::Writable for CH4CFGR2 {}
#[doc = "CHCFG4R2"]
pub mod ch4cfgr2;
#[doc = "AWSCD4R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4awscdr](ch4awscdr) module"]
pub type CH4AWSCDR = crate::Reg<u32, _CH4AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4AWSCDR;
#[doc = "`read()` method returns [ch4awscdr::R](ch4awscdr::R) reader structure"]
impl crate::Readable for CH4AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch4awscdr::W](ch4awscdr::W) writer structure"]
impl crate::Writable for CH4AWSCDR {}
#[doc = "AWSCD4R"]
pub mod ch4awscdr;
#[doc = "CHWDAT4R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4wdatr](ch4wdatr) module"]
pub type CH4WDATR = crate::Reg<u32, _CH4WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4WDATR;
#[doc = "`read()` method returns [ch4wdatr::R](ch4wdatr::R) reader structure"]
impl crate::Readable for CH4WDATR {}
#[doc = "`write(|w| ..)` method takes [ch4wdatr::W](ch4wdatr::W) writer structure"]
impl crate::Writable for CH4WDATR {}
#[doc = "CHWDAT4R"]
pub mod ch4wdatr;
#[doc = "CHDATIN4R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4datinr](ch4datinr) module"]
pub type CH4DATINR = crate::Reg<u32, _CH4DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4DATINR;
#[doc = "`read()` method returns [ch4datinr::R](ch4datinr::R) reader structure"]
impl crate::Readable for CH4DATINR {}
#[doc = "`write(|w| ..)` method takes [ch4datinr::W](ch4datinr::W) writer structure"]
impl crate::Writable for CH4DATINR {}
#[doc = "CHDATIN4R"]
pub mod ch4datinr;
#[doc = "CHCFG5R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5cfgr1](ch5cfgr1) module"]
pub type CH5CFGR1 = crate::Reg<u32, _CH5CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CFGR1;
#[doc = "`read()` method returns [ch5cfgr1::R](ch5cfgr1::R) reader structure"]
impl crate::Readable for CH5CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch5cfgr1::W](ch5cfgr1::W) writer structure"]
impl crate::Writable for CH5CFGR1 {}
#[doc = "CHCFG5R1"]
pub mod ch5cfgr1;
#[doc = "CHCFG5R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5cfgr2](ch5cfgr2) module"]
pub type CH5CFGR2 = crate::Reg<u32, _CH5CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CFGR2;
#[doc = "`read()` method returns [ch5cfgr2::R](ch5cfgr2::R) reader structure"]
impl crate::Readable for CH5CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch5cfgr2::W](ch5cfgr2::W) writer structure"]
impl crate::Writable for CH5CFGR2 {}
#[doc = "CHCFG5R2"]
pub mod ch5cfgr2;
#[doc = "AWSCD5R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5awscdr](ch5awscdr) module"]
pub type CH5AWSCDR = crate::Reg<u32, _CH5AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5AWSCDR;
#[doc = "`read()` method returns [ch5awscdr::R](ch5awscdr::R) reader structure"]
impl crate::Readable for CH5AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch5awscdr::W](ch5awscdr::W) writer structure"]
impl crate::Writable for CH5AWSCDR {}
#[doc = "AWSCD5R"]
pub mod ch5awscdr;
#[doc = "CHWDAT5R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5wdatr](ch5wdatr) module"]
pub type CH5WDATR = crate::Reg<u32, _CH5WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5WDATR;
#[doc = "`read()` method returns [ch5wdatr::R](ch5wdatr::R) reader structure"]
impl crate::Readable for CH5WDATR {}
#[doc = "`write(|w| ..)` method takes [ch5wdatr::W](ch5wdatr::W) writer structure"]
impl crate::Writable for CH5WDATR {}
#[doc = "CHWDAT5R"]
pub mod ch5wdatr;
#[doc = "CHDATIN5R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5datinr](ch5datinr) module"]
pub type CH5DATINR = crate::Reg<u32, _CH5DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5DATINR;
#[doc = "`read()` method returns [ch5datinr::R](ch5datinr::R) reader structure"]
impl crate::Readable for CH5DATINR {}
#[doc = "`write(|w| ..)` method takes [ch5datinr::W](ch5datinr::W) writer structure"]
impl crate::Writable for CH5DATINR {}
#[doc = "CHDATIN5R"]
pub mod ch5datinr;
#[doc = "CHCFG6R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6cfgr1](ch6cfgr1) module"]
pub type CH6CFGR1 = crate::Reg<u32, _CH6CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CFGR1;
#[doc = "`read()` method returns [ch6cfgr1::R](ch6cfgr1::R) reader structure"]
impl crate::Readable for CH6CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch6cfgr1::W](ch6cfgr1::W) writer structure"]
impl crate::Writable for CH6CFGR1 {}
#[doc = "CHCFG6R1"]
pub mod ch6cfgr1;
#[doc = "CH6CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6cfgr2](ch6cfgr2) module"]
pub type CH6CFGR2 = crate::Reg<u32, _CH6CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CFGR2;
#[doc = "`read()` method returns [ch6cfgr2::R](ch6cfgr2::R) reader structure"]
impl crate::Readable for CH6CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch6cfgr2::W](ch6cfgr2::W) writer structure"]
impl crate::Writable for CH6CFGR2 {}
#[doc = "CH6CFGR2"]
pub mod ch6cfgr2;
#[doc = "AWSCD6R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6awscdr](ch6awscdr) module"]
pub type CH6AWSCDR = crate::Reg<u32, _CH6AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6AWSCDR;
#[doc = "`read()` method returns [ch6awscdr::R](ch6awscdr::R) reader structure"]
impl crate::Readable for CH6AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch6awscdr::W](ch6awscdr::W) writer structure"]
impl crate::Writable for CH6AWSCDR {}
#[doc = "AWSCD6R"]
pub mod ch6awscdr;
#[doc = "CHWDAT6R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6wdatr](ch6wdatr) module"]
pub type CH6WDATR = crate::Reg<u32, _CH6WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6WDATR;
#[doc = "`read()` method returns [ch6wdatr::R](ch6wdatr::R) reader structure"]
impl crate::Readable for CH6WDATR {}
#[doc = "`write(|w| ..)` method takes [ch6wdatr::W](ch6wdatr::W) writer structure"]
impl crate::Writable for CH6WDATR {}
#[doc = "CHWDAT6R"]
pub mod ch6wdatr;
#[doc = "CHDATIN6R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6datinr](ch6datinr) module"]
pub type CH6DATINR = crate::Reg<u32, _CH6DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6DATINR;
#[doc = "`read()` method returns [ch6datinr::R](ch6datinr::R) reader structure"]
impl crate::Readable for CH6DATINR {}
#[doc = "`write(|w| ..)` method takes [ch6datinr::W](ch6datinr::W) writer structure"]
impl crate::Writable for CH6DATINR {}
#[doc = "CHDATIN6R"]
pub mod ch6datinr;
#[doc = "CHCFG7R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7cfgr1](ch7cfgr1) module"]
pub type CH7CFGR1 = crate::Reg<u32, _CH7CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CFGR1;
#[doc = "`read()` method returns [ch7cfgr1::R](ch7cfgr1::R) reader structure"]
impl crate::Readable for CH7CFGR1 {}
#[doc = "`write(|w| ..)` method takes [ch7cfgr1::W](ch7cfgr1::W) writer structure"]
impl crate::Writable for CH7CFGR1 {}
#[doc = "CHCFG7R1"]
pub mod ch7cfgr1;
#[doc = "CHCFG7R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7cfgr2](ch7cfgr2) module"]
pub type CH7CFGR2 = crate::Reg<u32, _CH7CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CFGR2;
#[doc = "`read()` method returns [ch7cfgr2::R](ch7cfgr2::R) reader structure"]
impl crate::Readable for CH7CFGR2 {}
#[doc = "`write(|w| ..)` method takes [ch7cfgr2::W](ch7cfgr2::W) writer structure"]
impl crate::Writable for CH7CFGR2 {}
#[doc = "CHCFG7R2"]
pub mod ch7cfgr2;
#[doc = "AWSCD7R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7awscdr](ch7awscdr) module"]
pub type CH7AWSCDR = crate::Reg<u32, _CH7AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7AWSCDR;
#[doc = "`read()` method returns [ch7awscdr::R](ch7awscdr::R) reader structure"]
impl crate::Readable for CH7AWSCDR {}
#[doc = "`write(|w| ..)` method takes [ch7awscdr::W](ch7awscdr::W) writer structure"]
impl crate::Writable for CH7AWSCDR {}
#[doc = "AWSCD7R"]
pub mod ch7awscdr;
#[doc = "CHWDAT7R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7wdatr](ch7wdatr) module"]
pub type CH7WDATR = crate::Reg<u32, _CH7WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7WDATR;
#[doc = "`read()` method returns [ch7wdatr::R](ch7wdatr::R) reader structure"]
impl crate::Readable for CH7WDATR {}
#[doc = "`write(|w| ..)` method takes [ch7wdatr::W](ch7wdatr::W) writer structure"]
impl crate::Writable for CH7WDATR {}
#[doc = "CHWDAT7R"]
pub mod ch7wdatr;
#[doc = "CHDATIN7R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7datinr](ch7datinr) module"]
pub type CH7DATINR = crate::Reg<u32, _CH7DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7DATINR;
#[doc = "`read()` method returns [ch7datinr::R](ch7datinr::R) reader structure"]
impl crate::Readable for CH7DATINR {}
#[doc = "`write(|w| ..)` method takes [ch7datinr::W](ch7datinr::W) writer structure"]
impl crate::Writable for CH7DATINR {}
#[doc = "CHDATIN7R"]
pub mod ch7datinr;
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0cr1](flt0cr1) module"]
pub type FLT0CR1 = crate::Reg<u32, _FLT0CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0CR1;
#[doc = "`read()` method returns [flt0cr1::R](flt0cr1::R) reader structure"]
impl crate::Readable for FLT0CR1 {}
#[doc = "`write(|w| ..)` method takes [flt0cr1::W](flt0cr1::W) writer structure"]
impl crate::Writable for FLT0CR1 {}
#[doc = "control register 1"]
pub mod flt0cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0cr2](flt0cr2) module"]
pub type FLT0CR2 = crate::Reg<u32, _FLT0CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0CR2;
#[doc = "`read()` method returns [flt0cr2::R](flt0cr2::R) reader structure"]
impl crate::Readable for FLT0CR2 {}
#[doc = "`write(|w| ..)` method takes [flt0cr2::W](flt0cr2::W) writer structure"]
impl crate::Writable for FLT0CR2 {}
#[doc = "control register 2"]
pub mod flt0cr2;
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0isr](flt0isr) module"]
pub type FLT0ISR = crate::Reg<u32, _FLT0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0ISR;
#[doc = "`read()` method returns [flt0isr::R](flt0isr::R) reader structure"]
impl crate::Readable for FLT0ISR {}
#[doc = "interrupt and status register"]
pub mod flt0isr;
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0icr](flt0icr) module"]
pub type FLT0ICR = crate::Reg<u32, _FLT0ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0ICR;
#[doc = "`read()` method returns [flt0icr::R](flt0icr::R) reader structure"]
impl crate::Readable for FLT0ICR {}
#[doc = "`write(|w| ..)` method takes [flt0icr::W](flt0icr::W) writer structure"]
impl crate::Writable for FLT0ICR {}
#[doc = "interrupt flag clear register"]
pub mod flt0icr;
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0jchgr](flt0jchgr) module"]
pub type FLT0JCHGR = crate::Reg<u32, _FLT0JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0JCHGR;
#[doc = "`read()` method returns [flt0jchgr::R](flt0jchgr::R) reader structure"]
impl crate::Readable for FLT0JCHGR {}
#[doc = "`write(|w| ..)` method takes [flt0jchgr::W](flt0jchgr::W) writer structure"]
impl crate::Writable for FLT0JCHGR {}
#[doc = "injected channel group selection register"]
pub mod flt0jchgr;
#[doc = "filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0fcr](flt0fcr) module"]
pub type FLT0FCR = crate::Reg<u32, _FLT0FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0FCR;
#[doc = "`read()` method returns [flt0fcr::R](flt0fcr::R) reader structure"]
impl crate::Readable for FLT0FCR {}
#[doc = "`write(|w| ..)` method takes [flt0fcr::W](flt0fcr::W) writer structure"]
impl crate::Writable for FLT0FCR {}
#[doc = "filter control register"]
pub mod flt0fcr;
#[doc = "data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0jdatar](flt0jdatar) module"]
pub type FLT0JDATAR = crate::Reg<u32, _FLT0JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0JDATAR;
#[doc = "`read()` method returns [flt0jdatar::R](flt0jdatar::R) reader structure"]
impl crate::Readable for FLT0JDATAR {}
#[doc = "data register for injected group"]
pub mod flt0jdatar;
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0rdatar](flt0rdatar) module"]
pub type FLT0RDATAR = crate::Reg<u32, _FLT0RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0RDATAR;
#[doc = "`read()` method returns [flt0rdatar::R](flt0rdatar::R) reader structure"]
impl crate::Readable for FLT0RDATAR {}
#[doc = "data register for the regular channel"]
pub mod flt0rdatar;
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0awhtr](flt0awhtr) module"]
pub type FLT0AWHTR = crate::Reg<u32, _FLT0AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0AWHTR;
#[doc = "`read()` method returns [flt0awhtr::R](flt0awhtr::R) reader structure"]
impl crate::Readable for FLT0AWHTR {}
#[doc = "`write(|w| ..)` method takes [flt0awhtr::W](flt0awhtr::W) writer structure"]
impl crate::Writable for FLT0AWHTR {}
#[doc = "analog watchdog high threshold register"]
pub mod flt0awhtr;
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0awltr](flt0awltr) module"]
pub type FLT0AWLTR = crate::Reg<u32, _FLT0AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0AWLTR;
#[doc = "`read()` method returns [flt0awltr::R](flt0awltr::R) reader structure"]
impl crate::Readable for FLT0AWLTR {}
#[doc = "`write(|w| ..)` method takes [flt0awltr::W](flt0awltr::W) writer structure"]
impl crate::Writable for FLT0AWLTR {}
#[doc = "analog watchdog low threshold register"]
pub mod flt0awltr;
#[doc = "analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0awsr](flt0awsr) module"]
pub type FLT0AWSR = crate::Reg<u32, _FLT0AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0AWSR;
#[doc = "`read()` method returns [flt0awsr::R](flt0awsr::R) reader structure"]
impl crate::Readable for FLT0AWSR {}
#[doc = "analog watchdog status register"]
pub mod flt0awsr;
#[doc = "analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0awcfr](flt0awcfr) module"]
pub type FLT0AWCFR = crate::Reg<u32, _FLT0AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0AWCFR;
#[doc = "`read()` method returns [flt0awcfr::R](flt0awcfr::R) reader structure"]
impl crate::Readable for FLT0AWCFR {}
#[doc = "`write(|w| ..)` method takes [flt0awcfr::W](flt0awcfr::W) writer structure"]
impl crate::Writable for FLT0AWCFR {}
#[doc = "analog watchdog clear flag register"]
pub mod flt0awcfr;
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0exmax](flt0exmax) module"]
pub type FLT0EXMAX = crate::Reg<u32, _FLT0EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0EXMAX;
#[doc = "`read()` method returns [flt0exmax::R](flt0exmax::R) reader structure"]
impl crate::Readable for FLT0EXMAX {}
#[doc = "Extremes detector maximum register"]
pub mod flt0exmax;
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0exmin](flt0exmin) module"]
pub type FLT0EXMIN = crate::Reg<u32, _FLT0EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0EXMIN;
#[doc = "`read()` method returns [flt0exmin::R](flt0exmin::R) reader structure"]
impl crate::Readable for FLT0EXMIN {}
#[doc = "Extremes detector minimum register"]
pub mod flt0exmin;
#[doc = "conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0cnvtimr](flt0cnvtimr) module"]
pub type FLT0CNVTIMR = crate::Reg<u32, _FLT0CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT0CNVTIMR;
#[doc = "`read()` method returns [flt0cnvtimr::R](flt0cnvtimr::R) reader structure"]
impl crate::Readable for FLT0CNVTIMR {}
#[doc = "conversion timer register"]
pub mod flt0cnvtimr;
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1cr1](flt1cr1) module"]
pub type FLT1CR1 = crate::Reg<u32, _FLT1CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1CR1;
#[doc = "`read()` method returns [flt1cr1::R](flt1cr1::R) reader structure"]
impl crate::Readable for FLT1CR1 {}
#[doc = "`write(|w| ..)` method takes [flt1cr1::W](flt1cr1::W) writer structure"]
impl crate::Writable for FLT1CR1 {}
#[doc = "control register 1"]
pub mod flt1cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1cr2](flt1cr2) module"]
pub type FLT1CR2 = crate::Reg<u32, _FLT1CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1CR2;
#[doc = "`read()` method returns [flt1cr2::R](flt1cr2::R) reader structure"]
impl crate::Readable for FLT1CR2 {}
#[doc = "`write(|w| ..)` method takes [flt1cr2::W](flt1cr2::W) writer structure"]
impl crate::Writable for FLT1CR2 {}
#[doc = "control register 2"]
pub mod flt1cr2;
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1isr](flt1isr) module"]
pub type FLT1ISR = crate::Reg<u32, _FLT1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1ISR;
#[doc = "`read()` method returns [flt1isr::R](flt1isr::R) reader structure"]
impl crate::Readable for FLT1ISR {}
#[doc = "interrupt and status register"]
pub mod flt1isr;
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1icr](flt1icr) module"]
pub type FLT1ICR = crate::Reg<u32, _FLT1ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1ICR;
#[doc = "`read()` method returns [flt1icr::R](flt1icr::R) reader structure"]
impl crate::Readable for FLT1ICR {}
#[doc = "`write(|w| ..)` method takes [flt1icr::W](flt1icr::W) writer structure"]
impl crate::Writable for FLT1ICR {}
#[doc = "interrupt flag clear register"]
pub mod flt1icr;
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1jchgr](flt1jchgr) module"]
pub type FLT1JCHGR = crate::Reg<u32, _FLT1JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1JCHGR;
#[doc = "`read()` method returns [flt1jchgr::R](flt1jchgr::R) reader structure"]
impl crate::Readable for FLT1JCHGR {}
#[doc = "`write(|w| ..)` method takes [flt1jchgr::W](flt1jchgr::W) writer structure"]
impl crate::Writable for FLT1JCHGR {}
#[doc = "injected channel group selection register"]
pub mod flt1jchgr;
#[doc = "filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1fcr](flt1fcr) module"]
pub type FLT1FCR = crate::Reg<u32, _FLT1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1FCR;
#[doc = "`read()` method returns [flt1fcr::R](flt1fcr::R) reader structure"]
impl crate::Readable for FLT1FCR {}
#[doc = "`write(|w| ..)` method takes [flt1fcr::W](flt1fcr::W) writer structure"]
impl crate::Writable for FLT1FCR {}
#[doc = "filter control register"]
pub mod flt1fcr;
#[doc = "data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1jdatar](flt1jdatar) module"]
pub type FLT1JDATAR = crate::Reg<u32, _FLT1JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1JDATAR;
#[doc = "`read()` method returns [flt1jdatar::R](flt1jdatar::R) reader structure"]
impl crate::Readable for FLT1JDATAR {}
#[doc = "data register for injected group"]
pub mod flt1jdatar;
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1rdatar](flt1rdatar) module"]
pub type FLT1RDATAR = crate::Reg<u32, _FLT1RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1RDATAR;
#[doc = "`read()` method returns [flt1rdatar::R](flt1rdatar::R) reader structure"]
impl crate::Readable for FLT1RDATAR {}
#[doc = "data register for the regular channel"]
pub mod flt1rdatar;
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1awhtr](flt1awhtr) module"]
pub type FLT1AWHTR = crate::Reg<u32, _FLT1AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1AWHTR;
#[doc = "`read()` method returns [flt1awhtr::R](flt1awhtr::R) reader structure"]
impl crate::Readable for FLT1AWHTR {}
#[doc = "`write(|w| ..)` method takes [flt1awhtr::W](flt1awhtr::W) writer structure"]
impl crate::Writable for FLT1AWHTR {}
#[doc = "analog watchdog high threshold register"]
pub mod flt1awhtr;
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1awltr](flt1awltr) module"]
pub type FLT1AWLTR = crate::Reg<u32, _FLT1AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1AWLTR;
#[doc = "`read()` method returns [flt1awltr::R](flt1awltr::R) reader structure"]
impl crate::Readable for FLT1AWLTR {}
#[doc = "`write(|w| ..)` method takes [flt1awltr::W](flt1awltr::W) writer structure"]
impl crate::Writable for FLT1AWLTR {}
#[doc = "analog watchdog low threshold register"]
pub mod flt1awltr;
#[doc = "analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1awsr](flt1awsr) module"]
pub type FLT1AWSR = crate::Reg<u32, _FLT1AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1AWSR;
#[doc = "`read()` method returns [flt1awsr::R](flt1awsr::R) reader structure"]
impl crate::Readable for FLT1AWSR {}
#[doc = "analog watchdog status register"]
pub mod flt1awsr;
#[doc = "analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1awcfr](flt1awcfr) module"]
pub type FLT1AWCFR = crate::Reg<u32, _FLT1AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1AWCFR;
#[doc = "`read()` method returns [flt1awcfr::R](flt1awcfr::R) reader structure"]
impl crate::Readable for FLT1AWCFR {}
#[doc = "`write(|w| ..)` method takes [flt1awcfr::W](flt1awcfr::W) writer structure"]
impl crate::Writable for FLT1AWCFR {}
#[doc = "analog watchdog clear flag register"]
pub mod flt1awcfr;
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1exmax](flt1exmax) module"]
pub type FLT1EXMAX = crate::Reg<u32, _FLT1EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1EXMAX;
#[doc = "`read()` method returns [flt1exmax::R](flt1exmax::R) reader structure"]
impl crate::Readable for FLT1EXMAX {}
#[doc = "Extremes detector maximum register"]
pub mod flt1exmax;
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1exmin](flt1exmin) module"]
pub type FLT1EXMIN = crate::Reg<u32, _FLT1EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1EXMIN;
#[doc = "`read()` method returns [flt1exmin::R](flt1exmin::R) reader structure"]
impl crate::Readable for FLT1EXMIN {}
#[doc = "Extremes detector minimum register"]
pub mod flt1exmin;
#[doc = "conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1cnvtimr](flt1cnvtimr) module"]
pub type FLT1CNVTIMR = crate::Reg<u32, _FLT1CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT1CNVTIMR;
#[doc = "`read()` method returns [flt1cnvtimr::R](flt1cnvtimr::R) reader structure"]
impl crate::Readable for FLT1CNVTIMR {}
#[doc = "conversion timer register"]
pub mod flt1cnvtimr;
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2cr1](flt2cr1) module"]
pub type FLT2CR1 = crate::Reg<u32, _FLT2CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2CR1;
#[doc = "`read()` method returns [flt2cr1::R](flt2cr1::R) reader structure"]
impl crate::Readable for FLT2CR1 {}
#[doc = "`write(|w| ..)` method takes [flt2cr1::W](flt2cr1::W) writer structure"]
impl crate::Writable for FLT2CR1 {}
#[doc = "control register 1"]
pub mod flt2cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2cr2](flt2cr2) module"]
pub type FLT2CR2 = crate::Reg<u32, _FLT2CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2CR2;
#[doc = "`read()` method returns [flt2cr2::R](flt2cr2::R) reader structure"]
impl crate::Readable for FLT2CR2 {}
#[doc = "`write(|w| ..)` method takes [flt2cr2::W](flt2cr2::W) writer structure"]
impl crate::Writable for FLT2CR2 {}
#[doc = "control register 2"]
pub mod flt2cr2;
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2isr](flt2isr) module"]
pub type FLT2ISR = crate::Reg<u32, _FLT2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2ISR;
#[doc = "`read()` method returns [flt2isr::R](flt2isr::R) reader structure"]
impl crate::Readable for FLT2ISR {}
#[doc = "interrupt and status register"]
pub mod flt2isr;
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2icr](flt2icr) module"]
pub type FLT2ICR = crate::Reg<u32, _FLT2ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2ICR;
#[doc = "`read()` method returns [flt2icr::R](flt2icr::R) reader structure"]
impl crate::Readable for FLT2ICR {}
#[doc = "`write(|w| ..)` method takes [flt2icr::W](flt2icr::W) writer structure"]
impl crate::Writable for FLT2ICR {}
#[doc = "interrupt flag clear register"]
pub mod flt2icr;
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2jchgr](flt2jchgr) module"]
pub type FLT2JCHGR = crate::Reg<u32, _FLT2JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2JCHGR;
#[doc = "`read()` method returns [flt2jchgr::R](flt2jchgr::R) reader structure"]
impl crate::Readable for FLT2JCHGR {}
#[doc = "`write(|w| ..)` method takes [flt2jchgr::W](flt2jchgr::W) writer structure"]
impl crate::Writable for FLT2JCHGR {}
#[doc = "injected channel group selection register"]
pub mod flt2jchgr;
#[doc = "filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2fcr](flt2fcr) module"]
pub type FLT2FCR = crate::Reg<u32, _FLT2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2FCR;
#[doc = "`read()` method returns [flt2fcr::R](flt2fcr::R) reader structure"]
impl crate::Readable for FLT2FCR {}
#[doc = "`write(|w| ..)` method takes [flt2fcr::W](flt2fcr::W) writer structure"]
impl crate::Writable for FLT2FCR {}
#[doc = "filter control register"]
pub mod flt2fcr;
#[doc = "data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2jdatar](flt2jdatar) module"]
pub type FLT2JDATAR = crate::Reg<u32, _FLT2JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2JDATAR;
#[doc = "`read()` method returns [flt2jdatar::R](flt2jdatar::R) reader structure"]
impl crate::Readable for FLT2JDATAR {}
#[doc = "data register for injected group"]
pub mod flt2jdatar;
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2rdatar](flt2rdatar) module"]
pub type FLT2RDATAR = crate::Reg<u32, _FLT2RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2RDATAR;
#[doc = "`read()` method returns [flt2rdatar::R](flt2rdatar::R) reader structure"]
impl crate::Readable for FLT2RDATAR {}
#[doc = "data register for the regular channel"]
pub mod flt2rdatar;
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2awhtr](flt2awhtr) module"]
pub type FLT2AWHTR = crate::Reg<u32, _FLT2AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2AWHTR;
#[doc = "`read()` method returns [flt2awhtr::R](flt2awhtr::R) reader structure"]
impl crate::Readable for FLT2AWHTR {}
#[doc = "`write(|w| ..)` method takes [flt2awhtr::W](flt2awhtr::W) writer structure"]
impl crate::Writable for FLT2AWHTR {}
#[doc = "analog watchdog high threshold register"]
pub mod flt2awhtr;
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2awltr](flt2awltr) module"]
pub type FLT2AWLTR = crate::Reg<u32, _FLT2AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2AWLTR;
#[doc = "`read()` method returns [flt2awltr::R](flt2awltr::R) reader structure"]
impl crate::Readable for FLT2AWLTR {}
#[doc = "`write(|w| ..)` method takes [flt2awltr::W](flt2awltr::W) writer structure"]
impl crate::Writable for FLT2AWLTR {}
#[doc = "analog watchdog low threshold register"]
pub mod flt2awltr;
#[doc = "analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2awsr](flt2awsr) module"]
pub type FLT2AWSR = crate::Reg<u32, _FLT2AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2AWSR;
#[doc = "`read()` method returns [flt2awsr::R](flt2awsr::R) reader structure"]
impl crate::Readable for FLT2AWSR {}
#[doc = "analog watchdog status register"]
pub mod flt2awsr;
#[doc = "analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2awcfr](flt2awcfr) module"]
pub type FLT2AWCFR = crate::Reg<u32, _FLT2AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2AWCFR;
#[doc = "`read()` method returns [flt2awcfr::R](flt2awcfr::R) reader structure"]
impl crate::Readable for FLT2AWCFR {}
#[doc = "`write(|w| ..)` method takes [flt2awcfr::W](flt2awcfr::W) writer structure"]
impl crate::Writable for FLT2AWCFR {}
#[doc = "analog watchdog clear flag register"]
pub mod flt2awcfr;
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2exmax](flt2exmax) module"]
pub type FLT2EXMAX = crate::Reg<u32, _FLT2EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2EXMAX;
#[doc = "`read()` method returns [flt2exmax::R](flt2exmax::R) reader structure"]
impl crate::Readable for FLT2EXMAX {}
#[doc = "Extremes detector maximum register"]
pub mod flt2exmax;
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2exmin](flt2exmin) module"]
pub type FLT2EXMIN = crate::Reg<u32, _FLT2EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2EXMIN;
#[doc = "`read()` method returns [flt2exmin::R](flt2exmin::R) reader structure"]
impl crate::Readable for FLT2EXMIN {}
#[doc = "Extremes detector minimum register"]
pub mod flt2exmin;
#[doc = "conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2cnvtimr](flt2cnvtimr) module"]
pub type FLT2CNVTIMR = crate::Reg<u32, _FLT2CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT2CNVTIMR;
#[doc = "`read()` method returns [flt2cnvtimr::R](flt2cnvtimr::R) reader structure"]
impl crate::Readable for FLT2CNVTIMR {}
#[doc = "conversion timer register"]
pub mod flt2cnvtimr;
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3cr1](flt3cr1) module"]
pub type FLT3CR1 = crate::Reg<u32, _FLT3CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3CR1;
#[doc = "`read()` method returns [flt3cr1::R](flt3cr1::R) reader structure"]
impl crate::Readable for FLT3CR1 {}
#[doc = "`write(|w| ..)` method takes [flt3cr1::W](flt3cr1::W) writer structure"]
impl crate::Writable for FLT3CR1 {}
#[doc = "control register 1"]
pub mod flt3cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3cr2](flt3cr2) module"]
pub type FLT3CR2 = crate::Reg<u32, _FLT3CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3CR2;
#[doc = "`read()` method returns [flt3cr2::R](flt3cr2::R) reader structure"]
impl crate::Readable for FLT3CR2 {}
#[doc = "`write(|w| ..)` method takes [flt3cr2::W](flt3cr2::W) writer structure"]
impl crate::Writable for FLT3CR2 {}
#[doc = "control register 2"]
pub mod flt3cr2;
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3isr](flt3isr) module"]
pub type FLT3ISR = crate::Reg<u32, _FLT3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3ISR;
#[doc = "`read()` method returns [flt3isr::R](flt3isr::R) reader structure"]
impl crate::Readable for FLT3ISR {}
#[doc = "interrupt and status register"]
pub mod flt3isr;
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3icr](flt3icr) module"]
pub type FLT3ICR = crate::Reg<u32, _FLT3ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3ICR;
#[doc = "`read()` method returns [flt3icr::R](flt3icr::R) reader structure"]
impl crate::Readable for FLT3ICR {}
#[doc = "`write(|w| ..)` method takes [flt3icr::W](flt3icr::W) writer structure"]
impl crate::Writable for FLT3ICR {}
#[doc = "interrupt flag clear register"]
pub mod flt3icr;
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3jchgr](flt3jchgr) module"]
pub type FLT3JCHGR = crate::Reg<u32, _FLT3JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3JCHGR;
#[doc = "`read()` method returns [flt3jchgr::R](flt3jchgr::R) reader structure"]
impl crate::Readable for FLT3JCHGR {}
#[doc = "`write(|w| ..)` method takes [flt3jchgr::W](flt3jchgr::W) writer structure"]
impl crate::Writable for FLT3JCHGR {}
#[doc = "injected channel group selection register"]
pub mod flt3jchgr;
#[doc = "filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3fcr](flt3fcr) module"]
pub type FLT3FCR = crate::Reg<u32, _FLT3FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3FCR;
#[doc = "`read()` method returns [flt3fcr::R](flt3fcr::R) reader structure"]
impl crate::Readable for FLT3FCR {}
#[doc = "`write(|w| ..)` method takes [flt3fcr::W](flt3fcr::W) writer structure"]
impl crate::Writable for FLT3FCR {}
#[doc = "filter control register"]
pub mod flt3fcr;
#[doc = "data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3jdatar](flt3jdatar) module"]
pub type FLT3JDATAR = crate::Reg<u32, _FLT3JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3JDATAR;
#[doc = "`read()` method returns [flt3jdatar::R](flt3jdatar::R) reader structure"]
impl crate::Readable for FLT3JDATAR {}
#[doc = "data register for injected group"]
pub mod flt3jdatar;
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3rdatar](flt3rdatar) module"]
pub type FLT3RDATAR = crate::Reg<u32, _FLT3RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3RDATAR;
#[doc = "`read()` method returns [flt3rdatar::R](flt3rdatar::R) reader structure"]
impl crate::Readable for FLT3RDATAR {}
#[doc = "data register for the regular channel"]
pub mod flt3rdatar;
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3awhtr](flt3awhtr) module"]
pub type FLT3AWHTR = crate::Reg<u32, _FLT3AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3AWHTR;
#[doc = "`read()` method returns [flt3awhtr::R](flt3awhtr::R) reader structure"]
impl crate::Readable for FLT3AWHTR {}
#[doc = "`write(|w| ..)` method takes [flt3awhtr::W](flt3awhtr::W) writer structure"]
impl crate::Writable for FLT3AWHTR {}
#[doc = "analog watchdog high threshold register"]
pub mod flt3awhtr;
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3awltr](flt3awltr) module"]
pub type FLT3AWLTR = crate::Reg<u32, _FLT3AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3AWLTR;
#[doc = "`read()` method returns [flt3awltr::R](flt3awltr::R) reader structure"]
impl crate::Readable for FLT3AWLTR {}
#[doc = "`write(|w| ..)` method takes [flt3awltr::W](flt3awltr::W) writer structure"]
impl crate::Writable for FLT3AWLTR {}
#[doc = "analog watchdog low threshold register"]
pub mod flt3awltr;
#[doc = "analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3awsr](flt3awsr) module"]
pub type FLT3AWSR = crate::Reg<u32, _FLT3AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3AWSR;
#[doc = "`read()` method returns [flt3awsr::R](flt3awsr::R) reader structure"]
impl crate::Readable for FLT3AWSR {}
#[doc = "analog watchdog status register"]
pub mod flt3awsr;
#[doc = "analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3awcfr](flt3awcfr) module"]
pub type FLT3AWCFR = crate::Reg<u32, _FLT3AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3AWCFR;
#[doc = "`read()` method returns [flt3awcfr::R](flt3awcfr::R) reader structure"]
impl crate::Readable for FLT3AWCFR {}
#[doc = "`write(|w| ..)` method takes [flt3awcfr::W](flt3awcfr::W) writer structure"]
impl crate::Writable for FLT3AWCFR {}
#[doc = "analog watchdog clear flag register"]
pub mod flt3awcfr;
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3exmax](flt3exmax) module"]
pub type FLT3EXMAX = crate::Reg<u32, _FLT3EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3EXMAX;
#[doc = "`read()` method returns [flt3exmax::R](flt3exmax::R) reader structure"]
impl crate::Readable for FLT3EXMAX {}
#[doc = "Extremes detector maximum register"]
pub mod flt3exmax;
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3exmin](flt3exmin) module"]
pub type FLT3EXMIN = crate::Reg<u32, _FLT3EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3EXMIN;
#[doc = "`read()` method returns [flt3exmin::R](flt3exmin::R) reader structure"]
impl crate::Readable for FLT3EXMIN {}
#[doc = "Extremes detector minimum register"]
pub mod flt3exmin;
#[doc = "conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt3cnvtimr](flt3cnvtimr) module"]
pub type FLT3CNVTIMR = crate::Reg<u32, _FLT3CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT3CNVTIMR;
#[doc = "`read()` method returns [flt3cnvtimr::R](flt3cnvtimr::R) reader structure"]
impl crate::Readable for FLT3CNVTIMR {}
#[doc = "conversion timer register"]
pub mod flt3cnvtimr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0dlyr](ch0dlyr) module"]
pub type CH0DLYR = crate::Reg<u32, _CH0DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0DLYR;
#[doc = "`read()` method returns [ch0dlyr::R](ch0dlyr::R) reader structure"]
impl crate::Readable for CH0DLYR {}
#[doc = "`write(|w| ..)` method takes [ch0dlyr::W](ch0dlyr::W) writer structure"]
impl crate::Writable for CH0DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch0dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1dlyr](ch1dlyr) module"]
pub type CH1DLYR = crate::Reg<u32, _CH1DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1DLYR;
#[doc = "`read()` method returns [ch1dlyr::R](ch1dlyr::R) reader structure"]
impl crate::Readable for CH1DLYR {}
#[doc = "`write(|w| ..)` method takes [ch1dlyr::W](ch1dlyr::W) writer structure"]
impl crate::Writable for CH1DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch1dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2dlyr](ch2dlyr) module"]
pub type CH2DLYR = crate::Reg<u32, _CH2DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2DLYR;
#[doc = "`read()` method returns [ch2dlyr::R](ch2dlyr::R) reader structure"]
impl crate::Readable for CH2DLYR {}
#[doc = "`write(|w| ..)` method takes [ch2dlyr::W](ch2dlyr::W) writer structure"]
impl crate::Writable for CH2DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch2dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3dlyr](ch3dlyr) module"]
pub type CH3DLYR = crate::Reg<u32, _CH3DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3DLYR;
#[doc = "`read()` method returns [ch3dlyr::R](ch3dlyr::R) reader structure"]
impl crate::Readable for CH3DLYR {}
#[doc = "`write(|w| ..)` method takes [ch3dlyr::W](ch3dlyr::W) writer structure"]
impl crate::Writable for CH3DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch3dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4dlyr](ch4dlyr) module"]
pub type CH4DLYR = crate::Reg<u32, _CH4DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4DLYR;
#[doc = "`read()` method returns [ch4dlyr::R](ch4dlyr::R) reader structure"]
impl crate::Readable for CH4DLYR {}
#[doc = "`write(|w| ..)` method takes [ch4dlyr::W](ch4dlyr::W) writer structure"]
impl crate::Writable for CH4DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch4dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5dlyr](ch5dlyr) module"]
pub type CH5DLYR = crate::Reg<u32, _CH5DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5DLYR;
#[doc = "`read()` method returns [ch5dlyr::R](ch5dlyr::R) reader structure"]
impl crate::Readable for CH5DLYR {}
#[doc = "`write(|w| ..)` method takes [ch5dlyr::W](ch5dlyr::W) writer structure"]
impl crate::Writable for CH5DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch5dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6dlyr](ch6dlyr) module"]
pub type CH6DLYR = crate::Reg<u32, _CH6DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6DLYR;
#[doc = "`read()` method returns [ch6dlyr::R](ch6dlyr::R) reader structure"]
impl crate::Readable for CH6DLYR {}
#[doc = "`write(|w| ..)` method takes [ch6dlyr::W](ch6dlyr::W) writer structure"]
impl crate::Writable for CH6DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch6dlyr;
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7dlyr](ch7dlyr) module"]
pub type CH7DLYR = crate::Reg<u32, _CH7DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7DLYR;
#[doc = "`read()` method returns [ch7dlyr::R](ch7dlyr::R) reader structure"]
impl crate::Readable for CH7DLYR {}
#[doc = "`write(|w| ..)` method takes [ch7dlyr::W](ch7dlyr::W) writer structure"]
impl crate::Writable for CH7DLYR {}
#[doc = "DFSDM channel y delay register"]
pub mod ch7dlyr;
