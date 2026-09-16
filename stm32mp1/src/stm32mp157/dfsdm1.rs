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
    flt0cr1: FLT0CR1,
    flt0cr2: FLT0CR2,
    flt0isr: FLT0ISR,
    flt0icr: FLT0ICR,
    flt0jchgr: FLT0JCHGR,
    flt0fcr: FLT0FCR,
    flt0jdatar: FLT0JDATAR,
    flt0rdatar: FLT0RDATAR,
    flt0awhtr: FLT0AWHTR,
    flt0awltr: FLT0AWLTR,
    flt0awsr: FLT0AWSR,
    flt0awcfr: FLT0AWCFR,
    flt0exmax: FLT0EXMAX,
    flt0exmin: FLT0EXMIN,
    flt0cnvtimr: FLT0CNVTIMR,
    _reserved63: [u8; 0x44],
    flt1cr1: FLT1CR1,
    flt1cr2: FLT1CR2,
    flt1isr: FLT1ISR,
    flt1icr: FLT1ICR,
    flt1jchgr: FLT1JCHGR,
    flt1fcr: FLT1FCR,
    flt1jdatar: FLT1JDATAR,
    flt1rdatar: FLT1RDATAR,
    flt1awhtr: FLT1AWHTR,
    flt1awltr: FLT1AWLTR,
    flt1awsr: FLT1AWSR,
    flt1awcfr: FLT1AWCFR,
    flt1exmax: FLT1EXMAX,
    flt1exmin: FLT1EXMIN,
    flt1cnvtimr: FLT1CNVTIMR,
    _reserved78: [u8; 0x44],
    flt2cr1: FLT2CR1,
    flt2cr2: FLT2CR2,
    flt2isr: FLT2ISR,
    flt2icr: FLT2ICR,
    flt2jchgr: FLT2JCHGR,
    flt2fcr: FLT2FCR,
    flt2jdatar: FLT2JDATAR,
    flt2rdatar: FLT2RDATAR,
    flt2awhtr: FLT2AWHTR,
    flt2awltr: FLT2AWLTR,
    flt2awsr: FLT2AWSR,
    flt2awcfr: FLT2AWCFR,
    flt2exmax: FLT2EXMAX,
    flt2exmin: FLT2EXMIN,
    flt2cnvtimr: FLT2CNVTIMR,
    _reserved93: [u8; 0x44],
    flt3cr1: FLT3CR1,
    flt3cr2: FLT3CR2,
    flt3isr: FLT3ISR,
    flt3icr: FLT3ICR,
    flt3jchgr: FLT3JCHGR,
    flt3fcr: FLT3FCR,
    flt3jdatar: FLT3JDATAR,
    flt3rdatar: FLT3RDATAR,
    flt3awhtr: FLT3AWHTR,
    flt3awltr: FLT3AWLTR,
    flt3awsr: FLT3AWSR,
    flt3awcfr: FLT3AWCFR,
    flt3exmax: FLT3EXMAX,
    flt3exmin: FLT3EXMIN,
    flt3cnvtimr: FLT3CNVTIMR,
    _reserved108: [u8; 0x44],
    flt4cr1: FLT4CR1,
    flt4cr2: FLT4CR2,
    flt4isr: FLT4ISR,
    flt4icr: FLT4ICR,
    flt4jchgr: FLT4JCHGR,
    flt4fcr: FLT4FCR,
    flt4jdatar: FLT4JDATAR,
    flt4rdatar: FLT4RDATAR,
    flt4awhtr: FLT4AWHTR,
    flt4awltr: FLT4AWLTR,
    flt4awsr: FLT4AWSR,
    flt4awcfr: FLT4AWCFR,
    flt4exmax: FLT4EXMAX,
    flt4exmin: FLT4EXMIN,
    flt4cnvtimr: FLT4CNVTIMR,
    _reserved123: [u8; 0x44],
    flt5cr1: FLT5CR1,
    flt5cr2: FLT5CR2,
    flt5isr: FLT5ISR,
    flt5icr: FLT5ICR,
    flt5jchgr: FLT5JCHGR,
    flt5fcr: FLT5FCR,
    flt5jdatar: FLT5JDATAR,
    flt5rdatar: FLT5RDATAR,
    flt5awhtr: FLT5AWHTR,
    flt5awltr: FLT5AWLTR,
    flt5awsr: FLT5AWSR,
    flt5awcfr: FLT5AWCFR,
    flt5exmax: FLT5EXMAX,
    flt5exmin: FLT5EXMIN,
    flt5cnvtimr: FLT5CNVTIMR,
    _reserved138: [u8; 0x0434],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch0cfgr1(&self) -> &CH0CFGR1 {
        &self.ch0cfgr1
    }
    ///0x04 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch0cfgr2(&self) -> &CH0CFGR2 {
        &self.ch0cfgr2
    }
    ///0x08 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch0awscdr(&self) -> &CH0AWSCDR {
        &self.ch0awscdr
    }
    ///0x0c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch0wdatr(&self) -> &CH0WDATR {
        &self.ch0wdatr
    }
    ///0x10 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch0datinr(&self) -> &CH0DATINR {
        &self.ch0datinr
    }
    ///0x14 - DFSDM channel 0 delay register
    #[inline(always)]
    pub const fn ch0dlyr(&self) -> &CH0DLYR {
        &self.ch0dlyr
    }
    ///0x20 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch1cfgr1(&self) -> &CH1CFGR1 {
        &self.ch1cfgr1
    }
    ///0x24 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch1cfgr2(&self) -> &CH1CFGR2 {
        &self.ch1cfgr2
    }
    ///0x28 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch1awscdr(&self) -> &CH1AWSCDR {
        &self.ch1awscdr
    }
    ///0x2c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch1wdatr(&self) -> &CH1WDATR {
        &self.ch1wdatr
    }
    ///0x30 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch1datinr(&self) -> &CH1DATINR {
        &self.ch1datinr
    }
    ///0x34 - DFSDM channel 1 delay register
    #[inline(always)]
    pub const fn ch1dlyr(&self) -> &CH1DLYR {
        &self.ch1dlyr
    }
    ///0x40 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch2cfgr1(&self) -> &CH2CFGR1 {
        &self.ch2cfgr1
    }
    ///0x44 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch2cfgr2(&self) -> &CH2CFGR2 {
        &self.ch2cfgr2
    }
    ///0x48 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch2awscdr(&self) -> &CH2AWSCDR {
        &self.ch2awscdr
    }
    ///0x4c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch2wdatr(&self) -> &CH2WDATR {
        &self.ch2wdatr
    }
    ///0x50 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch2datinr(&self) -> &CH2DATINR {
        &self.ch2datinr
    }
    ///0x54 - DFSDM channel 2 delay register
    #[inline(always)]
    pub const fn ch2dlyr(&self) -> &CH2DLYR {
        &self.ch2dlyr
    }
    ///0x60 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch3cfgr1(&self) -> &CH3CFGR1 {
        &self.ch3cfgr1
    }
    ///0x64 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch3cfgr2(&self) -> &CH3CFGR2 {
        &self.ch3cfgr2
    }
    ///0x68 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch3awscdr(&self) -> &CH3AWSCDR {
        &self.ch3awscdr
    }
    ///0x6c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch3wdatr(&self) -> &CH3WDATR {
        &self.ch3wdatr
    }
    ///0x70 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch3datinr(&self) -> &CH3DATINR {
        &self.ch3datinr
    }
    ///0x74 - DFSDM channel 3 delay register
    #[inline(always)]
    pub const fn ch3dlyr(&self) -> &CH3DLYR {
        &self.ch3dlyr
    }
    ///0x80 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch4cfgr1(&self) -> &CH4CFGR1 {
        &self.ch4cfgr1
    }
    ///0x84 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch4cfgr2(&self) -> &CH4CFGR2 {
        &self.ch4cfgr2
    }
    ///0x88 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch4awscdr(&self) -> &CH4AWSCDR {
        &self.ch4awscdr
    }
    ///0x8c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch4wdatr(&self) -> &CH4WDATR {
        &self.ch4wdatr
    }
    ///0x90 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch4datinr(&self) -> &CH4DATINR {
        &self.ch4datinr
    }
    ///0x94 - DFSDM channel 4 delay register
    #[inline(always)]
    pub const fn ch4dlyr(&self) -> &CH4DLYR {
        &self.ch4dlyr
    }
    ///0xa0 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch5cfgr1(&self) -> &CH5CFGR1 {
        &self.ch5cfgr1
    }
    ///0xa4 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch5cfgr2(&self) -> &CH5CFGR2 {
        &self.ch5cfgr2
    }
    ///0xa8 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch5awscdr(&self) -> &CH5AWSCDR {
        &self.ch5awscdr
    }
    ///0xac - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch5wdatr(&self) -> &CH5WDATR {
        &self.ch5wdatr
    }
    ///0xb0 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch5datinr(&self) -> &CH5DATINR {
        &self.ch5datinr
    }
    ///0xb4 - DFSDM channel 5 delay register
    #[inline(always)]
    pub const fn ch5dlyr(&self) -> &CH5DLYR {
        &self.ch5dlyr
    }
    ///0xc0 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch6cfgr1(&self) -> &CH6CFGR1 {
        &self.ch6cfgr1
    }
    ///0xc4 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch6cfgr2(&self) -> &CH6CFGR2 {
        &self.ch6cfgr2
    }
    ///0xc8 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch6awscdr(&self) -> &CH6AWSCDR {
        &self.ch6awscdr
    }
    ///0xcc - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch6wdatr(&self) -> &CH6WDATR {
        &self.ch6wdatr
    }
    ///0xd0 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch6datinr(&self) -> &CH6DATINR {
        &self.ch6datinr
    }
    ///0xd4 - DFSDM channel 6 delay register
    #[inline(always)]
    pub const fn ch6dlyr(&self) -> &CH6DLYR {
        &self.ch6dlyr
    }
    ///0xe0 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch7cfgr1(&self) -> &CH7CFGR1 {
        &self.ch7cfgr1
    }
    ///0xe4 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn ch7cfgr2(&self) -> &CH7CFGR2 {
        &self.ch7cfgr2
    }
    ///0xe8 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn ch7awscdr(&self) -> &CH7AWSCDR {
        &self.ch7awscdr
    }
    ///0xec - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn ch7wdatr(&self) -> &CH7WDATR {
        &self.ch7wdatr
    }
    ///0xf0 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn ch7datinr(&self) -> &CH7DATINR {
        &self.ch7datinr
    }
    ///0xf4 - DFSDM channel 7 delay register
    #[inline(always)]
    pub const fn ch7dlyr(&self) -> &CH7DLYR {
        &self.ch7dlyr
    }
    ///0x100 - DFSDM filter 0 control register 1
    #[inline(always)]
    pub const fn flt0cr1(&self) -> &FLT0CR1 {
        &self.flt0cr1
    }
    ///0x104 - DFSDM filter 0 control register 2
    #[inline(always)]
    pub const fn flt0cr2(&self) -> &FLT0CR2 {
        &self.flt0cr2
    }
    ///0x108 - DFSDM filter 0 interrupt and status register
    #[inline(always)]
    pub const fn flt0isr(&self) -> &FLT0ISR {
        &self.flt0isr
    }
    ///0x10c - DFSDM filter 0 interrupt flag clear register
    #[inline(always)]
    pub const fn flt0icr(&self) -> &FLT0ICR {
        &self.flt0icr
    }
    ///0x110 - DFSDM filter 0 injected channel group selection register
    #[inline(always)]
    pub const fn flt0jchgr(&self) -> &FLT0JCHGR {
        &self.flt0jchgr
    }
    ///0x114 - DFSDM filter 0 control register
    #[inline(always)]
    pub const fn flt0fcr(&self) -> &FLT0FCR {
        &self.flt0fcr
    }
    ///0x118 - DFSDM filter 0 data register for injected group
    #[inline(always)]
    pub const fn flt0jdatar(&self) -> &FLT0JDATAR {
        &self.flt0jdatar
    }
    ///0x11c - DFSDM filter 0 data register for the regular channel
    #[inline(always)]
    pub const fn flt0rdatar(&self) -> &FLT0RDATAR {
        &self.flt0rdatar
    }
    ///0x120 - DFSDM filter 0 analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt0awhtr(&self) -> &FLT0AWHTR {
        &self.flt0awhtr
    }
    ///0x124 - DFSDM filter 0 analog watchdog low threshold register
    #[inline(always)]
    pub const fn flt0awltr(&self) -> &FLT0AWLTR {
        &self.flt0awltr
    }
    ///0x128 - DFSDM filter 0 analog watchdog status register
    #[inline(always)]
    pub const fn flt0awsr(&self) -> &FLT0AWSR {
        &self.flt0awsr
    }
    ///0x12c - DFSDM filter 0 analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt0awcfr(&self) -> &FLT0AWCFR {
        &self.flt0awcfr
    }
    ///0x130 - DFSDM filter 0 extremes detector maximum register
    #[inline(always)]
    pub const fn flt0exmax(&self) -> &FLT0EXMAX {
        &self.flt0exmax
    }
    ///0x134 - DFSDM filter 0 extremes detector minimum register
    #[inline(always)]
    pub const fn flt0exmin(&self) -> &FLT0EXMIN {
        &self.flt0exmin
    }
    ///0x138 - DFSDM filter 0 conversion timer register
    #[inline(always)]
    pub const fn flt0cnvtimr(&self) -> &FLT0CNVTIMR {
        &self.flt0cnvtimr
    }
    ///0x180 - DFSDM filter 1 control register 1
    #[inline(always)]
    pub const fn flt1cr1(&self) -> &FLT1CR1 {
        &self.flt1cr1
    }
    ///0x184 - DFSDM filter 1 control register 2
    #[inline(always)]
    pub const fn flt1cr2(&self) -> &FLT1CR2 {
        &self.flt1cr2
    }
    ///0x188 - DFSDM filter 1 interrupt and status register
    #[inline(always)]
    pub const fn flt1isr(&self) -> &FLT1ISR {
        &self.flt1isr
    }
    ///0x18c - DFSDM filter 1 interrupt flag clear register
    #[inline(always)]
    pub const fn flt1icr(&self) -> &FLT1ICR {
        &self.flt1icr
    }
    ///0x190 - DFSDM filter 1 injected channel group selection register
    #[inline(always)]
    pub const fn flt1jchgr(&self) -> &FLT1JCHGR {
        &self.flt1jchgr
    }
    ///0x194 - DFSDM filter 1 control register
    #[inline(always)]
    pub const fn flt1fcr(&self) -> &FLT1FCR {
        &self.flt1fcr
    }
    ///0x198 - DFSDM filter 1 data register for injected group
    #[inline(always)]
    pub const fn flt1jdatar(&self) -> &FLT1JDATAR {
        &self.flt1jdatar
    }
    ///0x19c - DFSDM filter 1 data register for the regular channel
    #[inline(always)]
    pub const fn flt1rdatar(&self) -> &FLT1RDATAR {
        &self.flt1rdatar
    }
    ///0x1a0 - DFSDM filter 1 analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt1awhtr(&self) -> &FLT1AWHTR {
        &self.flt1awhtr
    }
    ///0x1a4 - DFSDM filter 1 analog watchdog low threshold register
    #[inline(always)]
    pub const fn flt1awltr(&self) -> &FLT1AWLTR {
        &self.flt1awltr
    }
    ///0x1a8 - DFSDM filter 1 analog watchdog status register
    #[inline(always)]
    pub const fn flt1awsr(&self) -> &FLT1AWSR {
        &self.flt1awsr
    }
    ///0x1ac - DFSDM filter 1 analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt1awcfr(&self) -> &FLT1AWCFR {
        &self.flt1awcfr
    }
    ///0x1b0 - DFSDM filter 1 extremes detector maximum register
    #[inline(always)]
    pub const fn flt1exmax(&self) -> &FLT1EXMAX {
        &self.flt1exmax
    }
    ///0x1b4 - DFSDM filter 1 extremes detector minimum register
    #[inline(always)]
    pub const fn flt1exmin(&self) -> &FLT1EXMIN {
        &self.flt1exmin
    }
    ///0x1b8 - DFSDM filter 1 conversion timer register
    #[inline(always)]
    pub const fn flt1cnvtimr(&self) -> &FLT1CNVTIMR {
        &self.flt1cnvtimr
    }
    ///0x200 - DFSDM filter 2 control register 1
    #[inline(always)]
    pub const fn flt2cr1(&self) -> &FLT2CR1 {
        &self.flt2cr1
    }
    ///0x204 - DFSDM filter 2 control register 2
    #[inline(always)]
    pub const fn flt2cr2(&self) -> &FLT2CR2 {
        &self.flt2cr2
    }
    ///0x208 - DFSDM filter 2 interrupt and status register
    #[inline(always)]
    pub const fn flt2isr(&self) -> &FLT2ISR {
        &self.flt2isr
    }
    ///0x20c - DFSDM filter 2 interrupt flag clear register
    #[inline(always)]
    pub const fn flt2icr(&self) -> &FLT2ICR {
        &self.flt2icr
    }
    ///0x210 - DFSDM filter 2 injected channel group selection register
    #[inline(always)]
    pub const fn flt2jchgr(&self) -> &FLT2JCHGR {
        &self.flt2jchgr
    }
    ///0x214 - DFSDM filter 2 control register
    #[inline(always)]
    pub const fn flt2fcr(&self) -> &FLT2FCR {
        &self.flt2fcr
    }
    ///0x218 - DFSDM filter 2 data register for injected group
    #[inline(always)]
    pub const fn flt2jdatar(&self) -> &FLT2JDATAR {
        &self.flt2jdatar
    }
    ///0x21c - DFSDM filter 2 data register for the regular channel
    #[inline(always)]
    pub const fn flt2rdatar(&self) -> &FLT2RDATAR {
        &self.flt2rdatar
    }
    ///0x220 - DFSDM filter 2 analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt2awhtr(&self) -> &FLT2AWHTR {
        &self.flt2awhtr
    }
    ///0x224 - DFSDM filter 2 analog watchdog low threshold register
    #[inline(always)]
    pub const fn flt2awltr(&self) -> &FLT2AWLTR {
        &self.flt2awltr
    }
    ///0x228 - DFSDM filter 2 analog watchdog status register
    #[inline(always)]
    pub const fn flt2awsr(&self) -> &FLT2AWSR {
        &self.flt2awsr
    }
    ///0x22c - DFSDM filter 2 analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt2awcfr(&self) -> &FLT2AWCFR {
        &self.flt2awcfr
    }
    ///0x230 - DFSDM filter 2 extremes detector maximum register
    #[inline(always)]
    pub const fn flt2exmax(&self) -> &FLT2EXMAX {
        &self.flt2exmax
    }
    ///0x234 - DFSDM filter 2 extremes detector minimum register
    #[inline(always)]
    pub const fn flt2exmin(&self) -> &FLT2EXMIN {
        &self.flt2exmin
    }
    ///0x238 - DFSDM filter 2 conversion timer register
    #[inline(always)]
    pub const fn flt2cnvtimr(&self) -> &FLT2CNVTIMR {
        &self.flt2cnvtimr
    }
    ///0x280 - DFSDM filter 3 control register 1
    #[inline(always)]
    pub const fn flt3cr1(&self) -> &FLT3CR1 {
        &self.flt3cr1
    }
    ///0x284 - DFSDM filter 3 control register 2
    #[inline(always)]
    pub const fn flt3cr2(&self) -> &FLT3CR2 {
        &self.flt3cr2
    }
    ///0x288 - DFSDM filter 3 interrupt and status register
    #[inline(always)]
    pub const fn flt3isr(&self) -> &FLT3ISR {
        &self.flt3isr
    }
    ///0x28c - DFSDM filter 3 interrupt flag clear register
    #[inline(always)]
    pub const fn flt3icr(&self) -> &FLT3ICR {
        &self.flt3icr
    }
    ///0x290 - DFSDM filter 3 injected channel group selection register
    #[inline(always)]
    pub const fn flt3jchgr(&self) -> &FLT3JCHGR {
        &self.flt3jchgr
    }
    ///0x294 - DFSDM filter 3 control register
    #[inline(always)]
    pub const fn flt3fcr(&self) -> &FLT3FCR {
        &self.flt3fcr
    }
    ///0x298 - DFSDM filter 3 data register for injected group
    #[inline(always)]
    pub const fn flt3jdatar(&self) -> &FLT3JDATAR {
        &self.flt3jdatar
    }
    ///0x29c - DFSDM filter 3 data register for the regular channel
    #[inline(always)]
    pub const fn flt3rdatar(&self) -> &FLT3RDATAR {
        &self.flt3rdatar
    }
    ///0x2a0 - DFSDM filter 3 analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt3awhtr(&self) -> &FLT3AWHTR {
        &self.flt3awhtr
    }
    ///0x2a4 - DFSDM filter 3 analog watchdog low threshold register
    #[inline(always)]
    pub const fn flt3awltr(&self) -> &FLT3AWLTR {
        &self.flt3awltr
    }
    ///0x2a8 - DFSDM filter 3 analog watchdog status register
    #[inline(always)]
    pub const fn flt3awsr(&self) -> &FLT3AWSR {
        &self.flt3awsr
    }
    ///0x2ac - DFSDM filter 3 analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt3awcfr(&self) -> &FLT3AWCFR {
        &self.flt3awcfr
    }
    ///0x2b0 - DFSDM filter 3 extremes detector maximum register
    #[inline(always)]
    pub const fn flt3exmax(&self) -> &FLT3EXMAX {
        &self.flt3exmax
    }
    ///0x2b4 - DFSDM filter 3 extremes detector minimum register
    #[inline(always)]
    pub const fn flt3exmin(&self) -> &FLT3EXMIN {
        &self.flt3exmin
    }
    ///0x2b8 - DFSDM filter 3 conversion timer register
    #[inline(always)]
    pub const fn flt3cnvtimr(&self) -> &FLT3CNVTIMR {
        &self.flt3cnvtimr
    }
    ///0x300 - DFSDM filter 4 control register 1
    #[inline(always)]
    pub const fn flt4cr1(&self) -> &FLT4CR1 {
        &self.flt4cr1
    }
    ///0x304 - DFSDM filter 4 control register 2
    #[inline(always)]
    pub const fn flt4cr2(&self) -> &FLT4CR2 {
        &self.flt4cr2
    }
    ///0x308 - DFSDM filter 4 interrupt and status register
    #[inline(always)]
    pub const fn flt4isr(&self) -> &FLT4ISR {
        &self.flt4isr
    }
    ///0x30c - DFSDM filter 4 interrupt flag clear register
    #[inline(always)]
    pub const fn flt4icr(&self) -> &FLT4ICR {
        &self.flt4icr
    }
    ///0x310 - DFSDM filter 4 injected channel group selection register
    #[inline(always)]
    pub const fn flt4jchgr(&self) -> &FLT4JCHGR {
        &self.flt4jchgr
    }
    ///0x314 - DFSDM filter 4 control register
    #[inline(always)]
    pub const fn flt4fcr(&self) -> &FLT4FCR {
        &self.flt4fcr
    }
    ///0x318 - DFSDM filter 4 data register for injected group
    #[inline(always)]
    pub const fn flt4jdatar(&self) -> &FLT4JDATAR {
        &self.flt4jdatar
    }
    ///0x31c - DFSDM filter 4 data register for the regular channel
    #[inline(always)]
    pub const fn flt4rdatar(&self) -> &FLT4RDATAR {
        &self.flt4rdatar
    }
    ///0x320 - DFSDM filter 4 analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt4awhtr(&self) -> &FLT4AWHTR {
        &self.flt4awhtr
    }
    ///0x324 - DFSDM filter 4 analog watchdog low threshold register
    #[inline(always)]
    pub const fn flt4awltr(&self) -> &FLT4AWLTR {
        &self.flt4awltr
    }
    ///0x328 - DFSDM filter 4 analog watchdog status register
    #[inline(always)]
    pub const fn flt4awsr(&self) -> &FLT4AWSR {
        &self.flt4awsr
    }
    ///0x32c - DFSDM filter 4 analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt4awcfr(&self) -> &FLT4AWCFR {
        &self.flt4awcfr
    }
    ///0x330 - DFSDM filter 4 extremes detector maximum register
    #[inline(always)]
    pub const fn flt4exmax(&self) -> &FLT4EXMAX {
        &self.flt4exmax
    }
    ///0x334 - DFSDM filter 4 extremes detector minimum register
    #[inline(always)]
    pub const fn flt4exmin(&self) -> &FLT4EXMIN {
        &self.flt4exmin
    }
    ///0x338 - DFSDM filter 4 conversion timer register
    #[inline(always)]
    pub const fn flt4cnvtimr(&self) -> &FLT4CNVTIMR {
        &self.flt4cnvtimr
    }
    ///0x380 - DFSDM filter 5 control register 1
    #[inline(always)]
    pub const fn flt5cr1(&self) -> &FLT5CR1 {
        &self.flt5cr1
    }
    ///0x384 - DFSDM filter 5 control register 2
    #[inline(always)]
    pub const fn flt5cr2(&self) -> &FLT5CR2 {
        &self.flt5cr2
    }
    ///0x388 - DFSDM filter 5 interrupt and status register
    #[inline(always)]
    pub const fn flt5isr(&self) -> &FLT5ISR {
        &self.flt5isr
    }
    ///0x38c - DFSDM filter 5 interrupt flag clear register
    #[inline(always)]
    pub const fn flt5icr(&self) -> &FLT5ICR {
        &self.flt5icr
    }
    ///0x390 - DFSDM filter 5 injected channel group selection register
    #[inline(always)]
    pub const fn flt5jchgr(&self) -> &FLT5JCHGR {
        &self.flt5jchgr
    }
    ///0x394 - DFSDM filter 5 control register
    #[inline(always)]
    pub const fn flt5fcr(&self) -> &FLT5FCR {
        &self.flt5fcr
    }
    ///0x398 - DFSDM filter 5 data register for injected group
    #[inline(always)]
    pub const fn flt5jdatar(&self) -> &FLT5JDATAR {
        &self.flt5jdatar
    }
    ///0x39c - DFSDM filter 5 data register for the regular channel
    #[inline(always)]
    pub const fn flt5rdatar(&self) -> &FLT5RDATAR {
        &self.flt5rdatar
    }
    ///0x3a0 - DFSDM filter 5 analog watchdog high threshold register
    #[inline(always)]
    pub const fn flt5awhtr(&self) -> &FLT5AWHTR {
        &self.flt5awhtr
    }
    ///0x3a4 - DFSDM filter 5 analog watchdog low threshold register
    #[inline(always)]
    pub const fn flt5awltr(&self) -> &FLT5AWLTR {
        &self.flt5awltr
    }
    ///0x3a8 - DFSDM filter 5 analog watchdog status register
    #[inline(always)]
    pub const fn flt5awsr(&self) -> &FLT5AWSR {
        &self.flt5awsr
    }
    ///0x3ac - DFSDM filter 5 analog watchdog clear flag register
    #[inline(always)]
    pub const fn flt5awcfr(&self) -> &FLT5AWCFR {
        &self.flt5awcfr
    }
    ///0x3b0 - DFSDM filter 5 extremes detector maximum register
    #[inline(always)]
    pub const fn flt5exmax(&self) -> &FLT5EXMAX {
        &self.flt5exmax
    }
    ///0x3b4 - DFSDM filter 5 extremes detector minimum register
    #[inline(always)]
    pub const fn flt5exmin(&self) -> &FLT5EXMIN {
        &self.flt5exmin
    }
    ///0x3b8 - DFSDM filter 5 conversion timer register
    #[inline(always)]
    pub const fn flt5cnvtimr(&self) -> &FLT5CNVTIMR {
        &self.flt5cnvtimr
    }
    ///0x7f0 - This register specifies the hardware configuration of DFSDM peripheral.
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x7f4 - This register specifies the version of DFSDM peripheral.
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x7f8 - This register specifies the identification of DFSDM peripheral.
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x7fc - This register specifies the size allocated to DFSDM registers.
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CH0CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch0cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0CFGR1)

