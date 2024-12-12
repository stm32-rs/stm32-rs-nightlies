#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - channel configuration y register
    #[inline(always)]
    pub const fn ch0cfgr1(&self) -> &CH0CFGR1 {
        &self.ch0cfgr1
    }
    ///0x04 - channel configuration y register
    #[inline(always)]
    pub const fn ch0cfgr2(&self) -> &CH0CFGR2 {
        &self.ch0cfgr2
    }
    ///0x08 - analog watchdog and short-circuit detector register
    #[inline(always)]
    pub const fn ch0awscdr(&self) -> &CH0AWSCDR {
        &self.ch0awscdr
    }
    ///0x0c - channel watchdog filter data register
    #[inline(always)]
    pub const fn ch0wdatr(&self) -> &CH0WDATR {
        &self.ch0wdatr
    }
    ///0x10 - channel data input register
    #[inline(always)]
    pub const fn ch0datinr(&self) -> &CH0DATINR {
        &self.ch0datinr
    }
    ///0x14 - channel y delay register
    #[inline(always)]
    pub const fn ch0dlyr(&self) -> &CH0DLYR {
        &self.ch0dlyr
    }
    ///0x20 - CH1CFGR1
    #[inline(always)]
    pub const fn ch1cfgr1(&self) -> &CH1CFGR1 {
        &self.ch1cfgr1
    }
    ///0x24 - CH1CFGR2
    #[inline(always)]
    pub const fn ch1cfgr2(&self) -> &CH1CFGR2 {
        &self.ch1cfgr2
    }
    ///0x28 - CH1AWSCDR
    #[inline(always)]
    pub const fn ch1awscdr(&self) -> &CH1AWSCDR {
        &self.ch1awscdr
    }
    ///0x2c - CH1WDATR
    #[inline(always)]
    pub const fn ch1wdatr(&self) -> &CH1WDATR {
        &self.ch1wdatr
    }
    ///0x30 - CH1DATINR
    #[inline(always)]
    pub const fn ch1datinr(&self) -> &CH1DATINR {
        &self.ch1datinr
    }
    ///0x34 - channel y delay register
    #[inline(always)]
    pub const fn ch1dlyr(&self) -> &CH1DLYR {
        &self.ch1dlyr
    }
    ///0x40 - CH2CFGR1
    #[inline(always)]
    pub const fn ch2cfgr1(&self) -> &CH2CFGR1 {
        &self.ch2cfgr1
    }
    ///0x44 - CH2CFGR2
    #[inline(always)]
    pub const fn ch2cfgr2(&self) -> &CH2CFGR2 {
        &self.ch2cfgr2
    }
    ///0x48 - CH2AWSCDR
    #[inline(always)]
    pub const fn ch2awscdr(&self) -> &CH2AWSCDR {
        &self.ch2awscdr
    }
    ///0x4c - CH2WDATR
    #[inline(always)]
    pub const fn ch2wdatr(&self) -> &CH2WDATR {
        &self.ch2wdatr
    }
    ///0x50 - CH2DATINR
    #[inline(always)]
    pub const fn ch2datinr(&self) -> &CH2DATINR {
        &self.ch2datinr
    }
    ///0x54 - channel y delay register
    #[inline(always)]
    pub const fn ch2dlyr(&self) -> &CH2DLYR {
        &self.ch2dlyr
    }
    ///0x60 - CH3CFGR1
    #[inline(always)]
    pub const fn ch3cfgr1(&self) -> &CH3CFGR1 {
        &self.ch3cfgr1
    }
    ///0x64 - CH3CFGR2
    #[inline(always)]
    pub const fn ch3cfgr2(&self) -> &CH3CFGR2 {
        &self.ch3cfgr2
    }
    ///0x68 - CH3AWSCDR
    #[inline(always)]
    pub const fn ch3awscdr(&self) -> &CH3AWSCDR {
        &self.ch3awscdr
    }
    ///0x6c - CH3WDATR
    #[inline(always)]
    pub const fn ch3wdatr(&self) -> &CH3WDATR {
        &self.ch3wdatr
    }
    ///0x70 - CH3DATINR
    #[inline(always)]
    pub const fn ch3datinr(&self) -> &CH3DATINR {
        &self.ch3datinr
    }
    ///0x74 - channel y delay register
    #[inline(always)]
    pub const fn ch3dlyr(&self) -> &CH3DLYR {
        &self.ch3dlyr
    }
    ///0x80 - CH4CFGR1
    #[inline(always)]
    pub const fn ch4cfgr1(&self) -> &CH4CFGR1 {
        &self.ch4cfgr1
    }
    ///0x84 - CH4CFGR2
    #[inline(always)]
    pub const fn ch4cfgr2(&self) -> &CH4CFGR2 {
        &self.ch4cfgr2
    }
    ///0x88 - CH4AWSCDR
    #[inline(always)]
    pub const fn ch4awscdr(&self) -> &CH4AWSCDR {
        &self.ch4awscdr
    }
    ///0x8c - CH4WDATR
    #[inline(always)]
    pub const fn ch4wdatr(&self) -> &CH4WDATR {
        &self.ch4wdatr
    }
    ///0x90 - CH4DATINR
    #[inline(always)]
    pub const fn ch4datinr(&self) -> &CH4DATINR {
        &self.ch4datinr
    }
    ///0x94 - channel y delay register
    #[inline(always)]
    pub const fn ch4dlyr(&self) -> &CH4DLYR {
        &self.ch4dlyr
    }
    ///0xa0 - CH5CFGR1
    #[inline(always)]
    pub const fn ch5cfgr1(&self) -> &CH5CFGR1 {
        &self.ch5cfgr1
    }
    ///0xa4 - CH5CFGR2
    #[inline(always)]
    pub const fn ch5cfgr2(&self) -> &CH5CFGR2 {
        &self.ch5cfgr2
    }
    ///0xa8 - CH5AWSCDR
    #[inline(always)]
    pub const fn ch5awscdr(&self) -> &CH5AWSCDR {
        &self.ch5awscdr
    }
    ///0xac - CH5WDATR
    #[inline(always)]
    pub const fn ch5wdatr(&self) -> &CH5WDATR {
        &self.ch5wdatr
    }
    ///0xb0 - CH5DATINR
    #[inline(always)]
    pub const fn ch5datinr(&self) -> &CH5DATINR {
        &self.ch5datinr
    }
    ///0xb4 - channel y delay register
    #[inline(always)]
    pub const fn ch5dlyr(&self) -> &CH5DLYR {
        &self.ch5dlyr
    }
    ///0xc0 - CH6CFGR1
    #[inline(always)]
    pub const fn ch6cfgr1(&self) -> &CH6CFGR1 {
        &self.ch6cfgr1
    }
    ///0xc4 - CH6CFGR2
    #[inline(always)]
    pub const fn ch6cfgr2(&self) -> &CH6CFGR2 {
        &self.ch6cfgr2
    }
    ///0xc8 - CH6AWSCDR
    #[inline(always)]
    pub const fn ch6awscdr(&self) -> &CH6AWSCDR {
        &self.ch6awscdr
    }
    ///0xcc - CH6WDATR
    #[inline(always)]
    pub const fn ch6wdatr(&self) -> &CH6WDATR {
        &self.ch6wdatr
    }
    ///0xd0 - CH6DATINR
    #[inline(always)]
    pub const fn ch6datinr(&self) -> &CH6DATINR {
        &self.ch6datinr
    }
    ///0xd4 - channel y delay register
    #[inline(always)]
    pub const fn ch6dlyr(&self) -> &CH6DLYR {
        &self.ch6dlyr
    }
    ///0xe0 - CH7CFGR1
    #[inline(always)]
    pub const fn ch7cfgr1(&self) -> &CH7CFGR1 {
        &self.ch7cfgr1
    }
    ///0xe4 - CH7CFGR2
    #[inline(always)]
    pub const fn ch7cfgr2(&self) -> &CH7CFGR2 {
        &self.ch7cfgr2
    }
    ///0xe8 - CH7AWSCDR
    #[inline(always)]
    pub const fn ch7awscdr(&self) -> &CH7AWSCDR {
        &self.ch7awscdr
    }
    ///0xec - CH7WDATR
    #[inline(always)]
    pub const fn ch7wdatr(&self) -> &CH7WDATR {
        &self.ch7wdatr
    }
    ///0xf0 - CH7DATINR
    #[inline(always)]
    pub const fn ch7datinr(&self) -> &CH7DATINR {
        &self.ch7datinr
    }
    ///0xf4 - channel y delay register
    #[inline(always)]
    pub const fn ch7dlyr(&self) -> &CH7DLYR {
        &self.ch7dlyr
    }
    ///0x100 - control register 1
    #[inline(always)]
    pub const fn dfsdm_flt0cr1(&self) -> &DFSDM_FLT0CR1 {
        &self.dfsdm_flt0cr1
    }
    ///0x104 - control register 2
    #[inline(always)]
    pub const fn dfsdm_flt0cr2(&self) -> &DFSDM_FLT0CR2 {
        &self.dfsdm_flt0cr2
    }
    ///0x108 - interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt0isr(&self) -> &DFSDM_FLT0ISR {
        &self.dfsdm_flt0isr
    }
    ///0x10c - interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt0icr(&self) -> &DFSDM_FLT0ICR {
        &self.dfsdm_flt0icr
    }
    ///0x110 - injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt0jchgr(&self) -> &DFSDM_FLT0JCHGR {
        &self.dfsdm_flt0jchgr
    }
    ///0x114 - filter control register
    #[inline(always)]
    pub const fn dfsdm_flt0fcr(&self) -> &DFSDM_FLT0FCR {
        &self.dfsdm_flt0fcr
    }
    ///0x118 - data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt0jdatar(&self) -> &DFSDM_FLT0JDATAR {
        &self.dfsdm_flt0jdatar
    }
    ///0x11c - data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt0rdatar(&self) -> &DFSDM_FLT0RDATAR {
        &self.dfsdm_flt0rdatar
    }
    ///0x120 - analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt0awhtr(&self) -> &DFSDM_FLT0AWHTR {
        &self.dfsdm_flt0awhtr
    }
    ///0x124 - analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt0awltr(&self) -> &DFSDM_FLT0AWLTR {
        &self.dfsdm_flt0awltr
    }
    ///0x128 - analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt0awsr(&self) -> &DFSDM_FLT0AWSR {
        &self.dfsdm_flt0awsr
    }
    ///0x12c - analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt0awcfr(&self) -> &DFSDM_FLT0AWCFR {
        &self.dfsdm_flt0awcfr
    }
    ///0x130 - Extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt0exmax(&self) -> &DFSDM_FLT0EXMAX {
        &self.dfsdm_flt0exmax
    }
    ///0x134 - Extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt0exmin(&self) -> &DFSDM_FLT0EXMIN {
        &self.dfsdm_flt0exmin
    }
    ///0x138 - conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt0cnvtimr(&self) -> &DFSDM_FLT0CNVTIMR {
        &self.dfsdm_flt0cnvtimr
    }
    ///0x180 - control register 1
    #[inline(always)]
    pub const fn dfsdm_flt1cr1(&self) -> &DFSDM_FLT1CR1 {
        &self.dfsdm_flt1cr1
    }
    ///0x184 - control register 2
    #[inline(always)]
    pub const fn dfsdm_flt1cr2(&self) -> &DFSDM_FLT1CR2 {
        &self.dfsdm_flt1cr2
    }
    ///0x188 - interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt1isr(&self) -> &DFSDM_FLT1ISR {
        &self.dfsdm_flt1isr
    }
    ///0x18c - interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt1icr(&self) -> &DFSDM_FLT1ICR {
        &self.dfsdm_flt1icr
    }
    ///0x190 - injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt1chgr(&self) -> &DFSDM_FLT1CHGR {
        &self.dfsdm_flt1chgr
    }
    ///0x194 - filter control register
    #[inline(always)]
    pub const fn dfsdm_flt1fcr(&self) -> &DFSDM_FLT1FCR {
        &self.dfsdm_flt1fcr
    }
    ///0x198 - data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt1jdatar(&self) -> &DFSDM_FLT1JDATAR {
        &self.dfsdm_flt1jdatar
    }
    ///0x19c - data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt1rdatar(&self) -> &DFSDM_FLT1RDATAR {
        &self.dfsdm_flt1rdatar
    }
    ///0x1a0 - analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt1awhtr(&self) -> &DFSDM_FLT1AWHTR {
        &self.dfsdm_flt1awhtr
    }
    ///0x1a4 - analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt1awltr(&self) -> &DFSDM_FLT1AWLTR {
        &self.dfsdm_flt1awltr
    }
    ///0x1a8 - analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt1awsr(&self) -> &DFSDM_FLT1AWSR {
        &self.dfsdm_flt1awsr
    }
    ///0x1ac - analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt1awcfr(&self) -> &DFSDM_FLT1AWCFR {
        &self.dfsdm_flt1awcfr
    }
    ///0x1b0 - Extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt1exmax(&self) -> &DFSDM_FLT1EXMAX {
        &self.dfsdm_flt1exmax
    }
    ///0x1b4 - Extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt1exmin(&self) -> &DFSDM_FLT1EXMIN {
        &self.dfsdm_flt1exmin
    }
    ///0x1b8 - conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt1cnvtimr(&self) -> &DFSDM_FLT1CNVTIMR {
        &self.dfsdm_flt1cnvtimr
    }
    ///0x200 - control register 1
    #[inline(always)]
    pub const fn dfsdm_flt2cr1(&self) -> &DFSDM_FLT2CR1 {
        &self.dfsdm_flt2cr1
    }
    ///0x204 - control register 2
    #[inline(always)]
    pub const fn dfsdm_flt2cr2(&self) -> &DFSDM_FLT2CR2 {
        &self.dfsdm_flt2cr2
    }
    ///0x208 - interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt2isr(&self) -> &DFSDM_FLT2ISR {
        &self.dfsdm_flt2isr
    }
    ///0x20c - interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt2icr(&self) -> &DFSDM_FLT2ICR {
        &self.dfsdm_flt2icr
    }
    ///0x210 - injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt2jchgr(&self) -> &DFSDM_FLT2JCHGR {
        &self.dfsdm_flt2jchgr
    }
    ///0x214 - filter control register
    #[inline(always)]
    pub const fn dfsdm_flt2fcr(&self) -> &DFSDM_FLT2FCR {
        &self.dfsdm_flt2fcr
    }
    ///0x218 - data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt2jdatar(&self) -> &DFSDM_FLT2JDATAR {
        &self.dfsdm_flt2jdatar
    }
    ///0x21c - data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt2rdatar(&self) -> &DFSDM_FLT2RDATAR {
        &self.dfsdm_flt2rdatar
    }
    ///0x220 - analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt2awhtr(&self) -> &DFSDM_FLT2AWHTR {
        &self.dfsdm_flt2awhtr
    }
    ///0x224 - analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt2awltr(&self) -> &DFSDM_FLT2AWLTR {
        &self.dfsdm_flt2awltr
    }
    ///0x228 - analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt2awsr(&self) -> &DFSDM_FLT2AWSR {
        &self.dfsdm_flt2awsr
    }
    ///0x22c - analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt2awcfr(&self) -> &DFSDM_FLT2AWCFR {
        &self.dfsdm_flt2awcfr
    }
    ///0x230 - Extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt2exmax(&self) -> &DFSDM_FLT2EXMAX {
        &self.dfsdm_flt2exmax
    }
    ///0x234 - Extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt2exmin(&self) -> &DFSDM_FLT2EXMIN {
        &self.dfsdm_flt2exmin
    }
    ///0x238 - conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt2cnvtimr(&self) -> &DFSDM_FLT2CNVTIMR {
        &self.dfsdm_flt2cnvtimr
    }
    ///0x280 - control register 1
    #[inline(always)]
    pub const fn dfsdm_flt3cr1(&self) -> &DFSDM_FLT3CR1 {
        &self.dfsdm_flt3cr1
    }
    ///0x284 - control register 2
    #[inline(always)]
    pub const fn dfsdm_flt3cr2(&self) -> &DFSDM_FLT3CR2 {
        &self.dfsdm_flt3cr2
    }
    ///0x288 - interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt3isr(&self) -> &DFSDM_FLT3ISR {
        &self.dfsdm_flt3isr
    }
    ///0x28c - interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt3icr(&self) -> &DFSDM_FLT3ICR {
        &self.dfsdm_flt3icr
    }
    ///0x290 - injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt3jchgr(&self) -> &DFSDM_FLT3JCHGR {
        &self.dfsdm_flt3jchgr
    }
    ///0x294 - filter control register
    #[inline(always)]
    pub const fn dfsdm_flt3fcr(&self) -> &DFSDM_FLT3FCR {
        &self.dfsdm_flt3fcr
    }
    ///0x298 - data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt3jdatar(&self) -> &DFSDM_FLT3JDATAR {
        &self.dfsdm_flt3jdatar
    }
    ///0x29c - data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt3rdatar(&self) -> &DFSDM_FLT3RDATAR {
        &self.dfsdm_flt3rdatar
    }
    ///0x2a0 - analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt3awhtr(&self) -> &DFSDM_FLT3AWHTR {
        &self.dfsdm_flt3awhtr
    }
    ///0x2a4 - analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt3awltr(&self) -> &DFSDM_FLT3AWLTR {
        &self.dfsdm_flt3awltr
    }
    ///0x2a8 - analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt3awsr(&self) -> &DFSDM_FLT3AWSR {
        &self.dfsdm_flt3awsr
    }
    ///0x2ac - analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt3awcfr(&self) -> &DFSDM_FLT3AWCFR {
        &self.dfsdm_flt3awcfr
    }
    ///0x2b0 - Extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt3exmax(&self) -> &DFSDM_FLT3EXMAX {
        &self.dfsdm_flt3exmax
    }
    ///0x2b4 - Extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt3exmin(&self) -> &DFSDM_FLT3EXMIN {
        &self.dfsdm_flt3exmin
    }
    ///0x2b8 - conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt3cnvtimr(&self) -> &DFSDM_FLT3CNVTIMR {
        &self.dfsdm_flt3cnvtimr
    }
}
/**CH0CFGR1 (rw) register accessor: channel configuration y register

You can [`read`](crate::Reg::read) this register and get [`ch0cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH0CFGR1)

For information about available fields see [`mod@ch0cfgr1`]
module*/
pub type CH0CFGR1 = crate::Reg<ch0cfgr1::CH0CFGR1rs>;
///channel configuration y register
pub mod ch0cfgr1;
/**CH0CFGR2 (rw) register accessor: channel configuration y register

You can [`read`](crate::Reg::read) this register and get [`ch0cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH0CFGR2)

For information about available fields see [`mod@ch0cfgr2`]
module*/
pub type CH0CFGR2 = crate::Reg<ch0cfgr2::CH0CFGR2rs>;
///channel configuration y register
pub mod ch0cfgr2;
/**CH0AWSCDR (rw) register accessor: analog watchdog and short-circuit detector register

You can [`read`](crate::Reg::read) this register and get [`ch0awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH0AWSCDR)

For information about available fields see [`mod@ch0awscdr`]
module*/
pub type CH0AWSCDR = crate::Reg<ch0awscdr::CH0AWSCDRrs>;
///analog watchdog and short-circuit detector register
pub mod ch0awscdr;
/**CH0WDATR (rw) register accessor: channel watchdog filter data register

You can [`read`](crate::Reg::read) this register and get [`ch0wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH0WDATR)

For information about available fields see [`mod@ch0wdatr`]
module*/
pub type CH0WDATR = crate::Reg<ch0wdatr::CH0WDATRrs>;
///channel watchdog filter data register
pub mod ch0wdatr;
/**CH0DATINR (rw) register accessor: channel data input register

You can [`read`](crate::Reg::read) this register and get [`ch0datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH0DATINR)

For information about available fields see [`mod@ch0datinr`]
module*/
pub type CH0DATINR = crate::Reg<ch0datinr::CH0DATINRrs>;
///channel data input register
pub mod ch0datinr;
/**CH0DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch0dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH0DLYR)

For information about available fields see [`mod@ch0dlyr`]
module*/
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYRrs>;
///channel y delay register
pub mod ch0dlyr;
/**CH1CFGR1 (rw) register accessor: CH1CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH1CFGR1)

For information about available fields see [`mod@ch1cfgr1`]
module*/
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1rs>;
///CH1CFGR1
pub mod ch1cfgr1;
/**CH1CFGR2 (rw) register accessor: CH1CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH1CFGR2)

For information about available fields see [`mod@ch1cfgr2`]
module*/
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2rs>;
///CH1CFGR2
pub mod ch1cfgr2;
/**CH1AWSCDR (rw) register accessor: CH1AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch1awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH1AWSCDR)

For information about available fields see [`mod@ch1awscdr`]
module*/
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDRrs>;
///CH1AWSCDR
pub mod ch1awscdr;
/**CH1WDATR (rw) register accessor: CH1WDATR

You can [`read`](crate::Reg::read) this register and get [`ch1wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH1WDATR)

For information about available fields see [`mod@ch1wdatr`]
module*/
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATRrs>;
///CH1WDATR
pub mod ch1wdatr;
/**CH1DATINR (rw) register accessor: CH1DATINR

You can [`read`](crate::Reg::read) this register and get [`ch1datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH1DATINR)

For information about available fields see [`mod@ch1datinr`]
module*/
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINRrs>;
///CH1DATINR
pub mod ch1datinr;
/**CH1DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch1dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH1DLYR)

For information about available fields see [`mod@ch1dlyr`]
module*/
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYRrs>;
///channel y delay register
pub mod ch1dlyr;
/**CH2CFGR1 (rw) register accessor: CH2CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH2CFGR1)

For information about available fields see [`mod@ch2cfgr1`]
module*/
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1rs>;
///CH2CFGR1
pub mod ch2cfgr1;
/**CH2CFGR2 (rw) register accessor: CH2CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH2CFGR2)

For information about available fields see [`mod@ch2cfgr2`]
module*/
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2rs>;
///CH2CFGR2
pub mod ch2cfgr2;
/**CH2AWSCDR (rw) register accessor: CH2AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch2awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH2AWSCDR)

For information about available fields see [`mod@ch2awscdr`]
module*/
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDRrs>;
///CH2AWSCDR
pub mod ch2awscdr;
/**CH2WDATR (rw) register accessor: CH2WDATR

You can [`read`](crate::Reg::read) this register and get [`ch2wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH2WDATR)

For information about available fields see [`mod@ch2wdatr`]
module*/
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATRrs>;
///CH2WDATR
pub mod ch2wdatr;
/**CH2DATINR (rw) register accessor: CH2DATINR

You can [`read`](crate::Reg::read) this register and get [`ch2datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH2DATINR)

For information about available fields see [`mod@ch2datinr`]
module*/
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINRrs>;
///CH2DATINR
pub mod ch2datinr;
/**CH2DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch2dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH2DLYR)

For information about available fields see [`mod@ch2dlyr`]
module*/
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYRrs>;
///channel y delay register
pub mod ch2dlyr;
/**CH3CFGR1 (rw) register accessor: CH3CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3CFGR1)

For information about available fields see [`mod@ch3cfgr1`]
module*/
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1rs>;
///CH3CFGR1
pub mod ch3cfgr1;
/**CH3CFGR2 (rw) register accessor: CH3CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3CFGR2)

For information about available fields see [`mod@ch3cfgr2`]
module*/
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2rs>;
///CH3CFGR2
pub mod ch3cfgr2;
/**CH3AWSCDR (rw) register accessor: CH3AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch3awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3AWSCDR)

For information about available fields see [`mod@ch3awscdr`]
module*/
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDRrs>;
///CH3AWSCDR
pub mod ch3awscdr;
/**CH3WDATR (rw) register accessor: CH3WDATR

You can [`read`](crate::Reg::read) this register and get [`ch3wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3WDATR)

For information about available fields see [`mod@ch3wdatr`]
module*/
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATRrs>;
///CH3WDATR
pub mod ch3wdatr;
/**CH3DATINR (rw) register accessor: CH3DATINR

You can [`read`](crate::Reg::read) this register and get [`ch3datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3DATINR)

For information about available fields see [`mod@ch3datinr`]
module*/
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINRrs>;
///CH3DATINR
pub mod ch3datinr;
/**CH3DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch3dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH3DLYR)

For information about available fields see [`mod@ch3dlyr`]
module*/
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYRrs>;
///channel y delay register
pub mod ch3dlyr;
/**CH4CFGR1 (rw) register accessor: CH4CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch4cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH4CFGR1)

For information about available fields see [`mod@ch4cfgr1`]
module*/
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1rs>;
///CH4CFGR1
pub mod ch4cfgr1;
/**CH4CFGR2 (rw) register accessor: CH4CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch4cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH4CFGR2)

For information about available fields see [`mod@ch4cfgr2`]
module*/
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2rs>;
///CH4CFGR2
pub mod ch4cfgr2;
/**CH4AWSCDR (rw) register accessor: CH4AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch4awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH4AWSCDR)

For information about available fields see [`mod@ch4awscdr`]
module*/
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDRrs>;
///CH4AWSCDR
pub mod ch4awscdr;
/**CH4WDATR (rw) register accessor: CH4WDATR

You can [`read`](crate::Reg::read) this register and get [`ch4wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH4WDATR)

For information about available fields see [`mod@ch4wdatr`]
module*/
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATRrs>;
///CH4WDATR
pub mod ch4wdatr;
/**CH4DATINR (rw) register accessor: CH4DATINR

You can [`read`](crate::Reg::read) this register and get [`ch4datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH4DATINR)

For information about available fields see [`mod@ch4datinr`]
module*/
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINRrs>;
///CH4DATINR
pub mod ch4datinr;
/**CH4DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch4dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH4DLYR)

For information about available fields see [`mod@ch4dlyr`]
module*/
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYRrs>;
///channel y delay register
pub mod ch4dlyr;
/**CH5CFGR1 (rw) register accessor: CH5CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch5cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH5CFGR1)

For information about available fields see [`mod@ch5cfgr1`]
module*/
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1rs>;
///CH5CFGR1
pub mod ch5cfgr1;
/**CH5CFGR2 (rw) register accessor: CH5CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch5cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH5CFGR2)

For information about available fields see [`mod@ch5cfgr2`]
module*/
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2rs>;
///CH5CFGR2
pub mod ch5cfgr2;
/**CH5AWSCDR (rw) register accessor: CH5AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch5awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH5AWSCDR)

For information about available fields see [`mod@ch5awscdr`]
module*/
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDRrs>;
///CH5AWSCDR
pub mod ch5awscdr;
/**CH5WDATR (rw) register accessor: CH5WDATR

You can [`read`](crate::Reg::read) this register and get [`ch5wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH5WDATR)

For information about available fields see [`mod@ch5wdatr`]
module*/
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATRrs>;
///CH5WDATR
pub mod ch5wdatr;
/**CH5DATINR (rw) register accessor: CH5DATINR

You can [`read`](crate::Reg::read) this register and get [`ch5datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH5DATINR)

For information about available fields see [`mod@ch5datinr`]
module*/
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINRrs>;
///CH5DATINR
pub mod ch5datinr;
/**CH5DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch5dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH5DLYR)

For information about available fields see [`mod@ch5dlyr`]
module*/
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYRrs>;
///channel y delay register
pub mod ch5dlyr;
/**CH6CFGR1 (rw) register accessor: CH6CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch6cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH6CFGR1)

For information about available fields see [`mod@ch6cfgr1`]
module*/
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1rs>;
///CH6CFGR1
pub mod ch6cfgr1;
/**CH6CFGR2 (rw) register accessor: CH6CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch6cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH6CFGR2)

For information about available fields see [`mod@ch6cfgr2`]
module*/
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2rs>;
///CH6CFGR2
pub mod ch6cfgr2;
/**CH6AWSCDR (rw) register accessor: CH6AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch6awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH6AWSCDR)

For information about available fields see [`mod@ch6awscdr`]
module*/
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDRrs>;
///CH6AWSCDR
pub mod ch6awscdr;
/**CH6WDATR (rw) register accessor: CH6WDATR

You can [`read`](crate::Reg::read) this register and get [`ch6wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH6WDATR)

For information about available fields see [`mod@ch6wdatr`]
module*/
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATRrs>;
///CH6WDATR
pub mod ch6wdatr;
/**CH6DATINR (rw) register accessor: CH6DATINR

You can [`read`](crate::Reg::read) this register and get [`ch6datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH6DATINR)

For information about available fields see [`mod@ch6datinr`]
module*/
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINRrs>;
///CH6DATINR
pub mod ch6datinr;
/**CH6DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch6dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH6DLYR)

For information about available fields see [`mod@ch6dlyr`]
module*/
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYRrs>;
///channel y delay register
pub mod ch6dlyr;
/**CH7CFGR1 (rw) register accessor: CH7CFGR1

You can [`read`](crate::Reg::read) this register and get [`ch7cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH7CFGR1)

For information about available fields see [`mod@ch7cfgr1`]
module*/
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1rs>;
///CH7CFGR1
pub mod ch7cfgr1;
/**CH7CFGR2 (rw) register accessor: CH7CFGR2

You can [`read`](crate::Reg::read) this register and get [`ch7cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH7CFGR2)

For information about available fields see [`mod@ch7cfgr2`]
module*/
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2rs>;
///CH7CFGR2
pub mod ch7cfgr2;
/**CH7AWSCDR (rw) register accessor: CH7AWSCDR

You can [`read`](crate::Reg::read) this register and get [`ch7awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH7AWSCDR)

For information about available fields see [`mod@ch7awscdr`]
module*/
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDRrs>;
///CH7AWSCDR
pub mod ch7awscdr;
/**CH7WDATR (rw) register accessor: CH7WDATR

You can [`read`](crate::Reg::read) this register and get [`ch7wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH7WDATR)

For information about available fields see [`mod@ch7wdatr`]
module*/
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATRrs>;
///CH7WDATR
pub mod ch7wdatr;
/**CH7DATINR (rw) register accessor: CH7DATINR

You can [`read`](crate::Reg::read) this register and get [`ch7datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH7DATINR)

For information about available fields see [`mod@ch7datinr`]
module*/
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINRrs>;
///CH7DATINR
pub mod ch7datinr;
/**CH7DLYR (rw) register accessor: channel y delay register

You can [`read`](crate::Reg::read) this register and get [`ch7dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:CH7DLYR)

For information about available fields see [`mod@ch7dlyr`]
module*/
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYRrs>;
///channel y delay register
pub mod ch7dlyr;
/**DFSDM_FLT0CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0CR1)

For information about available fields see [`mod@dfsdm_flt0cr1`]
module*/
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1rs>;
///control register 1
pub mod dfsdm_flt0cr1;
/**DFSDM_FLT0CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0CR2)

For information about available fields see [`mod@dfsdm_flt0cr2`]
module*/
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2rs>;
///control register 2
pub mod dfsdm_flt0cr2;
/**DFSDM_FLT0ISR (r) register accessor: interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0ISR)

For information about available fields see [`mod@dfsdm_flt0isr`]
module*/
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISRrs>;
///interrupt and status register
pub mod dfsdm_flt0isr;
/**DFSDM_FLT0ICR (rw) register accessor: interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0ICR)

For information about available fields see [`mod@dfsdm_flt0icr`]
module*/
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICRrs>;
///interrupt flag clear register
pub mod dfsdm_flt0icr;
/**DFSDM_FLT0JCHGR (rw) register accessor: injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0JCHGR)

For information about available fields see [`mod@dfsdm_flt0jchgr`]
module*/
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGRrs>;
///injected channel group selection register
pub mod dfsdm_flt0jchgr;
/**DFSDM_FLT0FCR (rw) register accessor: filter control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0FCR)

For information about available fields see [`mod@dfsdm_flt0fcr`]
module*/
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCRrs>;
///filter control register
pub mod dfsdm_flt0fcr;
/**DFSDM_FLT0JDATAR (r) register accessor: data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0JDATAR)

For information about available fields see [`mod@dfsdm_flt0jdatar`]
module*/
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATARrs>;
///data register for injected group
pub mod dfsdm_flt0jdatar;
/**DFSDM_FLT0RDATAR (r) register accessor: data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0RDATAR)

For information about available fields see [`mod@dfsdm_flt0rdatar`]
module*/
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATARrs>;
///data register for the regular channel
pub mod dfsdm_flt0rdatar;
/**DFSDM_FLT0AWHTR (rw) register accessor: analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0AWHTR)

For information about available fields see [`mod@dfsdm_flt0awhtr`]
module*/
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTRrs>;
///analog watchdog high threshold register
pub mod dfsdm_flt0awhtr;
/**DFSDM_FLT0AWLTR (rw) register accessor: analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0AWLTR)

For information about available fields see [`mod@dfsdm_flt0awltr`]
module*/
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTRrs>;
///analog watchdog low threshold register
pub mod dfsdm_flt0awltr;
/**DFSDM_FLT0AWSR (r) register accessor: analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0AWSR)

For information about available fields see [`mod@dfsdm_flt0awsr`]
module*/
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSRrs>;
///analog watchdog status register
pub mod dfsdm_flt0awsr;
/**DFSDM_FLT0AWCFR (rw) register accessor: analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0AWCFR)

For information about available fields see [`mod@dfsdm_flt0awcfr`]
module*/
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFRrs>;
///analog watchdog clear flag register
pub mod dfsdm_flt0awcfr;
/**DFSDM_FLT0EXMAX (r) register accessor: Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0EXMAX)

For information about available fields see [`mod@dfsdm_flt0exmax`]
module*/
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAXrs>;
///Extremes detector maximum register
pub mod dfsdm_flt0exmax;
/**DFSDM_FLT0EXMIN (r) register accessor: Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0EXMIN)

For information about available fields see [`mod@dfsdm_flt0exmin`]
module*/
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMINrs>;
///Extremes detector minimum register
pub mod dfsdm_flt0exmin;
/**DFSDM_FLT0CNVTIMR (r) register accessor: conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0CNVTIMR)

For information about available fields see [`mod@dfsdm_flt0cnvtimr`]
module*/
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMRrs>;
///conversion timer register
pub mod dfsdm_flt0cnvtimr;
/**DFSDM_FLT1CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1CR1)

For information about available fields see [`mod@dfsdm_flt1cr1`]
module*/
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1rs>;
///control register 1
pub mod dfsdm_flt1cr1;
/**DFSDM_FLT1CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1CR2)

For information about available fields see [`mod@dfsdm_flt1cr2`]
module*/
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2rs>;
///control register 2
pub mod dfsdm_flt1cr2;
/**DFSDM_FLT1ISR (r) register accessor: interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1ISR)

For information about available fields see [`mod@dfsdm_flt1isr`]
module*/
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISRrs>;
///interrupt and status register
pub mod dfsdm_flt1isr;
/**DFSDM_FLT1ICR (rw) register accessor: interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1ICR)

For information about available fields see [`mod@dfsdm_flt1icr`]
module*/
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICRrs>;
///interrupt flag clear register
pub mod dfsdm_flt1icr;
/**DFSDM_FLT1CHGR (rw) register accessor: injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1chgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1chgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1CHGR)

For information about available fields see [`mod@dfsdm_flt1chgr`]
module*/
pub type DFSDM_FLT1CHGR = crate::Reg<dfsdm_flt1chgr::DFSDM_FLT1CHGRrs>;
///injected channel group selection register
pub mod dfsdm_flt1chgr;
/**DFSDM_FLT1FCR (rw) register accessor: filter control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1FCR)

For information about available fields see [`mod@dfsdm_flt1fcr`]
module*/
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCRrs>;
///filter control register
pub mod dfsdm_flt1fcr;
/**DFSDM_FLT1JDATAR (r) register accessor: data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1JDATAR)

For information about available fields see [`mod@dfsdm_flt1jdatar`]
module*/
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATARrs>;
///data register for injected group
pub mod dfsdm_flt1jdatar;
/**DFSDM_FLT1RDATAR (r) register accessor: data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1RDATAR)

For information about available fields see [`mod@dfsdm_flt1rdatar`]
module*/
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATARrs>;
///data register for the regular channel
pub mod dfsdm_flt1rdatar;
/**DFSDM_FLT1AWHTR (rw) register accessor: analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1AWHTR)

For information about available fields see [`mod@dfsdm_flt1awhtr`]
module*/
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTRrs>;
///analog watchdog high threshold register
pub mod dfsdm_flt1awhtr;
/**DFSDM_FLT1AWLTR (rw) register accessor: analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1AWLTR)

For information about available fields see [`mod@dfsdm_flt1awltr`]
module*/
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTRrs>;
///analog watchdog low threshold register
pub mod dfsdm_flt1awltr;
/**DFSDM_FLT1AWSR (r) register accessor: analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1AWSR)

For information about available fields see [`mod@dfsdm_flt1awsr`]
module*/
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSRrs>;
///analog watchdog status register
pub mod dfsdm_flt1awsr;
/**DFSDM_FLT1AWCFR (rw) register accessor: analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1AWCFR)

For information about available fields see [`mod@dfsdm_flt1awcfr`]
module*/
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFRrs>;
///analog watchdog clear flag register
pub mod dfsdm_flt1awcfr;
/**DFSDM_FLT1EXMAX (r) register accessor: Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1EXMAX)

For information about available fields see [`mod@dfsdm_flt1exmax`]
module*/
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAXrs>;
///Extremes detector maximum register
pub mod dfsdm_flt1exmax;
/**DFSDM_FLT1EXMIN (r) register accessor: Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1EXMIN)

For information about available fields see [`mod@dfsdm_flt1exmin`]
module*/
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMINrs>;
///Extremes detector minimum register
pub mod dfsdm_flt1exmin;
/**DFSDM_FLT1CNVTIMR (r) register accessor: conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT1CNVTIMR)

For information about available fields see [`mod@dfsdm_flt1cnvtimr`]
module*/
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMRrs>;
///conversion timer register
pub mod dfsdm_flt1cnvtimr;
/**DFSDM_FLT2CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2CR1)

For information about available fields see [`mod@dfsdm_flt2cr1`]
module*/
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1rs>;
///control register 1
pub mod dfsdm_flt2cr1;
/**DFSDM_FLT2CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2CR2)

For information about available fields see [`mod@dfsdm_flt2cr2`]
module*/
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2rs>;
///control register 2
pub mod dfsdm_flt2cr2;
/**DFSDM_FLT2ISR (r) register accessor: interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2ISR)

For information about available fields see [`mod@dfsdm_flt2isr`]
module*/
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISRrs>;
///interrupt and status register
pub mod dfsdm_flt2isr;
/**DFSDM_FLT2ICR (rw) register accessor: interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2ICR)

For information about available fields see [`mod@dfsdm_flt2icr`]
module*/
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICRrs>;
///interrupt flag clear register
pub mod dfsdm_flt2icr;
/**DFSDM_FLT2JCHGR (rw) register accessor: injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2JCHGR)

For information about available fields see [`mod@dfsdm_flt2jchgr`]
module*/
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGRrs>;
///injected channel group selection register
pub mod dfsdm_flt2jchgr;
/**DFSDM_FLT2FCR (rw) register accessor: filter control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2FCR)

For information about available fields see [`mod@dfsdm_flt2fcr`]
module*/
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCRrs>;
///filter control register
pub mod dfsdm_flt2fcr;
/**DFSDM_FLT2JDATAR (r) register accessor: data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2JDATAR)

For information about available fields see [`mod@dfsdm_flt2jdatar`]
module*/
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATARrs>;
///data register for injected group
pub mod dfsdm_flt2jdatar;
/**DFSDM_FLT2RDATAR (r) register accessor: data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2RDATAR)

For information about available fields see [`mod@dfsdm_flt2rdatar`]
module*/
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATARrs>;
///data register for the regular channel
pub mod dfsdm_flt2rdatar;
/**DFSDM_FLT2AWHTR (rw) register accessor: analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2AWHTR)

For information about available fields see [`mod@dfsdm_flt2awhtr`]
module*/
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTRrs>;
///analog watchdog high threshold register
pub mod dfsdm_flt2awhtr;
/**DFSDM_FLT2AWLTR (rw) register accessor: analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2AWLTR)

For information about available fields see [`mod@dfsdm_flt2awltr`]
module*/
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTRrs>;
///analog watchdog low threshold register
pub mod dfsdm_flt2awltr;
/**DFSDM_FLT2AWSR (r) register accessor: analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2AWSR)

For information about available fields see [`mod@dfsdm_flt2awsr`]
module*/
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSRrs>;
///analog watchdog status register
pub mod dfsdm_flt2awsr;
/**DFSDM_FLT2AWCFR (rw) register accessor: analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2AWCFR)

For information about available fields see [`mod@dfsdm_flt2awcfr`]
module*/
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFRrs>;
///analog watchdog clear flag register
pub mod dfsdm_flt2awcfr;
/**DFSDM_FLT2EXMAX (r) register accessor: Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2EXMAX)

For information about available fields see [`mod@dfsdm_flt2exmax`]
module*/
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAXrs>;
///Extremes detector maximum register
pub mod dfsdm_flt2exmax;
/**DFSDM_FLT2EXMIN (r) register accessor: Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2EXMIN)

For information about available fields see [`mod@dfsdm_flt2exmin`]
module*/
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMINrs>;
///Extremes detector minimum register
pub mod dfsdm_flt2exmin;
/**DFSDM_FLT2CNVTIMR (r) register accessor: conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT2CNVTIMR)

For information about available fields see [`mod@dfsdm_flt2cnvtimr`]
module*/
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMRrs>;
///conversion timer register
pub mod dfsdm_flt2cnvtimr;
/**DFSDM_FLT3CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3CR1)

For information about available fields see [`mod@dfsdm_flt3cr1`]
module*/
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1rs>;
///control register 1
pub mod dfsdm_flt3cr1;
/**DFSDM_FLT3CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3CR2)

For information about available fields see [`mod@dfsdm_flt3cr2`]
module*/
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2rs>;
///control register 2
pub mod dfsdm_flt3cr2;
/**DFSDM_FLT3ISR (r) register accessor: interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3ISR)

For information about available fields see [`mod@dfsdm_flt3isr`]
module*/
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISRrs>;
///interrupt and status register
pub mod dfsdm_flt3isr;
/**DFSDM_FLT3ICR (rw) register accessor: interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3ICR)

For information about available fields see [`mod@dfsdm_flt3icr`]
module*/
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICRrs>;
///interrupt flag clear register
pub mod dfsdm_flt3icr;
/**DFSDM_FLT3JCHGR (rw) register accessor: injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3JCHGR)

For information about available fields see [`mod@dfsdm_flt3jchgr`]
module*/
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGRrs>;
///injected channel group selection register
pub mod dfsdm_flt3jchgr;
/**DFSDM_FLT3FCR (rw) register accessor: filter control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3FCR)

For information about available fields see [`mod@dfsdm_flt3fcr`]
module*/
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCRrs>;
///filter control register
pub mod dfsdm_flt3fcr;
/**DFSDM_FLT3JDATAR (r) register accessor: data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3JDATAR)

For information about available fields see [`mod@dfsdm_flt3jdatar`]
module*/
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATARrs>;
///data register for injected group
pub mod dfsdm_flt3jdatar;
/**DFSDM_FLT3RDATAR (r) register accessor: data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3RDATAR)

For information about available fields see [`mod@dfsdm_flt3rdatar`]
module*/
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATARrs>;
///data register for the regular channel
pub mod dfsdm_flt3rdatar;
/**DFSDM_FLT3AWHTR (rw) register accessor: analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3AWHTR)

For information about available fields see [`mod@dfsdm_flt3awhtr`]
module*/
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTRrs>;
///analog watchdog high threshold register
pub mod dfsdm_flt3awhtr;
/**DFSDM_FLT3AWLTR (rw) register accessor: analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3AWLTR)

For information about available fields see [`mod@dfsdm_flt3awltr`]
module*/
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTRrs>;
///analog watchdog low threshold register
pub mod dfsdm_flt3awltr;
/**DFSDM_FLT3AWSR (r) register accessor: analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3AWSR)

For information about available fields see [`mod@dfsdm_flt3awsr`]
module*/
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSRrs>;
///analog watchdog status register
pub mod dfsdm_flt3awsr;
/**DFSDM_FLT3AWCFR (rw) register accessor: analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3AWCFR)

For information about available fields see [`mod@dfsdm_flt3awcfr`]
module*/
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFRrs>;
///analog watchdog clear flag register
pub mod dfsdm_flt3awcfr;
/**DFSDM_FLT3EXMAX (r) register accessor: Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3EXMAX)

For information about available fields see [`mod@dfsdm_flt3exmax`]
module*/
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAXrs>;
///Extremes detector maximum register
pub mod dfsdm_flt3exmax;
/**DFSDM_FLT3EXMIN (r) register accessor: Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3EXMIN)

For information about available fields see [`mod@dfsdm_flt3exmin`]
module*/
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMINrs>;
///Extremes detector minimum register
pub mod dfsdm_flt3exmin;
/**DFSDM_FLT3CNVTIMR (r) register accessor: conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT3CNVTIMR)

For information about available fields see [`mod@dfsdm_flt3cnvtimr`]
module*/
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMRrs>;
///conversion timer register
pub mod dfsdm_flt3cnvtimr;
