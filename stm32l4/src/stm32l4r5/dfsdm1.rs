#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch0cfgr1: CH0CFGR1,
    ch0cfgr2: CH0CFGR2,
    ch0awscdr: CH0AWSCDR,
    ch0wdatr: CH0WDATR,
    ch0datinr: CH0DATINR,
    ch0dlyr: CH0DLYR,
    _reserved6: [u8; 0x08],
    ch1cfgr1: CH1CFGR1,
    ch1cfgr2: CH1CFGR2,
    ch1awscdr: CH1AWSCDR,
    ch1wdatr: CH1WDATR,
    ch1datinr: CH1DATINR,
    ch1dlyr: CH1DLYR,
    _reserved12: [u8; 0x08],
    ch2cfgr1: CH2CFGR1,
    ch2cfgr2: CH2CFGR2,
    ch2awscdr: CH2AWSCDR,
    ch2wdatr: CH2WDATR,
    ch2datinr: CH2DATINR,
    ch2dlyr: CH2DLYR,
    _reserved18: [u8; 0x08],
    ch3cfgr1: CH3CFGR1,
    ch3cfgr2: CH3CFGR2,
    ch3awscdr: CH3AWSCDR,
    ch3wdatr: CH3WDATR,
    ch3datinr: CH3DATINR,
    ch3dlyr: CH3DLYR,
    _reserved24: [u8; 0x08],
    ch4cfgr1: CH4CFGR1,
    ch4cfgr2: CH4CFGR2,
    ch4awscdr: CH4AWSCDR,
    ch4wdatr: CH4WDATR,
    ch4datinr: CH4DATINR,
    ch4dlyr: CH4DLYR,
    _reserved30: [u8; 0x08],
    ch5cfgr1: CH5CFGR1,
    ch5cfgr2: CH5CFGR2,
    ch5awscdr: CH5AWSCDR,
    ch5wdatr: CH5WDATR,
    ch5datinr: CH5DATINR,
    ch5dlyr: CH5DLYR,
    _reserved36: [u8; 0x08],
    ch6cfgr1: CH6CFGR1,
    ch6cfgr2: CH6CFGR2,
    ch6awscdr: CH6AWSCDR,
    ch6wdatr: CH6WDATR,
    ch6datinr: CH6DATINR,
    ch6dlyr: CH6DLYR,
    _reserved42: [u8; 0x08],
    ch7cfgr1: CH7CFGR1,
    ch7cfgr2: CH7CFGR2,
    ch7awscdr: CH7AWSCDR,
    ch7wdatr: CH7WDATR,
    ch7datinr: CH7DATINR,
    ch7dlyr: CH7DLYR,
    _reserved48: [u8; 0x08],
    dfsdm_flt0cr1: DFSDM_FLT0CR1,
    dfsdm_flt0cr2: DFSDM_FLT0CR2,
    dfsdm_flt0isr: DFSDM_FLT0ISR,
    dfsdm_flt0icr: DFSDM_FLT0ICR,
    dfsdm_flt0jchgr: DFSDM_FLT0JCHGR,
    dfsdm_flt0fcr: DFSDM_FLT0FCR,
    dfsdm_flt0jdatar: DFSDM_FLT0JDATAR,
    dfsdm_flt0rdatar: DFSDM_FLT0RDATAR,
    dfsdm_flt0awhtr: DFSDM_FLT0AWHTR,
    dfsdm_flt0awltr: DFSDM_FLT0AWLTR,
    dfsdm_flt0awsr: DFSDM_FLT0AWSR,
    dfsdm_flt0awcfr: DFSDM_FLT0AWCFR,
    dfsdm_flt0exmax: DFSDM_FLT0EXMAX,
    dfsdm_flt0exmin: DFSDM_FLT0EXMIN,
    dfsdm_flt0cnvtimr: DFSDM_FLT0CNVTIMR,
    _reserved63: [u8; 0x44],
    dfsdm_flt1cr1: DFSDM_FLT1CR1,
    dfsdm_flt1cr2: DFSDM_FLT1CR2,
    dfsdm_flt1isr: DFSDM_FLT1ISR,
    dfsdm_flt1icr: DFSDM_FLT1ICR,
    dfsdm_flt1chgr: DFSDM_FLT1CHGR,
    dfsdm_flt1fcr: DFSDM_FLT1FCR,
    dfsdm_flt1jdatar: DFSDM_FLT1JDATAR,
    dfsdm_flt1rdatar: DFSDM_FLT1RDATAR,
    dfsdm_flt1awhtr: DFSDM_FLT1AWHTR,
    dfsdm_flt1awltr: DFSDM_FLT1AWLTR,
    dfsdm_flt1awsr: DFSDM_FLT1AWSR,
    dfsdm_flt1awcfr: DFSDM_FLT1AWCFR,
    dfsdm_flt1exmax: DFSDM_FLT1EXMAX,
    dfsdm_flt1exmin: DFSDM_FLT1EXMIN,
    dfsdm_flt1cnvtimr: DFSDM_FLT1CNVTIMR,
    _reserved78: [u8; 0x44],
    dfsdm_flt2cr1: DFSDM_FLT2CR1,
    dfsdm_flt2cr2: DFSDM_FLT2CR2,
    dfsdm_flt2isr: DFSDM_FLT2ISR,
    dfsdm_flt2icr: DFSDM_FLT2ICR,
    dfsdm_flt2jchgr: DFSDM_FLT2JCHGR,
    dfsdm_flt2fcr: DFSDM_FLT2FCR,
    dfsdm_flt2jdatar: DFSDM_FLT2JDATAR,
    dfsdm_flt2rdatar: DFSDM_FLT2RDATAR,
    dfsdm_flt2awhtr: DFSDM_FLT2AWHTR,
    dfsdm_flt2awltr: DFSDM_FLT2AWLTR,
    dfsdm_flt2awsr: DFSDM_FLT2AWSR,
    dfsdm_flt2awcfr: DFSDM_FLT2AWCFR,
    dfsdm_flt2exmax: DFSDM_FLT2EXMAX,
    dfsdm_flt2exmin: DFSDM_FLT2EXMIN,
    dfsdm_flt2cnvtimr: DFSDM_FLT2CNVTIMR,
    _reserved93: [u8; 0x44],
    dfsdm_flt3cr1: DFSDM_FLT3CR1,
    dfsdm_flt3cr2: DFSDM_FLT3CR2,
    dfsdm_flt3isr: DFSDM_FLT3ISR,
    dfsdm_flt3icr: DFSDM_FLT3ICR,
    dfsdm_flt3jchgr: DFSDM_FLT3JCHGR,
    dfsdm_flt3fcr: DFSDM_FLT3FCR,
    dfsdm_flt3jdatar: DFSDM_FLT3JDATAR,
    dfsdm_flt3rdatar: DFSDM_FLT3RDATAR,
    dfsdm_flt3awhtr: DFSDM_FLT3AWHTR,
    dfsdm_flt3awltr: DFSDM_FLT3AWLTR,
    dfsdm_flt3awsr: DFSDM_FLT3AWSR,
    dfsdm_flt3awcfr: DFSDM_FLT3AWCFR,
    dfsdm_flt3exmax: DFSDM_FLT3EXMAX,
    dfsdm_flt3exmin: DFSDM_FLT3EXMIN,
    dfsdm_flt3cnvtimr: DFSDM_FLT3CNVTIMR,
}
impl RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    #[inline(always)]
    pub const fn ch0cfgr1(&self) -> &CH0CFGR1 {
        &self.ch0cfgr1
    }
    #[doc = "0x04 - channel configuration y register"]
    #[inline(always)]
    pub const fn ch0cfgr2(&self) -> &CH0CFGR2 {
        &self.ch0cfgr2
    }
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn ch0awscdr(&self) -> &CH0AWSCDR {
        &self.ch0awscdr
    }
    #[doc = "0x0c - channel watchdog filter data register"]
    #[inline(always)]
    pub const fn ch0wdatr(&self) -> &CH0WDATR {
        &self.ch0wdatr
    }
    #[doc = "0x10 - channel data input register"]
    #[inline(always)]
    pub const fn ch0datinr(&self) -> &CH0DATINR {
        &self.ch0datinr
    }
    #[doc = "0x14 - channel y delay register"]
    #[inline(always)]
    pub const fn ch0dlyr(&self) -> &CH0DLYR {
        &self.ch0dlyr
    }
    #[doc = "0x20 - CH1CFGR1"]
    #[inline(always)]
    pub const fn ch1cfgr1(&self) -> &CH1CFGR1 {
        &self.ch1cfgr1
    }
    #[doc = "0x24 - CH1CFGR2"]
    #[inline(always)]
    pub const fn ch1cfgr2(&self) -> &CH1CFGR2 {
        &self.ch1cfgr2
    }
    #[doc = "0x28 - CH1AWSCDR"]
    #[inline(always)]
    pub const fn ch1awscdr(&self) -> &CH1AWSCDR {
        &self.ch1awscdr
    }
    #[doc = "0x2c - CH1WDATR"]
    #[inline(always)]
    pub const fn ch1wdatr(&self) -> &CH1WDATR {
        &self.ch1wdatr
    }
    #[doc = "0x30 - CH1DATINR"]
    #[inline(always)]
    pub const fn ch1datinr(&self) -> &CH1DATINR {
        &self.ch1datinr
    }
    #[doc = "0x34 - channel y delay register"]
    #[inline(always)]
    pub const fn ch1dlyr(&self) -> &CH1DLYR {
        &self.ch1dlyr
    }
    #[doc = "0x40 - CH2CFGR1"]
    #[inline(always)]
    pub const fn ch2cfgr1(&self) -> &CH2CFGR1 {
        &self.ch2cfgr1
    }
    #[doc = "0x44 - CH2CFGR2"]
    #[inline(always)]
    pub const fn ch2cfgr2(&self) -> &CH2CFGR2 {
        &self.ch2cfgr2
    }
    #[doc = "0x48 - CH2AWSCDR"]
    #[inline(always)]
    pub const fn ch2awscdr(&self) -> &CH2AWSCDR {
        &self.ch2awscdr
    }
    #[doc = "0x4c - CH2WDATR"]
    #[inline(always)]
    pub const fn ch2wdatr(&self) -> &CH2WDATR {
        &self.ch2wdatr
    }
    #[doc = "0x50 - CH2DATINR"]
    #[inline(always)]
    pub const fn ch2datinr(&self) -> &CH2DATINR {
        &self.ch2datinr
    }
    #[doc = "0x54 - channel y delay register"]
    #[inline(always)]
    pub const fn ch2dlyr(&self) -> &CH2DLYR {
        &self.ch2dlyr
    }
    #[doc = "0x60 - CH3CFGR1"]
    #[inline(always)]
    pub const fn ch3cfgr1(&self) -> &CH3CFGR1 {
        &self.ch3cfgr1
    }
    #[doc = "0x64 - CH3CFGR2"]
    #[inline(always)]
    pub const fn ch3cfgr2(&self) -> &CH3CFGR2 {
        &self.ch3cfgr2
    }
    #[doc = "0x68 - CH3AWSCDR"]
    #[inline(always)]
    pub const fn ch3awscdr(&self) -> &CH3AWSCDR {
        &self.ch3awscdr
    }
    #[doc = "0x6c - CH3WDATR"]
    #[inline(always)]
    pub const fn ch3wdatr(&self) -> &CH3WDATR {
        &self.ch3wdatr
    }
    #[doc = "0x70 - CH3DATINR"]
    #[inline(always)]
    pub const fn ch3datinr(&self) -> &CH3DATINR {
        &self.ch3datinr
    }
    #[doc = "0x74 - channel y delay register"]
    #[inline(always)]
    pub const fn ch3dlyr(&self) -> &CH3DLYR {
        &self.ch3dlyr
    }
    #[doc = "0x80 - CH4CFGR1"]
    #[inline(always)]
    pub const fn ch4cfgr1(&self) -> &CH4CFGR1 {
        &self.ch4cfgr1
    }
    #[doc = "0x84 - CH4CFGR2"]
    #[inline(always)]
    pub const fn ch4cfgr2(&self) -> &CH4CFGR2 {
        &self.ch4cfgr2
    }
    #[doc = "0x88 - CH4AWSCDR"]
    #[inline(always)]
    pub const fn ch4awscdr(&self) -> &CH4AWSCDR {
        &self.ch4awscdr
    }
    #[doc = "0x8c - CH4WDATR"]
    #[inline(always)]
    pub const fn ch4wdatr(&self) -> &CH4WDATR {
        &self.ch4wdatr
    }
    #[doc = "0x90 - CH4DATINR"]
    #[inline(always)]
    pub const fn ch4datinr(&self) -> &CH4DATINR {
        &self.ch4datinr
    }
    #[doc = "0x94 - channel y delay register"]
    #[inline(always)]
    pub const fn ch4dlyr(&self) -> &CH4DLYR {
        &self.ch4dlyr
    }
    #[doc = "0xa0 - CH5CFGR1"]
    #[inline(always)]
    pub const fn ch5cfgr1(&self) -> &CH5CFGR1 {
        &self.ch5cfgr1
    }
    #[doc = "0xa4 - CH5CFGR2"]
    #[inline(always)]
    pub const fn ch5cfgr2(&self) -> &CH5CFGR2 {
        &self.ch5cfgr2
    }
    #[doc = "0xa8 - CH5AWSCDR"]
    #[inline(always)]
    pub const fn ch5awscdr(&self) -> &CH5AWSCDR {
        &self.ch5awscdr
    }
    #[doc = "0xac - CH5WDATR"]
    #[inline(always)]
    pub const fn ch5wdatr(&self) -> &CH5WDATR {
        &self.ch5wdatr
    }
    #[doc = "0xb0 - CH5DATINR"]
    #[inline(always)]
    pub const fn ch5datinr(&self) -> &CH5DATINR {
        &self.ch5datinr
    }
    #[doc = "0xb4 - channel y delay register"]
    #[inline(always)]
    pub const fn ch5dlyr(&self) -> &CH5DLYR {
        &self.ch5dlyr
    }
    #[doc = "0xc0 - CH6CFGR1"]
    #[inline(always)]
    pub const fn ch6cfgr1(&self) -> &CH6CFGR1 {
        &self.ch6cfgr1
    }
    #[doc = "0xc4 - CH6CFGR2"]
    #[inline(always)]
    pub const fn ch6cfgr2(&self) -> &CH6CFGR2 {
        &self.ch6cfgr2
    }
    #[doc = "0xc8 - CH6AWSCDR"]
    #[inline(always)]
    pub const fn ch6awscdr(&self) -> &CH6AWSCDR {
        &self.ch6awscdr
    }
    #[doc = "0xcc - CH6WDATR"]
    #[inline(always)]
    pub const fn ch6wdatr(&self) -> &CH6WDATR {
        &self.ch6wdatr
    }
    #[doc = "0xd0 - CH6DATINR"]
    #[inline(always)]
    pub const fn ch6datinr(&self) -> &CH6DATINR {
        &self.ch6datinr
    }
    #[doc = "0xd4 - channel y delay register"]
    #[inline(always)]
    pub const fn ch6dlyr(&self) -> &CH6DLYR {
        &self.ch6dlyr
    }
    #[doc = "0xe0 - CH7CFGR1"]
    #[inline(always)]
    pub const fn ch7cfgr1(&self) -> &CH7CFGR1 {
        &self.ch7cfgr1
    }
    #[doc = "0xe4 - CH7CFGR2"]
    #[inline(always)]
    pub const fn ch7cfgr2(&self) -> &CH7CFGR2 {
        &self.ch7cfgr2
    }
    #[doc = "0xe8 - CH7AWSCDR"]
    #[inline(always)]
    pub const fn ch7awscdr(&self) -> &CH7AWSCDR {
        &self.ch7awscdr
    }
    #[doc = "0xec - CH7WDATR"]
    #[inline(always)]
    pub const fn ch7wdatr(&self) -> &CH7WDATR {
        &self.ch7wdatr
    }
    #[doc = "0xf0 - CH7DATINR"]
    #[inline(always)]
    pub const fn ch7datinr(&self) -> &CH7DATINR {
        &self.ch7datinr
    }
    #[doc = "0xf4 - channel y delay register"]
    #[inline(always)]
    pub const fn ch7dlyr(&self) -> &CH7DLYR {
        &self.ch7dlyr
    }
    #[doc = "0x100 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt0cr1(&self) -> &DFSDM_FLT0CR1 {
        &self.dfsdm_flt0cr1
    }
    #[doc = "0x104 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt0cr2(&self) -> &DFSDM_FLT0CR2 {
        &self.dfsdm_flt0cr2
    }
    #[doc = "0x108 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt0isr(&self) -> &DFSDM_FLT0ISR {
        &self.dfsdm_flt0isr
    }
    #[doc = "0x10c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt0icr(&self) -> &DFSDM_FLT0ICR {
        &self.dfsdm_flt0icr
    }
    #[doc = "0x110 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt0jchgr(&self) -> &DFSDM_FLT0JCHGR {
        &self.dfsdm_flt0jchgr
    }
    #[doc = "0x114 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt0fcr(&self) -> &DFSDM_FLT0FCR {
        &self.dfsdm_flt0fcr
    }
    #[doc = "0x118 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt0jdatar(&self) -> &DFSDM_FLT0JDATAR {
        &self.dfsdm_flt0jdatar
    }
    #[doc = "0x11c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt0rdatar(&self) -> &DFSDM_FLT0RDATAR {
        &self.dfsdm_flt0rdatar
    }
    #[doc = "0x120 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awhtr(&self) -> &DFSDM_FLT0AWHTR {
        &self.dfsdm_flt0awhtr
    }
    #[doc = "0x124 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awltr(&self) -> &DFSDM_FLT0AWLTR {
        &self.dfsdm_flt0awltr
    }
    #[doc = "0x128 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awsr(&self) -> &DFSDM_FLT0AWSR {
        &self.dfsdm_flt0awsr
    }
    #[doc = "0x12c - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt0awcfr(&self) -> &DFSDM_FLT0AWCFR {
        &self.dfsdm_flt0awcfr
    }
    #[doc = "0x130 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt0exmax(&self) -> &DFSDM_FLT0EXMAX {
        &self.dfsdm_flt0exmax
    }
    #[doc = "0x134 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt0exmin(&self) -> &DFSDM_FLT0EXMIN {
        &self.dfsdm_flt0exmin
    }
    #[doc = "0x138 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt0cnvtimr(&self) -> &DFSDM_FLT0CNVTIMR {
        &self.dfsdm_flt0cnvtimr
    }
    #[doc = "0x180 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt1cr1(&self) -> &DFSDM_FLT1CR1 {
        &self.dfsdm_flt1cr1
    }
    #[doc = "0x184 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt1cr2(&self) -> &DFSDM_FLT1CR2 {
        &self.dfsdm_flt1cr2
    }
    #[doc = "0x188 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt1isr(&self) -> &DFSDM_FLT1ISR {
        &self.dfsdm_flt1isr
    }
    #[doc = "0x18c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt1icr(&self) -> &DFSDM_FLT1ICR {
        &self.dfsdm_flt1icr
    }
    #[doc = "0x190 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt1chgr(&self) -> &DFSDM_FLT1CHGR {
        &self.dfsdm_flt1chgr
    }
    #[doc = "0x194 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt1fcr(&self) -> &DFSDM_FLT1FCR {
        &self.dfsdm_flt1fcr
    }
    #[doc = "0x198 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt1jdatar(&self) -> &DFSDM_FLT1JDATAR {
        &self.dfsdm_flt1jdatar
    }
    #[doc = "0x19c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt1rdatar(&self) -> &DFSDM_FLT1RDATAR {
        &self.dfsdm_flt1rdatar
    }
    #[doc = "0x1a0 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awhtr(&self) -> &DFSDM_FLT1AWHTR {
        &self.dfsdm_flt1awhtr
    }
    #[doc = "0x1a4 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awltr(&self) -> &DFSDM_FLT1AWLTR {
        &self.dfsdm_flt1awltr
    }
    #[doc = "0x1a8 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awsr(&self) -> &DFSDM_FLT1AWSR {
        &self.dfsdm_flt1awsr
    }
    #[doc = "0x1ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt1awcfr(&self) -> &DFSDM_FLT1AWCFR {
        &self.dfsdm_flt1awcfr
    }
    #[doc = "0x1b0 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt1exmax(&self) -> &DFSDM_FLT1EXMAX {
        &self.dfsdm_flt1exmax
    }
    #[doc = "0x1b4 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt1exmin(&self) -> &DFSDM_FLT1EXMIN {
        &self.dfsdm_flt1exmin
    }
    #[doc = "0x1b8 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt1cnvtimr(&self) -> &DFSDM_FLT1CNVTIMR {
        &self.dfsdm_flt1cnvtimr
    }
    #[doc = "0x200 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt2cr1(&self) -> &DFSDM_FLT2CR1 {
        &self.dfsdm_flt2cr1
    }
    #[doc = "0x204 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt2cr2(&self) -> &DFSDM_FLT2CR2 {
        &self.dfsdm_flt2cr2
    }
    #[doc = "0x208 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt2isr(&self) -> &DFSDM_FLT2ISR {
        &self.dfsdm_flt2isr
    }
    #[doc = "0x20c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt2icr(&self) -> &DFSDM_FLT2ICR {
        &self.dfsdm_flt2icr
    }
    #[doc = "0x210 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt2jchgr(&self) -> &DFSDM_FLT2JCHGR {
        &self.dfsdm_flt2jchgr
    }
    #[doc = "0x214 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt2fcr(&self) -> &DFSDM_FLT2FCR {
        &self.dfsdm_flt2fcr
    }
    #[doc = "0x218 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt2jdatar(&self) -> &DFSDM_FLT2JDATAR {
        &self.dfsdm_flt2jdatar
    }
    #[doc = "0x21c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt2rdatar(&self) -> &DFSDM_FLT2RDATAR {
        &self.dfsdm_flt2rdatar
    }
    #[doc = "0x220 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awhtr(&self) -> &DFSDM_FLT2AWHTR {
        &self.dfsdm_flt2awhtr
    }
    #[doc = "0x224 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awltr(&self) -> &DFSDM_FLT2AWLTR {
        &self.dfsdm_flt2awltr
    }
    #[doc = "0x228 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awsr(&self) -> &DFSDM_FLT2AWSR {
        &self.dfsdm_flt2awsr
    }
    #[doc = "0x22c - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt2awcfr(&self) -> &DFSDM_FLT2AWCFR {
        &self.dfsdm_flt2awcfr
    }
    #[doc = "0x230 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt2exmax(&self) -> &DFSDM_FLT2EXMAX {
        &self.dfsdm_flt2exmax
    }
    #[doc = "0x234 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt2exmin(&self) -> &DFSDM_FLT2EXMIN {
        &self.dfsdm_flt2exmin
    }
    #[doc = "0x238 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt2cnvtimr(&self) -> &DFSDM_FLT2CNVTIMR {
        &self.dfsdm_flt2cnvtimr
    }
    #[doc = "0x280 - control register 1"]
    #[inline(always)]
    pub const fn dfsdm_flt3cr1(&self) -> &DFSDM_FLT3CR1 {
        &self.dfsdm_flt3cr1
    }
    #[doc = "0x284 - control register 2"]
    #[inline(always)]
    pub const fn dfsdm_flt3cr2(&self) -> &DFSDM_FLT3CR2 {
        &self.dfsdm_flt3cr2
    }
    #[doc = "0x288 - interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm_flt3isr(&self) -> &DFSDM_FLT3ISR {
        &self.dfsdm_flt3isr
    }
    #[doc = "0x28c - interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm_flt3icr(&self) -> &DFSDM_FLT3ICR {
        &self.dfsdm_flt3icr
    }
    #[doc = "0x290 - injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm_flt3jchgr(&self) -> &DFSDM_FLT3JCHGR {
        &self.dfsdm_flt3jchgr
    }
    #[doc = "0x294 - filter control register"]
    #[inline(always)]
    pub const fn dfsdm_flt3fcr(&self) -> &DFSDM_FLT3FCR {
        &self.dfsdm_flt3fcr
    }
    #[doc = "0x298 - data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm_flt3jdatar(&self) -> &DFSDM_FLT3JDATAR {
        &self.dfsdm_flt3jdatar
    }
    #[doc = "0x29c - data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm_flt3rdatar(&self) -> &DFSDM_FLT3RDATAR {
        &self.dfsdm_flt3rdatar
    }
    #[doc = "0x2a0 - analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awhtr(&self) -> &DFSDM_FLT3AWHTR {
        &self.dfsdm_flt3awhtr
    }
    #[doc = "0x2a4 - analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awltr(&self) -> &DFSDM_FLT3AWLTR {
        &self.dfsdm_flt3awltr
    }
    #[doc = "0x2a8 - analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awsr(&self) -> &DFSDM_FLT3AWSR {
        &self.dfsdm_flt3awsr
    }
    #[doc = "0x2ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm_flt3awcfr(&self) -> &DFSDM_FLT3AWCFR {
        &self.dfsdm_flt3awcfr
    }
    #[doc = "0x2b0 - Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm_flt3exmax(&self) -> &DFSDM_FLT3EXMAX {
        &self.dfsdm_flt3exmax
    }
    #[doc = "0x2b4 - Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm_flt3exmin(&self) -> &DFSDM_FLT3EXMIN {
        &self.dfsdm_flt3exmin
    }
    #[doc = "0x2b8 - conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm_flt3cnvtimr(&self) -> &DFSDM_FLT3CNVTIMR {
        &self.dfsdm_flt3cnvtimr
    }
}
#[doc = "CH0CFGR1 (rw) register accessor: channel configuration y register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cfgr1`]
module"]
pub type CH0CFGR1 = crate::Reg<ch0cfgr1::CH0CFGR1rs>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr1;
#[doc = "CH0CFGR2 (rw) register accessor: channel configuration y register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cfgr2`]
module"]
pub type CH0CFGR2 = crate::Reg<ch0cfgr2::CH0CFGR2rs>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr2;
#[doc = "CH0AWSCDR (rw) register accessor: analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0awscdr`]
module"]
pub type CH0AWSCDR = crate::Reg<ch0awscdr::CH0AWSCDRrs>;
#[doc = "analog watchdog and short-circuit detector register"]
pub mod ch0awscdr;
#[doc = "CH0WDATR (rw) register accessor: channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0wdatr`]
module"]
pub type CH0WDATR = crate::Reg<ch0wdatr::CH0WDATRrs>;
#[doc = "channel watchdog filter data register"]
pub mod ch0wdatr;
#[doc = "CH0DATINR (rw) register accessor: channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0datinr`]
module"]
pub type CH0DATINR = crate::Reg<ch0datinr::CH0DATINRrs>;
#[doc = "channel data input register"]
pub mod ch0datinr;
#[doc = "CH0DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0dlyr`]
module"]
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch0dlyr;
#[doc = "CH1CFGR1 (rw) register accessor: CH1CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cfgr1`]
module"]
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1rs>;
#[doc = "CH1CFGR1"]
pub mod ch1cfgr1;
#[doc = "CH1CFGR2 (rw) register accessor: CH1CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cfgr2`]
module"]
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2rs>;
#[doc = "CH1CFGR2"]
pub mod ch1cfgr2;
#[doc = "CH1AWSCDR (rw) register accessor: CH1AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1awscdr`]
module"]
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDRrs>;
#[doc = "CH1AWSCDR"]
pub mod ch1awscdr;
#[doc = "CH1WDATR (rw) register accessor: CH1WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1wdatr`]
module"]
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATRrs>;
#[doc = "CH1WDATR"]
pub mod ch1wdatr;
#[doc = "CH1DATINR (rw) register accessor: CH1DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1datinr`]
module"]
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINRrs>;
#[doc = "CH1DATINR"]
pub mod ch1datinr;
#[doc = "CH1DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1dlyr`]
module"]
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch1dlyr;
#[doc = "CH2CFGR1 (rw) register accessor: CH2CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cfgr1`]
module"]
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1rs>;
#[doc = "CH2CFGR1"]
pub mod ch2cfgr1;
#[doc = "CH2CFGR2 (rw) register accessor: CH2CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cfgr2`]
module"]
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2rs>;
#[doc = "CH2CFGR2"]
pub mod ch2cfgr2;
#[doc = "CH2AWSCDR (rw) register accessor: CH2AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2awscdr`]
module"]
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDRrs>;
#[doc = "CH2AWSCDR"]
pub mod ch2awscdr;
#[doc = "CH2WDATR (rw) register accessor: CH2WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2wdatr`]
module"]
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATRrs>;
#[doc = "CH2WDATR"]
pub mod ch2wdatr;
#[doc = "CH2DATINR (rw) register accessor: CH2DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2datinr`]
module"]
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINRrs>;
#[doc = "CH2DATINR"]
pub mod ch2datinr;
#[doc = "CH2DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2dlyr`]
module"]
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch2dlyr;
#[doc = "CH3CFGR1 (rw) register accessor: CH3CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cfgr1`]
module"]
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1rs>;
#[doc = "CH3CFGR1"]
pub mod ch3cfgr1;
#[doc = "CH3CFGR2 (rw) register accessor: CH3CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cfgr2`]
module"]
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2rs>;
#[doc = "CH3CFGR2"]
pub mod ch3cfgr2;
#[doc = "CH3AWSCDR (rw) register accessor: CH3AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3awscdr`]
module"]
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDRrs>;
#[doc = "CH3AWSCDR"]
pub mod ch3awscdr;
#[doc = "CH3WDATR (rw) register accessor: CH3WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3wdatr`]
module"]
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATRrs>;
#[doc = "CH3WDATR"]
pub mod ch3wdatr;
#[doc = "CH3DATINR (rw) register accessor: CH3DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3datinr`]
module"]
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINRrs>;
#[doc = "CH3DATINR"]
pub mod ch3datinr;
#[doc = "CH3DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3dlyr`]
module"]
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch3dlyr;
#[doc = "CH4CFGR1 (rw) register accessor: CH4CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cfgr1`]
module"]
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1rs>;
#[doc = "CH4CFGR1"]
pub mod ch4cfgr1;
#[doc = "CH4CFGR2 (rw) register accessor: CH4CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cfgr2`]
module"]
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2rs>;
#[doc = "CH4CFGR2"]
pub mod ch4cfgr2;
#[doc = "CH4AWSCDR (rw) register accessor: CH4AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4awscdr`]
module"]
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDRrs>;
#[doc = "CH4AWSCDR"]
pub mod ch4awscdr;
#[doc = "CH4WDATR (rw) register accessor: CH4WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4wdatr`]
module"]
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATRrs>;
#[doc = "CH4WDATR"]
pub mod ch4wdatr;
#[doc = "CH4DATINR (rw) register accessor: CH4DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4datinr`]
module"]
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINRrs>;
#[doc = "CH4DATINR"]
pub mod ch4datinr;
#[doc = "CH4DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4dlyr`]
module"]
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch4dlyr;
#[doc = "CH5CFGR1 (rw) register accessor: CH5CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cfgr1`]
module"]
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1rs>;
#[doc = "CH5CFGR1"]
pub mod ch5cfgr1;
#[doc = "CH5CFGR2 (rw) register accessor: CH5CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cfgr2`]
module"]
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2rs>;
#[doc = "CH5CFGR2"]
pub mod ch5cfgr2;
#[doc = "CH5AWSCDR (rw) register accessor: CH5AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5awscdr`]
module"]
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDRrs>;
#[doc = "CH5AWSCDR"]
pub mod ch5awscdr;
#[doc = "CH5WDATR (rw) register accessor: CH5WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5wdatr`]
module"]
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATRrs>;
#[doc = "CH5WDATR"]
pub mod ch5wdatr;
#[doc = "CH5DATINR (rw) register accessor: CH5DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5datinr`]
module"]
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINRrs>;
#[doc = "CH5DATINR"]
pub mod ch5datinr;
#[doc = "CH5DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5dlyr`]
module"]
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch5dlyr;
#[doc = "CH6CFGR1 (rw) register accessor: CH6CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cfgr1`]
module"]
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1rs>;
#[doc = "CH6CFGR1"]
pub mod ch6cfgr1;
#[doc = "CH6CFGR2 (rw) register accessor: CH6CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cfgr2`]
module"]
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2rs>;
#[doc = "CH6CFGR2"]
pub mod ch6cfgr2;
#[doc = "CH6AWSCDR (rw) register accessor: CH6AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6awscdr`]
module"]
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDRrs>;
#[doc = "CH6AWSCDR"]
pub mod ch6awscdr;
#[doc = "CH6WDATR (rw) register accessor: CH6WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6wdatr`]
module"]
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATRrs>;
#[doc = "CH6WDATR"]
pub mod ch6wdatr;
#[doc = "CH6DATINR (rw) register accessor: CH6DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6datinr`]
module"]
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINRrs>;
#[doc = "CH6DATINR"]
pub mod ch6datinr;
#[doc = "CH6DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6dlyr`]
module"]
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch6dlyr;
#[doc = "CH7CFGR1 (rw) register accessor: CH7CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cfgr1`]
module"]
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1rs>;
#[doc = "CH7CFGR1"]
pub mod ch7cfgr1;
#[doc = "CH7CFGR2 (rw) register accessor: CH7CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cfgr2`]
module"]
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2rs>;
#[doc = "CH7CFGR2"]
pub mod ch7cfgr2;
#[doc = "CH7AWSCDR (rw) register accessor: CH7AWSCDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7awscdr`]
module"]
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDRrs>;
#[doc = "CH7AWSCDR"]
pub mod ch7awscdr;
#[doc = "CH7DLYR (rw) register accessor: channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7dlyr`]
module"]
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYRrs>;
#[doc = "channel y delay register"]
pub mod ch7dlyr;
#[doc = "DFSDM_FLT0CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0cr1`]
module"]
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1rs>;
#[doc = "control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM_FLT0CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0cr2`]
module"]
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2rs>;
#[doc = "control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM_FLT0ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0isr`]
module"]
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISRrs>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM_FLT0ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0icr`]
module"]
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICRrs>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM_FLT0JCHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0jchgr`]
module"]
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGRrs>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM_FLT0FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0fcr`]
module"]
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCRrs>;
#[doc = "filter control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM_FLT0JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0jdatar`]
module"]
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATARrs>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM_FLT0RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0rdatar`]
module"]
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATARrs>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM_FLT0AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awhtr`]
module"]
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTRrs>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM_FLT0AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awltr`]
module"]
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTRrs>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM_FLT0AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awsr`]
module"]
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSRrs>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM_FLT0AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0awcfr`]
module"]
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFRrs>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM_FLT0EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0exmax`]
module"]
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAXrs>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM_FLT0EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0exmin`]
module"]
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMINrs>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM_FLT0CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt0cnvtimr`]
module"]
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMRrs>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM_FLT1CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1cr1`]
module"]
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1rs>;
#[doc = "control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM_FLT1CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1cr2`]
module"]
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2rs>;
#[doc = "control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM_FLT1ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1isr`]
module"]
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISRrs>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM_FLT1ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1icr`]
module"]
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICRrs>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM_FLT1CHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1chgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1chgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1chgr`]
module"]
pub type DFSDM_FLT1CHGR = crate::Reg<dfsdm_flt1chgr::DFSDM_FLT1CHGRrs>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt1chgr;
#[doc = "DFSDM_FLT1FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1fcr`]
module"]
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCRrs>;
#[doc = "filter control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM_FLT1JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1jdatar`]
module"]
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATARrs>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM_FLT1RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1rdatar`]
module"]
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATARrs>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM_FLT1AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awhtr`]
module"]
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTRrs>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM_FLT1AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awltr`]
module"]
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTRrs>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM_FLT1AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awsr`]
module"]
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSRrs>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM_FLT1AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt1awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1awcfr`]
module"]
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFRrs>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM_FLT1EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1exmax`]
module"]
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAXrs>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM_FLT1EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1exmin`]
module"]
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMINrs>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM_FLT1CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt1cnvtimr`]
module"]
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMRrs>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM_FLT2CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2cr1`]
module"]
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1rs>;
#[doc = "control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM_FLT2CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2cr2`]
module"]
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2rs>;
#[doc = "control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM_FLT2ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2isr`]
module"]
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISRrs>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM_FLT2ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2icr`]
module"]
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICRrs>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM_FLT2JCHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2jchgr`]
module"]
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGRrs>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM_FLT2FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2fcr`]
module"]
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCRrs>;
#[doc = "filter control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM_FLT2JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2jdatar`]
module"]
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATARrs>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM_FLT2RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2rdatar`]
module"]
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATARrs>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM_FLT2AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awhtr`]
module"]
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTRrs>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM_FLT2AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awltr`]
module"]
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTRrs>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM_FLT2AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awsr`]
module"]
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSRrs>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM_FLT2AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt2awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2awcfr`]
module"]
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFRrs>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM_FLT2EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2exmax`]
module"]
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAXrs>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM_FLT2EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2exmin`]
module"]
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMINrs>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM_FLT2CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt2cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt2cnvtimr`]
module"]
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMRrs>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM_FLT3CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3cr1`]
module"]
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1rs>;
#[doc = "control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM_FLT3CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3cr2`]
module"]
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2rs>;
#[doc = "control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM_FLT3ISR (r) register accessor: interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3isr`]
module"]
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISRrs>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM_FLT3ICR (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3icr`]
module"]
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICRrs>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM_FLT3JCHGR (rw) register accessor: injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3jchgr`]
module"]
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGRrs>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM_FLT3FCR (rw) register accessor: filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3fcr`]
module"]
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCRrs>;
#[doc = "filter control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM_FLT3JDATAR (r) register accessor: data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3jdatar`]
module"]
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATARrs>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM_FLT3RDATAR (r) register accessor: data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3rdatar`]
module"]
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATARrs>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM_FLT3AWHTR (rw) register accessor: analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awhtr`]
module"]
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTRrs>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM_FLT3AWLTR (rw) register accessor: analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awltr`]
module"]
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTRrs>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM_FLT3AWSR (r) register accessor: analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awsr`]
module"]
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSRrs>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM_FLT3AWCFR (rw) register accessor: analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3awcfr`]
module"]
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFRrs>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM_FLT3EXMAX (r) register accessor: Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3exmax`]
module"]
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAXrs>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM_FLT3EXMIN (r) register accessor: Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3exmin`]
module"]
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMINrs>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM_FLT3CNVTIMR (r) register accessor: conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_flt3cnvtimr`]
module"]
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMRrs>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
#[doc = "CH7WDATR (rw) register accessor: CH7WDATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7wdatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7wdatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7wdatr`]
module"]
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATRrs>;
#[doc = "CH7WDATR"]
pub mod ch7wdatr;
#[doc = "CH7DATINR (rw) register accessor: CH7DATINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7datinr`]
module"]
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINRrs>;
#[doc = "CH7DATINR"]
pub mod ch7datinr;
