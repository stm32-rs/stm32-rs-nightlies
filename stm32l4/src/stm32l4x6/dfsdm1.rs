#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    pub chcfg0r1: crate::Reg<chcfg0r1::CHCFG0R1_SPEC>,
    #[doc = "0x04 - channel configuration y register"]
    pub chcfg0r2: crate::Reg<chcfg0r2::CHCFG0R2_SPEC>,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub awscd0r: crate::Reg<awscd0r::AWSCD0R_SPEC>,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub chwdat0r: crate::Reg<chwdat0r::CHWDAT0R_SPEC>,
    #[doc = "0x10 - channel data input register"]
    pub chdatin0r: crate::Reg<chdatin0r::CHDATIN0R_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - CHCFG1R1"]
    pub chcfg1r1: crate::Reg<chcfg1r1::CHCFG1R1_SPEC>,
    #[doc = "0x24 - CHCFG1R2"]
    pub chcfg1r2: crate::Reg<chcfg1r2::CHCFG1R2_SPEC>,
    #[doc = "0x28 - AWSCD1R"]
    pub awscd1r: crate::Reg<awscd1r::AWSCD1R_SPEC>,
    #[doc = "0x2c - CHWDAT1R"]
    pub chwdat1r: crate::Reg<chwdat1r::CHWDAT1R_SPEC>,
    #[doc = "0x30 - CHDATIN1R"]
    pub chdatin1r: crate::Reg<chdatin1r::CHDATIN1R_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40 - CHCFG2R1"]
    pub chcfg2r1: crate::Reg<chcfg2r1::CHCFG2R1_SPEC>,
    #[doc = "0x44 - CHCFG2R2"]
    pub chcfg2r2: crate::Reg<chcfg2r2::CHCFG2R2_SPEC>,
    #[doc = "0x48 - AWSCD2R"]
    pub awscd2r: crate::Reg<awscd2r::AWSCD2R_SPEC>,
    #[doc = "0x4c - CHWDAT2R"]
    pub chwdat2r: crate::Reg<chwdat2r::CHWDAT2R_SPEC>,
    #[doc = "0x50 - CHDATIN2R"]
    pub chdatin2r: crate::Reg<chdatin2r::CHDATIN2R_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x60 - CHCFG3R1"]
    pub chcfg3r1: crate::Reg<chcfg3r1::CHCFG3R1_SPEC>,
    #[doc = "0x64 - CHCFG3R2"]
    pub chcfg3r2: crate::Reg<chcfg3r2::CHCFG3R2_SPEC>,
    #[doc = "0x68 - AWSCD3R"]
    pub awscd3r: crate::Reg<awscd3r::AWSCD3R_SPEC>,
    #[doc = "0x6c - CHWDAT3R"]
    pub chwdat3r: crate::Reg<chwdat3r::CHWDAT3R_SPEC>,
    #[doc = "0x70 - CHDATIN3R"]
    pub chdatin3r: crate::Reg<chdatin3r::CHDATIN3R_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x80 - CHCFG4R1"]
    pub chcfg4r1: crate::Reg<chcfg4r1::CHCFG4R1_SPEC>,
    #[doc = "0x84 - CHCFG4R2"]
    pub chcfg4r2: crate::Reg<chcfg4r2::CHCFG4R2_SPEC>,
    #[doc = "0x88 - AWSCD4R"]
    pub awscd4r: crate::Reg<awscd4r::AWSCD4R_SPEC>,
    #[doc = "0x8c - CHWDAT4R"]
    pub chwdat4r: crate::Reg<chwdat4r::CHWDAT4R_SPEC>,
    #[doc = "0x90 - CHDATIN4R"]
    pub chdatin4r: crate::Reg<chdatin4r::CHDATIN4R_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0xa0 - CHCFG5R1"]
    pub chcfg5r1: crate::Reg<chcfg5r1::CHCFG5R1_SPEC>,
    #[doc = "0xa4 - CHCFG5R2"]
    pub chcfg5r2: crate::Reg<chcfg5r2::CHCFG5R2_SPEC>,
    #[doc = "0xa8 - AWSCD5R"]
    pub awscd5r: crate::Reg<awscd5r::AWSCD5R_SPEC>,
    #[doc = "0xac - CHWDAT5R"]
    pub chwdat5r: crate::Reg<chwdat5r::CHWDAT5R_SPEC>,
    #[doc = "0xb0 - CHDATIN5R"]
    pub chdatin5r: crate::Reg<chdatin5r::CHDATIN5R_SPEC>,
    _reserved30: [u8; 0x0c],
    #[doc = "0xc0 - CHCFG6R1"]
    pub chcfg6r1: crate::Reg<chcfg6r1::CHCFG6R1_SPEC>,
    #[doc = "0xc4 - CHCFG6R2"]
    pub chcfg6r2: crate::Reg<chcfg6r2::CHCFG6R2_SPEC>,
    #[doc = "0xc8 - AWSCD6R"]
    pub awscd6r: crate::Reg<awscd6r::AWSCD6R_SPEC>,
    #[doc = "0xcc - CHWDAT6R"]
    pub chwdat6r: crate::Reg<chwdat6r::CHWDAT6R_SPEC>,
    #[doc = "0xd0 - CHDATIN6R"]
    pub chdatin6r: crate::Reg<chdatin6r::CHDATIN6R_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0xe0 - CHCFG7R1"]
    pub chcfg7r1: crate::Reg<chcfg7r1::CHCFG7R1_SPEC>,
    #[doc = "0xe4 - CHCFG7R2"]
    pub chcfg7r2: crate::Reg<chcfg7r2::CHCFG7R2_SPEC>,
    #[doc = "0xe8 - AWSCD7R"]
    pub awscd7r: crate::Reg<awscd7r::AWSCD7R_SPEC>,
    #[doc = "0xec - CHWDAT7R"]
    pub chwdat7r: crate::Reg<chwdat7r::CHWDAT7R_SPEC>,
    #[doc = "0xf0 - CHDATIN7R"]
    pub chdatin7r: crate::Reg<chdatin7r::CHDATIN7R_SPEC>,
    _reserved40: [u8; 0x0c],
    #[doc = "0x100 - control register 1"]
    pub dfsdm0_cr1: crate::Reg<dfsdm0_cr1::DFSDM0_CR1_SPEC>,
    #[doc = "0x104 - control register 2"]
    pub dfsdm0_cr2: crate::Reg<dfsdm0_cr2::DFSDM0_CR2_SPEC>,
    #[doc = "0x108 - interrupt and status register"]
    pub dfsdm0_isr: crate::Reg<dfsdm0_isr::DFSDM0_ISR_SPEC>,
    #[doc = "0x10c - interrupt flag clear register"]
    pub dfsdm0_icr: crate::Reg<dfsdm0_icr::DFSDM0_ICR_SPEC>,
    #[doc = "0x110 - injected channel group selection register"]
    pub dfsdm0_jchgr: crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGR_SPEC>,
    #[doc = "0x114 - filter control register"]
    pub dfsdm0_fcr: crate::Reg<dfsdm0_fcr::DFSDM0_FCR_SPEC>,
    #[doc = "0x118 - data register for injected group"]
    pub dfsdm0_jdatar: crate::Reg<dfsdm0_jdatar::DFSDM0_JDATAR_SPEC>,
    #[doc = "0x11c - data register for the regular channel"]
    pub dfsdm0_rdatar: crate::Reg<dfsdm0_rdatar::DFSDM0_RDATAR_SPEC>,
    #[doc = "0x120 - analog watchdog high threshold register"]
    pub dfsdm0_awhtr: crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTR_SPEC>,
    #[doc = "0x124 - analog watchdog low threshold register"]
    pub dfsdm0_awltr: crate::Reg<dfsdm0_awltr::DFSDM0_AWLTR_SPEC>,
    #[doc = "0x128 - analog watchdog status register"]
    pub dfsdm0_awsr: crate::Reg<dfsdm0_awsr::DFSDM0_AWSR_SPEC>,
    #[doc = "0x12c - analog watchdog clear flag register"]
    pub dfsdm0_awcfr: crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFR_SPEC>,
    #[doc = "0x130 - Extremes detector maximum register"]
    pub dfsdm0_exmax: crate::Reg<dfsdm0_exmax::DFSDM0_EXMAX_SPEC>,
    #[doc = "0x134 - Extremes detector minimum register"]
    pub dfsdm0_exmin: crate::Reg<dfsdm0_exmin::DFSDM0_EXMIN_SPEC>,
    #[doc = "0x138 - conversion timer register"]
    pub dfsdm0_cnvtimr: crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMR_SPEC>,
    _reserved55: [u8; 0xc4],
    #[doc = "0x200 - control register 1"]
    pub dfsdm1_cr1: crate::Reg<dfsdm1_cr1::DFSDM1_CR1_SPEC>,
    #[doc = "0x204 - control register 2"]
    pub dfsdm1_cr2: crate::Reg<dfsdm1_cr2::DFSDM1_CR2_SPEC>,
    #[doc = "0x208 - interrupt and status register"]
    pub dfsdm1_isr: crate::Reg<dfsdm1_isr::DFSDM1_ISR_SPEC>,
    #[doc = "0x20c - interrupt flag clear register"]
    pub dfsdm1_icr: crate::Reg<dfsdm1_icr::DFSDM1_ICR_SPEC>,
    #[doc = "0x210 - injected channel group selection register"]
    pub dfsdm1_jchgr: crate::Reg<dfsdm1_jchgr::DFSDM1_JCHGR_SPEC>,
    #[doc = "0x214 - filter control register"]
    pub dfsdm1_fcr: crate::Reg<dfsdm1_fcr::DFSDM1_FCR_SPEC>,
    #[doc = "0x218 - data register for injected group"]
    pub dfsdm1_jdatar: crate::Reg<dfsdm1_jdatar::DFSDM1_JDATAR_SPEC>,
    #[doc = "0x21c - data register for the regular channel"]
    pub dfsdm1_rdatar: crate::Reg<dfsdm1_rdatar::DFSDM1_RDATAR_SPEC>,
    #[doc = "0x220 - analog watchdog high threshold register"]
    pub dfsdm1_awhtr: crate::Reg<dfsdm1_awhtr::DFSDM1_AWHTR_SPEC>,
    #[doc = "0x224 - analog watchdog low threshold register"]
    pub dfsdm1_awltr: crate::Reg<dfsdm1_awltr::DFSDM1_AWLTR_SPEC>,
    #[doc = "0x228 - analog watchdog status register"]
    pub dfsdm1_awsr: crate::Reg<dfsdm1_awsr::DFSDM1_AWSR_SPEC>,
    #[doc = "0x22c - analog watchdog clear flag register"]
    pub dfsdm1_awcfr: crate::Reg<dfsdm1_awcfr::DFSDM1_AWCFR_SPEC>,
    #[doc = "0x230 - Extremes detector maximum register"]
    pub dfsdm1_exmax: crate::Reg<dfsdm1_exmax::DFSDM1_EXMAX_SPEC>,
    #[doc = "0x234 - Extremes detector minimum register"]
    pub dfsdm1_exmin: crate::Reg<dfsdm1_exmin::DFSDM1_EXMIN_SPEC>,
    #[doc = "0x238 - conversion timer register"]
    pub dfsdm1_cnvtimr: crate::Reg<dfsdm1_cnvtimr::DFSDM1_CNVTIMR_SPEC>,
    _reserved70: [u8; 0xc4],
    #[doc = "0x300 - control register 1"]
    pub dfsdm2_cr1: crate::Reg<dfsdm2_cr1::DFSDM2_CR1_SPEC>,
    #[doc = "0x304 - control register 2"]
    pub dfsdm2_cr2: crate::Reg<dfsdm2_cr2::DFSDM2_CR2_SPEC>,
    #[doc = "0x308 - interrupt and status register"]
    pub dfsdm2_isr: crate::Reg<dfsdm2_isr::DFSDM2_ISR_SPEC>,
    #[doc = "0x30c - interrupt flag clear register"]
    pub dfsdm2_icr: crate::Reg<dfsdm2_icr::DFSDM2_ICR_SPEC>,
    #[doc = "0x310 - injected channel group selection register"]
    pub dfsdm2_jchgr: crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGR_SPEC>,
    #[doc = "0x314 - filter control register"]
    pub dfsdm2_fcr: crate::Reg<dfsdm2_fcr::DFSDM2_FCR_SPEC>,
    #[doc = "0x318 - data register for injected group"]
    pub dfsdm2_jdatar: crate::Reg<dfsdm2_jdatar::DFSDM2_JDATAR_SPEC>,
    #[doc = "0x31c - data register for the regular channel"]
    pub dfsdm2_rdatar: crate::Reg<dfsdm2_rdatar::DFSDM2_RDATAR_SPEC>,
    #[doc = "0x320 - analog watchdog high threshold register"]
    pub dfsdm2_awhtr: crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTR_SPEC>,
    #[doc = "0x324 - analog watchdog low threshold register"]
    pub dfsdm2_awltr: crate::Reg<dfsdm2_awltr::DFSDM2_AWLTR_SPEC>,
    #[doc = "0x328 - analog watchdog status register"]
    pub dfsdm2_awsr: crate::Reg<dfsdm2_awsr::DFSDM2_AWSR_SPEC>,
    #[doc = "0x32c - analog watchdog clear flag register"]
    pub dfsdm2_awcfr: crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFR_SPEC>,
    #[doc = "0x330 - Extremes detector maximum register"]
    pub dfsdm2_exmax: crate::Reg<dfsdm2_exmax::DFSDM2_EXMAX_SPEC>,
    #[doc = "0x334 - Extremes detector minimum register"]
    pub dfsdm2_exmin: crate::Reg<dfsdm2_exmin::DFSDM2_EXMIN_SPEC>,
    #[doc = "0x338 - conversion timer register"]
    pub dfsdm2_cnvtimr: crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMR_SPEC>,
    _reserved85: [u8; 0xc4],
    #[doc = "0x400 - control register 1"]
    pub dfsdm3_cr1: crate::Reg<dfsdm3_cr1::DFSDM3_CR1_SPEC>,
    #[doc = "0x404 - control register 2"]
    pub dfsdm3_cr2: crate::Reg<dfsdm3_cr2::DFSDM3_CR2_SPEC>,
    #[doc = "0x408 - interrupt and status register"]
    pub dfsdm3_isr: crate::Reg<dfsdm3_isr::DFSDM3_ISR_SPEC>,
    #[doc = "0x40c - interrupt flag clear register"]
    pub dfsdm3_icr: crate::Reg<dfsdm3_icr::DFSDM3_ICR_SPEC>,
    #[doc = "0x410 - injected channel group selection register"]
    pub dfsdm3_jchgr: crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGR_SPEC>,
    #[doc = "0x414 - filter control register"]
    pub dfsdm3_fcr: crate::Reg<dfsdm3_fcr::DFSDM3_FCR_SPEC>,
    #[doc = "0x418 - data register for injected group"]
    pub dfsdm3_jdatar: crate::Reg<dfsdm3_jdatar::DFSDM3_JDATAR_SPEC>,
    #[doc = "0x41c - data register for the regular channel"]
    pub dfsdm3_rdatar: crate::Reg<dfsdm3_rdatar::DFSDM3_RDATAR_SPEC>,
    #[doc = "0x420 - analog watchdog high threshold register"]
    pub dfsdm3_awhtr: crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTR_SPEC>,
    #[doc = "0x424 - analog watchdog low threshold register"]
    pub dfsdm3_awltr: crate::Reg<dfsdm3_awltr::DFSDM3_AWLTR_SPEC>,
    #[doc = "0x428 - analog watchdog status register"]
    pub dfsdm3_awsr: crate::Reg<dfsdm3_awsr::DFSDM3_AWSR_SPEC>,
    #[doc = "0x42c - analog watchdog clear flag register"]
    pub dfsdm3_awcfr: crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFR_SPEC>,
    #[doc = "0x430 - Extremes detector maximum register"]
    pub dfsdm3_exmax: crate::Reg<dfsdm3_exmax::DFSDM3_EXMAX_SPEC>,
    #[doc = "0x434 - Extremes detector minimum register"]
    pub dfsdm3_exmin: crate::Reg<dfsdm3_exmin::DFSDM3_EXMIN_SPEC>,
    #[doc = "0x438 - conversion timer register"]
    pub dfsdm3_cnvtimr: crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMR_SPEC>,
}
#[doc = "CHCFG0R1 register accessor: an alias for `Reg<CHCFG0R1_SPEC>`"]
pub type CHCFG0R1 = crate::Reg<chcfg0r1::CHCFG0R1_SPEC>;
#[doc = "channel configuration y register"]
pub mod chcfg0r1;
#[doc = "CHCFG0R2 register accessor: an alias for `Reg<CHCFG0R2_SPEC>`"]
pub type CHCFG0R2 = crate::Reg<chcfg0r2::CHCFG0R2_SPEC>;
#[doc = "channel configuration y register"]
pub mod chcfg0r2;
#[doc = "AWSCD0R register accessor: an alias for `Reg<AWSCD0R_SPEC>`"]
pub type AWSCD0R = crate::Reg<awscd0r::AWSCD0R_SPEC>;
#[doc = "analog watchdog and short-circuit detector register"]
pub mod awscd0r;
#[doc = "CHWDAT0R register accessor: an alias for `Reg<CHWDAT0R_SPEC>`"]
pub type CHWDAT0R = crate::Reg<chwdat0r::CHWDAT0R_SPEC>;
#[doc = "channel watchdog filter data register"]
pub mod chwdat0r;
#[doc = "CHDATIN0R register accessor: an alias for `Reg<CHDATIN0R_SPEC>`"]
pub type CHDATIN0R = crate::Reg<chdatin0r::CHDATIN0R_SPEC>;
#[doc = "channel data input register"]
pub mod chdatin0r;
#[doc = "CHCFG1R1 register accessor: an alias for `Reg<CHCFG1R1_SPEC>`"]
pub type CHCFG1R1 = crate::Reg<chcfg1r1::CHCFG1R1_SPEC>;
#[doc = "CHCFG1R1"]
pub mod chcfg1r1;
#[doc = "CHCFG1R2 register accessor: an alias for `Reg<CHCFG1R2_SPEC>`"]
pub type CHCFG1R2 = crate::Reg<chcfg1r2::CHCFG1R2_SPEC>;
#[doc = "CHCFG1R2"]
pub mod chcfg1r2;
#[doc = "AWSCD1R register accessor: an alias for `Reg<AWSCD1R_SPEC>`"]
pub type AWSCD1R = crate::Reg<awscd1r::AWSCD1R_SPEC>;
#[doc = "AWSCD1R"]
pub mod awscd1r;
#[doc = "CHWDAT1R register accessor: an alias for `Reg<CHWDAT1R_SPEC>`"]
pub type CHWDAT1R = crate::Reg<chwdat1r::CHWDAT1R_SPEC>;
#[doc = "CHWDAT1R"]
pub mod chwdat1r;
#[doc = "CHDATIN1R register accessor: an alias for `Reg<CHDATIN1R_SPEC>`"]
pub type CHDATIN1R = crate::Reg<chdatin1r::CHDATIN1R_SPEC>;
#[doc = "CHDATIN1R"]
pub mod chdatin1r;
#[doc = "CHCFG2R1 register accessor: an alias for `Reg<CHCFG2R1_SPEC>`"]
pub type CHCFG2R1 = crate::Reg<chcfg2r1::CHCFG2R1_SPEC>;
#[doc = "CHCFG2R1"]
pub mod chcfg2r1;
#[doc = "CHCFG2R2 register accessor: an alias for `Reg<CHCFG2R2_SPEC>`"]
pub type CHCFG2R2 = crate::Reg<chcfg2r2::CHCFG2R2_SPEC>;
#[doc = "CHCFG2R2"]
pub mod chcfg2r2;
#[doc = "AWSCD2R register accessor: an alias for `Reg<AWSCD2R_SPEC>`"]
pub type AWSCD2R = crate::Reg<awscd2r::AWSCD2R_SPEC>;
#[doc = "AWSCD2R"]
pub mod awscd2r;
#[doc = "CHWDAT2R register accessor: an alias for `Reg<CHWDAT2R_SPEC>`"]
pub type CHWDAT2R = crate::Reg<chwdat2r::CHWDAT2R_SPEC>;
#[doc = "CHWDAT2R"]
pub mod chwdat2r;
#[doc = "CHDATIN2R register accessor: an alias for `Reg<CHDATIN2R_SPEC>`"]
pub type CHDATIN2R = crate::Reg<chdatin2r::CHDATIN2R_SPEC>;
#[doc = "CHDATIN2R"]
pub mod chdatin2r;
#[doc = "CHCFG3R1 register accessor: an alias for `Reg<CHCFG3R1_SPEC>`"]
pub type CHCFG3R1 = crate::Reg<chcfg3r1::CHCFG3R1_SPEC>;
#[doc = "CHCFG3R1"]
pub mod chcfg3r1;
#[doc = "CHCFG3R2 register accessor: an alias for `Reg<CHCFG3R2_SPEC>`"]
pub type CHCFG3R2 = crate::Reg<chcfg3r2::CHCFG3R2_SPEC>;
#[doc = "CHCFG3R2"]
pub mod chcfg3r2;
#[doc = "AWSCD3R register accessor: an alias for `Reg<AWSCD3R_SPEC>`"]
pub type AWSCD3R = crate::Reg<awscd3r::AWSCD3R_SPEC>;
#[doc = "AWSCD3R"]
pub mod awscd3r;
#[doc = "CHWDAT3R register accessor: an alias for `Reg<CHWDAT3R_SPEC>`"]
pub type CHWDAT3R = crate::Reg<chwdat3r::CHWDAT3R_SPEC>;
#[doc = "CHWDAT3R"]
pub mod chwdat3r;
#[doc = "CHDATIN3R register accessor: an alias for `Reg<CHDATIN3R_SPEC>`"]
pub type CHDATIN3R = crate::Reg<chdatin3r::CHDATIN3R_SPEC>;
#[doc = "CHDATIN3R"]
pub mod chdatin3r;
#[doc = "CHCFG4R1 register accessor: an alias for `Reg<CHCFG4R1_SPEC>`"]
pub type CHCFG4R1 = crate::Reg<chcfg4r1::CHCFG4R1_SPEC>;
#[doc = "CHCFG4R1"]
pub mod chcfg4r1;
#[doc = "CHCFG4R2 register accessor: an alias for `Reg<CHCFG4R2_SPEC>`"]
pub type CHCFG4R2 = crate::Reg<chcfg4r2::CHCFG4R2_SPEC>;
#[doc = "CHCFG4R2"]
pub mod chcfg4r2;
#[doc = "AWSCD4R register accessor: an alias for `Reg<AWSCD4R_SPEC>`"]
pub type AWSCD4R = crate::Reg<awscd4r::AWSCD4R_SPEC>;
#[doc = "AWSCD4R"]
pub mod awscd4r;
#[doc = "CHWDAT4R register accessor: an alias for `Reg<CHWDAT4R_SPEC>`"]
pub type CHWDAT4R = crate::Reg<chwdat4r::CHWDAT4R_SPEC>;
#[doc = "CHWDAT4R"]
pub mod chwdat4r;
#[doc = "CHDATIN4R register accessor: an alias for `Reg<CHDATIN4R_SPEC>`"]
pub type CHDATIN4R = crate::Reg<chdatin4r::CHDATIN4R_SPEC>;
#[doc = "CHDATIN4R"]
pub mod chdatin4r;
#[doc = "CHCFG5R1 register accessor: an alias for `Reg<CHCFG5R1_SPEC>`"]
pub type CHCFG5R1 = crate::Reg<chcfg5r1::CHCFG5R1_SPEC>;
#[doc = "CHCFG5R1"]
pub mod chcfg5r1;
#[doc = "CHCFG5R2 register accessor: an alias for `Reg<CHCFG5R2_SPEC>`"]
pub type CHCFG5R2 = crate::Reg<chcfg5r2::CHCFG5R2_SPEC>;
#[doc = "CHCFG5R2"]
pub mod chcfg5r2;
#[doc = "AWSCD5R register accessor: an alias for `Reg<AWSCD5R_SPEC>`"]
pub type AWSCD5R = crate::Reg<awscd5r::AWSCD5R_SPEC>;
#[doc = "AWSCD5R"]
pub mod awscd5r;
#[doc = "CHWDAT5R register accessor: an alias for `Reg<CHWDAT5R_SPEC>`"]
pub type CHWDAT5R = crate::Reg<chwdat5r::CHWDAT5R_SPEC>;
#[doc = "CHWDAT5R"]
pub mod chwdat5r;
#[doc = "CHDATIN5R register accessor: an alias for `Reg<CHDATIN5R_SPEC>`"]
pub type CHDATIN5R = crate::Reg<chdatin5r::CHDATIN5R_SPEC>;
#[doc = "CHDATIN5R"]
pub mod chdatin5r;
#[doc = "CHCFG6R1 register accessor: an alias for `Reg<CHCFG6R1_SPEC>`"]
pub type CHCFG6R1 = crate::Reg<chcfg6r1::CHCFG6R1_SPEC>;
#[doc = "CHCFG6R1"]
pub mod chcfg6r1;
#[doc = "CHCFG6R2 register accessor: an alias for `Reg<CHCFG6R2_SPEC>`"]
pub type CHCFG6R2 = crate::Reg<chcfg6r2::CHCFG6R2_SPEC>;
#[doc = "CHCFG6R2"]
pub mod chcfg6r2;
#[doc = "AWSCD6R register accessor: an alias for `Reg<AWSCD6R_SPEC>`"]
pub type AWSCD6R = crate::Reg<awscd6r::AWSCD6R_SPEC>;
#[doc = "AWSCD6R"]
pub mod awscd6r;
#[doc = "CHWDAT6R register accessor: an alias for `Reg<CHWDAT6R_SPEC>`"]
pub type CHWDAT6R = crate::Reg<chwdat6r::CHWDAT6R_SPEC>;
#[doc = "CHWDAT6R"]
pub mod chwdat6r;
#[doc = "CHDATIN6R register accessor: an alias for `Reg<CHDATIN6R_SPEC>`"]
pub type CHDATIN6R = crate::Reg<chdatin6r::CHDATIN6R_SPEC>;
#[doc = "CHDATIN6R"]
pub mod chdatin6r;
#[doc = "CHCFG7R1 register accessor: an alias for `Reg<CHCFG7R1_SPEC>`"]
pub type CHCFG7R1 = crate::Reg<chcfg7r1::CHCFG7R1_SPEC>;
#[doc = "CHCFG7R1"]
pub mod chcfg7r1;
#[doc = "CHCFG7R2 register accessor: an alias for `Reg<CHCFG7R2_SPEC>`"]
pub type CHCFG7R2 = crate::Reg<chcfg7r2::CHCFG7R2_SPEC>;
#[doc = "CHCFG7R2"]
pub mod chcfg7r2;
#[doc = "AWSCD7R register accessor: an alias for `Reg<AWSCD7R_SPEC>`"]
pub type AWSCD7R = crate::Reg<awscd7r::AWSCD7R_SPEC>;
#[doc = "AWSCD7R"]
pub mod awscd7r;
#[doc = "CHWDAT7R register accessor: an alias for `Reg<CHWDAT7R_SPEC>`"]
pub type CHWDAT7R = crate::Reg<chwdat7r::CHWDAT7R_SPEC>;
#[doc = "CHWDAT7R"]
pub mod chwdat7r;
#[doc = "CHDATIN7R register accessor: an alias for `Reg<CHDATIN7R_SPEC>`"]
pub type CHDATIN7R = crate::Reg<chdatin7r::CHDATIN7R_SPEC>;
#[doc = "CHDATIN7R"]
pub mod chdatin7r;
#[doc = "DFSDM0_CR1 register accessor: an alias for `Reg<DFSDM0_CR1_SPEC>`"]
pub type DFSDM0_CR1 = crate::Reg<dfsdm0_cr1::DFSDM0_CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm0_cr1;
#[doc = "DFSDM0_CR2 register accessor: an alias for `Reg<DFSDM0_CR2_SPEC>`"]
pub type DFSDM0_CR2 = crate::Reg<dfsdm0_cr2::DFSDM0_CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm0_cr2;
#[doc = "DFSDM0_ISR register accessor: an alias for `Reg<DFSDM0_ISR_SPEC>`"]
pub type DFSDM0_ISR = crate::Reg<dfsdm0_isr::DFSDM0_ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm0_isr;
#[doc = "DFSDM0_ICR register accessor: an alias for `Reg<DFSDM0_ICR_SPEC>`"]
pub type DFSDM0_ICR = crate::Reg<dfsdm0_icr::DFSDM0_ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm0_icr;
#[doc = "DFSDM0_JCHGR register accessor: an alias for `Reg<DFSDM0_JCHGR_SPEC>`"]
pub type DFSDM0_JCHGR = crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm0_jchgr;
#[doc = "DFSDM0_FCR register accessor: an alias for `Reg<DFSDM0_FCR_SPEC>`"]
pub type DFSDM0_FCR = crate::Reg<dfsdm0_fcr::DFSDM0_FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm0_fcr;
#[doc = "DFSDM0_JDATAR register accessor: an alias for `Reg<DFSDM0_JDATAR_SPEC>`"]
pub type DFSDM0_JDATAR = crate::Reg<dfsdm0_jdatar::DFSDM0_JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm0_jdatar;
#[doc = "DFSDM0_RDATAR register accessor: an alias for `Reg<DFSDM0_RDATAR_SPEC>`"]
pub type DFSDM0_RDATAR = crate::Reg<dfsdm0_rdatar::DFSDM0_RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm0_rdatar;
#[doc = "DFSDM0_AWHTR register accessor: an alias for `Reg<DFSDM0_AWHTR_SPEC>`"]
pub type DFSDM0_AWHTR = crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm0_awhtr;
#[doc = "DFSDM0_AWLTR register accessor: an alias for `Reg<DFSDM0_AWLTR_SPEC>`"]
pub type DFSDM0_AWLTR = crate::Reg<dfsdm0_awltr::DFSDM0_AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm0_awltr;
#[doc = "DFSDM0_AWSR register accessor: an alias for `Reg<DFSDM0_AWSR_SPEC>`"]
pub type DFSDM0_AWSR = crate::Reg<dfsdm0_awsr::DFSDM0_AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm0_awsr;
#[doc = "DFSDM0_AWCFR register accessor: an alias for `Reg<DFSDM0_AWCFR_SPEC>`"]
pub type DFSDM0_AWCFR = crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm0_awcfr;
#[doc = "DFSDM0_EXMAX register accessor: an alias for `Reg<DFSDM0_EXMAX_SPEC>`"]
pub type DFSDM0_EXMAX = crate::Reg<dfsdm0_exmax::DFSDM0_EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm0_exmax;
#[doc = "DFSDM0_EXMIN register accessor: an alias for `Reg<DFSDM0_EXMIN_SPEC>`"]
pub type DFSDM0_EXMIN = crate::Reg<dfsdm0_exmin::DFSDM0_EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm0_exmin;
#[doc = "DFSDM0_CNVTIMR register accessor: an alias for `Reg<DFSDM0_CNVTIMR_SPEC>`"]
pub type DFSDM0_CNVTIMR = crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm0_cnvtimr;
#[doc = "DFSDM1_CR1 register accessor: an alias for `Reg<DFSDM1_CR1_SPEC>`"]
pub type DFSDM1_CR1 = crate::Reg<dfsdm1_cr1::DFSDM1_CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm1_cr1;
#[doc = "DFSDM1_CR2 register accessor: an alias for `Reg<DFSDM1_CR2_SPEC>`"]
pub type DFSDM1_CR2 = crate::Reg<dfsdm1_cr2::DFSDM1_CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm1_cr2;
#[doc = "DFSDM1_ISR register accessor: an alias for `Reg<DFSDM1_ISR_SPEC>`"]
pub type DFSDM1_ISR = crate::Reg<dfsdm1_isr::DFSDM1_ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm1_isr;
#[doc = "DFSDM1_ICR register accessor: an alias for `Reg<DFSDM1_ICR_SPEC>`"]
pub type DFSDM1_ICR = crate::Reg<dfsdm1_icr::DFSDM1_ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm1_icr;
#[doc = "DFSDM1_JCHGR register accessor: an alias for `Reg<DFSDM1_JCHGR_SPEC>`"]
pub type DFSDM1_JCHGR = crate::Reg<dfsdm1_jchgr::DFSDM1_JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm1_jchgr;
#[doc = "DFSDM1_FCR register accessor: an alias for `Reg<DFSDM1_FCR_SPEC>`"]
pub type DFSDM1_FCR = crate::Reg<dfsdm1_fcr::DFSDM1_FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm1_fcr;
#[doc = "DFSDM1_JDATAR register accessor: an alias for `Reg<DFSDM1_JDATAR_SPEC>`"]
pub type DFSDM1_JDATAR = crate::Reg<dfsdm1_jdatar::DFSDM1_JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm1_jdatar;
#[doc = "DFSDM1_RDATAR register accessor: an alias for `Reg<DFSDM1_RDATAR_SPEC>`"]
pub type DFSDM1_RDATAR = crate::Reg<dfsdm1_rdatar::DFSDM1_RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm1_rdatar;
#[doc = "DFSDM1_AWHTR register accessor: an alias for `Reg<DFSDM1_AWHTR_SPEC>`"]
pub type DFSDM1_AWHTR = crate::Reg<dfsdm1_awhtr::DFSDM1_AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm1_awhtr;
#[doc = "DFSDM1_AWLTR register accessor: an alias for `Reg<DFSDM1_AWLTR_SPEC>`"]
pub type DFSDM1_AWLTR = crate::Reg<dfsdm1_awltr::DFSDM1_AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm1_awltr;
#[doc = "DFSDM1_AWSR register accessor: an alias for `Reg<DFSDM1_AWSR_SPEC>`"]
pub type DFSDM1_AWSR = crate::Reg<dfsdm1_awsr::DFSDM1_AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm1_awsr;
#[doc = "DFSDM1_AWCFR register accessor: an alias for `Reg<DFSDM1_AWCFR_SPEC>`"]
pub type DFSDM1_AWCFR = crate::Reg<dfsdm1_awcfr::DFSDM1_AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm1_awcfr;
#[doc = "DFSDM1_EXMAX register accessor: an alias for `Reg<DFSDM1_EXMAX_SPEC>`"]
pub type DFSDM1_EXMAX = crate::Reg<dfsdm1_exmax::DFSDM1_EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm1_exmax;
#[doc = "DFSDM1_EXMIN register accessor: an alias for `Reg<DFSDM1_EXMIN_SPEC>`"]
pub type DFSDM1_EXMIN = crate::Reg<dfsdm1_exmin::DFSDM1_EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm1_exmin;
#[doc = "DFSDM1_CNVTIMR register accessor: an alias for `Reg<DFSDM1_CNVTIMR_SPEC>`"]
pub type DFSDM1_CNVTIMR = crate::Reg<dfsdm1_cnvtimr::DFSDM1_CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm1_cnvtimr;
#[doc = "DFSDM2_CR1 register accessor: an alias for `Reg<DFSDM2_CR1_SPEC>`"]
pub type DFSDM2_CR1 = crate::Reg<dfsdm2_cr1::DFSDM2_CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm2_cr1;
#[doc = "DFSDM2_CR2 register accessor: an alias for `Reg<DFSDM2_CR2_SPEC>`"]
pub type DFSDM2_CR2 = crate::Reg<dfsdm2_cr2::DFSDM2_CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm2_cr2;
#[doc = "DFSDM2_ISR register accessor: an alias for `Reg<DFSDM2_ISR_SPEC>`"]
pub type DFSDM2_ISR = crate::Reg<dfsdm2_isr::DFSDM2_ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm2_isr;
#[doc = "DFSDM2_ICR register accessor: an alias for `Reg<DFSDM2_ICR_SPEC>`"]
pub type DFSDM2_ICR = crate::Reg<dfsdm2_icr::DFSDM2_ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm2_icr;
#[doc = "DFSDM2_JCHGR register accessor: an alias for `Reg<DFSDM2_JCHGR_SPEC>`"]
pub type DFSDM2_JCHGR = crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm2_jchgr;
#[doc = "DFSDM2_FCR register accessor: an alias for `Reg<DFSDM2_FCR_SPEC>`"]
pub type DFSDM2_FCR = crate::Reg<dfsdm2_fcr::DFSDM2_FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm2_fcr;
#[doc = "DFSDM2_JDATAR register accessor: an alias for `Reg<DFSDM2_JDATAR_SPEC>`"]
pub type DFSDM2_JDATAR = crate::Reg<dfsdm2_jdatar::DFSDM2_JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm2_jdatar;
#[doc = "DFSDM2_RDATAR register accessor: an alias for `Reg<DFSDM2_RDATAR_SPEC>`"]
pub type DFSDM2_RDATAR = crate::Reg<dfsdm2_rdatar::DFSDM2_RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm2_rdatar;
#[doc = "DFSDM2_AWHTR register accessor: an alias for `Reg<DFSDM2_AWHTR_SPEC>`"]
pub type DFSDM2_AWHTR = crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm2_awhtr;
#[doc = "DFSDM2_AWLTR register accessor: an alias for `Reg<DFSDM2_AWLTR_SPEC>`"]
pub type DFSDM2_AWLTR = crate::Reg<dfsdm2_awltr::DFSDM2_AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm2_awltr;
#[doc = "DFSDM2_AWSR register accessor: an alias for `Reg<DFSDM2_AWSR_SPEC>`"]
pub type DFSDM2_AWSR = crate::Reg<dfsdm2_awsr::DFSDM2_AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm2_awsr;
#[doc = "DFSDM2_AWCFR register accessor: an alias for `Reg<DFSDM2_AWCFR_SPEC>`"]
pub type DFSDM2_AWCFR = crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm2_awcfr;
#[doc = "DFSDM2_EXMAX register accessor: an alias for `Reg<DFSDM2_EXMAX_SPEC>`"]
pub type DFSDM2_EXMAX = crate::Reg<dfsdm2_exmax::DFSDM2_EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm2_exmax;
#[doc = "DFSDM2_EXMIN register accessor: an alias for `Reg<DFSDM2_EXMIN_SPEC>`"]
pub type DFSDM2_EXMIN = crate::Reg<dfsdm2_exmin::DFSDM2_EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm2_exmin;
#[doc = "DFSDM2_CNVTIMR register accessor: an alias for `Reg<DFSDM2_CNVTIMR_SPEC>`"]
pub type DFSDM2_CNVTIMR = crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm2_cnvtimr;
#[doc = "DFSDM3_CR1 register accessor: an alias for `Reg<DFSDM3_CR1_SPEC>`"]
pub type DFSDM3_CR1 = crate::Reg<dfsdm3_cr1::DFSDM3_CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm3_cr1;
#[doc = "DFSDM3_CR2 register accessor: an alias for `Reg<DFSDM3_CR2_SPEC>`"]
pub type DFSDM3_CR2 = crate::Reg<dfsdm3_cr2::DFSDM3_CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm3_cr2;
#[doc = "DFSDM3_ISR register accessor: an alias for `Reg<DFSDM3_ISR_SPEC>`"]
pub type DFSDM3_ISR = crate::Reg<dfsdm3_isr::DFSDM3_ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm3_isr;
#[doc = "DFSDM3_ICR register accessor: an alias for `Reg<DFSDM3_ICR_SPEC>`"]
pub type DFSDM3_ICR = crate::Reg<dfsdm3_icr::DFSDM3_ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm3_icr;
#[doc = "DFSDM3_JCHGR register accessor: an alias for `Reg<DFSDM3_JCHGR_SPEC>`"]
pub type DFSDM3_JCHGR = crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm3_jchgr;
#[doc = "DFSDM3_FCR register accessor: an alias for `Reg<DFSDM3_FCR_SPEC>`"]
pub type DFSDM3_FCR = crate::Reg<dfsdm3_fcr::DFSDM3_FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm3_fcr;
#[doc = "DFSDM3_JDATAR register accessor: an alias for `Reg<DFSDM3_JDATAR_SPEC>`"]
pub type DFSDM3_JDATAR = crate::Reg<dfsdm3_jdatar::DFSDM3_JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm3_jdatar;
#[doc = "DFSDM3_RDATAR register accessor: an alias for `Reg<DFSDM3_RDATAR_SPEC>`"]
pub type DFSDM3_RDATAR = crate::Reg<dfsdm3_rdatar::DFSDM3_RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm3_rdatar;
#[doc = "DFSDM3_AWHTR register accessor: an alias for `Reg<DFSDM3_AWHTR_SPEC>`"]
pub type DFSDM3_AWHTR = crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm3_awhtr;
#[doc = "DFSDM3_AWLTR register accessor: an alias for `Reg<DFSDM3_AWLTR_SPEC>`"]
pub type DFSDM3_AWLTR = crate::Reg<dfsdm3_awltr::DFSDM3_AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm3_awltr;
#[doc = "DFSDM3_AWSR register accessor: an alias for `Reg<DFSDM3_AWSR_SPEC>`"]
pub type DFSDM3_AWSR = crate::Reg<dfsdm3_awsr::DFSDM3_AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm3_awsr;
#[doc = "DFSDM3_AWCFR register accessor: an alias for `Reg<DFSDM3_AWCFR_SPEC>`"]
pub type DFSDM3_AWCFR = crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm3_awcfr;
#[doc = "DFSDM3_EXMAX register accessor: an alias for `Reg<DFSDM3_EXMAX_SPEC>`"]
pub type DFSDM3_EXMAX = crate::Reg<dfsdm3_exmax::DFSDM3_EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm3_exmax;
#[doc = "DFSDM3_EXMIN register accessor: an alias for `Reg<DFSDM3_EXMIN_SPEC>`"]
pub type DFSDM3_EXMIN = crate::Reg<dfsdm3_exmin::DFSDM3_EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm3_exmin;
#[doc = "DFSDM3_CNVTIMR register accessor: an alias for `Reg<DFSDM3_CNVTIMR_SPEC>`"]
pub type DFSDM3_CNVTIMR = crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm3_cnvtimr;
