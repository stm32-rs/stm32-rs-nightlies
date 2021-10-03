#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch0cfgr1: crate::Reg<dfsdm_ch0cfgr1::DFSDM_CH0CFGR1_SPEC>,
    #[doc = "0x04 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch0cfgr2: crate::Reg<dfsdm_ch0cfgr2::DFSDM_CH0CFGR2_SPEC>,
    #[doc = "0x08 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch0awscdr: crate::Reg<dfsdm_ch0awscdr::DFSDM_CH0AWSCDR_SPEC>,
    #[doc = "0x0c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch0wdatr: crate::Reg<dfsdm_ch0wdatr::DFSDM_CH0WDATR_SPEC>,
    #[doc = "0x10 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch0datinr: crate::Reg<dfsdm_ch0datinr::DFSDM_CH0DATINR_SPEC>,
    #[doc = "0x14 - DFSDM channel 0 delay register"]
    pub dfsdm_ch0dlyr: crate::Reg<dfsdm_ch0dlyr::DFSDM_CH0DLYR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch1cfgr1: crate::Reg<dfsdm_ch1cfgr1::DFSDM_CH1CFGR1_SPEC>,
    #[doc = "0x24 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch1cfgr2: crate::Reg<dfsdm_ch1cfgr2::DFSDM_CH1CFGR2_SPEC>,
    #[doc = "0x28 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch1awscdr: crate::Reg<dfsdm_ch1awscdr::DFSDM_CH1AWSCDR_SPEC>,
    #[doc = "0x2c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch1wdatr: crate::Reg<dfsdm_ch1wdatr::DFSDM_CH1WDATR_SPEC>,
    #[doc = "0x30 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch1datinr: crate::Reg<dfsdm_ch1datinr::DFSDM_CH1DATINR_SPEC>,
    #[doc = "0x34 - DFSDM channel 1 delay register"]
    pub dfsdm_ch1dlyr: crate::Reg<dfsdm_ch1dlyr::DFSDM_CH1DLYR_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch2cfgr1: crate::Reg<dfsdm_ch2cfgr1::DFSDM_CH2CFGR1_SPEC>,
    #[doc = "0x44 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch2cfgr2: crate::Reg<dfsdm_ch2cfgr2::DFSDM_CH2CFGR2_SPEC>,
    #[doc = "0x48 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch2awscdr: crate::Reg<dfsdm_ch2awscdr::DFSDM_CH2AWSCDR_SPEC>,
    #[doc = "0x4c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch2wdatr: crate::Reg<dfsdm_ch2wdatr::DFSDM_CH2WDATR_SPEC>,
    #[doc = "0x50 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch2datinr: crate::Reg<dfsdm_ch2datinr::DFSDM_CH2DATINR_SPEC>,
    #[doc = "0x54 - DFSDM channel 2 delay register"]
    pub dfsdm_ch2dlyr: crate::Reg<dfsdm_ch2dlyr::DFSDM_CH2DLYR_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x60 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch3cfgr1: crate::Reg<dfsdm_ch3cfgr1::DFSDM_CH3CFGR1_SPEC>,
    #[doc = "0x64 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch3cfgr2: crate::Reg<dfsdm_ch3cfgr2::DFSDM_CH3CFGR2_SPEC>,
    #[doc = "0x68 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch3awscdr: crate::Reg<dfsdm_ch3awscdr::DFSDM_CH3AWSCDR_SPEC>,
    #[doc = "0x6c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch3wdatr: crate::Reg<dfsdm_ch3wdatr::DFSDM_CH3WDATR_SPEC>,
    #[doc = "0x70 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch3datinr: crate::Reg<dfsdm_ch3datinr::DFSDM_CH3DATINR_SPEC>,
    #[doc = "0x74 - DFSDM channel 3 delay register"]
    pub dfsdm_ch3dlyr: crate::Reg<dfsdm_ch3dlyr::DFSDM_CH3DLYR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch4cfgr1: crate::Reg<dfsdm_ch4cfgr1::DFSDM_CH4CFGR1_SPEC>,
    #[doc = "0x84 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch4cfgr2: crate::Reg<dfsdm_ch4cfgr2::DFSDM_CH4CFGR2_SPEC>,
    #[doc = "0x88 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch4awscdr: crate::Reg<dfsdm_ch4awscdr::DFSDM_CH4AWSCDR_SPEC>,
    #[doc = "0x8c - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch4wdatr: crate::Reg<dfsdm_ch4wdatr::DFSDM_CH4WDATR_SPEC>,
    #[doc = "0x90 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch4datinr: crate::Reg<dfsdm_ch4datinr::DFSDM_CH4DATINR_SPEC>,
    #[doc = "0x94 - DFSDM channel 4 delay register"]
    pub dfsdm_ch4dlyr: crate::Reg<dfsdm_ch4dlyr::DFSDM_CH4DLYR_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0xa0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch5cfgr1: crate::Reg<dfsdm_ch5cfgr1::DFSDM_CH5CFGR1_SPEC>,
    #[doc = "0xa4 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch5cfgr2: crate::Reg<dfsdm_ch5cfgr2::DFSDM_CH5CFGR2_SPEC>,
    #[doc = "0xa8 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch5awscdr: crate::Reg<dfsdm_ch5awscdr::DFSDM_CH5AWSCDR_SPEC>,
    #[doc = "0xac - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch5wdatr: crate::Reg<dfsdm_ch5wdatr::DFSDM_CH5WDATR_SPEC>,
    #[doc = "0xb0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch5datinr: crate::Reg<dfsdm_ch5datinr::DFSDM_CH5DATINR_SPEC>,
    #[doc = "0xb4 - DFSDM channel 5 delay register"]
    pub dfsdm_ch5dlyr: crate::Reg<dfsdm_ch5dlyr::DFSDM_CH5DLYR_SPEC>,
    _reserved36: [u8; 0x08],
    #[doc = "0xc0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch6cfgr1: crate::Reg<dfsdm_ch6cfgr1::DFSDM_CH6CFGR1_SPEC>,
    #[doc = "0xc4 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch6cfgr2: crate::Reg<dfsdm_ch6cfgr2::DFSDM_CH6CFGR2_SPEC>,
    #[doc = "0xc8 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch6awscdr: crate::Reg<dfsdm_ch6awscdr::DFSDM_CH6AWSCDR_SPEC>,
    #[doc = "0xcc - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch6wdatr: crate::Reg<dfsdm_ch6wdatr::DFSDM_CH6WDATR_SPEC>,
    #[doc = "0xd0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch6datinr: crate::Reg<dfsdm_ch6datinr::DFSDM_CH6DATINR_SPEC>,
    #[doc = "0xd4 - DFSDM channel 6 delay register"]
    pub dfsdm_ch6dlyr: crate::Reg<dfsdm_ch6dlyr::DFSDM_CH6DLYR_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0xe0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch7cfgr1: crate::Reg<dfsdm_ch7cfgr1::DFSDM_CH7CFGR1_SPEC>,
    #[doc = "0xe4 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch7cfgr2: crate::Reg<dfsdm_ch7cfgr2::DFSDM_CH7CFGR2_SPEC>,
    #[doc = "0xe8 - Short-circuit detector and analog watchdog settings for channel y."]
    pub dfsdm_ch7awscdr: crate::Reg<dfsdm_ch7awscdr::DFSDM_CH7AWSCDR_SPEC>,
    #[doc = "0xec - This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
    pub dfsdm_ch7wdatr: crate::Reg<dfsdm_ch7wdatr::DFSDM_CH7WDATR_SPEC>,
    #[doc = "0xf0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch7datinr: crate::Reg<dfsdm_ch7datinr::DFSDM_CH7DATINR_SPEC>,
    #[doc = "0xf4 - DFSDM channel 7 delay register"]
    pub dfsdm_ch7dlyr: crate::Reg<dfsdm_ch7dlyr::DFSDM_CH7DLYR_SPEC>,
    _reserved48: [u8; 0x08],
    #[doc = "0x100 - DFSDM filter 0 control register 1"]
    pub dfsdm_flt0cr1: crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1_SPEC>,
    #[doc = "0x104 - DFSDM filter 0 control register 2"]
    pub dfsdm_flt0cr2: crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2_SPEC>,
    #[doc = "0x108 - DFSDM filter 0 interrupt and status register"]
    pub dfsdm_flt0isr: crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISR_SPEC>,
    #[doc = "0x10c - DFSDM filter 0 interrupt flag clear register"]
    pub dfsdm_flt0icr: crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICR_SPEC>,
    #[doc = "0x110 - DFSDM filter 0 injected channel group selection register"]
    pub dfsdm_flt0jchgr: crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGR_SPEC>,
    #[doc = "0x114 - DFSDM filter 0 control register"]
    pub dfsdm_flt0fcr: crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCR_SPEC>,
    #[doc = "0x118 - DFSDM filter 0 data register for injected group"]
    pub dfsdm_flt0jdatar: crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATAR_SPEC>,
    #[doc = "0x11c - DFSDM filter 0 data register for the regular channel"]
    pub dfsdm_flt0rdatar: crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATAR_SPEC>,
    #[doc = "0x120 - DFSDM filter 0 analog watchdog high threshold register"]
    pub dfsdm_flt0awhtr: crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTR_SPEC>,
    #[doc = "0x124 - DFSDM filter 0 analog watchdog low threshold register"]
    pub dfsdm_flt0awltr: crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTR_SPEC>,
    #[doc = "0x128 - DFSDM filter 0 analog watchdog status register"]
    pub dfsdm_flt0awsr: crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSR_SPEC>,
    #[doc = "0x12c - DFSDM filter 0 analog watchdog clear flag register"]
    pub dfsdm_flt0awcfr: crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFR_SPEC>,
    #[doc = "0x130 - DFSDM filter 0 extremes detector maximum register"]
    pub dfsdm_flt0exmax: crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAX_SPEC>,
    #[doc = "0x134 - DFSDM filter 0 extremes detector minimum register"]
    pub dfsdm_flt0exmin: crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMIN_SPEC>,
    #[doc = "0x138 - DFSDM filter 0 conversion timer register"]
    pub dfsdm_flt0cnvtimr: crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMR_SPEC>,
    _reserved63: [u8; 0x44],
    #[doc = "0x180 - DFSDM filter 1 control register 1"]
    pub dfsdm_flt1cr1: crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1_SPEC>,
    #[doc = "0x184 - DFSDM filter 1 control register 2"]
    pub dfsdm_flt1cr2: crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2_SPEC>,
    #[doc = "0x188 - DFSDM filter 1 interrupt and status register"]
    pub dfsdm_flt1isr: crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISR_SPEC>,
    #[doc = "0x18c - DFSDM filter 1 interrupt flag clear register"]
    pub dfsdm_flt1icr: crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICR_SPEC>,
    #[doc = "0x190 - DFSDM filter 1 injected channel group selection register"]
    pub dfsdm_flt1jchgr: crate::Reg<dfsdm_flt1jchgr::DFSDM_FLT1JCHGR_SPEC>,
    #[doc = "0x194 - DFSDM filter 1 control register"]
    pub dfsdm_flt1fcr: crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCR_SPEC>,
    #[doc = "0x198 - DFSDM filter 1 data register for injected group"]
    pub dfsdm_flt1jdatar: crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATAR_SPEC>,
    #[doc = "0x19c - DFSDM filter 1 data register for the regular channel"]
    pub dfsdm_flt1rdatar: crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATAR_SPEC>,
    #[doc = "0x1a0 - DFSDM filter 1 analog watchdog high threshold register"]
    pub dfsdm_flt1awhtr: crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTR_SPEC>,
    #[doc = "0x1a4 - DFSDM filter 1 analog watchdog low threshold register"]
    pub dfsdm_flt1awltr: crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTR_SPEC>,
    #[doc = "0x1a8 - DFSDM filter 1 analog watchdog status register"]
    pub dfsdm_flt1awsr: crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSR_SPEC>,
    #[doc = "0x1ac - DFSDM filter 1 analog watchdog clear flag register"]
    pub dfsdm_flt1awcfr: crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFR_SPEC>,
    #[doc = "0x1b0 - DFSDM filter 1 extremes detector maximum register"]
    pub dfsdm_flt1exmax: crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAX_SPEC>,
    #[doc = "0x1b4 - DFSDM filter 1 extremes detector minimum register"]
    pub dfsdm_flt1exmin: crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMIN_SPEC>,
    #[doc = "0x1b8 - DFSDM filter 1 conversion timer register"]
    pub dfsdm_flt1cnvtimr: crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMR_SPEC>,
    _reserved78: [u8; 0x44],
    #[doc = "0x200 - DFSDM filter 2 control register 1"]
    pub dfsdm_flt2cr1: crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1_SPEC>,
    #[doc = "0x204 - DFSDM filter 2 control register 2"]
    pub dfsdm_flt2cr2: crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2_SPEC>,
    #[doc = "0x208 - DFSDM filter 2 interrupt and status register"]
    pub dfsdm_flt2isr: crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISR_SPEC>,
    #[doc = "0x20c - DFSDM filter 2 interrupt flag clear register"]
    pub dfsdm_flt2icr: crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICR_SPEC>,
    #[doc = "0x210 - DFSDM filter 2 injected channel group selection register"]
    pub dfsdm_flt2jchgr: crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGR_SPEC>,
    #[doc = "0x214 - DFSDM filter 2 control register"]
    pub dfsdm_flt2fcr: crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCR_SPEC>,
    #[doc = "0x218 - DFSDM filter 2 data register for injected group"]
    pub dfsdm_flt2jdatar: crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATAR_SPEC>,
    #[doc = "0x21c - DFSDM filter 2 data register for the regular channel"]
    pub dfsdm_flt2rdatar: crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATAR_SPEC>,
    #[doc = "0x220 - DFSDM filter 2 analog watchdog high threshold register"]
    pub dfsdm_flt2awhtr: crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTR_SPEC>,
    #[doc = "0x224 - DFSDM filter 2 analog watchdog low threshold register"]
    pub dfsdm_flt2awltr: crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTR_SPEC>,
    #[doc = "0x228 - DFSDM filter 2 analog watchdog status register"]
    pub dfsdm_flt2awsr: crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSR_SPEC>,
    #[doc = "0x22c - DFSDM filter 2 analog watchdog clear flag register"]
    pub dfsdm_flt2awcfr: crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFR_SPEC>,
    #[doc = "0x230 - DFSDM filter 2 extremes detector maximum register"]
    pub dfsdm_flt2exmax: crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAX_SPEC>,
    #[doc = "0x234 - DFSDM filter 2 extremes detector minimum register"]
    pub dfsdm_flt2exmin: crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMIN_SPEC>,
    #[doc = "0x238 - DFSDM filter 2 conversion timer register"]
    pub dfsdm_flt2cnvtimr: crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMR_SPEC>,
    _reserved93: [u8; 0x44],
    #[doc = "0x280 - DFSDM filter 3 control register 1"]
    pub dfsdm_flt3cr1: crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1_SPEC>,
    #[doc = "0x284 - DFSDM filter 3 control register 2"]
    pub dfsdm_flt3cr2: crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2_SPEC>,
    #[doc = "0x288 - DFSDM filter 3 interrupt and status register"]
    pub dfsdm_flt3isr: crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISR_SPEC>,
    #[doc = "0x28c - DFSDM filter 3 interrupt flag clear register"]
    pub dfsdm_flt3icr: crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICR_SPEC>,
    #[doc = "0x290 - DFSDM filter 3 injected channel group selection register"]
    pub dfsdm_flt3jchgr: crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGR_SPEC>,
    #[doc = "0x294 - DFSDM filter 3 control register"]
    pub dfsdm_flt3fcr: crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCR_SPEC>,
    #[doc = "0x298 - DFSDM filter 3 data register for injected group"]
    pub dfsdm_flt3jdatar: crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATAR_SPEC>,
    #[doc = "0x29c - DFSDM filter 3 data register for the regular channel"]
    pub dfsdm_flt3rdatar: crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATAR_SPEC>,
    #[doc = "0x2a0 - DFSDM filter 3 analog watchdog high threshold register"]
    pub dfsdm_flt3awhtr: crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTR_SPEC>,
    #[doc = "0x2a4 - DFSDM filter 3 analog watchdog low threshold register"]
    pub dfsdm_flt3awltr: crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTR_SPEC>,
    #[doc = "0x2a8 - DFSDM filter 3 analog watchdog status register"]
    pub dfsdm_flt3awsr: crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSR_SPEC>,
    #[doc = "0x2ac - DFSDM filter 3 analog watchdog clear flag register"]
    pub dfsdm_flt3awcfr: crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFR_SPEC>,
    #[doc = "0x2b0 - DFSDM filter 3 extremes detector maximum register"]
    pub dfsdm_flt3exmax: crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAX_SPEC>,
    #[doc = "0x2b4 - DFSDM filter 3 extremes detector minimum register"]
    pub dfsdm_flt3exmin: crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMIN_SPEC>,
    #[doc = "0x2b8 - DFSDM filter 3 conversion timer register"]
    pub dfsdm_flt3cnvtimr: crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMR_SPEC>,
    _reserved108: [u8; 0x44],
    #[doc = "0x300 - DFSDM filter 4 control register 1"]
    pub dfsdm_flt4cr1: crate::Reg<dfsdm_flt4cr1::DFSDM_FLT4CR1_SPEC>,
    #[doc = "0x304 - DFSDM filter 4 control register 2"]
    pub dfsdm_flt4cr2: crate::Reg<dfsdm_flt4cr2::DFSDM_FLT4CR2_SPEC>,
    #[doc = "0x308 - DFSDM filter 4 interrupt and status register"]
    pub dfsdm_flt4isr: crate::Reg<dfsdm_flt4isr::DFSDM_FLT4ISR_SPEC>,
    #[doc = "0x30c - DFSDM filter 4 interrupt flag clear register"]
    pub dfsdm_flt4icr: crate::Reg<dfsdm_flt4icr::DFSDM_FLT4ICR_SPEC>,
    #[doc = "0x310 - DFSDM filter 4 injected channel group selection register"]
    pub dfsdm_flt4jchgr: crate::Reg<dfsdm_flt4jchgr::DFSDM_FLT4JCHGR_SPEC>,
    #[doc = "0x314 - DFSDM filter 4 control register"]
    pub dfsdm_flt4fcr: crate::Reg<dfsdm_flt4fcr::DFSDM_FLT4FCR_SPEC>,
    #[doc = "0x318 - DFSDM filter 4 data register for injected group"]
    pub dfsdm_flt4jdatar: crate::Reg<dfsdm_flt4jdatar::DFSDM_FLT4JDATAR_SPEC>,
    #[doc = "0x31c - DFSDM filter 4 data register for the regular channel"]
    pub dfsdm_flt4rdatar: crate::Reg<dfsdm_flt4rdatar::DFSDM_FLT4RDATAR_SPEC>,
    #[doc = "0x320 - DFSDM filter 4 analog watchdog high threshold register"]
    pub dfsdm_flt4awhtr: crate::Reg<dfsdm_flt4awhtr::DFSDM_FLT4AWHTR_SPEC>,
    #[doc = "0x324 - DFSDM filter 4 analog watchdog low threshold register"]
    pub dfsdm_flt4awltr: crate::Reg<dfsdm_flt4awltr::DFSDM_FLT4AWLTR_SPEC>,
    #[doc = "0x328 - DFSDM filter 4 analog watchdog status register"]
    pub dfsdm_flt4awsr: crate::Reg<dfsdm_flt4awsr::DFSDM_FLT4AWSR_SPEC>,
    #[doc = "0x32c - DFSDM filter 4 analog watchdog clear flag register"]
    pub dfsdm_flt4awcfr: crate::Reg<dfsdm_flt4awcfr::DFSDM_FLT4AWCFR_SPEC>,
    #[doc = "0x330 - DFSDM filter 4 extremes detector maximum register"]
    pub dfsdm_flt4exmax: crate::Reg<dfsdm_flt4exmax::DFSDM_FLT4EXMAX_SPEC>,
    #[doc = "0x334 - DFSDM filter 4 extremes detector minimum register"]
    pub dfsdm_flt4exmin: crate::Reg<dfsdm_flt4exmin::DFSDM_FLT4EXMIN_SPEC>,
    #[doc = "0x338 - DFSDM filter 4 conversion timer register"]
    pub dfsdm_flt4cnvtimr: crate::Reg<dfsdm_flt4cnvtimr::DFSDM_FLT4CNVTIMR_SPEC>,
    _reserved123: [u8; 0x44],
    #[doc = "0x380 - DFSDM filter 5 control register 1"]
    pub dfsdm_flt5cr1: crate::Reg<dfsdm_flt5cr1::DFSDM_FLT5CR1_SPEC>,
    #[doc = "0x384 - DFSDM filter 5 control register 2"]
    pub dfsdm_flt5cr2: crate::Reg<dfsdm_flt5cr2::DFSDM_FLT5CR2_SPEC>,
    #[doc = "0x388 - DFSDM filter 5 interrupt and status register"]
    pub dfsdm_flt5isr: crate::Reg<dfsdm_flt5isr::DFSDM_FLT5ISR_SPEC>,
    #[doc = "0x38c - DFSDM filter 5 interrupt flag clear register"]
    pub dfsdm_flt5icr: crate::Reg<dfsdm_flt5icr::DFSDM_FLT5ICR_SPEC>,
    #[doc = "0x390 - DFSDM filter 5 injected channel group selection register"]
    pub dfsdm_flt5jchgr: crate::Reg<dfsdm_flt5jchgr::DFSDM_FLT5JCHGR_SPEC>,
    #[doc = "0x394 - DFSDM filter 5 control register"]
    pub dfsdm_flt5fcr: crate::Reg<dfsdm_flt5fcr::DFSDM_FLT5FCR_SPEC>,
    #[doc = "0x398 - DFSDM filter 5 data register for injected group"]
    pub dfsdm_flt5jdatar: crate::Reg<dfsdm_flt5jdatar::DFSDM_FLT5JDATAR_SPEC>,
    #[doc = "0x39c - DFSDM filter 5 data register for the regular channel"]
    pub dfsdm_flt5rdatar: crate::Reg<dfsdm_flt5rdatar::DFSDM_FLT5RDATAR_SPEC>,
    #[doc = "0x3a0 - DFSDM filter 5 analog watchdog high threshold register"]
    pub dfsdm_flt5awhtr: crate::Reg<dfsdm_flt5awhtr::DFSDM_FLT5AWHTR_SPEC>,
    #[doc = "0x3a4 - DFSDM filter 5 analog watchdog low threshold register"]
    pub dfsdm_flt5awltr: crate::Reg<dfsdm_flt5awltr::DFSDM_FLT5AWLTR_SPEC>,
    #[doc = "0x3a8 - DFSDM filter 5 analog watchdog status register"]
    pub dfsdm_flt5awsr: crate::Reg<dfsdm_flt5awsr::DFSDM_FLT5AWSR_SPEC>,
    #[doc = "0x3ac - DFSDM filter 5 analog watchdog clear flag register"]
    pub dfsdm_flt5awcfr: crate::Reg<dfsdm_flt5awcfr::DFSDM_FLT5AWCFR_SPEC>,
    #[doc = "0x3b0 - DFSDM filter 5 extremes detector maximum register"]
    pub dfsdm_flt5exmax: crate::Reg<dfsdm_flt5exmax::DFSDM_FLT5EXMAX_SPEC>,
    #[doc = "0x3b4 - DFSDM filter 5 extremes detector minimum register"]
    pub dfsdm_flt5exmin: crate::Reg<dfsdm_flt5exmin::DFSDM_FLT5EXMIN_SPEC>,
    #[doc = "0x3b8 - DFSDM filter 5 conversion timer register"]
    pub dfsdm_flt5cnvtimr: crate::Reg<dfsdm_flt5cnvtimr::DFSDM_FLT5CNVTIMR_SPEC>,
    _reserved138: [u8; 0x0434],
    #[doc = "0x7f0 - This register specifies the hardware configuration of DFSDM peripheral."]
    pub dfsdm_hwcfgr: crate::Reg<dfsdm_hwcfgr::DFSDM_HWCFGR_SPEC>,
    #[doc = "0x7f4 - This register specifies the version of DFSDM peripheral."]
    pub dfsdm_verr: crate::Reg<dfsdm_verr::DFSDM_VERR_SPEC>,
    #[doc = "0x7f8 - This register specifies the identification of DFSDM peripheral."]
    pub dfsdm_ipidr: crate::Reg<dfsdm_ipidr::DFSDM_IPIDR_SPEC>,
    #[doc = "0x7fc - This register specifies the size allocated to DFSDM registers."]
    pub dfsdm_sidr: crate::Reg<dfsdm_sidr::DFSDM_SIDR_SPEC>,
}
#[doc = "DFSDM_CH0CFGR1 register accessor: an alias for `Reg<DFSDM_CH0CFGR1_SPEC>`"]
pub type DFSDM_CH0CFGR1 = crate::Reg<dfsdm_ch0cfgr1::DFSDM_CH0CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch0cfgr1;
#[doc = "DFSDM_CH0CFGR2 register accessor: an alias for `Reg<DFSDM_CH0CFGR2_SPEC>`"]
pub type DFSDM_CH0CFGR2 = crate::Reg<dfsdm_ch0cfgr2::DFSDM_CH0CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch0cfgr2;
#[doc = "DFSDM_CH0AWSCDR register accessor: an alias for `Reg<DFSDM_CH0AWSCDR_SPEC>`"]
pub type DFSDM_CH0AWSCDR = crate::Reg<dfsdm_ch0awscdr::DFSDM_CH0AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch0awscdr;
#[doc = "DFSDM_CH0WDATR register accessor: an alias for `Reg<DFSDM_CH0WDATR_SPEC>`"]
pub type DFSDM_CH0WDATR = crate::Reg<dfsdm_ch0wdatr::DFSDM_CH0WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch0wdatr;
#[doc = "DFSDM_CH0DATINR register accessor: an alias for `Reg<DFSDM_CH0DATINR_SPEC>`"]
pub type DFSDM_CH0DATINR = crate::Reg<dfsdm_ch0datinr::DFSDM_CH0DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch0datinr;
#[doc = "DFSDM_CH0DLYR register accessor: an alias for `Reg<DFSDM_CH0DLYR_SPEC>`"]
pub type DFSDM_CH0DLYR = crate::Reg<dfsdm_ch0dlyr::DFSDM_CH0DLYR_SPEC>;
#[doc = "DFSDM channel 0 delay register"]
pub mod dfsdm_ch0dlyr;
#[doc = "DFSDM_CH1CFGR1 register accessor: an alias for `Reg<DFSDM_CH1CFGR1_SPEC>`"]
pub type DFSDM_CH1CFGR1 = crate::Reg<dfsdm_ch1cfgr1::DFSDM_CH1CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch1cfgr1;
#[doc = "DFSDM_CH1CFGR2 register accessor: an alias for `Reg<DFSDM_CH1CFGR2_SPEC>`"]
pub type DFSDM_CH1CFGR2 = crate::Reg<dfsdm_ch1cfgr2::DFSDM_CH1CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch1cfgr2;
#[doc = "DFSDM_CH1AWSCDR register accessor: an alias for `Reg<DFSDM_CH1AWSCDR_SPEC>`"]
pub type DFSDM_CH1AWSCDR = crate::Reg<dfsdm_ch1awscdr::DFSDM_CH1AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch1awscdr;
#[doc = "DFSDM_CH1WDATR register accessor: an alias for `Reg<DFSDM_CH1WDATR_SPEC>`"]
pub type DFSDM_CH1WDATR = crate::Reg<dfsdm_ch1wdatr::DFSDM_CH1WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch1wdatr;
#[doc = "DFSDM_CH1DATINR register accessor: an alias for `Reg<DFSDM_CH1DATINR_SPEC>`"]
pub type DFSDM_CH1DATINR = crate::Reg<dfsdm_ch1datinr::DFSDM_CH1DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch1datinr;
#[doc = "DFSDM_CH1DLYR register accessor: an alias for `Reg<DFSDM_CH1DLYR_SPEC>`"]
pub type DFSDM_CH1DLYR = crate::Reg<dfsdm_ch1dlyr::DFSDM_CH1DLYR_SPEC>;
#[doc = "DFSDM channel 1 delay register"]
pub mod dfsdm_ch1dlyr;
#[doc = "DFSDM_CH2CFGR1 register accessor: an alias for `Reg<DFSDM_CH2CFGR1_SPEC>`"]
pub type DFSDM_CH2CFGR1 = crate::Reg<dfsdm_ch2cfgr1::DFSDM_CH2CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch2cfgr1;
#[doc = "DFSDM_CH2CFGR2 register accessor: an alias for `Reg<DFSDM_CH2CFGR2_SPEC>`"]
pub type DFSDM_CH2CFGR2 = crate::Reg<dfsdm_ch2cfgr2::DFSDM_CH2CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch2cfgr2;
#[doc = "DFSDM_CH2AWSCDR register accessor: an alias for `Reg<DFSDM_CH2AWSCDR_SPEC>`"]
pub type DFSDM_CH2AWSCDR = crate::Reg<dfsdm_ch2awscdr::DFSDM_CH2AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch2awscdr;
#[doc = "DFSDM_CH2WDATR register accessor: an alias for `Reg<DFSDM_CH2WDATR_SPEC>`"]
pub type DFSDM_CH2WDATR = crate::Reg<dfsdm_ch2wdatr::DFSDM_CH2WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch2wdatr;
#[doc = "DFSDM_CH2DATINR register accessor: an alias for `Reg<DFSDM_CH2DATINR_SPEC>`"]
pub type DFSDM_CH2DATINR = crate::Reg<dfsdm_ch2datinr::DFSDM_CH2DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch2datinr;
#[doc = "DFSDM_CH2DLYR register accessor: an alias for `Reg<DFSDM_CH2DLYR_SPEC>`"]
pub type DFSDM_CH2DLYR = crate::Reg<dfsdm_ch2dlyr::DFSDM_CH2DLYR_SPEC>;
#[doc = "DFSDM channel 2 delay register"]
pub mod dfsdm_ch2dlyr;
#[doc = "DFSDM_CH3CFGR1 register accessor: an alias for `Reg<DFSDM_CH3CFGR1_SPEC>`"]
pub type DFSDM_CH3CFGR1 = crate::Reg<dfsdm_ch3cfgr1::DFSDM_CH3CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch3cfgr1;
#[doc = "DFSDM_CH3CFGR2 register accessor: an alias for `Reg<DFSDM_CH3CFGR2_SPEC>`"]
pub type DFSDM_CH3CFGR2 = crate::Reg<dfsdm_ch3cfgr2::DFSDM_CH3CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch3cfgr2;
#[doc = "DFSDM_CH3AWSCDR register accessor: an alias for `Reg<DFSDM_CH3AWSCDR_SPEC>`"]
pub type DFSDM_CH3AWSCDR = crate::Reg<dfsdm_ch3awscdr::DFSDM_CH3AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch3awscdr;
#[doc = "DFSDM_CH3WDATR register accessor: an alias for `Reg<DFSDM_CH3WDATR_SPEC>`"]
pub type DFSDM_CH3WDATR = crate::Reg<dfsdm_ch3wdatr::DFSDM_CH3WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch3wdatr;
#[doc = "DFSDM_CH3DATINR register accessor: an alias for `Reg<DFSDM_CH3DATINR_SPEC>`"]
pub type DFSDM_CH3DATINR = crate::Reg<dfsdm_ch3datinr::DFSDM_CH3DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch3datinr;
#[doc = "DFSDM_CH3DLYR register accessor: an alias for `Reg<DFSDM_CH3DLYR_SPEC>`"]
pub type DFSDM_CH3DLYR = crate::Reg<dfsdm_ch3dlyr::DFSDM_CH3DLYR_SPEC>;
#[doc = "DFSDM channel 3 delay register"]
pub mod dfsdm_ch3dlyr;
#[doc = "DFSDM_CH4CFGR1 register accessor: an alias for `Reg<DFSDM_CH4CFGR1_SPEC>`"]
pub type DFSDM_CH4CFGR1 = crate::Reg<dfsdm_ch4cfgr1::DFSDM_CH4CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch4cfgr1;
#[doc = "DFSDM_CH4CFGR2 register accessor: an alias for `Reg<DFSDM_CH4CFGR2_SPEC>`"]
pub type DFSDM_CH4CFGR2 = crate::Reg<dfsdm_ch4cfgr2::DFSDM_CH4CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch4cfgr2;
#[doc = "DFSDM_CH4AWSCDR register accessor: an alias for `Reg<DFSDM_CH4AWSCDR_SPEC>`"]
pub type DFSDM_CH4AWSCDR = crate::Reg<dfsdm_ch4awscdr::DFSDM_CH4AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch4awscdr;
#[doc = "DFSDM_CH4WDATR register accessor: an alias for `Reg<DFSDM_CH4WDATR_SPEC>`"]
pub type DFSDM_CH4WDATR = crate::Reg<dfsdm_ch4wdatr::DFSDM_CH4WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch4wdatr;
#[doc = "DFSDM_CH4DATINR register accessor: an alias for `Reg<DFSDM_CH4DATINR_SPEC>`"]
pub type DFSDM_CH4DATINR = crate::Reg<dfsdm_ch4datinr::DFSDM_CH4DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch4datinr;
#[doc = "DFSDM_CH4DLYR register accessor: an alias for `Reg<DFSDM_CH4DLYR_SPEC>`"]
pub type DFSDM_CH4DLYR = crate::Reg<dfsdm_ch4dlyr::DFSDM_CH4DLYR_SPEC>;
#[doc = "DFSDM channel 4 delay register"]
pub mod dfsdm_ch4dlyr;
#[doc = "DFSDM_CH5CFGR1 register accessor: an alias for `Reg<DFSDM_CH5CFGR1_SPEC>`"]
pub type DFSDM_CH5CFGR1 = crate::Reg<dfsdm_ch5cfgr1::DFSDM_CH5CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch5cfgr1;
#[doc = "DFSDM_CH5CFGR2 register accessor: an alias for `Reg<DFSDM_CH5CFGR2_SPEC>`"]
pub type DFSDM_CH5CFGR2 = crate::Reg<dfsdm_ch5cfgr2::DFSDM_CH5CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch5cfgr2;
#[doc = "DFSDM_CH5AWSCDR register accessor: an alias for `Reg<DFSDM_CH5AWSCDR_SPEC>`"]
pub type DFSDM_CH5AWSCDR = crate::Reg<dfsdm_ch5awscdr::DFSDM_CH5AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch5awscdr;
#[doc = "DFSDM_CH5WDATR register accessor: an alias for `Reg<DFSDM_CH5WDATR_SPEC>`"]
pub type DFSDM_CH5WDATR = crate::Reg<dfsdm_ch5wdatr::DFSDM_CH5WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch5wdatr;
#[doc = "DFSDM_CH5DATINR register accessor: an alias for `Reg<DFSDM_CH5DATINR_SPEC>`"]
pub type DFSDM_CH5DATINR = crate::Reg<dfsdm_ch5datinr::DFSDM_CH5DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch5datinr;
#[doc = "DFSDM_CH5DLYR register accessor: an alias for `Reg<DFSDM_CH5DLYR_SPEC>`"]
pub type DFSDM_CH5DLYR = crate::Reg<dfsdm_ch5dlyr::DFSDM_CH5DLYR_SPEC>;
#[doc = "DFSDM channel 5 delay register"]
pub mod dfsdm_ch5dlyr;
#[doc = "DFSDM_CH6CFGR1 register accessor: an alias for `Reg<DFSDM_CH6CFGR1_SPEC>`"]
pub type DFSDM_CH6CFGR1 = crate::Reg<dfsdm_ch6cfgr1::DFSDM_CH6CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch6cfgr1;
#[doc = "DFSDM_CH6CFGR2 register accessor: an alias for `Reg<DFSDM_CH6CFGR2_SPEC>`"]
pub type DFSDM_CH6CFGR2 = crate::Reg<dfsdm_ch6cfgr2::DFSDM_CH6CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch6cfgr2;
#[doc = "DFSDM_CH6AWSCDR register accessor: an alias for `Reg<DFSDM_CH6AWSCDR_SPEC>`"]
pub type DFSDM_CH6AWSCDR = crate::Reg<dfsdm_ch6awscdr::DFSDM_CH6AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch6awscdr;
#[doc = "DFSDM_CH6WDATR register accessor: an alias for `Reg<DFSDM_CH6WDATR_SPEC>`"]
pub type DFSDM_CH6WDATR = crate::Reg<dfsdm_ch6wdatr::DFSDM_CH6WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch6wdatr;
#[doc = "DFSDM_CH6DATINR register accessor: an alias for `Reg<DFSDM_CH6DATINR_SPEC>`"]
pub type DFSDM_CH6DATINR = crate::Reg<dfsdm_ch6datinr::DFSDM_CH6DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch6datinr;
#[doc = "DFSDM_CH6DLYR register accessor: an alias for `Reg<DFSDM_CH6DLYR_SPEC>`"]
pub type DFSDM_CH6DLYR = crate::Reg<dfsdm_ch6dlyr::DFSDM_CH6DLYR_SPEC>;
#[doc = "DFSDM channel 6 delay register"]
pub mod dfsdm_ch6dlyr;
#[doc = "DFSDM_CH7CFGR1 register accessor: an alias for `Reg<DFSDM_CH7CFGR1_SPEC>`"]
pub type DFSDM_CH7CFGR1 = crate::Reg<dfsdm_ch7cfgr1::DFSDM_CH7CFGR1_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch7cfgr1;
#[doc = "DFSDM_CH7CFGR2 register accessor: an alias for `Reg<DFSDM_CH7CFGR2_SPEC>`"]
pub type DFSDM_CH7CFGR2 = crate::Reg<dfsdm_ch7cfgr2::DFSDM_CH7CFGR2_SPEC>;
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch7cfgr2;
#[doc = "DFSDM_CH7AWSCDR register accessor: an alias for `Reg<DFSDM_CH7AWSCDR_SPEC>`"]
pub type DFSDM_CH7AWSCDR = crate::Reg<dfsdm_ch7awscdr::DFSDM_CH7AWSCDR_SPEC>;
#[doc = "Short-circuit detector and analog watchdog settings for channel y."]
pub mod dfsdm_ch7awscdr;
#[doc = "DFSDM_CH7WDATR register accessor: an alias for `Reg<DFSDM_CH7WDATR_SPEC>`"]
pub type DFSDM_CH7WDATR = crate::Reg<dfsdm_ch7wdatr::DFSDM_CH7WDATR_SPEC>;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y."]
pub mod dfsdm_ch7wdatr;
#[doc = "DFSDM_CH7DATINR register accessor: an alias for `Reg<DFSDM_CH7DATINR_SPEC>`"]
pub type DFSDM_CH7DATINR = crate::Reg<dfsdm_ch7datinr::DFSDM_CH7DATINR_SPEC>;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch7datinr;
#[doc = "DFSDM_CH7DLYR register accessor: an alias for `Reg<DFSDM_CH7DLYR_SPEC>`"]
pub type DFSDM_CH7DLYR = crate::Reg<dfsdm_ch7dlyr::DFSDM_CH7DLYR_SPEC>;
#[doc = "DFSDM channel 7 delay register"]
pub mod dfsdm_ch7dlyr;
#[doc = "DFSDM_FLT0CR1 register accessor: an alias for `Reg<DFSDM_FLT0CR1_SPEC>`"]
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1_SPEC>;
#[doc = "DFSDM filter 0 control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM_FLT0CR2 register accessor: an alias for `Reg<DFSDM_FLT0CR2_SPEC>`"]
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2_SPEC>;
#[doc = "DFSDM filter 0 control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM_FLT0ISR register accessor: an alias for `Reg<DFSDM_FLT0ISR_SPEC>`"]
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISR_SPEC>;
#[doc = "DFSDM filter 0 interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM_FLT0ICR register accessor: an alias for `Reg<DFSDM_FLT0ICR_SPEC>`"]
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICR_SPEC>;
#[doc = "DFSDM filter 0 interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM_FLT0JCHGR register accessor: an alias for `Reg<DFSDM_FLT0JCHGR_SPEC>`"]
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGR_SPEC>;
#[doc = "DFSDM filter 0 injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM_FLT0FCR register accessor: an alias for `Reg<DFSDM_FLT0FCR_SPEC>`"]
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCR_SPEC>;
#[doc = "DFSDM filter 0 control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM_FLT0JDATAR register accessor: an alias for `Reg<DFSDM_FLT0JDATAR_SPEC>`"]
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATAR_SPEC>;
#[doc = "DFSDM filter 0 data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM_FLT0RDATAR register accessor: an alias for `Reg<DFSDM_FLT0RDATAR_SPEC>`"]
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATAR_SPEC>;
#[doc = "DFSDM filter 0 data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM_FLT0AWHTR register accessor: an alias for `Reg<DFSDM_FLT0AWHTR_SPEC>`"]
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM_FLT0AWLTR register accessor: an alias for `Reg<DFSDM_FLT0AWLTR_SPEC>`"]
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM_FLT0AWSR register accessor: an alias for `Reg<DFSDM_FLT0AWSR_SPEC>`"]
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM_FLT0AWCFR register accessor: an alias for `Reg<DFSDM_FLT0AWCFR_SPEC>`"]
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFR_SPEC>;
#[doc = "DFSDM filter 0 analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM_FLT0EXMAX register accessor: an alias for `Reg<DFSDM_FLT0EXMAX_SPEC>`"]
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAX_SPEC>;
#[doc = "DFSDM filter 0 extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM_FLT0EXMIN register accessor: an alias for `Reg<DFSDM_FLT0EXMIN_SPEC>`"]
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMIN_SPEC>;
#[doc = "DFSDM filter 0 extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM_FLT0CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT0CNVTIMR_SPEC>`"]
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMR_SPEC>;
#[doc = "DFSDM filter 0 conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM_FLT1CR1 register accessor: an alias for `Reg<DFSDM_FLT1CR1_SPEC>`"]
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1_SPEC>;
#[doc = "DFSDM filter 1 control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM_FLT1CR2 register accessor: an alias for `Reg<DFSDM_FLT1CR2_SPEC>`"]
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2_SPEC>;
#[doc = "DFSDM filter 1 control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM_FLT1ISR register accessor: an alias for `Reg<DFSDM_FLT1ISR_SPEC>`"]
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISR_SPEC>;
#[doc = "DFSDM filter 1 interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM_FLT1ICR register accessor: an alias for `Reg<DFSDM_FLT1ICR_SPEC>`"]
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICR_SPEC>;
#[doc = "DFSDM filter 1 interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM_FLT1JCHGR register accessor: an alias for `Reg<DFSDM_FLT1JCHGR_SPEC>`"]
pub type DFSDM_FLT1JCHGR = crate::Reg<dfsdm_flt1jchgr::DFSDM_FLT1JCHGR_SPEC>;
#[doc = "DFSDM filter 1 injected channel group selection register"]
pub mod dfsdm_flt1jchgr;
#[doc = "DFSDM_FLT1FCR register accessor: an alias for `Reg<DFSDM_FLT1FCR_SPEC>`"]
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCR_SPEC>;
#[doc = "DFSDM filter 1 control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM_FLT1JDATAR register accessor: an alias for `Reg<DFSDM_FLT1JDATAR_SPEC>`"]
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATAR_SPEC>;
#[doc = "DFSDM filter 1 data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM_FLT1RDATAR register accessor: an alias for `Reg<DFSDM_FLT1RDATAR_SPEC>`"]
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATAR_SPEC>;
#[doc = "DFSDM filter 1 data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM_FLT1AWHTR register accessor: an alias for `Reg<DFSDM_FLT1AWHTR_SPEC>`"]
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM_FLT1AWLTR register accessor: an alias for `Reg<DFSDM_FLT1AWLTR_SPEC>`"]
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM_FLT1AWSR register accessor: an alias for `Reg<DFSDM_FLT1AWSR_SPEC>`"]
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM_FLT1AWCFR register accessor: an alias for `Reg<DFSDM_FLT1AWCFR_SPEC>`"]
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFR_SPEC>;
#[doc = "DFSDM filter 1 analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM_FLT1EXMAX register accessor: an alias for `Reg<DFSDM_FLT1EXMAX_SPEC>`"]
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAX_SPEC>;
#[doc = "DFSDM filter 1 extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM_FLT1EXMIN register accessor: an alias for `Reg<DFSDM_FLT1EXMIN_SPEC>`"]
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMIN_SPEC>;
#[doc = "DFSDM filter 1 extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM_FLT1CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT1CNVTIMR_SPEC>`"]
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMR_SPEC>;
#[doc = "DFSDM filter 1 conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM_FLT2CR1 register accessor: an alias for `Reg<DFSDM_FLT2CR1_SPEC>`"]
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1_SPEC>;
#[doc = "DFSDM filter 2 control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM_FLT2CR2 register accessor: an alias for `Reg<DFSDM_FLT2CR2_SPEC>`"]
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2_SPEC>;
#[doc = "DFSDM filter 2 control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM_FLT2ISR register accessor: an alias for `Reg<DFSDM_FLT2ISR_SPEC>`"]
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISR_SPEC>;
#[doc = "DFSDM filter 2 interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM_FLT2ICR register accessor: an alias for `Reg<DFSDM_FLT2ICR_SPEC>`"]
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICR_SPEC>;
#[doc = "DFSDM filter 2 interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM_FLT2JCHGR register accessor: an alias for `Reg<DFSDM_FLT2JCHGR_SPEC>`"]
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGR_SPEC>;
#[doc = "DFSDM filter 2 injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM_FLT2FCR register accessor: an alias for `Reg<DFSDM_FLT2FCR_SPEC>`"]
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCR_SPEC>;
#[doc = "DFSDM filter 2 control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM_FLT2JDATAR register accessor: an alias for `Reg<DFSDM_FLT2JDATAR_SPEC>`"]
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATAR_SPEC>;
#[doc = "DFSDM filter 2 data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM_FLT2RDATAR register accessor: an alias for `Reg<DFSDM_FLT2RDATAR_SPEC>`"]
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATAR_SPEC>;
#[doc = "DFSDM filter 2 data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM_FLT2AWHTR register accessor: an alias for `Reg<DFSDM_FLT2AWHTR_SPEC>`"]
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM_FLT2AWLTR register accessor: an alias for `Reg<DFSDM_FLT2AWLTR_SPEC>`"]
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM_FLT2AWSR register accessor: an alias for `Reg<DFSDM_FLT2AWSR_SPEC>`"]
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM_FLT2AWCFR register accessor: an alias for `Reg<DFSDM_FLT2AWCFR_SPEC>`"]
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFR_SPEC>;
#[doc = "DFSDM filter 2 analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM_FLT2EXMAX register accessor: an alias for `Reg<DFSDM_FLT2EXMAX_SPEC>`"]
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAX_SPEC>;
#[doc = "DFSDM filter 2 extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM_FLT2EXMIN register accessor: an alias for `Reg<DFSDM_FLT2EXMIN_SPEC>`"]
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMIN_SPEC>;
#[doc = "DFSDM filter 2 extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM_FLT2CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT2CNVTIMR_SPEC>`"]
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMR_SPEC>;
#[doc = "DFSDM filter 2 conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM_FLT3CR1 register accessor: an alias for `Reg<DFSDM_FLT3CR1_SPEC>`"]
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1_SPEC>;
#[doc = "DFSDM filter 3 control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM_FLT3CR2 register accessor: an alias for `Reg<DFSDM_FLT3CR2_SPEC>`"]
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2_SPEC>;
#[doc = "DFSDM filter 3 control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM_FLT3ISR register accessor: an alias for `Reg<DFSDM_FLT3ISR_SPEC>`"]
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISR_SPEC>;
#[doc = "DFSDM filter 3 interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM_FLT3ICR register accessor: an alias for `Reg<DFSDM_FLT3ICR_SPEC>`"]
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICR_SPEC>;
#[doc = "DFSDM filter 3 interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM_FLT3JCHGR register accessor: an alias for `Reg<DFSDM_FLT3JCHGR_SPEC>`"]
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGR_SPEC>;
#[doc = "DFSDM filter 3 injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM_FLT3FCR register accessor: an alias for `Reg<DFSDM_FLT3FCR_SPEC>`"]
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCR_SPEC>;
#[doc = "DFSDM filter 3 control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM_FLT3JDATAR register accessor: an alias for `Reg<DFSDM_FLT3JDATAR_SPEC>`"]
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATAR_SPEC>;
#[doc = "DFSDM filter 3 data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM_FLT3RDATAR register accessor: an alias for `Reg<DFSDM_FLT3RDATAR_SPEC>`"]
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATAR_SPEC>;
#[doc = "DFSDM filter 3 data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM_FLT3AWHTR register accessor: an alias for `Reg<DFSDM_FLT3AWHTR_SPEC>`"]
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM_FLT3AWLTR register accessor: an alias for `Reg<DFSDM_FLT3AWLTR_SPEC>`"]
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM_FLT3AWSR register accessor: an alias for `Reg<DFSDM_FLT3AWSR_SPEC>`"]
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM_FLT3AWCFR register accessor: an alias for `Reg<DFSDM_FLT3AWCFR_SPEC>`"]
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFR_SPEC>;
#[doc = "DFSDM filter 3 analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM_FLT3EXMAX register accessor: an alias for `Reg<DFSDM_FLT3EXMAX_SPEC>`"]
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAX_SPEC>;
#[doc = "DFSDM filter 3 extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM_FLT3EXMIN register accessor: an alias for `Reg<DFSDM_FLT3EXMIN_SPEC>`"]
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMIN_SPEC>;
#[doc = "DFSDM filter 3 extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM_FLT3CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT3CNVTIMR_SPEC>`"]
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMR_SPEC>;
#[doc = "DFSDM filter 3 conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
#[doc = "DFSDM_FLT4CR1 register accessor: an alias for `Reg<DFSDM_FLT4CR1_SPEC>`"]
pub type DFSDM_FLT4CR1 = crate::Reg<dfsdm_flt4cr1::DFSDM_FLT4CR1_SPEC>;
#[doc = "DFSDM filter 4 control register 1"]
pub mod dfsdm_flt4cr1;
#[doc = "DFSDM_FLT4CR2 register accessor: an alias for `Reg<DFSDM_FLT4CR2_SPEC>`"]
pub type DFSDM_FLT4CR2 = crate::Reg<dfsdm_flt4cr2::DFSDM_FLT4CR2_SPEC>;
#[doc = "DFSDM filter 4 control register 2"]
pub mod dfsdm_flt4cr2;
#[doc = "DFSDM_FLT4ISR register accessor: an alias for `Reg<DFSDM_FLT4ISR_SPEC>`"]
pub type DFSDM_FLT4ISR = crate::Reg<dfsdm_flt4isr::DFSDM_FLT4ISR_SPEC>;
#[doc = "DFSDM filter 4 interrupt and status register"]
pub mod dfsdm_flt4isr;
#[doc = "DFSDM_FLT4ICR register accessor: an alias for `Reg<DFSDM_FLT4ICR_SPEC>`"]
pub type DFSDM_FLT4ICR = crate::Reg<dfsdm_flt4icr::DFSDM_FLT4ICR_SPEC>;
#[doc = "DFSDM filter 4 interrupt flag clear register"]
pub mod dfsdm_flt4icr;
#[doc = "DFSDM_FLT4JCHGR register accessor: an alias for `Reg<DFSDM_FLT4JCHGR_SPEC>`"]
pub type DFSDM_FLT4JCHGR = crate::Reg<dfsdm_flt4jchgr::DFSDM_FLT4JCHGR_SPEC>;
#[doc = "DFSDM filter 4 injected channel group selection register"]
pub mod dfsdm_flt4jchgr;
#[doc = "DFSDM_FLT4FCR register accessor: an alias for `Reg<DFSDM_FLT4FCR_SPEC>`"]
pub type DFSDM_FLT4FCR = crate::Reg<dfsdm_flt4fcr::DFSDM_FLT4FCR_SPEC>;
#[doc = "DFSDM filter 4 control register"]
pub mod dfsdm_flt4fcr;
#[doc = "DFSDM_FLT4JDATAR register accessor: an alias for `Reg<DFSDM_FLT4JDATAR_SPEC>`"]
pub type DFSDM_FLT4JDATAR = crate::Reg<dfsdm_flt4jdatar::DFSDM_FLT4JDATAR_SPEC>;
#[doc = "DFSDM filter 4 data register for injected group"]
pub mod dfsdm_flt4jdatar;
#[doc = "DFSDM_FLT4RDATAR register accessor: an alias for `Reg<DFSDM_FLT4RDATAR_SPEC>`"]
pub type DFSDM_FLT4RDATAR = crate::Reg<dfsdm_flt4rdatar::DFSDM_FLT4RDATAR_SPEC>;
#[doc = "DFSDM filter 4 data register for the regular channel"]
pub mod dfsdm_flt4rdatar;
#[doc = "DFSDM_FLT4AWHTR register accessor: an alias for `Reg<DFSDM_FLT4AWHTR_SPEC>`"]
pub type DFSDM_FLT4AWHTR = crate::Reg<dfsdm_flt4awhtr::DFSDM_FLT4AWHTR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog high threshold register"]
pub mod dfsdm_flt4awhtr;
#[doc = "DFSDM_FLT4AWLTR register accessor: an alias for `Reg<DFSDM_FLT4AWLTR_SPEC>`"]
pub type DFSDM_FLT4AWLTR = crate::Reg<dfsdm_flt4awltr::DFSDM_FLT4AWLTR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog low threshold register"]
pub mod dfsdm_flt4awltr;
#[doc = "DFSDM_FLT4AWSR register accessor: an alias for `Reg<DFSDM_FLT4AWSR_SPEC>`"]
pub type DFSDM_FLT4AWSR = crate::Reg<dfsdm_flt4awsr::DFSDM_FLT4AWSR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog status register"]
pub mod dfsdm_flt4awsr;
#[doc = "DFSDM_FLT4AWCFR register accessor: an alias for `Reg<DFSDM_FLT4AWCFR_SPEC>`"]
pub type DFSDM_FLT4AWCFR = crate::Reg<dfsdm_flt4awcfr::DFSDM_FLT4AWCFR_SPEC>;
#[doc = "DFSDM filter 4 analog watchdog clear flag register"]
pub mod dfsdm_flt4awcfr;
#[doc = "DFSDM_FLT4EXMAX register accessor: an alias for `Reg<DFSDM_FLT4EXMAX_SPEC>`"]
pub type DFSDM_FLT4EXMAX = crate::Reg<dfsdm_flt4exmax::DFSDM_FLT4EXMAX_SPEC>;
#[doc = "DFSDM filter 4 extremes detector maximum register"]
pub mod dfsdm_flt4exmax;
#[doc = "DFSDM_FLT4EXMIN register accessor: an alias for `Reg<DFSDM_FLT4EXMIN_SPEC>`"]
pub type DFSDM_FLT4EXMIN = crate::Reg<dfsdm_flt4exmin::DFSDM_FLT4EXMIN_SPEC>;
#[doc = "DFSDM filter 4 extremes detector minimum register"]
pub mod dfsdm_flt4exmin;
#[doc = "DFSDM_FLT4CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT4CNVTIMR_SPEC>`"]
pub type DFSDM_FLT4CNVTIMR = crate::Reg<dfsdm_flt4cnvtimr::DFSDM_FLT4CNVTIMR_SPEC>;
#[doc = "DFSDM filter 4 conversion timer register"]
pub mod dfsdm_flt4cnvtimr;
#[doc = "DFSDM_FLT5CR1 register accessor: an alias for `Reg<DFSDM_FLT5CR1_SPEC>`"]
pub type DFSDM_FLT5CR1 = crate::Reg<dfsdm_flt5cr1::DFSDM_FLT5CR1_SPEC>;
#[doc = "DFSDM filter 5 control register 1"]
pub mod dfsdm_flt5cr1;
#[doc = "DFSDM_FLT5CR2 register accessor: an alias for `Reg<DFSDM_FLT5CR2_SPEC>`"]
pub type DFSDM_FLT5CR2 = crate::Reg<dfsdm_flt5cr2::DFSDM_FLT5CR2_SPEC>;
#[doc = "DFSDM filter 5 control register 2"]
pub mod dfsdm_flt5cr2;
#[doc = "DFSDM_FLT5ISR register accessor: an alias for `Reg<DFSDM_FLT5ISR_SPEC>`"]
pub type DFSDM_FLT5ISR = crate::Reg<dfsdm_flt5isr::DFSDM_FLT5ISR_SPEC>;
#[doc = "DFSDM filter 5 interrupt and status register"]
pub mod dfsdm_flt5isr;
#[doc = "DFSDM_FLT5ICR register accessor: an alias for `Reg<DFSDM_FLT5ICR_SPEC>`"]
pub type DFSDM_FLT5ICR = crate::Reg<dfsdm_flt5icr::DFSDM_FLT5ICR_SPEC>;
#[doc = "DFSDM filter 5 interrupt flag clear register"]
pub mod dfsdm_flt5icr;
#[doc = "DFSDM_FLT5JCHGR register accessor: an alias for `Reg<DFSDM_FLT5JCHGR_SPEC>`"]
pub type DFSDM_FLT5JCHGR = crate::Reg<dfsdm_flt5jchgr::DFSDM_FLT5JCHGR_SPEC>;
#[doc = "DFSDM filter 5 injected channel group selection register"]
pub mod dfsdm_flt5jchgr;
#[doc = "DFSDM_FLT5FCR register accessor: an alias for `Reg<DFSDM_FLT5FCR_SPEC>`"]
pub type DFSDM_FLT5FCR = crate::Reg<dfsdm_flt5fcr::DFSDM_FLT5FCR_SPEC>;
#[doc = "DFSDM filter 5 control register"]
pub mod dfsdm_flt5fcr;
#[doc = "DFSDM_FLT5JDATAR register accessor: an alias for `Reg<DFSDM_FLT5JDATAR_SPEC>`"]
pub type DFSDM_FLT5JDATAR = crate::Reg<dfsdm_flt5jdatar::DFSDM_FLT5JDATAR_SPEC>;
#[doc = "DFSDM filter 5 data register for injected group"]
pub mod dfsdm_flt5jdatar;
#[doc = "DFSDM_FLT5RDATAR register accessor: an alias for `Reg<DFSDM_FLT5RDATAR_SPEC>`"]
pub type DFSDM_FLT5RDATAR = crate::Reg<dfsdm_flt5rdatar::DFSDM_FLT5RDATAR_SPEC>;
#[doc = "DFSDM filter 5 data register for the regular channel"]
pub mod dfsdm_flt5rdatar;
#[doc = "DFSDM_FLT5AWHTR register accessor: an alias for `Reg<DFSDM_FLT5AWHTR_SPEC>`"]
pub type DFSDM_FLT5AWHTR = crate::Reg<dfsdm_flt5awhtr::DFSDM_FLT5AWHTR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog high threshold register"]
pub mod dfsdm_flt5awhtr;
#[doc = "DFSDM_FLT5AWLTR register accessor: an alias for `Reg<DFSDM_FLT5AWLTR_SPEC>`"]
pub type DFSDM_FLT5AWLTR = crate::Reg<dfsdm_flt5awltr::DFSDM_FLT5AWLTR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog low threshold register"]
pub mod dfsdm_flt5awltr;
#[doc = "DFSDM_FLT5AWSR register accessor: an alias for `Reg<DFSDM_FLT5AWSR_SPEC>`"]
pub type DFSDM_FLT5AWSR = crate::Reg<dfsdm_flt5awsr::DFSDM_FLT5AWSR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog status register"]
pub mod dfsdm_flt5awsr;
#[doc = "DFSDM_FLT5AWCFR register accessor: an alias for `Reg<DFSDM_FLT5AWCFR_SPEC>`"]
pub type DFSDM_FLT5AWCFR = crate::Reg<dfsdm_flt5awcfr::DFSDM_FLT5AWCFR_SPEC>;
#[doc = "DFSDM filter 5 analog watchdog clear flag register"]
pub mod dfsdm_flt5awcfr;
#[doc = "DFSDM_FLT5EXMAX register accessor: an alias for `Reg<DFSDM_FLT5EXMAX_SPEC>`"]
pub type DFSDM_FLT5EXMAX = crate::Reg<dfsdm_flt5exmax::DFSDM_FLT5EXMAX_SPEC>;
#[doc = "DFSDM filter 5 extremes detector maximum register"]
pub mod dfsdm_flt5exmax;
#[doc = "DFSDM_FLT5EXMIN register accessor: an alias for `Reg<DFSDM_FLT5EXMIN_SPEC>`"]
pub type DFSDM_FLT5EXMIN = crate::Reg<dfsdm_flt5exmin::DFSDM_FLT5EXMIN_SPEC>;
#[doc = "DFSDM filter 5 extremes detector minimum register"]
pub mod dfsdm_flt5exmin;
#[doc = "DFSDM_FLT5CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT5CNVTIMR_SPEC>`"]
pub type DFSDM_FLT5CNVTIMR = crate::Reg<dfsdm_flt5cnvtimr::DFSDM_FLT5CNVTIMR_SPEC>;
#[doc = "DFSDM filter 5 conversion timer register"]
pub mod dfsdm_flt5cnvtimr;
#[doc = "DFSDM_HWCFGR register accessor: an alias for `Reg<DFSDM_HWCFGR_SPEC>`"]
pub type DFSDM_HWCFGR = crate::Reg<dfsdm_hwcfgr::DFSDM_HWCFGR_SPEC>;
#[doc = "This register specifies the hardware configuration of DFSDM peripheral."]
pub mod dfsdm_hwcfgr;
#[doc = "DFSDM_VERR register accessor: an alias for `Reg<DFSDM_VERR_SPEC>`"]
pub type DFSDM_VERR = crate::Reg<dfsdm_verr::DFSDM_VERR_SPEC>;
#[doc = "This register specifies the version of DFSDM peripheral."]
pub mod dfsdm_verr;
#[doc = "DFSDM_IPIDR register accessor: an alias for `Reg<DFSDM_IPIDR_SPEC>`"]
pub type DFSDM_IPIDR = crate::Reg<dfsdm_ipidr::DFSDM_IPIDR_SPEC>;
#[doc = "This register specifies the identification of DFSDM peripheral."]
pub mod dfsdm_ipidr;
#[doc = "DFSDM_SIDR register accessor: an alias for `Reg<DFSDM_SIDR_SPEC>`"]
pub type DFSDM_SIDR = crate::Reg<dfsdm_sidr::DFSDM_SIDR_SPEC>;
#[doc = "This register specifies the size allocated to DFSDM registers."]
pub mod dfsdm_sidr;
