#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dfsdm_ch0cfgr1: DFSDM_CH0CFGR1,
    dfsdm_ch0cfgr2: DFSDM_CH0CFGR2,
    dfsdm_ch0awscdr: DFSDM_CH0AWSCDR,
    dfsdm_ch0wdatr: DFSDM_CH0WDATR,
    dfsdm_ch0datinr: DFSDM_CH0DATINR,
    dfsdm_ch0dlyr: DFSDM_CH0DLYR,
    _reserved6: [u8; 0x08],
    dfsdm_ch1cfgr1: DFSDM_CH1CFGR1,
    dfsdm_ch1cfgr2: DFSDM_CH1CFGR2,
    dfsdm_ch1awscdr: DFSDM_CH1AWSCDR,
    dfsdm_ch1wdatr: DFSDM_CH1WDATR,
    dfsdm_ch1datinr: DFSDM_CH1DATINR,
    dfsdm_ch1dlyr: DFSDM_CH1DLYR,
    _reserved12: [u8; 0x08],
    dfsdm_ch2cfgr1: DFSDM_CH2CFGR1,
    dfsdm_ch2cfgr2: DFSDM_CH2CFGR2,
    dfsdm_ch2awscdr: DFSDM_CH2AWSCDR,
    dfsdm_ch2wdatr: DFSDM_CH2WDATR,
    dfsdm_ch2datinr: DFSDM_CH2DATINR,
    dfsdm_ch2dlyr: DFSDM_CH2DLYR,
    _reserved18: [u8; 0x08],
    dfsdm_ch3cfgr1: DFSDM_CH3CFGR1,
    dfsdm_ch3cfgr2: DFSDM_CH3CFGR2,
    dfsdm_ch3awscdr: DFSDM_CH3AWSCDR,
    dfsdm_ch3wdatr: DFSDM_CH3WDATR,
    dfsdm_ch3datinr: DFSDM_CH3DATINR,
    dfsdm_ch3dlyr: DFSDM_CH3DLYR,
    _reserved24: [u8; 0x08],
    dfsdm_ch4cfgr1: DFSDM_CH4CFGR1,
    dfsdm_ch4cfgr2: DFSDM_CH4CFGR2,
    dfsdm_ch4awscdr: DFSDM_CH4AWSCDR,
    dfsdm_ch4wdatr: DFSDM_CH4WDATR,
    dfsdm_ch4datinr: DFSDM_CH4DATINR,
    dfsdm_ch4dlyr: DFSDM_CH4DLYR,
    _reserved30: [u8; 0x08],
    dfsdm_ch5cfgr1: DFSDM_CH5CFGR1,
    dfsdm_ch5cfgr2: DFSDM_CH5CFGR2,
    dfsdm_ch5awscdr: DFSDM_CH5AWSCDR,
    dfsdm_ch5wdatr: DFSDM_CH5WDATR,
    dfsdm_ch5datinr: DFSDM_CH5DATINR,
    dfsdm_ch5dlyr: DFSDM_CH5DLYR,
    _reserved36: [u8; 0x08],
    dfsdm_ch6cfgr1: DFSDM_CH6CFGR1,
    dfsdm_ch6cfgr2: DFSDM_CH6CFGR2,
    dfsdm_ch6awscdr: DFSDM_CH6AWSCDR,
    dfsdm_ch6wdatr: DFSDM_CH6WDATR,
    dfsdm_ch6datinr: DFSDM_CH6DATINR,
    dfsdm_ch6dlyr: DFSDM_CH6DLYR,
    _reserved42: [u8; 0x08],
    dfsdm_ch7cfgr1: DFSDM_CH7CFGR1,
    dfsdm_ch7cfgr2: DFSDM_CH7CFGR2,
    dfsdm_ch7awscdr: DFSDM_CH7AWSCDR,
    dfsdm_ch7wdatr: DFSDM_CH7WDATR,
    dfsdm_ch7datinr: DFSDM_CH7DATINR,
    dfsdm_ch7dlyr: DFSDM_CH7DLYR,
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
    dfsdm_flt1jchgr: DFSDM_FLT1JCHGR,
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
    _reserved108: [u8; 0x44],
    dfsdm_flt4cr1: DFSDM_FLT4CR1,
    dfsdm_flt4cr2: DFSDM_FLT4CR2,
    dfsdm_flt4isr: DFSDM_FLT4ISR,
    dfsdm_flt4icr: DFSDM_FLT4ICR,
    dfsdm_flt4jchgr: DFSDM_FLT4JCHGR,
    dfsdm_flt4fcr: DFSDM_FLT4FCR,
    dfsdm_flt4jdatar: DFSDM_FLT4JDATAR,
    dfsdm_flt4rdatar: DFSDM_FLT4RDATAR,
    dfsdm_flt4awhtr: DFSDM_FLT4AWHTR,
    dfsdm_flt4awltr: DFSDM_FLT4AWLTR,
    dfsdm_flt4awsr: DFSDM_FLT4AWSR,
    dfsdm_flt4awcfr: DFSDM_FLT4AWCFR,
    dfsdm_flt4exmax: DFSDM_FLT4EXMAX,
    dfsdm_flt4exmin: DFSDM_FLT4EXMIN,
    dfsdm_flt4cnvtimr: DFSDM_FLT4CNVTIMR,
    _reserved123: [u8; 0x44],
    dfsdm_flt5cr1: DFSDM_FLT5CR1,
    dfsdm_flt5cr2: DFSDM_FLT5CR2,
    dfsdm_flt5isr: DFSDM_FLT5ISR,
    dfsdm_flt5icr: DFSDM_FLT5ICR,
    dfsdm_flt5jchgr: DFSDM_FLT5JCHGR,
    dfsdm_flt5fcr: DFSDM_FLT5FCR,
    dfsdm_flt5jdatar: DFSDM_FLT5JDATAR,
    dfsdm_flt5rdatar: DFSDM_FLT5RDATAR,
    dfsdm_flt5awhtr: DFSDM_FLT5AWHTR,
    dfsdm_flt5awltr: DFSDM_FLT5AWLTR,
    dfsdm_flt5awsr: DFSDM_FLT5AWSR,
    dfsdm_flt5awcfr: DFSDM_FLT5AWCFR,
    dfsdm_flt5exmax: DFSDM_FLT5EXMAX,
    dfsdm_flt5exmin: DFSDM_FLT5EXMIN,
    dfsdm_flt5cnvtimr: DFSDM_FLT5CNVTIMR,
    _reserved138: [u8; 0x0434],
    dfsdm_hwcfgr: DFSDM_HWCFGR,
    dfsdm_verr: DFSDM_VERR,
    dfsdm_ipidr: DFSDM_IPIDR,
    dfsdm_sidr: DFSDM_SIDR,
}
impl RegisterBlock {
    ///0x00 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch0cfgr1(&self) -> &DFSDM_CH0CFGR1 {
        &self.dfsdm_ch0cfgr1
    }
    ///0x04 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch0cfgr2(&self) -> &DFSDM_CH0CFGR2 {
        &self.dfsdm_ch0cfgr2
    }
    ///0x08 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch0awscdr(&self) -> &DFSDM_CH0AWSCDR {
        &self.dfsdm_ch0awscdr
    }
    ///0x0c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch0wdatr(&self) -> &DFSDM_CH0WDATR {
        &self.dfsdm_ch0wdatr
    }
    ///0x10 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch0datinr(&self) -> &DFSDM_CH0DATINR {
        &self.dfsdm_ch0datinr
    }
    ///0x14 - DFSDM channel 0 delay register
    #[inline(always)]
    pub const fn dfsdm_ch0dlyr(&self) -> &DFSDM_CH0DLYR {
        &self.dfsdm_ch0dlyr
    }
    ///0x20 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch1cfgr1(&self) -> &DFSDM_CH1CFGR1 {
        &self.dfsdm_ch1cfgr1
    }
    ///0x24 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch1cfgr2(&self) -> &DFSDM_CH1CFGR2 {
        &self.dfsdm_ch1cfgr2
    }
    ///0x28 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch1awscdr(&self) -> &DFSDM_CH1AWSCDR {
        &self.dfsdm_ch1awscdr
    }
    ///0x2c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch1wdatr(&self) -> &DFSDM_CH1WDATR {
        &self.dfsdm_ch1wdatr
    }
    ///0x30 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch1datinr(&self) -> &DFSDM_CH1DATINR {
        &self.dfsdm_ch1datinr
    }
    ///0x34 - DFSDM channel 1 delay register
    #[inline(always)]
    pub const fn dfsdm_ch1dlyr(&self) -> &DFSDM_CH1DLYR {
        &self.dfsdm_ch1dlyr
    }
    ///0x40 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch2cfgr1(&self) -> &DFSDM_CH2CFGR1 {
        &self.dfsdm_ch2cfgr1
    }
    ///0x44 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch2cfgr2(&self) -> &DFSDM_CH2CFGR2 {
        &self.dfsdm_ch2cfgr2
    }
    ///0x48 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch2awscdr(&self) -> &DFSDM_CH2AWSCDR {
        &self.dfsdm_ch2awscdr
    }
    ///0x4c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch2wdatr(&self) -> &DFSDM_CH2WDATR {
        &self.dfsdm_ch2wdatr
    }
    ///0x50 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch2datinr(&self) -> &DFSDM_CH2DATINR {
        &self.dfsdm_ch2datinr
    }
    ///0x54 - DFSDM channel 2 delay register
    #[inline(always)]
    pub const fn dfsdm_ch2dlyr(&self) -> &DFSDM_CH2DLYR {
        &self.dfsdm_ch2dlyr
    }
    ///0x60 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch3cfgr1(&self) -> &DFSDM_CH3CFGR1 {
        &self.dfsdm_ch3cfgr1
    }
    ///0x64 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch3cfgr2(&self) -> &DFSDM_CH3CFGR2 {
        &self.dfsdm_ch3cfgr2
    }
    ///0x68 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch3awscdr(&self) -> &DFSDM_CH3AWSCDR {
        &self.dfsdm_ch3awscdr
    }
    ///0x6c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch3wdatr(&self) -> &DFSDM_CH3WDATR {
        &self.dfsdm_ch3wdatr
    }
    ///0x70 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch3datinr(&self) -> &DFSDM_CH3DATINR {
        &self.dfsdm_ch3datinr
    }
    ///0x74 - DFSDM channel 3 delay register
    #[inline(always)]
    pub const fn dfsdm_ch3dlyr(&self) -> &DFSDM_CH3DLYR {
        &self.dfsdm_ch3dlyr
    }
    ///0x80 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch4cfgr1(&self) -> &DFSDM_CH4CFGR1 {
        &self.dfsdm_ch4cfgr1
    }
    ///0x84 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch4cfgr2(&self) -> &DFSDM_CH4CFGR2 {
        &self.dfsdm_ch4cfgr2
    }
    ///0x88 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch4awscdr(&self) -> &DFSDM_CH4AWSCDR {
        &self.dfsdm_ch4awscdr
    }
    ///0x8c - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch4wdatr(&self) -> &DFSDM_CH4WDATR {
        &self.dfsdm_ch4wdatr
    }
    ///0x90 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch4datinr(&self) -> &DFSDM_CH4DATINR {
        &self.dfsdm_ch4datinr
    }
    ///0x94 - DFSDM channel 4 delay register
    #[inline(always)]
    pub const fn dfsdm_ch4dlyr(&self) -> &DFSDM_CH4DLYR {
        &self.dfsdm_ch4dlyr
    }
    ///0xa0 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch5cfgr1(&self) -> &DFSDM_CH5CFGR1 {
        &self.dfsdm_ch5cfgr1
    }
    ///0xa4 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch5cfgr2(&self) -> &DFSDM_CH5CFGR2 {
        &self.dfsdm_ch5cfgr2
    }
    ///0xa8 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch5awscdr(&self) -> &DFSDM_CH5AWSCDR {
        &self.dfsdm_ch5awscdr
    }
    ///0xac - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch5wdatr(&self) -> &DFSDM_CH5WDATR {
        &self.dfsdm_ch5wdatr
    }
    ///0xb0 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch5datinr(&self) -> &DFSDM_CH5DATINR {
        &self.dfsdm_ch5datinr
    }
    ///0xb4 - DFSDM channel 5 delay register
    #[inline(always)]
    pub const fn dfsdm_ch5dlyr(&self) -> &DFSDM_CH5DLYR {
        &self.dfsdm_ch5dlyr
    }
    ///0xc0 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch6cfgr1(&self) -> &DFSDM_CH6CFGR1 {
        &self.dfsdm_ch6cfgr1
    }
    ///0xc4 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch6cfgr2(&self) -> &DFSDM_CH6CFGR2 {
        &self.dfsdm_ch6cfgr2
    }
    ///0xc8 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch6awscdr(&self) -> &DFSDM_CH6AWSCDR {
        &self.dfsdm_ch6awscdr
    }
    ///0xcc - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch6wdatr(&self) -> &DFSDM_CH6WDATR {
        &self.dfsdm_ch6wdatr
    }
    ///0xd0 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch6datinr(&self) -> &DFSDM_CH6DATINR {
        &self.dfsdm_ch6datinr
    }
    ///0xd4 - DFSDM channel 6 delay register
    #[inline(always)]
    pub const fn dfsdm_ch6dlyr(&self) -> &DFSDM_CH6DLYR {
        &self.dfsdm_ch6dlyr
    }
    ///0xe0 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch7cfgr1(&self) -> &DFSDM_CH7CFGR1 {
        &self.dfsdm_ch7cfgr1
    }
    ///0xe4 - This register specifies the parameters used by channel y.
    #[inline(always)]
    pub const fn dfsdm_ch7cfgr2(&self) -> &DFSDM_CH7CFGR2 {
        &self.dfsdm_ch7cfgr2
    }
    ///0xe8 - Short-circuit detector and analog watchdog settings for channel y.
    #[inline(always)]
    pub const fn dfsdm_ch7awscdr(&self) -> &DFSDM_CH7AWSCDR {
        &self.dfsdm_ch7awscdr
    }
    ///0xec - This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    #[inline(always)]
    pub const fn dfsdm_ch7wdatr(&self) -> &DFSDM_CH7WDATR {
        &self.dfsdm_ch7wdatr
    }
    ///0xf0 - This register contains 16-bit input data to be processed by DFSDM filter module.
    #[inline(always)]
    pub const fn dfsdm_ch7datinr(&self) -> &DFSDM_CH7DATINR {
        &self.dfsdm_ch7datinr
    }
    ///0xf4 - DFSDM channel 7 delay register
    #[inline(always)]
    pub const fn dfsdm_ch7dlyr(&self) -> &DFSDM_CH7DLYR {
        &self.dfsdm_ch7dlyr
    }
    ///0x100 - DFSDM filter 0 control register 1
    #[inline(always)]
    pub const fn dfsdm_flt0cr1(&self) -> &DFSDM_FLT0CR1 {
        &self.dfsdm_flt0cr1
    }
    ///0x104 - DFSDM filter 0 control register 2
    #[inline(always)]
    pub const fn dfsdm_flt0cr2(&self) -> &DFSDM_FLT0CR2 {
        &self.dfsdm_flt0cr2
    }
    ///0x108 - DFSDM filter 0 interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt0isr(&self) -> &DFSDM_FLT0ISR {
        &self.dfsdm_flt0isr
    }
    ///0x10c - DFSDM filter 0 interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt0icr(&self) -> &DFSDM_FLT0ICR {
        &self.dfsdm_flt0icr
    }
    ///0x110 - DFSDM filter 0 injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt0jchgr(&self) -> &DFSDM_FLT0JCHGR {
        &self.dfsdm_flt0jchgr
    }
    ///0x114 - DFSDM filter 0 control register
    #[inline(always)]
    pub const fn dfsdm_flt0fcr(&self) -> &DFSDM_FLT0FCR {
        &self.dfsdm_flt0fcr
    }
    ///0x118 - DFSDM filter 0 data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt0jdatar(&self) -> &DFSDM_FLT0JDATAR {
        &self.dfsdm_flt0jdatar
    }
    ///0x11c - DFSDM filter 0 data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt0rdatar(&self) -> &DFSDM_FLT0RDATAR {
        &self.dfsdm_flt0rdatar
    }
    ///0x120 - DFSDM filter 0 analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt0awhtr(&self) -> &DFSDM_FLT0AWHTR {
        &self.dfsdm_flt0awhtr
    }
    ///0x124 - DFSDM filter 0 analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt0awltr(&self) -> &DFSDM_FLT0AWLTR {
        &self.dfsdm_flt0awltr
    }
    ///0x128 - DFSDM filter 0 analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt0awsr(&self) -> &DFSDM_FLT0AWSR {
        &self.dfsdm_flt0awsr
    }
    ///0x12c - DFSDM filter 0 analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt0awcfr(&self) -> &DFSDM_FLT0AWCFR {
        &self.dfsdm_flt0awcfr
    }
    ///0x130 - DFSDM filter 0 extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt0exmax(&self) -> &DFSDM_FLT0EXMAX {
        &self.dfsdm_flt0exmax
    }
    ///0x134 - DFSDM filter 0 extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt0exmin(&self) -> &DFSDM_FLT0EXMIN {
        &self.dfsdm_flt0exmin
    }
    ///0x138 - DFSDM filter 0 conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt0cnvtimr(&self) -> &DFSDM_FLT0CNVTIMR {
        &self.dfsdm_flt0cnvtimr
    }
    ///0x180 - DFSDM filter 1 control register 1
    #[inline(always)]
    pub const fn dfsdm_flt1cr1(&self) -> &DFSDM_FLT1CR1 {
        &self.dfsdm_flt1cr1
    }
    ///0x184 - DFSDM filter 1 control register 2
    #[inline(always)]
    pub const fn dfsdm_flt1cr2(&self) -> &DFSDM_FLT1CR2 {
        &self.dfsdm_flt1cr2
    }
    ///0x188 - DFSDM filter 1 interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt1isr(&self) -> &DFSDM_FLT1ISR {
        &self.dfsdm_flt1isr
    }
    ///0x18c - DFSDM filter 1 interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt1icr(&self) -> &DFSDM_FLT1ICR {
        &self.dfsdm_flt1icr
    }
    ///0x190 - DFSDM filter 1 injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt1jchgr(&self) -> &DFSDM_FLT1JCHGR {
        &self.dfsdm_flt1jchgr
    }
    ///0x194 - DFSDM filter 1 control register
    #[inline(always)]
    pub const fn dfsdm_flt1fcr(&self) -> &DFSDM_FLT1FCR {
        &self.dfsdm_flt1fcr
    }
    ///0x198 - DFSDM filter 1 data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt1jdatar(&self) -> &DFSDM_FLT1JDATAR {
        &self.dfsdm_flt1jdatar
    }
    ///0x19c - DFSDM filter 1 data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt1rdatar(&self) -> &DFSDM_FLT1RDATAR {
        &self.dfsdm_flt1rdatar
    }
    ///0x1a0 - DFSDM filter 1 analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt1awhtr(&self) -> &DFSDM_FLT1AWHTR {
        &self.dfsdm_flt1awhtr
    }
    ///0x1a4 - DFSDM filter 1 analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt1awltr(&self) -> &DFSDM_FLT1AWLTR {
        &self.dfsdm_flt1awltr
    }
    ///0x1a8 - DFSDM filter 1 analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt1awsr(&self) -> &DFSDM_FLT1AWSR {
        &self.dfsdm_flt1awsr
    }
    ///0x1ac - DFSDM filter 1 analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt1awcfr(&self) -> &DFSDM_FLT1AWCFR {
        &self.dfsdm_flt1awcfr
    }
    ///0x1b0 - DFSDM filter 1 extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt1exmax(&self) -> &DFSDM_FLT1EXMAX {
        &self.dfsdm_flt1exmax
    }
    ///0x1b4 - DFSDM filter 1 extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt1exmin(&self) -> &DFSDM_FLT1EXMIN {
        &self.dfsdm_flt1exmin
    }
    ///0x1b8 - DFSDM filter 1 conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt1cnvtimr(&self) -> &DFSDM_FLT1CNVTIMR {
        &self.dfsdm_flt1cnvtimr
    }
    ///0x200 - DFSDM filter 2 control register 1
    #[inline(always)]
    pub const fn dfsdm_flt2cr1(&self) -> &DFSDM_FLT2CR1 {
        &self.dfsdm_flt2cr1
    }
    ///0x204 - DFSDM filter 2 control register 2
    #[inline(always)]
    pub const fn dfsdm_flt2cr2(&self) -> &DFSDM_FLT2CR2 {
        &self.dfsdm_flt2cr2
    }
    ///0x208 - DFSDM filter 2 interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt2isr(&self) -> &DFSDM_FLT2ISR {
        &self.dfsdm_flt2isr
    }
    ///0x20c - DFSDM filter 2 interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt2icr(&self) -> &DFSDM_FLT2ICR {
        &self.dfsdm_flt2icr
    }
    ///0x210 - DFSDM filter 2 injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt2jchgr(&self) -> &DFSDM_FLT2JCHGR {
        &self.dfsdm_flt2jchgr
    }
    ///0x214 - DFSDM filter 2 control register
    #[inline(always)]
    pub const fn dfsdm_flt2fcr(&self) -> &DFSDM_FLT2FCR {
        &self.dfsdm_flt2fcr
    }
    ///0x218 - DFSDM filter 2 data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt2jdatar(&self) -> &DFSDM_FLT2JDATAR {
        &self.dfsdm_flt2jdatar
    }
    ///0x21c - DFSDM filter 2 data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt2rdatar(&self) -> &DFSDM_FLT2RDATAR {
        &self.dfsdm_flt2rdatar
    }
    ///0x220 - DFSDM filter 2 analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt2awhtr(&self) -> &DFSDM_FLT2AWHTR {
        &self.dfsdm_flt2awhtr
    }
    ///0x224 - DFSDM filter 2 analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt2awltr(&self) -> &DFSDM_FLT2AWLTR {
        &self.dfsdm_flt2awltr
    }
    ///0x228 - DFSDM filter 2 analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt2awsr(&self) -> &DFSDM_FLT2AWSR {
        &self.dfsdm_flt2awsr
    }
    ///0x22c - DFSDM filter 2 analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt2awcfr(&self) -> &DFSDM_FLT2AWCFR {
        &self.dfsdm_flt2awcfr
    }
    ///0x230 - DFSDM filter 2 extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt2exmax(&self) -> &DFSDM_FLT2EXMAX {
        &self.dfsdm_flt2exmax
    }
    ///0x234 - DFSDM filter 2 extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt2exmin(&self) -> &DFSDM_FLT2EXMIN {
        &self.dfsdm_flt2exmin
    }
    ///0x238 - DFSDM filter 2 conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt2cnvtimr(&self) -> &DFSDM_FLT2CNVTIMR {
        &self.dfsdm_flt2cnvtimr
    }
    ///0x280 - DFSDM filter 3 control register 1
    #[inline(always)]
    pub const fn dfsdm_flt3cr1(&self) -> &DFSDM_FLT3CR1 {
        &self.dfsdm_flt3cr1
    }
    ///0x284 - DFSDM filter 3 control register 2
    #[inline(always)]
    pub const fn dfsdm_flt3cr2(&self) -> &DFSDM_FLT3CR2 {
        &self.dfsdm_flt3cr2
    }
    ///0x288 - DFSDM filter 3 interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt3isr(&self) -> &DFSDM_FLT3ISR {
        &self.dfsdm_flt3isr
    }
    ///0x28c - DFSDM filter 3 interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt3icr(&self) -> &DFSDM_FLT3ICR {
        &self.dfsdm_flt3icr
    }
    ///0x290 - DFSDM filter 3 injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt3jchgr(&self) -> &DFSDM_FLT3JCHGR {
        &self.dfsdm_flt3jchgr
    }
    ///0x294 - DFSDM filter 3 control register
    #[inline(always)]
    pub const fn dfsdm_flt3fcr(&self) -> &DFSDM_FLT3FCR {
        &self.dfsdm_flt3fcr
    }
    ///0x298 - DFSDM filter 3 data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt3jdatar(&self) -> &DFSDM_FLT3JDATAR {
        &self.dfsdm_flt3jdatar
    }
    ///0x29c - DFSDM filter 3 data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt3rdatar(&self) -> &DFSDM_FLT3RDATAR {
        &self.dfsdm_flt3rdatar
    }
    ///0x2a0 - DFSDM filter 3 analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt3awhtr(&self) -> &DFSDM_FLT3AWHTR {
        &self.dfsdm_flt3awhtr
    }
    ///0x2a4 - DFSDM filter 3 analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt3awltr(&self) -> &DFSDM_FLT3AWLTR {
        &self.dfsdm_flt3awltr
    }
    ///0x2a8 - DFSDM filter 3 analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt3awsr(&self) -> &DFSDM_FLT3AWSR {
        &self.dfsdm_flt3awsr
    }
    ///0x2ac - DFSDM filter 3 analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt3awcfr(&self) -> &DFSDM_FLT3AWCFR {
        &self.dfsdm_flt3awcfr
    }
    ///0x2b0 - DFSDM filter 3 extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt3exmax(&self) -> &DFSDM_FLT3EXMAX {
        &self.dfsdm_flt3exmax
    }
    ///0x2b4 - DFSDM filter 3 extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt3exmin(&self) -> &DFSDM_FLT3EXMIN {
        &self.dfsdm_flt3exmin
    }
    ///0x2b8 - DFSDM filter 3 conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt3cnvtimr(&self) -> &DFSDM_FLT3CNVTIMR {
        &self.dfsdm_flt3cnvtimr
    }
    ///0x300 - DFSDM filter 4 control register 1
    #[inline(always)]
    pub const fn dfsdm_flt4cr1(&self) -> &DFSDM_FLT4CR1 {
        &self.dfsdm_flt4cr1
    }
    ///0x304 - DFSDM filter 4 control register 2
    #[inline(always)]
    pub const fn dfsdm_flt4cr2(&self) -> &DFSDM_FLT4CR2 {
        &self.dfsdm_flt4cr2
    }
    ///0x308 - DFSDM filter 4 interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt4isr(&self) -> &DFSDM_FLT4ISR {
        &self.dfsdm_flt4isr
    }
    ///0x30c - DFSDM filter 4 interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt4icr(&self) -> &DFSDM_FLT4ICR {
        &self.dfsdm_flt4icr
    }
    ///0x310 - DFSDM filter 4 injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt4jchgr(&self) -> &DFSDM_FLT4JCHGR {
        &self.dfsdm_flt4jchgr
    }
    ///0x314 - DFSDM filter 4 control register
    #[inline(always)]
    pub const fn dfsdm_flt4fcr(&self) -> &DFSDM_FLT4FCR {
        &self.dfsdm_flt4fcr
    }
    ///0x318 - DFSDM filter 4 data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt4jdatar(&self) -> &DFSDM_FLT4JDATAR {
        &self.dfsdm_flt4jdatar
    }
    ///0x31c - DFSDM filter 4 data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt4rdatar(&self) -> &DFSDM_FLT4RDATAR {
        &self.dfsdm_flt4rdatar
    }
    ///0x320 - DFSDM filter 4 analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt4awhtr(&self) -> &DFSDM_FLT4AWHTR {
        &self.dfsdm_flt4awhtr
    }
    ///0x324 - DFSDM filter 4 analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt4awltr(&self) -> &DFSDM_FLT4AWLTR {
        &self.dfsdm_flt4awltr
    }
    ///0x328 - DFSDM filter 4 analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt4awsr(&self) -> &DFSDM_FLT4AWSR {
        &self.dfsdm_flt4awsr
    }
    ///0x32c - DFSDM filter 4 analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt4awcfr(&self) -> &DFSDM_FLT4AWCFR {
        &self.dfsdm_flt4awcfr
    }
    ///0x330 - DFSDM filter 4 extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt4exmax(&self) -> &DFSDM_FLT4EXMAX {
        &self.dfsdm_flt4exmax
    }
    ///0x334 - DFSDM filter 4 extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt4exmin(&self) -> &DFSDM_FLT4EXMIN {
        &self.dfsdm_flt4exmin
    }
    ///0x338 - DFSDM filter 4 conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt4cnvtimr(&self) -> &DFSDM_FLT4CNVTIMR {
        &self.dfsdm_flt4cnvtimr
    }
    ///0x380 - DFSDM filter 5 control register 1
    #[inline(always)]
    pub const fn dfsdm_flt5cr1(&self) -> &DFSDM_FLT5CR1 {
        &self.dfsdm_flt5cr1
    }
    ///0x384 - DFSDM filter 5 control register 2
    #[inline(always)]
    pub const fn dfsdm_flt5cr2(&self) -> &DFSDM_FLT5CR2 {
        &self.dfsdm_flt5cr2
    }
    ///0x388 - DFSDM filter 5 interrupt and status register
    #[inline(always)]
    pub const fn dfsdm_flt5isr(&self) -> &DFSDM_FLT5ISR {
        &self.dfsdm_flt5isr
    }
    ///0x38c - DFSDM filter 5 interrupt flag clear register
    #[inline(always)]
    pub const fn dfsdm_flt5icr(&self) -> &DFSDM_FLT5ICR {
        &self.dfsdm_flt5icr
    }
    ///0x390 - DFSDM filter 5 injected channel group selection register
    #[inline(always)]
    pub const fn dfsdm_flt5jchgr(&self) -> &DFSDM_FLT5JCHGR {
        &self.dfsdm_flt5jchgr
    }
    ///0x394 - DFSDM filter 5 control register
    #[inline(always)]
    pub const fn dfsdm_flt5fcr(&self) -> &DFSDM_FLT5FCR {
        &self.dfsdm_flt5fcr
    }
    ///0x398 - DFSDM filter 5 data register for injected group
    #[inline(always)]
    pub const fn dfsdm_flt5jdatar(&self) -> &DFSDM_FLT5JDATAR {
        &self.dfsdm_flt5jdatar
    }
    ///0x39c - DFSDM filter 5 data register for the regular channel
    #[inline(always)]
    pub const fn dfsdm_flt5rdatar(&self) -> &DFSDM_FLT5RDATAR {
        &self.dfsdm_flt5rdatar
    }
    ///0x3a0 - DFSDM filter 5 analog watchdog high threshold register
    #[inline(always)]
    pub const fn dfsdm_flt5awhtr(&self) -> &DFSDM_FLT5AWHTR {
        &self.dfsdm_flt5awhtr
    }
    ///0x3a4 - DFSDM filter 5 analog watchdog low threshold register
    #[inline(always)]
    pub const fn dfsdm_flt5awltr(&self) -> &DFSDM_FLT5AWLTR {
        &self.dfsdm_flt5awltr
    }
    ///0x3a8 - DFSDM filter 5 analog watchdog status register
    #[inline(always)]
    pub const fn dfsdm_flt5awsr(&self) -> &DFSDM_FLT5AWSR {
        &self.dfsdm_flt5awsr
    }
    ///0x3ac - DFSDM filter 5 analog watchdog clear flag register
    #[inline(always)]
    pub const fn dfsdm_flt5awcfr(&self) -> &DFSDM_FLT5AWCFR {
        &self.dfsdm_flt5awcfr
    }
    ///0x3b0 - DFSDM filter 5 extremes detector maximum register
    #[inline(always)]
    pub const fn dfsdm_flt5exmax(&self) -> &DFSDM_FLT5EXMAX {
        &self.dfsdm_flt5exmax
    }
    ///0x3b4 - DFSDM filter 5 extremes detector minimum register
    #[inline(always)]
    pub const fn dfsdm_flt5exmin(&self) -> &DFSDM_FLT5EXMIN {
        &self.dfsdm_flt5exmin
    }
    ///0x3b8 - DFSDM filter 5 conversion timer register
    #[inline(always)]
    pub const fn dfsdm_flt5cnvtimr(&self) -> &DFSDM_FLT5CNVTIMR {
        &self.dfsdm_flt5cnvtimr
    }
    ///0x7f0 - This register specifies the hardware configuration of DFSDM peripheral.
    #[inline(always)]
    pub const fn dfsdm_hwcfgr(&self) -> &DFSDM_HWCFGR {
        &self.dfsdm_hwcfgr
    }
    ///0x7f4 - This register specifies the version of DFSDM peripheral.
    #[inline(always)]
    pub const fn dfsdm_verr(&self) -> &DFSDM_VERR {
        &self.dfsdm_verr
    }
    ///0x7f8 - This register specifies the identification of DFSDM peripheral.
    #[inline(always)]
    pub const fn dfsdm_ipidr(&self) -> &DFSDM_IPIDR {
        &self.dfsdm_ipidr
    }
    ///0x7fc - This register specifies the size allocated to DFSDM registers.
    #[inline(always)]
    pub const fn dfsdm_sidr(&self) -> &DFSDM_SIDR {
        &self.dfsdm_sidr
    }
}
/**DFSDM_CH0CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0CFGR1)

For information about available fields see [`mod@dfsdm_ch0cfgr1`]
module*/
pub type DFSDM_CH0CFGR1 = crate::Reg<dfsdm_ch0cfgr1::DFSDM_CH0CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch0cfgr1;
/**DFSDM_CH0CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0CFGR2)

For information about available fields see [`mod@dfsdm_ch0cfgr2`]
module*/
pub type DFSDM_CH0CFGR2 = crate::Reg<dfsdm_ch0cfgr2::DFSDM_CH0CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch0cfgr2;
/**DFSDM_CH0AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0AWSCDR)

For information about available fields see [`mod@dfsdm_ch0awscdr`]
module*/
pub type DFSDM_CH0AWSCDR = crate::Reg<dfsdm_ch0awscdr::DFSDM_CH0AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch0awscdr;
/**DFSDM_CH0WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0WDATR)

For information about available fields see [`mod@dfsdm_ch0wdatr`]
module*/
pub type DFSDM_CH0WDATR = crate::Reg<dfsdm_ch0wdatr::DFSDM_CH0WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch0wdatr;
/**DFSDM_CH0DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0DATINR)

For information about available fields see [`mod@dfsdm_ch0datinr`]
module*/
pub type DFSDM_CH0DATINR = crate::Reg<dfsdm_ch0datinr::DFSDM_CH0DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch0datinr;
/**DFSDM_CH0DLYR (rw) register accessor: DFSDM channel 0 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0DLYR)

For information about available fields see [`mod@dfsdm_ch0dlyr`]
module*/
pub type DFSDM_CH0DLYR = crate::Reg<dfsdm_ch0dlyr::DFSDM_CH0DLYRrs>;
///DFSDM channel 0 delay register
pub mod dfsdm_ch0dlyr;
/**DFSDM_CH1CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH1CFGR1)

For information about available fields see [`mod@dfsdm_ch1cfgr1`]
module*/
pub type DFSDM_CH1CFGR1 = crate::Reg<dfsdm_ch1cfgr1::DFSDM_CH1CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch1cfgr1;
/**DFSDM_CH1CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH1CFGR2)

For information about available fields see [`mod@dfsdm_ch1cfgr2`]
module*/
pub type DFSDM_CH1CFGR2 = crate::Reg<dfsdm_ch1cfgr2::DFSDM_CH1CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch1cfgr2;
/**DFSDM_CH1AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch1awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH1AWSCDR)

For information about available fields see [`mod@dfsdm_ch1awscdr`]
module*/
pub type DFSDM_CH1AWSCDR = crate::Reg<dfsdm_ch1awscdr::DFSDM_CH1AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch1awscdr;
/**DFSDM_CH1WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH1WDATR)

For information about available fields see [`mod@dfsdm_ch1wdatr`]
module*/
pub type DFSDM_CH1WDATR = crate::Reg<dfsdm_ch1wdatr::DFSDM_CH1WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch1wdatr;
/**DFSDM_CH1DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch1datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH1DATINR)

For information about available fields see [`mod@dfsdm_ch1datinr`]
module*/
pub type DFSDM_CH1DATINR = crate::Reg<dfsdm_ch1datinr::DFSDM_CH1DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch1datinr;
/**DFSDM_CH1DLYR (rw) register accessor: DFSDM channel 1 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch1dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH1DLYR)

For information about available fields see [`mod@dfsdm_ch1dlyr`]
module*/
pub type DFSDM_CH1DLYR = crate::Reg<dfsdm_ch1dlyr::DFSDM_CH1DLYRrs>;
///DFSDM channel 1 delay register
pub mod dfsdm_ch1dlyr;
/**DFSDM_CH2CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH2CFGR1)

For information about available fields see [`mod@dfsdm_ch2cfgr1`]
module*/
pub type DFSDM_CH2CFGR1 = crate::Reg<dfsdm_ch2cfgr1::DFSDM_CH2CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch2cfgr1;
/**DFSDM_CH2CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH2CFGR2)

For information about available fields see [`mod@dfsdm_ch2cfgr2`]
module*/
pub type DFSDM_CH2CFGR2 = crate::Reg<dfsdm_ch2cfgr2::DFSDM_CH2CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch2cfgr2;
/**DFSDM_CH2AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch2awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch2awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH2AWSCDR)

For information about available fields see [`mod@dfsdm_ch2awscdr`]
module*/
pub type DFSDM_CH2AWSCDR = crate::Reg<dfsdm_ch2awscdr::DFSDM_CH2AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch2awscdr;
/**DFSDM_CH2WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch2wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH2WDATR)

For information about available fields see [`mod@dfsdm_ch2wdatr`]
module*/
pub type DFSDM_CH2WDATR = crate::Reg<dfsdm_ch2wdatr::DFSDM_CH2WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch2wdatr;
/**DFSDM_CH2DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch2datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch2datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH2DATINR)

For information about available fields see [`mod@dfsdm_ch2datinr`]
module*/
pub type DFSDM_CH2DATINR = crate::Reg<dfsdm_ch2datinr::DFSDM_CH2DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch2datinr;
/**DFSDM_CH2DLYR (rw) register accessor: DFSDM channel 2 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch2dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch2dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH2DLYR)

For information about available fields see [`mod@dfsdm_ch2dlyr`]
module*/
pub type DFSDM_CH2DLYR = crate::Reg<dfsdm_ch2dlyr::DFSDM_CH2DLYRrs>;
///DFSDM channel 2 delay register
pub mod dfsdm_ch2dlyr;
/**DFSDM_CH3CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH3CFGR1)

For information about available fields see [`mod@dfsdm_ch3cfgr1`]
module*/
pub type DFSDM_CH3CFGR1 = crate::Reg<dfsdm_ch3cfgr1::DFSDM_CH3CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch3cfgr1;
/**DFSDM_CH3CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH3CFGR2)

For information about available fields see [`mod@dfsdm_ch3cfgr2`]
module*/
pub type DFSDM_CH3CFGR2 = crate::Reg<dfsdm_ch3cfgr2::DFSDM_CH3CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch3cfgr2;
/**DFSDM_CH3AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch3awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH3AWSCDR)

For information about available fields see [`mod@dfsdm_ch3awscdr`]
module*/
pub type DFSDM_CH3AWSCDR = crate::Reg<dfsdm_ch3awscdr::DFSDM_CH3AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch3awscdr;
/**DFSDM_CH3WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH3WDATR)

For information about available fields see [`mod@dfsdm_ch3wdatr`]
module*/
pub type DFSDM_CH3WDATR = crate::Reg<dfsdm_ch3wdatr::DFSDM_CH3WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch3wdatr;
/**DFSDM_CH3DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch3datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH3DATINR)

For information about available fields see [`mod@dfsdm_ch3datinr`]
module*/
pub type DFSDM_CH3DATINR = crate::Reg<dfsdm_ch3datinr::DFSDM_CH3DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch3datinr;
/**DFSDM_CH3DLYR (rw) register accessor: DFSDM channel 3 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch3dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH3DLYR)

For information about available fields see [`mod@dfsdm_ch3dlyr`]
module*/
pub type DFSDM_CH3DLYR = crate::Reg<dfsdm_ch3dlyr::DFSDM_CH3DLYRrs>;
///DFSDM channel 3 delay register
pub mod dfsdm_ch3dlyr;
/**DFSDM_CH4CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch4cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch4cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH4CFGR1)

For information about available fields see [`mod@dfsdm_ch4cfgr1`]
module*/
pub type DFSDM_CH4CFGR1 = crate::Reg<dfsdm_ch4cfgr1::DFSDM_CH4CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch4cfgr1;
/**DFSDM_CH4CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch4cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch4cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH4CFGR2)

For information about available fields see [`mod@dfsdm_ch4cfgr2`]
module*/
pub type DFSDM_CH4CFGR2 = crate::Reg<dfsdm_ch4cfgr2::DFSDM_CH4CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch4cfgr2;
/**DFSDM_CH4AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch4awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch4awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH4AWSCDR)

For information about available fields see [`mod@dfsdm_ch4awscdr`]
module*/
pub type DFSDM_CH4AWSCDR = crate::Reg<dfsdm_ch4awscdr::DFSDM_CH4AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch4awscdr;
/**DFSDM_CH4WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch4wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH4WDATR)

For information about available fields see [`mod@dfsdm_ch4wdatr`]
module*/
pub type DFSDM_CH4WDATR = crate::Reg<dfsdm_ch4wdatr::DFSDM_CH4WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch4wdatr;
/**DFSDM_CH4DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch4datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch4datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH4DATINR)

For information about available fields see [`mod@dfsdm_ch4datinr`]
module*/
pub type DFSDM_CH4DATINR = crate::Reg<dfsdm_ch4datinr::DFSDM_CH4DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch4datinr;
/**DFSDM_CH4DLYR (rw) register accessor: DFSDM channel 4 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch4dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch4dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH4DLYR)

For information about available fields see [`mod@dfsdm_ch4dlyr`]
module*/
pub type DFSDM_CH4DLYR = crate::Reg<dfsdm_ch4dlyr::DFSDM_CH4DLYRrs>;
///DFSDM channel 4 delay register
pub mod dfsdm_ch4dlyr;
/**DFSDM_CH5CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch5cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch5cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH5CFGR1)

For information about available fields see [`mod@dfsdm_ch5cfgr1`]
module*/
pub type DFSDM_CH5CFGR1 = crate::Reg<dfsdm_ch5cfgr1::DFSDM_CH5CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch5cfgr1;
/**DFSDM_CH5CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch5cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch5cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH5CFGR2)

For information about available fields see [`mod@dfsdm_ch5cfgr2`]
module*/
pub type DFSDM_CH5CFGR2 = crate::Reg<dfsdm_ch5cfgr2::DFSDM_CH5CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch5cfgr2;
/**DFSDM_CH5AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch5awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch5awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH5AWSCDR)

For information about available fields see [`mod@dfsdm_ch5awscdr`]
module*/
pub type DFSDM_CH5AWSCDR = crate::Reg<dfsdm_ch5awscdr::DFSDM_CH5AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch5awscdr;
/**DFSDM_CH5WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch5wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH5WDATR)

For information about available fields see [`mod@dfsdm_ch5wdatr`]
module*/
pub type DFSDM_CH5WDATR = crate::Reg<dfsdm_ch5wdatr::DFSDM_CH5WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch5wdatr;
/**DFSDM_CH5DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch5datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch5datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH5DATINR)

For information about available fields see [`mod@dfsdm_ch5datinr`]
module*/
pub type DFSDM_CH5DATINR = crate::Reg<dfsdm_ch5datinr::DFSDM_CH5DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch5datinr;
/**DFSDM_CH5DLYR (rw) register accessor: DFSDM channel 5 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch5dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch5dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH5DLYR)

For information about available fields see [`mod@dfsdm_ch5dlyr`]
module*/
pub type DFSDM_CH5DLYR = crate::Reg<dfsdm_ch5dlyr::DFSDM_CH5DLYRrs>;
///DFSDM channel 5 delay register
pub mod dfsdm_ch5dlyr;
/**DFSDM_CH6CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch6cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH6CFGR1)

For information about available fields see [`mod@dfsdm_ch6cfgr1`]
module*/
pub type DFSDM_CH6CFGR1 = crate::Reg<dfsdm_ch6cfgr1::DFSDM_CH6CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch6cfgr1;
/**DFSDM_CH6CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch6cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH6CFGR2)

For information about available fields see [`mod@dfsdm_ch6cfgr2`]
module*/
pub type DFSDM_CH6CFGR2 = crate::Reg<dfsdm_ch6cfgr2::DFSDM_CH6CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch6cfgr2;
/**DFSDM_CH6AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch6awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH6AWSCDR)

For information about available fields see [`mod@dfsdm_ch6awscdr`]
module*/
pub type DFSDM_CH6AWSCDR = crate::Reg<dfsdm_ch6awscdr::DFSDM_CH6AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch6awscdr;
/**DFSDM_CH6WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH6WDATR)

For information about available fields see [`mod@dfsdm_ch6wdatr`]
module*/
pub type DFSDM_CH6WDATR = crate::Reg<dfsdm_ch6wdatr::DFSDM_CH6WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch6wdatr;
/**DFSDM_CH6DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch6datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH6DATINR)

For information about available fields see [`mod@dfsdm_ch6datinr`]
module*/
pub type DFSDM_CH6DATINR = crate::Reg<dfsdm_ch6datinr::DFSDM_CH6DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch6datinr;
/**DFSDM_CH6DLYR (rw) register accessor: DFSDM channel 6 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch6dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch6dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH6DLYR)

For information about available fields see [`mod@dfsdm_ch6dlyr`]
module*/
pub type DFSDM_CH6DLYR = crate::Reg<dfsdm_ch6dlyr::DFSDM_CH6DLYRrs>;
///DFSDM channel 6 delay register
pub mod dfsdm_ch6dlyr;
/**DFSDM_CH7CFGR1 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch7cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch7cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH7CFGR1)

For information about available fields see [`mod@dfsdm_ch7cfgr1`]
module*/
pub type DFSDM_CH7CFGR1 = crate::Reg<dfsdm_ch7cfgr1::DFSDM_CH7CFGR1rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch7cfgr1;
/**DFSDM_CH7CFGR2 (rw) register accessor: This register specifies the parameters used by channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch7cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch7cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH7CFGR2)

For information about available fields see [`mod@dfsdm_ch7cfgr2`]
module*/
pub type DFSDM_CH7CFGR2 = crate::Reg<dfsdm_ch7cfgr2::DFSDM_CH7CFGR2rs>;
///This register specifies the parameters used by channel y.
pub mod dfsdm_ch7cfgr2;
/**DFSDM_CH7AWSCDR (rw) register accessor: Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch7awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch7awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH7AWSCDR)

For information about available fields see [`mod@dfsdm_ch7awscdr`]
module*/
pub type DFSDM_CH7AWSCDR = crate::Reg<dfsdm_ch7awscdr::DFSDM_CH7AWSCDRrs>;
///Short-circuit detector and analog watchdog settings for channel y.
pub mod dfsdm_ch7awscdr;
/**DFSDM_CH7WDATR (r) register accessor: This register contains the data resulting from the analog watchdog filter associated to the input channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch7wdatr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH7WDATR)

For information about available fields see [`mod@dfsdm_ch7wdatr`]
module*/
pub type DFSDM_CH7WDATR = crate::Reg<dfsdm_ch7wdatr::DFSDM_CH7WDATRrs>;
///This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod dfsdm_ch7wdatr;
/**DFSDM_CH7DATINR (rw) register accessor: This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch7datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch7datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH7DATINR)

For information about available fields see [`mod@dfsdm_ch7datinr`]
module*/
pub type DFSDM_CH7DATINR = crate::Reg<dfsdm_ch7datinr::DFSDM_CH7DATINRrs>;
///This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod dfsdm_ch7datinr;
/**DFSDM_CH7DLYR (rw) register accessor: DFSDM channel 7 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch7dlyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch7dlyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH7DLYR)

For information about available fields see [`mod@dfsdm_ch7dlyr`]
module*/
pub type DFSDM_CH7DLYR = crate::Reg<dfsdm_ch7dlyr::DFSDM_CH7DLYRrs>;
///DFSDM channel 7 delay register
pub mod dfsdm_ch7dlyr;
/**DFSDM_FLT0CR1 (rw) register accessor: DFSDM filter 0 control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0CR1)

For information about available fields see [`mod@dfsdm_flt0cr1`]
module*/
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1rs>;
///DFSDM filter 0 control register 1
pub mod dfsdm_flt0cr1;
/**DFSDM_FLT0CR2 (rw) register accessor: DFSDM filter 0 control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0CR2)

For information about available fields see [`mod@dfsdm_flt0cr2`]
module*/
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2rs>;
///DFSDM filter 0 control register 2
pub mod dfsdm_flt0cr2;
/**DFSDM_FLT0ISR (r) register accessor: DFSDM filter 0 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0ISR)

For information about available fields see [`mod@dfsdm_flt0isr`]
module*/
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISRrs>;
///DFSDM filter 0 interrupt and status register
pub mod dfsdm_flt0isr;
/**DFSDM_FLT0ICR (rw) register accessor: DFSDM filter 0 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0ICR)

For information about available fields see [`mod@dfsdm_flt0icr`]
module*/
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICRrs>;
///DFSDM filter 0 interrupt flag clear register
pub mod dfsdm_flt0icr;
/**DFSDM_FLT0JCHGR (rw) register accessor: DFSDM filter 0 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0JCHGR)

For information about available fields see [`mod@dfsdm_flt0jchgr`]
module*/
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGRrs>;
///DFSDM filter 0 injected channel group selection register
pub mod dfsdm_flt0jchgr;
/**DFSDM_FLT0FCR (rw) register accessor: DFSDM filter 0 control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0FCR)

For information about available fields see [`mod@dfsdm_flt0fcr`]
module*/
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCRrs>;
///DFSDM filter 0 control register
pub mod dfsdm_flt0fcr;
/**DFSDM_FLT0JDATAR (r) register accessor: DFSDM filter 0 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0JDATAR)

For information about available fields see [`mod@dfsdm_flt0jdatar`]
module*/
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATARrs>;
///DFSDM filter 0 data register for injected group
pub mod dfsdm_flt0jdatar;
/**DFSDM_FLT0RDATAR (r) register accessor: DFSDM filter 0 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0RDATAR)

For information about available fields see [`mod@dfsdm_flt0rdatar`]
module*/
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATARrs>;
///DFSDM filter 0 data register for the regular channel
pub mod dfsdm_flt0rdatar;
/**DFSDM_FLT0AWHTR (rw) register accessor: DFSDM filter 0 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0AWHTR)

For information about available fields see [`mod@dfsdm_flt0awhtr`]
module*/
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTRrs>;
///DFSDM filter 0 analog watchdog high threshold register
pub mod dfsdm_flt0awhtr;
/**DFSDM_FLT0AWLTR (rw) register accessor: DFSDM filter 0 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0AWLTR)

For information about available fields see [`mod@dfsdm_flt0awltr`]
module*/
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTRrs>;
///DFSDM filter 0 analog watchdog low threshold register
pub mod dfsdm_flt0awltr;
/**DFSDM_FLT0AWSR (r) register accessor: DFSDM filter 0 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0AWSR)

For information about available fields see [`mod@dfsdm_flt0awsr`]
module*/
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSRrs>;
///DFSDM filter 0 analog watchdog status register
pub mod dfsdm_flt0awsr;
/**DFSDM_FLT0AWCFR (rw) register accessor: DFSDM filter 0 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0AWCFR)

For information about available fields see [`mod@dfsdm_flt0awcfr`]
module*/
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFRrs>;
///DFSDM filter 0 analog watchdog clear flag register
pub mod dfsdm_flt0awcfr;
/**DFSDM_FLT0EXMAX (r) register accessor: DFSDM filter 0 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0EXMAX)

For information about available fields see [`mod@dfsdm_flt0exmax`]
module*/
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAXrs>;
///DFSDM filter 0 extremes detector maximum register
pub mod dfsdm_flt0exmax;
/**DFSDM_FLT0EXMIN (rw) register accessor: DFSDM filter 0 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0EXMIN)

For information about available fields see [`mod@dfsdm_flt0exmin`]
module*/
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMINrs>;
///DFSDM filter 0 extremes detector minimum register
pub mod dfsdm_flt0exmin;
/**DFSDM_FLT0CNVTIMR (r) register accessor: DFSDM filter 0 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT0CNVTIMR)

For information about available fields see [`mod@dfsdm_flt0cnvtimr`]
module*/
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMRrs>;
///DFSDM filter 0 conversion timer register
pub mod dfsdm_flt0cnvtimr;
/**DFSDM_FLT1CR1 (rw) register accessor: DFSDM filter 1 control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1CR1)

For information about available fields see [`mod@dfsdm_flt1cr1`]
module*/
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1rs>;
///DFSDM filter 1 control register 1
pub mod dfsdm_flt1cr1;
/**DFSDM_FLT1CR2 (rw) register accessor: DFSDM filter 1 control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1CR2)

For information about available fields see [`mod@dfsdm_flt1cr2`]
module*/
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2rs>;
///DFSDM filter 1 control register 2
pub mod dfsdm_flt1cr2;
/**DFSDM_FLT1ISR (r) register accessor: DFSDM filter 1 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1ISR)

For information about available fields see [`mod@dfsdm_flt1isr`]
module*/
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISRrs>;
///DFSDM filter 1 interrupt and status register
pub mod dfsdm_flt1isr;
/**DFSDM_FLT1ICR (rw) register accessor: DFSDM filter 1 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1ICR)

For information about available fields see [`mod@dfsdm_flt1icr`]
module*/
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICRrs>;
///DFSDM filter 1 interrupt flag clear register
pub mod dfsdm_flt1icr;
/**DFSDM_FLT1JCHGR (rw) register accessor: DFSDM filter 1 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1JCHGR)

For information about available fields see [`mod@dfsdm_flt1jchgr`]
module*/
pub type DFSDM_FLT1JCHGR = crate::Reg<dfsdm_flt1jchgr::DFSDM_FLT1JCHGRrs>;
///DFSDM filter 1 injected channel group selection register
pub mod dfsdm_flt1jchgr;
/**DFSDM_FLT1FCR (rw) register accessor: DFSDM filter 1 control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1FCR)

For information about available fields see [`mod@dfsdm_flt1fcr`]
module*/
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCRrs>;
///DFSDM filter 1 control register
pub mod dfsdm_flt1fcr;
/**DFSDM_FLT1JDATAR (r) register accessor: DFSDM filter 1 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1JDATAR)

For information about available fields see [`mod@dfsdm_flt1jdatar`]
module*/
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATARrs>;
///DFSDM filter 1 data register for injected group
pub mod dfsdm_flt1jdatar;
/**DFSDM_FLT1RDATAR (r) register accessor: DFSDM filter 1 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1RDATAR)

For information about available fields see [`mod@dfsdm_flt1rdatar`]
module*/
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATARrs>;
///DFSDM filter 1 data register for the regular channel
pub mod dfsdm_flt1rdatar;
/**DFSDM_FLT1AWHTR (rw) register accessor: DFSDM filter 1 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1AWHTR)

For information about available fields see [`mod@dfsdm_flt1awhtr`]
module*/
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTRrs>;
///DFSDM filter 1 analog watchdog high threshold register
pub mod dfsdm_flt1awhtr;
/**DFSDM_FLT1AWLTR (rw) register accessor: DFSDM filter 1 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1AWLTR)

For information about available fields see [`mod@dfsdm_flt1awltr`]
module*/
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTRrs>;
///DFSDM filter 1 analog watchdog low threshold register
pub mod dfsdm_flt1awltr;
/**DFSDM_FLT1AWSR (r) register accessor: DFSDM filter 1 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1AWSR)

For information about available fields see [`mod@dfsdm_flt1awsr`]
module*/
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSRrs>;
///DFSDM filter 1 analog watchdog status register
pub mod dfsdm_flt1awsr;
/**DFSDM_FLT1AWCFR (rw) register accessor: DFSDM filter 1 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1AWCFR)

For information about available fields see [`mod@dfsdm_flt1awcfr`]
module*/
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFRrs>;
///DFSDM filter 1 analog watchdog clear flag register
pub mod dfsdm_flt1awcfr;
/**DFSDM_FLT1EXMAX (r) register accessor: DFSDM filter 1 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1EXMAX)

For information about available fields see [`mod@dfsdm_flt1exmax`]
module*/
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAXrs>;
///DFSDM filter 1 extremes detector maximum register
pub mod dfsdm_flt1exmax;
/**DFSDM_FLT1EXMIN (rw) register accessor: DFSDM filter 1 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1EXMIN)

For information about available fields see [`mod@dfsdm_flt1exmin`]
module*/
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMINrs>;
///DFSDM filter 1 extremes detector minimum register
pub mod dfsdm_flt1exmin;
/**DFSDM_FLT1CNVTIMR (r) register accessor: DFSDM filter 1 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT1CNVTIMR)

For information about available fields see [`mod@dfsdm_flt1cnvtimr`]
module*/
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMRrs>;
///DFSDM filter 1 conversion timer register
pub mod dfsdm_flt1cnvtimr;
/**DFSDM_FLT2CR1 (rw) register accessor: DFSDM filter 2 control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2CR1)

For information about available fields see [`mod@dfsdm_flt2cr1`]
module*/
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1rs>;
///DFSDM filter 2 control register 1
pub mod dfsdm_flt2cr1;
/**DFSDM_FLT2CR2 (rw) register accessor: DFSDM filter 2 control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2CR2)

For information about available fields see [`mod@dfsdm_flt2cr2`]
module*/
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2rs>;
///DFSDM filter 2 control register 2
pub mod dfsdm_flt2cr2;
/**DFSDM_FLT2ISR (r) register accessor: DFSDM filter 2 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2ISR)

For information about available fields see [`mod@dfsdm_flt2isr`]
module*/
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISRrs>;
///DFSDM filter 2 interrupt and status register
pub mod dfsdm_flt2isr;
/**DFSDM_FLT2ICR (rw) register accessor: DFSDM filter 2 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2ICR)

For information about available fields see [`mod@dfsdm_flt2icr`]
module*/
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICRrs>;
///DFSDM filter 2 interrupt flag clear register
pub mod dfsdm_flt2icr;
/**DFSDM_FLT2JCHGR (rw) register accessor: DFSDM filter 2 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2JCHGR)

For information about available fields see [`mod@dfsdm_flt2jchgr`]
module*/
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGRrs>;
///DFSDM filter 2 injected channel group selection register
pub mod dfsdm_flt2jchgr;
/**DFSDM_FLT2FCR (rw) register accessor: DFSDM filter 2 control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2FCR)

For information about available fields see [`mod@dfsdm_flt2fcr`]
module*/
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCRrs>;
///DFSDM filter 2 control register
pub mod dfsdm_flt2fcr;
/**DFSDM_FLT2JDATAR (r) register accessor: DFSDM filter 2 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2JDATAR)

For information about available fields see [`mod@dfsdm_flt2jdatar`]
module*/
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATARrs>;
///DFSDM filter 2 data register for injected group
pub mod dfsdm_flt2jdatar;
/**DFSDM_FLT2RDATAR (r) register accessor: DFSDM filter 2 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2RDATAR)

For information about available fields see [`mod@dfsdm_flt2rdatar`]
module*/
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATARrs>;
///DFSDM filter 2 data register for the regular channel
pub mod dfsdm_flt2rdatar;
/**DFSDM_FLT2AWHTR (rw) register accessor: DFSDM filter 2 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2AWHTR)

For information about available fields see [`mod@dfsdm_flt2awhtr`]
module*/
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTRrs>;
///DFSDM filter 2 analog watchdog high threshold register
pub mod dfsdm_flt2awhtr;
/**DFSDM_FLT2AWLTR (rw) register accessor: DFSDM filter 2 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2AWLTR)

For information about available fields see [`mod@dfsdm_flt2awltr`]
module*/
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTRrs>;
///DFSDM filter 2 analog watchdog low threshold register
pub mod dfsdm_flt2awltr;
/**DFSDM_FLT2AWSR (r) register accessor: DFSDM filter 2 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2AWSR)

For information about available fields see [`mod@dfsdm_flt2awsr`]
module*/
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSRrs>;
///DFSDM filter 2 analog watchdog status register
pub mod dfsdm_flt2awsr;
/**DFSDM_FLT2AWCFR (rw) register accessor: DFSDM filter 2 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2AWCFR)

For information about available fields see [`mod@dfsdm_flt2awcfr`]
module*/
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFRrs>;
///DFSDM filter 2 analog watchdog clear flag register
pub mod dfsdm_flt2awcfr;
/**DFSDM_FLT2EXMAX (r) register accessor: DFSDM filter 2 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2EXMAX)

For information about available fields see [`mod@dfsdm_flt2exmax`]
module*/
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAXrs>;
///DFSDM filter 2 extremes detector maximum register
pub mod dfsdm_flt2exmax;
/**DFSDM_FLT2EXMIN (rw) register accessor: DFSDM filter 2 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2EXMIN)

For information about available fields see [`mod@dfsdm_flt2exmin`]
module*/
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMINrs>;
///DFSDM filter 2 extremes detector minimum register
pub mod dfsdm_flt2exmin;
/**DFSDM_FLT2CNVTIMR (r) register accessor: DFSDM filter 2 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT2CNVTIMR)

For information about available fields see [`mod@dfsdm_flt2cnvtimr`]
module*/
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMRrs>;
///DFSDM filter 2 conversion timer register
pub mod dfsdm_flt2cnvtimr;
/**DFSDM_FLT3CR1 (rw) register accessor: DFSDM filter 3 control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3CR1)

For information about available fields see [`mod@dfsdm_flt3cr1`]
module*/
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1rs>;
///DFSDM filter 3 control register 1
pub mod dfsdm_flt3cr1;
/**DFSDM_FLT3CR2 (rw) register accessor: DFSDM filter 3 control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3CR2)

For information about available fields see [`mod@dfsdm_flt3cr2`]
module*/
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2rs>;
///DFSDM filter 3 control register 2
pub mod dfsdm_flt3cr2;
/**DFSDM_FLT3ISR (r) register accessor: DFSDM filter 3 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3ISR)

For information about available fields see [`mod@dfsdm_flt3isr`]
module*/
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISRrs>;
///DFSDM filter 3 interrupt and status register
pub mod dfsdm_flt3isr;
/**DFSDM_FLT3ICR (rw) register accessor: DFSDM filter 3 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3ICR)

For information about available fields see [`mod@dfsdm_flt3icr`]
module*/
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICRrs>;
///DFSDM filter 3 interrupt flag clear register
pub mod dfsdm_flt3icr;
/**DFSDM_FLT3JCHGR (rw) register accessor: DFSDM filter 3 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3JCHGR)

For information about available fields see [`mod@dfsdm_flt3jchgr`]
module*/
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGRrs>;
///DFSDM filter 3 injected channel group selection register
pub mod dfsdm_flt3jchgr;
/**DFSDM_FLT3FCR (rw) register accessor: DFSDM filter 3 control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3FCR)

For information about available fields see [`mod@dfsdm_flt3fcr`]
module*/
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCRrs>;
///DFSDM filter 3 control register
pub mod dfsdm_flt3fcr;
/**DFSDM_FLT3JDATAR (r) register accessor: DFSDM filter 3 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3JDATAR)

For information about available fields see [`mod@dfsdm_flt3jdatar`]
module*/
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATARrs>;
///DFSDM filter 3 data register for injected group
pub mod dfsdm_flt3jdatar;
/**DFSDM_FLT3RDATAR (r) register accessor: DFSDM filter 3 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3RDATAR)

For information about available fields see [`mod@dfsdm_flt3rdatar`]
module*/
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATARrs>;
///DFSDM filter 3 data register for the regular channel
pub mod dfsdm_flt3rdatar;
/**DFSDM_FLT3AWHTR (rw) register accessor: DFSDM filter 3 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3AWHTR)

For information about available fields see [`mod@dfsdm_flt3awhtr`]
module*/
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTRrs>;
///DFSDM filter 3 analog watchdog high threshold register
pub mod dfsdm_flt3awhtr;
/**DFSDM_FLT3AWLTR (rw) register accessor: DFSDM filter 3 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3AWLTR)

For information about available fields see [`mod@dfsdm_flt3awltr`]
module*/
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTRrs>;
///DFSDM filter 3 analog watchdog low threshold register
pub mod dfsdm_flt3awltr;
/**DFSDM_FLT3AWSR (r) register accessor: DFSDM filter 3 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3AWSR)

For information about available fields see [`mod@dfsdm_flt3awsr`]
module*/
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSRrs>;
///DFSDM filter 3 analog watchdog status register
pub mod dfsdm_flt3awsr;
/**DFSDM_FLT3AWCFR (rw) register accessor: DFSDM filter 3 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3AWCFR)

For information about available fields see [`mod@dfsdm_flt3awcfr`]
module*/
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFRrs>;
///DFSDM filter 3 analog watchdog clear flag register
pub mod dfsdm_flt3awcfr;
/**DFSDM_FLT3EXMAX (r) register accessor: DFSDM filter 3 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3EXMAX)

For information about available fields see [`mod@dfsdm_flt3exmax`]
module*/
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAXrs>;
///DFSDM filter 3 extremes detector maximum register
pub mod dfsdm_flt3exmax;
/**DFSDM_FLT3EXMIN (rw) register accessor: DFSDM filter 3 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3EXMIN)

For information about available fields see [`mod@dfsdm_flt3exmin`]
module*/
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMINrs>;
///DFSDM filter 3 extremes detector minimum register
pub mod dfsdm_flt3exmin;
/**DFSDM_FLT3CNVTIMR (r) register accessor: DFSDM filter 3 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT3CNVTIMR)

For information about available fields see [`mod@dfsdm_flt3cnvtimr`]
module*/
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMRrs>;
///DFSDM filter 3 conversion timer register
pub mod dfsdm_flt3cnvtimr;
/**DFSDM_FLT4CR1 (rw) register accessor: DFSDM filter 4 control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4CR1)

For information about available fields see [`mod@dfsdm_flt4cr1`]
module*/
pub type DFSDM_FLT4CR1 = crate::Reg<dfsdm_flt4cr1::DFSDM_FLT4CR1rs>;
///DFSDM filter 4 control register 1
pub mod dfsdm_flt4cr1;
/**DFSDM_FLT4CR2 (rw) register accessor: DFSDM filter 4 control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4CR2)

For information about available fields see [`mod@dfsdm_flt4cr2`]
module*/
pub type DFSDM_FLT4CR2 = crate::Reg<dfsdm_flt4cr2::DFSDM_FLT4CR2rs>;
///DFSDM filter 4 control register 2
pub mod dfsdm_flt4cr2;
/**DFSDM_FLT4ISR (r) register accessor: DFSDM filter 4 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4ISR)

For information about available fields see [`mod@dfsdm_flt4isr`]
module*/
pub type DFSDM_FLT4ISR = crate::Reg<dfsdm_flt4isr::DFSDM_FLT4ISRrs>;
///DFSDM filter 4 interrupt and status register
pub mod dfsdm_flt4isr;
/**DFSDM_FLT4ICR (rw) register accessor: DFSDM filter 4 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4ICR)

For information about available fields see [`mod@dfsdm_flt4icr`]
module*/
pub type DFSDM_FLT4ICR = crate::Reg<dfsdm_flt4icr::DFSDM_FLT4ICRrs>;
///DFSDM filter 4 interrupt flag clear register
pub mod dfsdm_flt4icr;
/**DFSDM_FLT4JCHGR (rw) register accessor: DFSDM filter 4 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4JCHGR)

For information about available fields see [`mod@dfsdm_flt4jchgr`]
module*/
pub type DFSDM_FLT4JCHGR = crate::Reg<dfsdm_flt4jchgr::DFSDM_FLT4JCHGRrs>;
///DFSDM filter 4 injected channel group selection register
pub mod dfsdm_flt4jchgr;
/**DFSDM_FLT4FCR (rw) register accessor: DFSDM filter 4 control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4FCR)

For information about available fields see [`mod@dfsdm_flt4fcr`]
module*/
pub type DFSDM_FLT4FCR = crate::Reg<dfsdm_flt4fcr::DFSDM_FLT4FCRrs>;
///DFSDM filter 4 control register
pub mod dfsdm_flt4fcr;
/**DFSDM_FLT4JDATAR (r) register accessor: DFSDM filter 4 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4JDATAR)

For information about available fields see [`mod@dfsdm_flt4jdatar`]
module*/
pub type DFSDM_FLT4JDATAR = crate::Reg<dfsdm_flt4jdatar::DFSDM_FLT4JDATARrs>;
///DFSDM filter 4 data register for injected group
pub mod dfsdm_flt4jdatar;
/**DFSDM_FLT4RDATAR (r) register accessor: DFSDM filter 4 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4RDATAR)

For information about available fields see [`mod@dfsdm_flt4rdatar`]
module*/
pub type DFSDM_FLT4RDATAR = crate::Reg<dfsdm_flt4rdatar::DFSDM_FLT4RDATARrs>;
///DFSDM filter 4 data register for the regular channel
pub mod dfsdm_flt4rdatar;
/**DFSDM_FLT4AWHTR (rw) register accessor: DFSDM filter 4 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4AWHTR)

For information about available fields see [`mod@dfsdm_flt4awhtr`]
module*/
pub type DFSDM_FLT4AWHTR = crate::Reg<dfsdm_flt4awhtr::DFSDM_FLT4AWHTRrs>;
///DFSDM filter 4 analog watchdog high threshold register
pub mod dfsdm_flt4awhtr;
/**DFSDM_FLT4AWLTR (rw) register accessor: DFSDM filter 4 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4AWLTR)

For information about available fields see [`mod@dfsdm_flt4awltr`]
module*/
pub type DFSDM_FLT4AWLTR = crate::Reg<dfsdm_flt4awltr::DFSDM_FLT4AWLTRrs>;
///DFSDM filter 4 analog watchdog low threshold register
pub mod dfsdm_flt4awltr;
/**DFSDM_FLT4AWSR (r) register accessor: DFSDM filter 4 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4AWSR)

For information about available fields see [`mod@dfsdm_flt4awsr`]
module*/
pub type DFSDM_FLT4AWSR = crate::Reg<dfsdm_flt4awsr::DFSDM_FLT4AWSRrs>;
///DFSDM filter 4 analog watchdog status register
pub mod dfsdm_flt4awsr;
/**DFSDM_FLT4AWCFR (rw) register accessor: DFSDM filter 4 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4AWCFR)

For information about available fields see [`mod@dfsdm_flt4awcfr`]
module*/
pub type DFSDM_FLT4AWCFR = crate::Reg<dfsdm_flt4awcfr::DFSDM_FLT4AWCFRrs>;
///DFSDM filter 4 analog watchdog clear flag register
pub mod dfsdm_flt4awcfr;
/**DFSDM_FLT4EXMAX (r) register accessor: DFSDM filter 4 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4EXMAX)

For information about available fields see [`mod@dfsdm_flt4exmax`]
module*/
pub type DFSDM_FLT4EXMAX = crate::Reg<dfsdm_flt4exmax::DFSDM_FLT4EXMAXrs>;
///DFSDM filter 4 extremes detector maximum register
pub mod dfsdm_flt4exmax;
/**DFSDM_FLT4EXMIN (rw) register accessor: DFSDM filter 4 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt4exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4EXMIN)

For information about available fields see [`mod@dfsdm_flt4exmin`]
module*/
pub type DFSDM_FLT4EXMIN = crate::Reg<dfsdm_flt4exmin::DFSDM_FLT4EXMINrs>;
///DFSDM filter 4 extremes detector minimum register
pub mod dfsdm_flt4exmin;
/**DFSDM_FLT4CNVTIMR (r) register accessor: DFSDM filter 4 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt4cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT4CNVTIMR)

For information about available fields see [`mod@dfsdm_flt4cnvtimr`]
module*/
pub type DFSDM_FLT4CNVTIMR = crate::Reg<dfsdm_flt4cnvtimr::DFSDM_FLT4CNVTIMRrs>;
///DFSDM filter 4 conversion timer register
pub mod dfsdm_flt4cnvtimr;
/**DFSDM_FLT5CR1 (rw) register accessor: DFSDM filter 5 control register 1

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5CR1)

For information about available fields see [`mod@dfsdm_flt5cr1`]
module*/
pub type DFSDM_FLT5CR1 = crate::Reg<dfsdm_flt5cr1::DFSDM_FLT5CR1rs>;
///DFSDM filter 5 control register 1
pub mod dfsdm_flt5cr1;
/**DFSDM_FLT5CR2 (rw) register accessor: DFSDM filter 5 control register 2

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5CR2)

For information about available fields see [`mod@dfsdm_flt5cr2`]
module*/
pub type DFSDM_FLT5CR2 = crate::Reg<dfsdm_flt5cr2::DFSDM_FLT5CR2rs>;
///DFSDM filter 5 control register 2
pub mod dfsdm_flt5cr2;
/**DFSDM_FLT5ISR (r) register accessor: DFSDM filter 5 interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5ISR)

For information about available fields see [`mod@dfsdm_flt5isr`]
module*/
pub type DFSDM_FLT5ISR = crate::Reg<dfsdm_flt5isr::DFSDM_FLT5ISRrs>;
///DFSDM filter 5 interrupt and status register
pub mod dfsdm_flt5isr;
/**DFSDM_FLT5ICR (rw) register accessor: DFSDM filter 5 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5ICR)

For information about available fields see [`mod@dfsdm_flt5icr`]
module*/
pub type DFSDM_FLT5ICR = crate::Reg<dfsdm_flt5icr::DFSDM_FLT5ICRrs>;
///DFSDM filter 5 interrupt flag clear register
pub mod dfsdm_flt5icr;
/**DFSDM_FLT5JCHGR (rw) register accessor: DFSDM filter 5 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5JCHGR)

For information about available fields see [`mod@dfsdm_flt5jchgr`]
module*/
pub type DFSDM_FLT5JCHGR = crate::Reg<dfsdm_flt5jchgr::DFSDM_FLT5JCHGRrs>;
///DFSDM filter 5 injected channel group selection register
pub mod dfsdm_flt5jchgr;
/**DFSDM_FLT5FCR (rw) register accessor: DFSDM filter 5 control register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5FCR)

For information about available fields see [`mod@dfsdm_flt5fcr`]
module*/
pub type DFSDM_FLT5FCR = crate::Reg<dfsdm_flt5fcr::DFSDM_FLT5FCRrs>;
///DFSDM filter 5 control register
pub mod dfsdm_flt5fcr;
/**DFSDM_FLT5JDATAR (r) register accessor: DFSDM filter 5 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5JDATAR)

For information about available fields see [`mod@dfsdm_flt5jdatar`]
module*/
pub type DFSDM_FLT5JDATAR = crate::Reg<dfsdm_flt5jdatar::DFSDM_FLT5JDATARrs>;
///DFSDM filter 5 data register for injected group
pub mod dfsdm_flt5jdatar;
/**DFSDM_FLT5RDATAR (r) register accessor: DFSDM filter 5 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5RDATAR)

For information about available fields see [`mod@dfsdm_flt5rdatar`]
module*/
pub type DFSDM_FLT5RDATAR = crate::Reg<dfsdm_flt5rdatar::DFSDM_FLT5RDATARrs>;
///DFSDM filter 5 data register for the regular channel
pub mod dfsdm_flt5rdatar;
/**DFSDM_FLT5AWHTR (rw) register accessor: DFSDM filter 5 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5AWHTR)

For information about available fields see [`mod@dfsdm_flt5awhtr`]
module*/
pub type DFSDM_FLT5AWHTR = crate::Reg<dfsdm_flt5awhtr::DFSDM_FLT5AWHTRrs>;
///DFSDM filter 5 analog watchdog high threshold register
pub mod dfsdm_flt5awhtr;
/**DFSDM_FLT5AWLTR (rw) register accessor: DFSDM filter 5 analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5AWLTR)

For information about available fields see [`mod@dfsdm_flt5awltr`]
module*/
pub type DFSDM_FLT5AWLTR = crate::Reg<dfsdm_flt5awltr::DFSDM_FLT5AWLTRrs>;
///DFSDM filter 5 analog watchdog low threshold register
pub mod dfsdm_flt5awltr;
/**DFSDM_FLT5AWSR (r) register accessor: DFSDM filter 5 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5AWSR)

For information about available fields see [`mod@dfsdm_flt5awsr`]
module*/
pub type DFSDM_FLT5AWSR = crate::Reg<dfsdm_flt5awsr::DFSDM_FLT5AWSRrs>;
///DFSDM filter 5 analog watchdog status register
pub mod dfsdm_flt5awsr;
/**DFSDM_FLT5AWCFR (rw) register accessor: DFSDM filter 5 analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5AWCFR)

For information about available fields see [`mod@dfsdm_flt5awcfr`]
module*/
pub type DFSDM_FLT5AWCFR = crate::Reg<dfsdm_flt5awcfr::DFSDM_FLT5AWCFRrs>;
///DFSDM filter 5 analog watchdog clear flag register
pub mod dfsdm_flt5awcfr;
/**DFSDM_FLT5EXMAX (r) register accessor: DFSDM filter 5 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5EXMAX)

For information about available fields see [`mod@dfsdm_flt5exmax`]
module*/
pub type DFSDM_FLT5EXMAX = crate::Reg<dfsdm_flt5exmax::DFSDM_FLT5EXMAXrs>;
///DFSDM filter 5 extremes detector maximum register
pub mod dfsdm_flt5exmax;
/**DFSDM_FLT5EXMIN (rw) register accessor: DFSDM filter 5 extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5exmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt5exmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5EXMIN)

For information about available fields see [`mod@dfsdm_flt5exmin`]
module*/
pub type DFSDM_FLT5EXMIN = crate::Reg<dfsdm_flt5exmin::DFSDM_FLT5EXMINrs>;
///DFSDM filter 5 extremes detector minimum register
pub mod dfsdm_flt5exmin;
/**DFSDM_FLT5CNVTIMR (r) register accessor: DFSDM filter 5 conversion timer register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt5cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_FLT5CNVTIMR)

For information about available fields see [`mod@dfsdm_flt5cnvtimr`]
module*/
pub type DFSDM_FLT5CNVTIMR = crate::Reg<dfsdm_flt5cnvtimr::DFSDM_FLT5CNVTIMRrs>;
///DFSDM filter 5 conversion timer register
pub mod dfsdm_flt5cnvtimr;
/**DFSDM_HWCFGR (r) register accessor: This register specifies the hardware configuration of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_HWCFGR)

For information about available fields see [`mod@dfsdm_hwcfgr`]
module*/
pub type DFSDM_HWCFGR = crate::Reg<dfsdm_hwcfgr::DFSDM_HWCFGRrs>;
///This register specifies the hardware configuration of DFSDM peripheral.
pub mod dfsdm_hwcfgr;
/**DFSDM_VERR (r) register accessor: This register specifies the version of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_VERR)

For information about available fields see [`mod@dfsdm_verr`]
module*/
pub type DFSDM_VERR = crate::Reg<dfsdm_verr::DFSDM_VERRrs>;
///This register specifies the version of DFSDM peripheral.
pub mod dfsdm_verr;
/**DFSDM_IPIDR (r) register accessor: This register specifies the identification of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_IPIDR)

For information about available fields see [`mod@dfsdm_ipidr`]
module*/
pub type DFSDM_IPIDR = crate::Reg<dfsdm_ipidr::DFSDM_IPIDRrs>;
///This register specifies the identification of DFSDM peripheral.
pub mod dfsdm_ipidr;
/**DFSDM_SIDR (r) register accessor: This register specifies the size allocated to DFSDM registers.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_SIDR)

For information about available fields see [`mod@dfsdm_sidr`]
module*/
pub type DFSDM_SIDR = crate::Reg<dfsdm_sidr::DFSDM_SIDRrs>;
///This register specifies the size allocated to DFSDM registers.
pub mod dfsdm_sidr;