For information about available fields see [`mod@ch0cfgr1`] module*/
pub type CH0CFGR1 = crate::Reg<ch0cfgr1::CH0CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch0cfgr1;
/**CH0CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch0cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0CFGR2)

For information about available fields see [`mod@ch0cfgr2`] module*/
pub type CH0CFGR2 = crate::Reg<ch0cfgr2::CH0CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch0cfgr2;
/**CH0AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch0awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0AWSCDR)

For information about available fields see [`mod@ch0awscdr`] module*/
pub type CH0AWSCDR = crate::Reg<ch0awscdr::CH0AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch0awscdr;
/**CH0WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch0wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0WDATR)

For information about available fields see [`mod@ch0wdatr`] module*/
pub type CH0WDATR = crate::Reg<ch0wdatr::CH0WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch0wdatr;
/**CH0DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch0datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0DATINR)

For information about available fields see [`mod@ch0datinr`] module*/
pub type CH0DATINR = crate::Reg<ch0datinr::CH0DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch0datinr;
/**CH0DLYR (rw) register accessor: DFSDM channel 0 delay register

You can [`read`](crate::Reg::read) this register and get [`ch0dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH0DLYR)

For information about available fields see [`mod@ch0dlyr`] module*/
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYRrs>;
///DFSDM channel 0 delay register
pub mod ch0dlyr;
/**CH1CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH1CFGR1)

For information about available fields see [`mod@ch1cfgr1`] module*/
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch1cfgr1;
/**CH1CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH1CFGR2)

For information about available fields see [`mod@ch1cfgr2`] module*/
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch1cfgr2;
/**CH1AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch1awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH1AWSCDR)

For information about available fields see [`mod@ch1awscdr`] module*/
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch1awscdr;
/**CH1WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch1wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH1WDATR)

For information about available fields see [`mod@ch1wdatr`] module*/
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch1wdatr;
/**CH1DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch1datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH1DATINR)

For information about available fields see [`mod@ch1datinr`] module*/
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch1datinr;
/**CH1DLYR (rw) register accessor: DFSDM channel 1 delay register

You can [`read`](crate::Reg::read) this register and get [`ch1dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH1DLYR)

For information about available fields see [`mod@ch1dlyr`] module*/
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYRrs>;
///DFSDM channel 1 delay register
pub mod ch1dlyr;
/**CH2CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH2CFGR1)

For information about available fields see [`mod@ch2cfgr1`] module*/
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch2cfgr1;
/**CH2CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH2CFGR2)

For information about available fields see [`mod@ch2cfgr2`] module*/
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch2cfgr2;
/**CH2AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch2awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH2AWSCDR)

For information about available fields see [`mod@ch2awscdr`] module*/
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch2awscdr;
/**CH2WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch2wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH2WDATR)

For information about available fields see [`mod@ch2wdatr`] module*/
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch2wdatr;
/**CH2DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch2datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH2DATINR)

For information about available fields see [`mod@ch2datinr`] module*/
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch2datinr;
/**CH2DLYR (rw) register accessor: DFSDM channel 2 delay register

You can [`read`](crate::Reg::read) this register and get [`ch2dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH2DLYR)

For information about available fields see [`mod@ch2dlyr`] module*/
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYRrs>;
///DFSDM channel 2 delay register
pub mod ch2dlyr;
/**CH3CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH3CFGR1)

For information about available fields see [`mod@ch3cfgr1`] module*/
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch3cfgr1;
/**CH3CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH3CFGR2)

For information about available fields see [`mod@ch3cfgr2`] module*/
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch3cfgr2;
/**CH3AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch3awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH3AWSCDR)

For information about available fields see [`mod@ch3awscdr`] module*/
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch3awscdr;
/**CH3WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch3wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH3WDATR)

For information about available fields see [`mod@ch3wdatr`] module*/
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch3wdatr;
/**CH3DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch3datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH3DATINR)

For information about available fields see [`mod@ch3datinr`] module*/
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch3datinr;
/**CH3DLYR (rw) register accessor: DFSDM channel 3 delay register

You can [`read`](crate::Reg::read) this register and get [`ch3dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH3DLYR)

For information about available fields see [`mod@ch3dlyr`] module*/
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYRrs>;
///DFSDM channel 3 delay register
pub mod ch3dlyr;
/**CH4CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch4cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4CFGR1)

For information about available fields see [`mod@ch4cfgr1`] module*/
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch4cfgr1;
/**CH4CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch4cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4CFGR2)

For information about available fields see [`mod@ch4cfgr2`] module*/
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch4cfgr2;
/**CH4AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch4awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4AWSCDR)

For information about available fields see [`mod@ch4awscdr`] module*/
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch4awscdr;
/**CH4WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch4wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4WDATR)

For information about available fields see [`mod@ch4wdatr`] module*/
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch4wdatr;
/**CH4DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch4datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4DATINR)

For information about available fields see [`mod@ch4datinr`] module*/
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch4datinr;
/**CH4DLYR (rw) register accessor: DFSDM channel 4 delay register

You can [`read`](crate::Reg::read) this register and get [`ch4dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4DLYR)

For information about available fields see [`mod@ch4dlyr`] module*/
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYRrs>;
///DFSDM channel 4 delay register
pub mod ch4dlyr;
/**CH5CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch5cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5CFGR1)

For information about available fields see [`mod@ch5cfgr1`] module*/
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch5cfgr1;
/**CH5CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch5cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5CFGR2)

For information about available fields see [`mod@ch5cfgr2`] module*/
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch5cfgr2;
/**CH5AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch5awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5AWSCDR)

For information about available fields see [`mod@ch5awscdr`] module*/
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch5awscdr;
/**CH5WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch5wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5WDATR)

For information about available fields see [`mod@ch5wdatr`] module*/
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch5wdatr;
/**CH5DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch5datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5DATINR)

For information about available fields see [`mod@ch5datinr`] module*/
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch5datinr;
/**CH5DLYR (rw) register accessor: DFSDM channel 5 delay register

You can [`read`](crate::Reg::read) this register and get [`ch5dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH5DLYR)

For information about available fields see [`mod@ch5dlyr`] module*/
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYRrs>;
///DFSDM channel 5 delay register
pub mod ch5dlyr;
/**CH6CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch6cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH6CFGR1)

For information about available fields see [`mod@ch6cfgr1`] module*/
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch6cfgr1;
/**CH6CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch6cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH6CFGR2)

For information about available fields see [`mod@ch6cfgr2`] module*/
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch6cfgr2;
/**CH6AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch6awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH6AWSCDR)

For information about available fields see [`mod@ch6awscdr`] module*/
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch6awscdr;
/**CH6WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch6wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH6WDATR)

For information about available fields see [`mod@ch6wdatr`] module*/
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch6wdatr;
/**CH6DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch6datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH6DATINR)

For information about available fields see [`mod@ch6datinr`] module*/
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch6datinr;
/**CH6DLYR (rw) register accessor: DFSDM channel 6 delay register

You can [`read`](crate::Reg::read) this register and get [`ch6dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH6DLYR)

For information about available fields see [`mod@ch6dlyr`] module*/
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYRrs>;
///DFSDM channel 6 delay register
pub mod ch6dlyr;
/**CH7CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch7cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH7CFGR1)

For information about available fields see [`mod@ch7cfgr1`] module*/
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod ch7cfgr1;
/**CH7CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`ch7cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH7CFGR2)

For information about available fields see [`mod@ch7cfgr2`] module*/
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod ch7cfgr2;
/**CH7AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch7awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH7AWSCDR)

For information about available fields see [`mod@ch7awscdr`] module*/
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod ch7awscdr;
/**CH7WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`ch7wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH7WDATR)

For information about available fields see [`mod@ch7wdatr`] module*/
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod ch7wdatr;
/**CH7DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch7datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH7DATINR)

For information about available fields see [`mod@ch7datinr`] module*/
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod ch7datinr;
/**CH7DLYR (rw) register accessor: DFSDM channel 7 delay register

You can [`read`](crate::Reg::read) this register and get [`ch7dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH7DLYR)

For information about available fields see [`mod@ch7dlyr`] module*/
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYRrs>;
///DFSDM channel 7 delay register
pub mod ch7dlyr;
/**FLT0CR1 (rw) register accessor: DFSDM filter 0 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt0cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0CR1)

For information about available fields see [`mod@flt0cr1`] module*/
pub type FLT0CR1 = crate::Reg<flt0cr1::FLT0CR1rs>;
///DFSDM filter 0 control register 1
pub mod flt0cr1;
/**FLT0CR2 (rw) register accessor: DFSDM filter 0 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt0cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0CR2)

For information about available fields see [`mod@flt0cr2`] module*/
pub type FLT0CR2 = crate::Reg<flt0cr2::FLT0CR2rs>;
///DFSDM filter 0 control register 2
pub mod flt0cr2;
/**FLT0ISR (r) register accessor: DFSDM filter 0 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0ISR)

For information about available fields see [`mod@flt0isr`] module*/
pub type FLT0ISR = crate::Reg<flt0isr::FLT0ISRrs>;
///DFSDM filter 0 interrupt and status register
pub mod flt0isr;
/**FLT0ICR (rw) register accessor: DFSDM filter 0 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt0icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0ICR)

For information about available fields see [`mod@flt0icr`] module*/
pub type FLT0ICR = crate::Reg<flt0icr::FLT0ICRrs>;
///DFSDM filter 0 interrupt flag clear register
pub mod flt0icr;
/**FLT0JCHGR (rw) register accessor: DFSDM filter 0 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt0jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0JCHGR)

For information about available fields see [`mod@flt0jchgr`] module*/
pub type FLT0JCHGR = crate::Reg<flt0jchgr::FLT0JCHGRrs>;
///DFSDM filter 0 injected channel group selection register
pub mod flt0jchgr;
/**FLT0FCR (rw) register accessor: DFSDM filter 0 control register

You can [`read`](crate::Reg::read) this register and get [`flt0fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0FCR)

For information about available fields see [`mod@flt0fcr`] module*/
pub type FLT0FCR = crate::Reg<flt0fcr::FLT0FCRrs>;
///DFSDM filter 0 control register
pub mod flt0fcr;
/**FLT0JDATAR (r) register accessor: DFSDM filter 0 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt0jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0JDATAR)

For information about available fields see [`mod@flt0jdatar`] module*/
pub type FLT0JDATAR = crate::Reg<flt0jdatar::FLT0JDATARrs>;
///DFSDM filter 0 data register for injected group
pub mod flt0jdatar;
/**FLT0RDATAR (r) register accessor: DFSDM filter 0 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt0rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0RDATAR)

For information about available fields see [`mod@flt0rdatar`] module*/
pub type FLT0RDATAR = crate::Reg<flt0rdatar::FLT0RDATARrs>;
///DFSDM filter 0 data register for the regular channel
pub mod flt0rdatar;
/**FLT0AWHTR (rw) register accessor: DFSDM filter 0 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt0awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0AWHTR)

For information about available fields see [`mod@flt0awhtr`] module*/
pub type FLT0AWHTR = crate::Reg<flt0awhtr::FLT0AWHTRrs>;
///DFSDM filter 0 analog watchdog high threshold register
pub mod flt0awhtr;
/**FLT0AWLTR (rw) register accessor: DFSDM filter 0 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt0awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0AWLTR)

For information about available fields see [`mod@flt0awltr`] module*/
pub type FLT0AWLTR = crate::Reg<flt0awltr::FLT0AWLTRrs>;
///DFSDM filter 0 analog watchdog low threshold register
pub mod flt0awltr;
/**FLT0AWSR (r) register accessor: DFSDM filter 0 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt0awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0AWSR)

For information about available fields see [`mod@flt0awsr`] module*/
pub type FLT0AWSR = crate::Reg<flt0awsr::FLT0AWSRrs>;
///DFSDM filter 0 analog watchdog status register
pub mod flt0awsr;
/**FLT0AWCFR (rw) register accessor: DFSDM filter 0 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt0awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0AWCFR)

For information about available fields see [`mod@flt0awcfr`] module*/
pub type FLT0AWCFR = crate::Reg<flt0awcfr::FLT0AWCFRrs>;
///DFSDM filter 0 analog watchdog clear flag register
pub mod flt0awcfr;
/**FLT0EXMAX (r) register accessor: DFSDM filter 0 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt0exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0EXMAX)

For information about available fields see [`mod@flt0exmax`] module*/
pub type FLT0EXMAX = crate::Reg<flt0exmax::FLT0EXMAXrs>;
///DFSDM filter 0 extremes detector maximum register
pub mod flt0exmax;
/**FLT0EXMIN (rw) register accessor: DFSDM filter 0 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt0exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0EXMIN)

For information about available fields see [`mod@flt0exmin`] module*/
pub type FLT0EXMIN = crate::Reg<flt0exmin::FLT0EXMINrs>;
///DFSDM filter 0 extremes detector minimum register
pub mod flt0exmin;
/**FLT0CNVTIMR (r) register accessor: DFSDM filter 0 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt0cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0CNVTIMR)

For information about available fields see [`mod@flt0cnvtimr`] module*/
pub type FLT0CNVTIMR = crate::Reg<flt0cnvtimr::FLT0CNVTIMRrs>;
///DFSDM filter 0 conversion timer register
pub mod flt0cnvtimr;
/**FLT1CR1 (rw) register accessor: DFSDM filter 1 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1CR1)

For information about available fields see [`mod@flt1cr1`] module*/
pub type FLT1CR1 = crate::Reg<flt1cr1::FLT1CR1rs>;
///DFSDM filter 1 control register 1
pub mod flt1cr1;
/**FLT1CR2 (rw) register accessor: DFSDM filter 1 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt1cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1CR2)

For information about available fields see [`mod@flt1cr2`] module*/
pub type FLT1CR2 = crate::Reg<flt1cr2::FLT1CR2rs>;
///DFSDM filter 1 control register 2
pub mod flt1cr2;
/**FLT1ISR (r) register accessor: DFSDM filter 1 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1ISR)

For information about available fields see [`mod@flt1isr`] module*/
pub type FLT1ISR = crate::Reg<flt1isr::FLT1ISRrs>;
///DFSDM filter 1 interrupt and status register
pub mod flt1isr;
/**FLT1ICR (rw) register accessor: DFSDM filter 1 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1ICR)

For information about available fields see [`mod@flt1icr`] module*/
pub type FLT1ICR = crate::Reg<flt1icr::FLT1ICRrs>;
///DFSDM filter 1 interrupt flag clear register
pub mod flt1icr;
/**FLT1JCHGR (rw) register accessor: DFSDM filter 1 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt1jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1JCHGR)

For information about available fields see [`mod@flt1jchgr`] module*/
pub type FLT1JCHGR = crate::Reg<flt1jchgr::FLT1JCHGRrs>;
///DFSDM filter 1 injected channel group selection register
pub mod flt1jchgr;
/**FLT1FCR (rw) register accessor: DFSDM filter 1 control register

You can [`read`](crate::Reg::read) this register and get [`flt1fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1FCR)

For information about available fields see [`mod@flt1fcr`] module*/
pub type FLT1FCR = crate::Reg<flt1fcr::FLT1FCRrs>;
///DFSDM filter 1 control register
pub mod flt1fcr;
/**FLT1JDATAR (r) register accessor: DFSDM filter 1 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt1jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1JDATAR)

For information about available fields see [`mod@flt1jdatar`] module*/
pub type FLT1JDATAR = crate::Reg<flt1jdatar::FLT1JDATARrs>;
///DFSDM filter 1 data register for injected group
pub mod flt1jdatar;
/**FLT1RDATAR (r) register accessor: DFSDM filter 1 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt1rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1RDATAR)

For information about available fields see [`mod@flt1rdatar`] module*/
pub type FLT1RDATAR = crate::Reg<flt1rdatar::FLT1RDATARrs>;
///DFSDM filter 1 data register for the regular channel
pub mod flt1rdatar;
/**FLT1AWHTR (rw) register accessor: DFSDM filter 1 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt1awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1AWHTR)

For information about available fields see [`mod@flt1awhtr`] module*/
pub type FLT1AWHTR = crate::Reg<flt1awhtr::FLT1AWHTRrs>;
///DFSDM filter 1 analog watchdog high threshold register
pub mod flt1awhtr;
/**FLT1AWLTR (rw) register accessor: DFSDM filter 1 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt1awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1AWLTR)

For information about available fields see [`mod@flt1awltr`] module*/
pub type FLT1AWLTR = crate::Reg<flt1awltr::FLT1AWLTRrs>;
///DFSDM filter 1 analog watchdog low threshold register
pub mod flt1awltr;
/**FLT1AWSR (r) register accessor: DFSDM filter 1 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt1awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1AWSR)

For information about available fields see [`mod@flt1awsr`] module*/
pub type FLT1AWSR = crate::Reg<flt1awsr::FLT1AWSRrs>;
///DFSDM filter 1 analog watchdog status register
pub mod flt1awsr;
/**FLT1AWCFR (rw) register accessor: DFSDM filter 1 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt1awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1AWCFR)

For information about available fields see [`mod@flt1awcfr`] module*/
pub type FLT1AWCFR = crate::Reg<flt1awcfr::FLT1AWCFRrs>;
///DFSDM filter 1 analog watchdog clear flag register
pub mod flt1awcfr;
/**FLT1EXMAX (r) register accessor: DFSDM filter 1 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt1exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1EXMAX)

For information about available fields see [`mod@flt1exmax`] module*/
pub type FLT1EXMAX = crate::Reg<flt1exmax::FLT1EXMAXrs>;
///DFSDM filter 1 extremes detector maximum register
pub mod flt1exmax;
/**FLT1EXMIN (rw) register accessor: DFSDM filter 1 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt1exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt1exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1EXMIN)

For information about available fields see [`mod@flt1exmin`] module*/
pub type FLT1EXMIN = crate::Reg<flt1exmin::FLT1EXMINrs>;
///DFSDM filter 1 extremes detector minimum register
pub mod flt1exmin;
/**FLT1CNVTIMR (r) register accessor: DFSDM filter 1 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt1cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1CNVTIMR)

For information about available fields see [`mod@flt1cnvtimr`] module*/
pub type FLT1CNVTIMR = crate::Reg<flt1cnvtimr::FLT1CNVTIMRrs>;
///DFSDM filter 1 conversion timer register
pub mod flt1cnvtimr;
/**FLT2CR1 (rw) register accessor: DFSDM filter 2 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2CR1)

For information about available fields see [`mod@flt2cr1`] module*/
pub type FLT2CR1 = crate::Reg<flt2cr1::FLT2CR1rs>;
///DFSDM filter 2 control register 1
pub mod flt2cr1;
/**FLT2CR2 (rw) register accessor: DFSDM filter 2 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt2cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2CR2)

For information about available fields see [`mod@flt2cr2`] module*/
pub type FLT2CR2 = crate::Reg<flt2cr2::FLT2CR2rs>;
///DFSDM filter 2 control register 2
pub mod flt2cr2;
/**FLT2ISR (r) register accessor: DFSDM filter 2 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2ISR)

For information about available fields see [`mod@flt2isr`] module*/
pub type FLT2ISR = crate::Reg<flt2isr::FLT2ISRrs>;
///DFSDM filter 2 interrupt and status register
pub mod flt2isr;
/**FLT2ICR (rw) register accessor: DFSDM filter 2 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2ICR)

For information about available fields see [`mod@flt2icr`] module*/
pub type FLT2ICR = crate::Reg<flt2icr::FLT2ICRrs>;
///DFSDM filter 2 interrupt flag clear register
pub mod flt2icr;
/**FLT2JCHGR (rw) register accessor: DFSDM filter 2 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt2jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2JCHGR)

For information about available fields see [`mod@flt2jchgr`] module*/
pub type FLT2JCHGR = crate::Reg<flt2jchgr::FLT2JCHGRrs>;
///DFSDM filter 2 injected channel group selection register
pub mod flt2jchgr;
/**FLT2FCR (rw) register accessor: DFSDM filter 2 control register

You can [`read`](crate::Reg::read) this register and get [`flt2fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2FCR)

For information about available fields see [`mod@flt2fcr`] module*/
pub type FLT2FCR = crate::Reg<flt2fcr::FLT2FCRrs>;
///DFSDM filter 2 control register
pub mod flt2fcr;
/**FLT2JDATAR (r) register accessor: DFSDM filter 2 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt2jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2JDATAR)

For information about available fields see [`mod@flt2jdatar`] module*/
pub type FLT2JDATAR = crate::Reg<flt2jdatar::FLT2JDATARrs>;
///DFSDM filter 2 data register for injected group
pub mod flt2jdatar;
/**FLT2RDATAR (r) register accessor: DFSDM filter 2 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt2rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2RDATAR)

For information about available fields see [`mod@flt2rdatar`] module*/
pub type FLT2RDATAR = crate::Reg<flt2rdatar::FLT2RDATARrs>;
///DFSDM filter 2 data register for the regular channel
pub mod flt2rdatar;
/**FLT2AWHTR (rw) register accessor: DFSDM filter 2 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt2awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2AWHTR)

For information about available fields see [`mod@flt2awhtr`] module*/
pub type FLT2AWHTR = crate::Reg<flt2awhtr::FLT2AWHTRrs>;
///DFSDM filter 2 analog watchdog high threshold register
pub mod flt2awhtr;
/**FLT2AWLTR (rw) register accessor: DFSDM filter 2 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt2awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2AWLTR)

For information about available fields see [`mod@flt2awltr`] module*/
pub type FLT2AWLTR = crate::Reg<flt2awltr::FLT2AWLTRrs>;
///DFSDM filter 2 analog watchdog low threshold register
pub mod flt2awltr;
/**FLT2AWSR (r) register accessor: DFSDM filter 2 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt2awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2AWSR)

For information about available fields see [`mod@flt2awsr`] module*/
pub type FLT2AWSR = crate::Reg<flt2awsr::FLT2AWSRrs>;
///DFSDM filter 2 analog watchdog status register
pub mod flt2awsr;
/**FLT2AWCFR (rw) register accessor: DFSDM filter 2 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt2awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2AWCFR)

For information about available fields see [`mod@flt2awcfr`] module*/
pub type FLT2AWCFR = crate::Reg<flt2awcfr::FLT2AWCFRrs>;
///DFSDM filter 2 analog watchdog clear flag register
pub mod flt2awcfr;
/**FLT2EXMAX (r) register accessor: DFSDM filter 2 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt2exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2EXMAX)

For information about available fields see [`mod@flt2exmax`] module*/
pub type FLT2EXMAX = crate::Reg<flt2exmax::FLT2EXMAXrs>;
///DFSDM filter 2 extremes detector maximum register
pub mod flt2exmax;
/**FLT2EXMIN (rw) register accessor: DFSDM filter 2 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt2exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2EXMIN)

For information about available fields see [`mod@flt2exmin`] module*/
pub type FLT2EXMIN = crate::Reg<flt2exmin::FLT2EXMINrs>;
///DFSDM filter 2 extremes detector minimum register
pub mod flt2exmin;
/**FLT2CNVTIMR (r) register accessor: DFSDM filter 2 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt2cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2CNVTIMR)

For information about available fields see [`mod@flt2cnvtimr`] module*/
pub type FLT2CNVTIMR = crate::Reg<flt2cnvtimr::FLT2CNVTIMRrs>;
///DFSDM filter 2 conversion timer register
pub mod flt2cnvtimr;
/**FLT3CR1 (rw) register accessor: DFSDM filter 3 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt3cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3CR1)

For information about available fields see [`mod@flt3cr1`] module*/
pub type FLT3CR1 = crate::Reg<flt3cr1::FLT3CR1rs>;
///DFSDM filter 3 control register 1
pub mod flt3cr1;
/**FLT3CR2 (rw) register accessor: DFSDM filter 3 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt3cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3CR2)

For information about available fields see [`mod@flt3cr2`] module*/
pub type FLT3CR2 = crate::Reg<flt3cr2::FLT3CR2rs>;
///DFSDM filter 3 control register 2
pub mod flt3cr2;
/**FLT3ISR (r) register accessor: DFSDM filter 3 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3ISR)

For information about available fields see [`mod@flt3isr`] module*/
pub type FLT3ISR = crate::Reg<flt3isr::FLT3ISRrs>;
///DFSDM filter 3 interrupt and status register
pub mod flt3isr;
/**FLT3ICR (rw) register accessor: DFSDM filter 3 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3ICR)

For information about available fields see [`mod@flt3icr`] module*/
pub type FLT3ICR = crate::Reg<flt3icr::FLT3ICRrs>;
///DFSDM filter 3 interrupt flag clear register
pub mod flt3icr;
/**FLT3JCHGR (rw) register accessor: DFSDM filter 3 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt3jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3JCHGR)

For information about available fields see [`mod@flt3jchgr`] module*/
pub type FLT3JCHGR = crate::Reg<flt3jchgr::FLT3JCHGRrs>;
///DFSDM filter 3 injected channel group selection register
pub mod flt3jchgr;
/**FLT3FCR (rw) register accessor: DFSDM filter 3 control register

You can [`read`](crate::Reg::read) this register and get [`flt3fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3FCR)

For information about available fields see [`mod@flt3fcr`] module*/
pub type FLT3FCR = crate::Reg<flt3fcr::FLT3FCRrs>;
///DFSDM filter 3 control register
pub mod flt3fcr;
/**FLT3JDATAR (r) register accessor: DFSDM filter 3 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt3jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3JDATAR)

For information about available fields see [`mod@flt3jdatar`] module*/
pub type FLT3JDATAR = crate::Reg<flt3jdatar::FLT3JDATARrs>;
///DFSDM filter 3 data register for injected group
pub mod flt3jdatar;
/**FLT3RDATAR (r) register accessor: DFSDM filter 3 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt3rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3RDATAR)

For information about available fields see [`mod@flt3rdatar`] module*/
pub type FLT3RDATAR = crate::Reg<flt3rdatar::FLT3RDATARrs>;
///DFSDM filter 3 data register for the regular channel
pub mod flt3rdatar;
/**FLT3AWHTR (rw) register accessor: DFSDM filter 3 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt3awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3AWHTR)

For information about available fields see [`mod@flt3awhtr`] module*/
pub type FLT3AWHTR = crate::Reg<flt3awhtr::FLT3AWHTRrs>;
///DFSDM filter 3 analog watchdog high threshold register
pub mod flt3awhtr;
/**FLT3AWLTR (rw) register accessor: DFSDM filter 3 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt3awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3AWLTR)

For information about available fields see [`mod@flt3awltr`] module*/
pub type FLT3AWLTR = crate::Reg<flt3awltr::FLT3AWLTRrs>;
///DFSDM filter 3 analog watchdog low threshold register
pub mod flt3awltr;
/**FLT3AWSR (r) register accessor: DFSDM filter 3 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt3awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3AWSR)

For information about available fields see [`mod@flt3awsr`] module*/
pub type FLT3AWSR = crate::Reg<flt3awsr::FLT3AWSRrs>;
///DFSDM filter 3 analog watchdog status register
pub mod flt3awsr;
/**FLT3AWCFR (rw) register accessor: DFSDM filter 3 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt3awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3AWCFR)

For information about available fields see [`mod@flt3awcfr`] module*/
pub type FLT3AWCFR = crate::Reg<flt3awcfr::FLT3AWCFRrs>;
///DFSDM filter 3 analog watchdog clear flag register
pub mod flt3awcfr;
/**FLT3EXMAX (r) register accessor: DFSDM filter 3 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt3exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3EXMAX)

For information about available fields see [`mod@flt3exmax`] module*/
pub type FLT3EXMAX = crate::Reg<flt3exmax::FLT3EXMAXrs>;
///DFSDM filter 3 extremes detector maximum register
pub mod flt3exmax;
/**FLT3EXMIN (rw) register accessor: DFSDM filter 3 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt3exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt3exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3EXMIN)

For information about available fields see [`mod@flt3exmin`] module*/
pub type FLT3EXMIN = crate::Reg<flt3exmin::FLT3EXMINrs>;
///DFSDM filter 3 extremes detector minimum register
pub mod flt3exmin;
/**FLT3CNVTIMR (r) register accessor: DFSDM filter 3 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt3cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT3CNVTIMR)

For information about available fields see [`mod@flt3cnvtimr`] module*/
pub type FLT3CNVTIMR = crate::Reg<flt3cnvtimr::FLT3CNVTIMRrs>;
///DFSDM filter 3 conversion timer register
pub mod flt3cnvtimr;
/**FLT4CR1 (rw) register accessor: DFSDM filter 4 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt4cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4CR1)

For information about available fields see [`mod@flt4cr1`] module*/
pub type FLT4CR1 = crate::Reg<flt4cr1::FLT4CR1rs>;
///DFSDM filter 4 control register 1
pub mod flt4cr1;
/**FLT4CR2 (rw) register accessor: DFSDM filter 4 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt4cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4CR2)

For information about available fields see [`mod@flt4cr2`] module*/
pub type FLT4CR2 = crate::Reg<flt4cr2::FLT4CR2rs>;
///DFSDM filter 4 control register 2
pub mod flt4cr2;
/**FLT4ISR (r) register accessor: DFSDM filter 4 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4ISR)

For information about available fields see [`mod@flt4isr`] module*/
pub type FLT4ISR = crate::Reg<flt4isr::FLT4ISRrs>;
///DFSDM filter 4 interrupt and status register
pub mod flt4isr;
/**FLT4ICR (rw) register accessor: DFSDM filter 4 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt4icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4ICR)

For information about available fields see [`mod@flt4icr`] module*/
pub type FLT4ICR = crate::Reg<flt4icr::FLT4ICRrs>;
///DFSDM filter 4 interrupt flag clear register
pub mod flt4icr;
/**FLT4JCHGR (rw) register accessor: DFSDM filter 4 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt4jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4JCHGR)

For information about available fields see [`mod@flt4jchgr`] module*/
pub type FLT4JCHGR = crate::Reg<flt4jchgr::FLT4JCHGRrs>;
///DFSDM filter 4 injected channel group selection register
pub mod flt4jchgr;
/**FLT4FCR (rw) register accessor: DFSDM filter 4 control register

You can [`read`](crate::Reg::read) this register and get [`flt4fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4FCR)

For information about available fields see [`mod@flt4fcr`] module*/
pub type FLT4FCR = crate::Reg<flt4fcr::FLT4FCRrs>;
///DFSDM filter 4 control register
pub mod flt4fcr;
/**FLT4JDATAR (r) register accessor: DFSDM filter 4 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt4jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4JDATAR)

For information about available fields see [`mod@flt4jdatar`] module*/
pub type FLT4JDATAR = crate::Reg<flt4jdatar::FLT4JDATARrs>;
///DFSDM filter 4 data register for injected group
pub mod flt4jdatar;
/**FLT4RDATAR (r) register accessor: DFSDM filter 4 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt4rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4RDATAR)

For information about available fields see [`mod@flt4rdatar`] module*/
pub type FLT4RDATAR = crate::Reg<flt4rdatar::FLT4RDATARrs>;
///DFSDM filter 4 data register for the regular channel
pub mod flt4rdatar;
/**FLT4AWHTR (rw) register accessor: DFSDM filter 4 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt4awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4AWHTR)

For information about available fields see [`mod@flt4awhtr`] module*/
pub type FLT4AWHTR = crate::Reg<flt4awhtr::FLT4AWHTRrs>;
///DFSDM filter 4 analog watchdog high threshold register
pub mod flt4awhtr;
/**FLT4AWLTR (rw) register accessor: DFSDM filter 4 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt4awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4AWLTR)

For information about available fields see [`mod@flt4awltr`] module*/
pub type FLT4AWLTR = crate::Reg<flt4awltr::FLT4AWLTRrs>;
///DFSDM filter 4 analog watchdog low threshold register
pub mod flt4awltr;
/**FLT4AWSR (r) register accessor: DFSDM filter 4 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt4awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4AWSR)

For information about available fields see [`mod@flt4awsr`] module*/
pub type FLT4AWSR = crate::Reg<flt4awsr::FLT4AWSRrs>;
///DFSDM filter 4 analog watchdog status register
pub mod flt4awsr;
/**FLT4AWCFR (rw) register accessor: DFSDM filter 4 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt4awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4AWCFR)

For information about available fields see [`mod@flt4awcfr`] module*/
pub type FLT4AWCFR = crate::Reg<flt4awcfr::FLT4AWCFRrs>;
///DFSDM filter 4 analog watchdog clear flag register
pub mod flt4awcfr;
/**FLT4EXMAX (r) register accessor: DFSDM filter 4 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt4exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4EXMAX)

For information about available fields see [`mod@flt4exmax`] module*/
pub type FLT4EXMAX = crate::Reg<flt4exmax::FLT4EXMAXrs>;
///DFSDM filter 4 extremes detector maximum register
pub mod flt4exmax;
/**FLT4EXMIN (rw) register accessor: DFSDM filter 4 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt4exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4EXMIN)

For information about available fields see [`mod@flt4exmin`] module*/
pub type FLT4EXMIN = crate::Reg<flt4exmin::FLT4EXMINrs>;
///DFSDM filter 4 extremes detector minimum register
pub mod flt4exmin;
/**FLT4CNVTIMR (r) register accessor: DFSDM filter 4 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt4cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT4CNVTIMR)

For information about available fields see [`mod@flt4cnvtimr`] module*/
pub type FLT4CNVTIMR = crate::Reg<flt4cnvtimr::FLT4CNVTIMRrs>;
///DFSDM filter 4 conversion timer register
pub mod flt4cnvtimr;
/**FLT5CR1 (rw) register accessor: DFSDM filter 5 control register 1

You can [`read`](crate::Reg::read) this register and get [`flt5cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5CR1)

For information about available fields see [`mod@flt5cr1`] module*/
pub type FLT5CR1 = crate::Reg<flt5cr1::FLT5CR1rs>;
///DFSDM filter 5 control register 1
pub mod flt5cr1;
/**FLT5CR2 (rw) register accessor: DFSDM filter 5 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt5cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5CR2)

For information about available fields see [`mod@flt5cr2`] module*/
pub type FLT5CR2 = crate::Reg<flt5cr2::FLT5CR2rs>;
///DFSDM filter 5 control register 2
pub mod flt5cr2;
/**FLT5ISR (r) register accessor: DFSDM filter 5 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`flt5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5ISR)

For information about available fields see [`mod@flt5isr`] module*/
pub type FLT5ISR = crate::Reg<flt5isr::FLT5ISRrs>;
///DFSDM filter 5 interrupt and status register
pub mod flt5isr;
/**FLT5ICR (rw) register accessor: DFSDM filter 5 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5ICR)

For information about available fields see [`mod@flt5icr`] module*/
pub type FLT5ICR = crate::Reg<flt5icr::FLT5ICRrs>;
///DFSDM filter 5 interrupt flag clear register
pub mod flt5icr;
/**FLT5JCHGR (rw) register accessor: DFSDM filter 5 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt5jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5JCHGR)

For information about available fields see [`mod@flt5jchgr`] module*/
pub type FLT5JCHGR = crate::Reg<flt5jchgr::FLT5JCHGRrs>;
///DFSDM filter 5 injected channel group selection register
pub mod flt5jchgr;
/**FLT5FCR (rw) register accessor: DFSDM filter 5 control register

You can [`read`](crate::Reg::read) this register and get [`flt5fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5FCR)

For information about available fields see [`mod@flt5fcr`] module*/
pub type FLT5FCR = crate::Reg<flt5fcr::FLT5FCRrs>;
///DFSDM filter 5 control register
pub mod flt5fcr;
/**FLT5JDATAR (r) register accessor: DFSDM filter 5 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt5jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5JDATAR)

For information about available fields see [`mod@flt5jdatar`] module*/
pub type FLT5JDATAR = crate::Reg<flt5jdatar::FLT5JDATARrs>;
///DFSDM filter 5 data register for injected group
pub mod flt5jdatar;
/**FLT5RDATAR (r) register accessor: DFSDM filter 5 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt5rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5RDATAR)

For information about available fields see [`mod@flt5rdatar`] module*/
pub type FLT5RDATAR = crate::Reg<flt5rdatar::FLT5RDATARrs>;
///DFSDM filter 5 data register for the regular channel
pub mod flt5rdatar;
/**FLT5AWHTR (rw) register accessor: DFSDM filter 5 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt5awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5AWHTR)

For information about available fields see [`mod@flt5awhtr`] module*/
pub type FLT5AWHTR = crate::Reg<flt5awhtr::FLT5AWHTRrs>;
///DFSDM filter 5 analog watchdog high threshold register
pub mod flt5awhtr;
/**FLT5AWLTR (rw) register accessor: DFSDM filter 5 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`flt5awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5AWLTR)

For information about available fields see [`mod@flt5awltr`] module*/
pub type FLT5AWLTR = crate::Reg<flt5awltr::FLT5AWLTRrs>;
///DFSDM filter 5 analog watchdog low threshold register
pub mod flt5awltr;
/**FLT5AWSR (r) register accessor: DFSDM filter 5 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt5awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5AWSR)

For information about available fields see [`mod@flt5awsr`] module*/
pub type FLT5AWSR = crate::Reg<flt5awsr::FLT5AWSRrs>;
///DFSDM filter 5 analog watchdog status register
pub mod flt5awsr;
/**FLT5AWCFR (rw) register accessor: DFSDM filter 5 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`flt5awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5AWCFR)

For information about available fields see [`mod@flt5awcfr`] module*/
pub type FLT5AWCFR = crate::Reg<flt5awcfr::FLT5AWCFRrs>;
///DFSDM filter 5 analog watchdog clear flag register
pub mod flt5awcfr;
/**FLT5EXMAX (r) register accessor: DFSDM filter 5 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt5exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5EXMAX)

For information about available fields see [`mod@flt5exmax`] module*/
pub type FLT5EXMAX = crate::Reg<flt5exmax::FLT5EXMAXrs>;
///DFSDM filter 5 extremes detector maximum register
pub mod flt5exmax;
/**FLT5EXMIN (rw) register accessor: DFSDM filter 5 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt5exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt5exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5EXMIN)

For information about available fields see [`mod@flt5exmin`] module*/
pub type FLT5EXMIN = crate::Reg<flt5exmin::FLT5EXMINrs>;
///DFSDM filter 5 extremes detector minimum register
pub mod flt5exmin;
/**FLT5CNVTIMR (r) register accessor: DFSDM filter 5 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`flt5cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT5CNVTIMR)

For information about available fields see [`mod@flt5cnvtimr`] module*/
pub type FLT5CNVTIMR = crate::Reg<flt5cnvtimr::FLT5CNVTIMRrs>;
///DFSDM filter 5 conversion timer register
pub mod flt5cnvtimr;
/**HWCFGR (r) register accessor: This register specifies the hardware configuration of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///This register specifies the hardware configuration of DFSDM peripheral.
pub mod hwcfgr;
/**VERR (r) register accessor: This register specifies the version of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///This register specifies the version of DFSDM peripheral.
pub mod verr;
/**IPIDR (r) register accessor: This register specifies the identification of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///This register specifies the identification of DFSDM peripheral.
pub mod ipidr;
/**SIDR (r) register accessor: This register specifies the size allocated to DFSDM registers.

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///This register specifies the size allocated to DFSDM registers.
pub mod sidr;
