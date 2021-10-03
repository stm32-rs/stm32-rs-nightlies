#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSDM channel configuration 0 register 1"]
    pub dfsdm_chcfg0r1: crate::Reg<dfsdm_chcfg0r1::DFSDM_CHCFG0R1_SPEC>,
    #[doc = "0x04 - DFSDM channel configuration 1 register 1"]
    pub dfsdm_chcfg1r1: crate::Reg<dfsdm_chcfg1r1::DFSDM_CHCFG1R1_SPEC>,
    #[doc = "0x08 - DFSDM channel configuration 2 register 1"]
    pub dfsdm_chcfg2r1: crate::Reg<dfsdm_chcfg2r1::DFSDM_CHCFG2R1_SPEC>,
    #[doc = "0x0c - DFSDM channel configuration 3 register 1"]
    pub dfsdm_chcfg3r1: crate::Reg<dfsdm_chcfg3r1::DFSDM_CHCFG3R1_SPEC>,
    #[doc = "0x10 - DFSDM channel configuration 4 register 1"]
    pub dfsdm_chcfg4r1: crate::Reg<dfsdm_chcfg4r1::DFSDM_CHCFG4R1_SPEC>,
    #[doc = "0x14 - DFSDM channel configuration 5 register 1"]
    pub dfsdm_chcfg5r1: crate::Reg<dfsdm_chcfg5r1::DFSDM_CHCFG5R1_SPEC>,
    #[doc = "0x18 - DFSDM channel configuration 6 register 1"]
    pub dfsdm_chcfg6r1: crate::Reg<dfsdm_chcfg6r1::DFSDM_CHCFG6R1_SPEC>,
    #[doc = "0x1c - DFSDM channel configuration 7 register 1"]
    pub dfsdm_chcfg7r1: crate::Reg<dfsdm_chcfg7r1::DFSDM_CHCFG7R1_SPEC>,
    #[doc = "0x20 - DFSDM channel configuration 0 register 2"]
    pub dfsdm_chcfg0r2: crate::Reg<dfsdm_chcfg0r2::DFSDM_CHCFG0R2_SPEC>,
    #[doc = "0x24 - DFSDM channel configuration 1 register 2"]
    pub dfsdm_chcfg1r2: crate::Reg<dfsdm_chcfg1r2::DFSDM_CHCFG1R2_SPEC>,
    #[doc = "0x28 - DFSDM channel configuration 2 register 2"]
    pub dfsdm_chcfg2r2: crate::Reg<dfsdm_chcfg2r2::DFSDM_CHCFG2R2_SPEC>,
    #[doc = "0x2c - DFSDM channel configuration 3 register 2"]
    pub dfsdm_chcfg3r2: crate::Reg<dfsdm_chcfg3r2::DFSDM_CHCFG3R2_SPEC>,
    #[doc = "0x30 - DFSDM channel configuration 4 register 2"]
    pub dfsdm_chcfg4r2: crate::Reg<dfsdm_chcfg4r2::DFSDM_CHCFG4R2_SPEC>,
    #[doc = "0x34 - DFSDM channel configuration 5 register 2"]
    pub dfsdm_chcfg5r2: crate::Reg<dfsdm_chcfg5r2::DFSDM_CHCFG5R2_SPEC>,
    #[doc = "0x38 - DFSDM channel configuration 6 register 2"]
    pub dfsdm_chcfg6r2: crate::Reg<dfsdm_chcfg6r2::DFSDM_CHCFG6R2_SPEC>,
    #[doc = "0x3c - DFSDM channel configuration 7 register 2"]
    pub dfsdm_chcfg7r2: crate::Reg<dfsdm_chcfg7r2::DFSDM_CHCFG7R2_SPEC>,
    #[doc = "0x40 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd0r: crate::Reg<dfsdm_awscd0r::DFSDM_AWSCD0R_SPEC>,
    #[doc = "0x44 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd1r: crate::Reg<dfsdm_awscd1r::DFSDM_AWSCD1R_SPEC>,
    #[doc = "0x48 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd2r: crate::Reg<dfsdm_awscd2r::DFSDM_AWSCD2R_SPEC>,
    #[doc = "0x4c - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd3r: crate::Reg<dfsdm_awscd3r::DFSDM_AWSCD3R_SPEC>,
    #[doc = "0x50 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd4r: crate::Reg<dfsdm_awscd4r::DFSDM_AWSCD4R_SPEC>,
    #[doc = "0x54 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd5r: crate::Reg<dfsdm_awscd5r::DFSDM_AWSCD5R_SPEC>,
    #[doc = "0x58 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd6r: crate::Reg<dfsdm_awscd6r::DFSDM_AWSCD6R_SPEC>,
    #[doc = "0x5c - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd7r: crate::Reg<dfsdm_awscd7r::DFSDM_AWSCD7R_SPEC>,
    #[doc = "0x60 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat0r: crate::Reg<dfsdm_chwdat0r::DFSDM_CHWDAT0R_SPEC>,
    #[doc = "0x64 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat1r: crate::Reg<dfsdm_chwdat1r::DFSDM_CHWDAT1R_SPEC>,
    #[doc = "0x68 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat2r: crate::Reg<dfsdm_chwdat2r::DFSDM_CHWDAT2R_SPEC>,
    #[doc = "0x6c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat3r: crate::Reg<dfsdm_chwdat3r::DFSDM_CHWDAT3R_SPEC>,
    #[doc = "0x70 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat4r: crate::Reg<dfsdm_chwdat4r::DFSDM_CHWDAT4R_SPEC>,
    #[doc = "0x74 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat5r: crate::Reg<dfsdm_chwdat5r::DFSDM_CHWDAT5R_SPEC>,
    #[doc = "0x78 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat6r: crate::Reg<dfsdm_chwdat6r::DFSDM_CHWDAT6R_SPEC>,
    #[doc = "0x7c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat7r: crate::Reg<dfsdm_chwdat7r::DFSDM_CHWDAT7R_SPEC>,
    #[doc = "0x80 - DFSDM channel data input register"]
    pub dfsdm_chdatin0r: crate::Reg<dfsdm_chdatin0r::DFSDM_CHDATIN0R_SPEC>,
    #[doc = "0x84 - DFSDM channel data input register"]
    pub dfsdm_chdatin1r: crate::Reg<dfsdm_chdatin1r::DFSDM_CHDATIN1R_SPEC>,
    #[doc = "0x88 - DFSDM channel data input register"]
    pub dfsdm_chdatin2r: crate::Reg<dfsdm_chdatin2r::DFSDM_CHDATIN2R_SPEC>,
    #[doc = "0x8c - DFSDM channel data input register"]
    pub dfsdm_chdatin3r: crate::Reg<dfsdm_chdatin3r::DFSDM_CHDATIN3R_SPEC>,
    #[doc = "0x90 - DFSDM channel data input register"]
    pub dfsdm_chdatin4r: crate::Reg<dfsdm_chdatin4r::DFSDM_CHDATIN4R_SPEC>,
    #[doc = "0x94 - DFSDM channel data input register"]
    pub dfsdm_chdatin5r: crate::Reg<dfsdm_chdatin5r::DFSDM_CHDATIN5R_SPEC>,
    #[doc = "0x98 - DFSDM channel data input register"]
    pub dfsdm_chdatin6r: crate::Reg<dfsdm_chdatin6r::DFSDM_CHDATIN6R_SPEC>,
    #[doc = "0x9c - DFSDM channel data input register"]
    pub dfsdm_chdatin7r: crate::Reg<dfsdm_chdatin7r::DFSDM_CHDATIN7R_SPEC>,
    #[doc = "0xa0 - DFSDM control register 1"]
    pub dfsdm0_cr1: crate::Reg<dfsdm0_cr1::DFSDM0_CR1_SPEC>,
    #[doc = "0xa4 - DFSDM control register 1"]
    pub dfsdm1_cr1: crate::Reg<dfsdm1_cr1::DFSDM1_CR1_SPEC>,
    #[doc = "0xa8 - DFSDM control register 1"]
    pub dfsdm2_cr1: crate::Reg<dfsdm2_cr1::DFSDM2_CR1_SPEC>,
    #[doc = "0xac - DFSDM control register 1"]
    pub dfsdm3_cr1: crate::Reg<dfsdm3_cr1::DFSDM3_CR1_SPEC>,
    #[doc = "0xb0 - DFSDM control register 2"]
    pub dfsdm0_cr2: crate::Reg<dfsdm0_cr2::DFSDM0_CR2_SPEC>,
    #[doc = "0xb4 - DFSDM control register 2"]
    pub dfsdm1_cr2: crate::Reg<dfsdm1_cr2::DFSDM1_CR2_SPEC>,
    #[doc = "0xb8 - DFSDM control register 2"]
    pub dfsdm2_cr2: crate::Reg<dfsdm2_cr2::DFSDM2_CR2_SPEC>,
    #[doc = "0xbc - DFSDM control register 2"]
    pub dfsdm3_cr2: crate::Reg<dfsdm3_cr2::DFSDM3_CR2_SPEC>,
    #[doc = "0xc0 - DFSDM interrupt and status register"]
    pub dfsdm0_isr: crate::Reg<dfsdm0_isr::DFSDM0_ISR_SPEC>,
    #[doc = "0xc4 - DFSDM interrupt and status register"]
    pub dfsdm1_isr: crate::Reg<dfsdm1_isr::DFSDM1_ISR_SPEC>,
    #[doc = "0xc8 - DFSDM interrupt and status register"]
    pub dfsdm2_isr: crate::Reg<dfsdm2_isr::DFSDM2_ISR_SPEC>,
    #[doc = "0xcc - DFSDM interrupt and status register"]
    pub dfsdm3_isr: crate::Reg<dfsdm3_isr::DFSDM3_ISR_SPEC>,
    #[doc = "0xd0 - DFSDM interrupt flag clear register"]
    pub dfsdm0_icr: crate::Reg<dfsdm0_icr::DFSDM0_ICR_SPEC>,
    #[doc = "0xd4 - DFSDM interrupt flag clear register"]
    pub dfsdm1_icr: crate::Reg<dfsdm1_icr::DFSDM1_ICR_SPEC>,
    #[doc = "0xd8 - DFSDM interrupt flag clear register"]
    pub dfsdm2_icr: crate::Reg<dfsdm2_icr::DFSDM2_ICR_SPEC>,
    #[doc = "0xdc - DFSDM interrupt flag clear register"]
    pub dfsdm3_icr: crate::Reg<dfsdm3_icr::DFSDM3_ICR_SPEC>,
    #[doc = "0xe0 - DFSDM injected channel group selection register"]
    pub dfsdm0_jchgr: crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGR_SPEC>,
    #[doc = "0xe4 - DFSDM injected channel group selection register"]
    pub dfsdm1_jchgr: crate::Reg<dfsdm1_jchgr::DFSDM1_JCHGR_SPEC>,
    #[doc = "0xe8 - DFSDM injected channel group selection register"]
    pub dfsdm2_jchgr: crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGR_SPEC>,
    #[doc = "0xec - DFSDM injected channel group selection register"]
    pub dfsdm3_jchgr: crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGR_SPEC>,
    #[doc = "0xf0 - DFSDM filter control register"]
    pub dfsdm0_fcr: crate::Reg<dfsdm0_fcr::DFSDM0_FCR_SPEC>,
    #[doc = "0xf4 - DFSDM filter control register"]
    pub dfsdm1_fcr: crate::Reg<dfsdm1_fcr::DFSDM1_FCR_SPEC>,
    #[doc = "0xf8 - DFSDM filter control register"]
    pub dfsdm2_fcr: crate::Reg<dfsdm2_fcr::DFSDM2_FCR_SPEC>,
    #[doc = "0xfc - DFSDM filter control register"]
    pub dfsdm3_fcr: crate::Reg<dfsdm3_fcr::DFSDM3_FCR_SPEC>,
    #[doc = "0x100 - DFSDM data register for injected group"]
    pub dfsdm0_jdatar: crate::Reg<dfsdm0_jdatar::DFSDM0_JDATAR_SPEC>,
    #[doc = "0x104 - DFSDM data register for injected group"]
    pub dfsdm1_jdatar: crate::Reg<dfsdm1_jdatar::DFSDM1_JDATAR_SPEC>,
    #[doc = "0x108 - DFSDM data register for injected group"]
    pub dfsdm2_jdatar: crate::Reg<dfsdm2_jdatar::DFSDM2_JDATAR_SPEC>,
    #[doc = "0x10c - DFSDM data register for injected group"]
    pub dfsdm3_jdatar: crate::Reg<dfsdm3_jdatar::DFSDM3_JDATAR_SPEC>,
    #[doc = "0x110 - DFSDM data register for the regular channel"]
    pub dfsdm0_rdatar: crate::Reg<dfsdm0_rdatar::DFSDM0_RDATAR_SPEC>,
    #[doc = "0x114 - DFSDM data register for the regular channel"]
    pub dfsdm1_rdatar: crate::Reg<dfsdm1_rdatar::DFSDM1_RDATAR_SPEC>,
    #[doc = "0x118 - DFSDM data register for the regular channel"]
    pub dfsdm2_rdatar: crate::Reg<dfsdm2_rdatar::DFSDM2_RDATAR_SPEC>,
    #[doc = "0x11c - DFSDM data register for the regular channel"]
    pub dfsdm3_rdatar: crate::Reg<dfsdm3_rdatar::DFSDM3_RDATAR_SPEC>,
    #[doc = "0x120 - DFSDM analog watchdog high threshold register"]
    pub dfsdm0_awhtr: crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTR_SPEC>,
    #[doc = "0x124 - DFSDM analog watchdog high threshold register"]
    pub dfsdm1_awhtr: crate::Reg<dfsdm1_awhtr::DFSDM1_AWHTR_SPEC>,
    #[doc = "0x128 - DFSDM analog watchdog high threshold register"]
    pub dfsdm2_awhtr: crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTR_SPEC>,
    #[doc = "0x12c - DFSDM analog watchdog high threshold register"]
    pub dfsdm3_awhtr: crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTR_SPEC>,
    #[doc = "0x130 - DFSDM analog watchdog low threshold register"]
    pub dfsdm0_awltr: crate::Reg<dfsdm0_awltr::DFSDM0_AWLTR_SPEC>,
    #[doc = "0x134 - DFSDM analog watchdog low threshold register"]
    pub dfsdm1_awltr: crate::Reg<dfsdm1_awltr::DFSDM1_AWLTR_SPEC>,
    #[doc = "0x138 - DFSDM analog watchdog low threshold register"]
    pub dfsdm2_awltr: crate::Reg<dfsdm2_awltr::DFSDM2_AWLTR_SPEC>,
    #[doc = "0x13c - DFSDM analog watchdog low threshold register"]
    pub dfsdm3_awltr: crate::Reg<dfsdm3_awltr::DFSDM3_AWLTR_SPEC>,
    #[doc = "0x140 - DFSDM analog watchdog status register"]
    pub dfsdm0_awsr: crate::Reg<dfsdm0_awsr::DFSDM0_AWSR_SPEC>,
    #[doc = "0x144 - DFSDM analog watchdog status register"]
    pub dfsdm1_awsr: crate::Reg<dfsdm1_awsr::DFSDM1_AWSR_SPEC>,
    #[doc = "0x148 - DFSDM analog watchdog status register"]
    pub dfsdm2_awsr: crate::Reg<dfsdm2_awsr::DFSDM2_AWSR_SPEC>,
    #[doc = "0x14c - DFSDM analog watchdog status register"]
    pub dfsdm3_awsr: crate::Reg<dfsdm3_awsr::DFSDM3_AWSR_SPEC>,
    #[doc = "0x150 - DFSDM analog watchdog clear flag register"]
    pub dfsdm0_awcfr: crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFR_SPEC>,
    #[doc = "0x154 - DFSDM analog watchdog clear flag register"]
    pub dfsdm1_awcfr: crate::Reg<dfsdm1_awcfr::DFSDM1_AWCFR_SPEC>,
    #[doc = "0x158 - DFSDM analog watchdog clear flag register"]
    pub dfsdm2_awcfr: crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFR_SPEC>,
    #[doc = "0x15c - DFSDM analog watchdog clear flag register"]
    pub dfsdm3_awcfr: crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFR_SPEC>,
    #[doc = "0x160 - DFSDM Extremes detector maximum register"]
    pub dfsdm0_exmax: crate::Reg<dfsdm0_exmax::DFSDM0_EXMAX_SPEC>,
    #[doc = "0x164 - DFSDM Extremes detector maximum register"]
    pub dfsdm1_exmax: crate::Reg<dfsdm1_exmax::DFSDM1_EXMAX_SPEC>,
    #[doc = "0x168 - DFSDM Extremes detector maximum register"]
    pub dfsdm2_exmax: crate::Reg<dfsdm2_exmax::DFSDM2_EXMAX_SPEC>,
    #[doc = "0x16c - DFSDM Extremes detector maximum register"]
    pub dfsdm3_exmax: crate::Reg<dfsdm3_exmax::DFSDM3_EXMAX_SPEC>,
    #[doc = "0x170 - DFSDM Extremes detector minimum register"]
    pub dfsdm0_exmin: crate::Reg<dfsdm0_exmin::DFSDM0_EXMIN_SPEC>,
    #[doc = "0x174 - DFSDM Extremes detector minimum register"]
    pub dfsdm1_exmin: crate::Reg<dfsdm1_exmin::DFSDM1_EXMIN_SPEC>,
    #[doc = "0x178 - DFSDM Extremes detector minimum register"]
    pub dfsdm2_exmin: crate::Reg<dfsdm2_exmin::DFSDM2_EXMIN_SPEC>,
    #[doc = "0x17c - DFSDM Extremes detector minimum register"]
    pub dfsdm3_exmin: crate::Reg<dfsdm3_exmin::DFSDM3_EXMIN_SPEC>,
    #[doc = "0x180 - DFSDM conversion timer register"]
    pub dfsdm0_cnvtimr: crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMR_SPEC>,
    #[doc = "0x184 - DFSDM conversion timer register"]
    pub dfsdm1_cnvtimr: crate::Reg<dfsdm1_cnvtimr::DFSDM1_CNVTIMR_SPEC>,
    #[doc = "0x188 - DFSDM conversion timer register"]
    pub dfsdm2_cnvtimr: crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMR_SPEC>,
    #[doc = "0x18c - DFSDM conversion timer register"]
    pub dfsdm3_cnvtimr: crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMR_SPEC>,
}
#[doc = "DFSDM_CHCFG0R1 register accessor: an alias for `Reg<DFSDM_CHCFG0R1_SPEC>`"]
pub type DFSDM_CHCFG0R1 = crate::Reg<dfsdm_chcfg0r1::DFSDM_CHCFG0R1_SPEC>;
#[doc = "DFSDM channel configuration 0 register 1"]
pub mod dfsdm_chcfg0r1;
#[doc = "DFSDM_CHCFG1R1 register accessor: an alias for `Reg<DFSDM_CHCFG1R1_SPEC>`"]
pub type DFSDM_CHCFG1R1 = crate::Reg<dfsdm_chcfg1r1::DFSDM_CHCFG1R1_SPEC>;
#[doc = "DFSDM channel configuration 1 register 1"]
pub mod dfsdm_chcfg1r1;
#[doc = "DFSDM_CHCFG2R1 register accessor: an alias for `Reg<DFSDM_CHCFG2R1_SPEC>`"]
pub type DFSDM_CHCFG2R1 = crate::Reg<dfsdm_chcfg2r1::DFSDM_CHCFG2R1_SPEC>;
#[doc = "DFSDM channel configuration 2 register 1"]
pub mod dfsdm_chcfg2r1;
#[doc = "DFSDM_CHCFG3R1 register accessor: an alias for `Reg<DFSDM_CHCFG3R1_SPEC>`"]
pub type DFSDM_CHCFG3R1 = crate::Reg<dfsdm_chcfg3r1::DFSDM_CHCFG3R1_SPEC>;
#[doc = "DFSDM channel configuration 3 register 1"]
pub mod dfsdm_chcfg3r1;
#[doc = "DFSDM_CHCFG4R1 register accessor: an alias for `Reg<DFSDM_CHCFG4R1_SPEC>`"]
pub type DFSDM_CHCFG4R1 = crate::Reg<dfsdm_chcfg4r1::DFSDM_CHCFG4R1_SPEC>;
#[doc = "DFSDM channel configuration 4 register 1"]
pub mod dfsdm_chcfg4r1;
#[doc = "DFSDM_CHCFG5R1 register accessor: an alias for `Reg<DFSDM_CHCFG5R1_SPEC>`"]
pub type DFSDM_CHCFG5R1 = crate::Reg<dfsdm_chcfg5r1::DFSDM_CHCFG5R1_SPEC>;
#[doc = "DFSDM channel configuration 5 register 1"]
pub mod dfsdm_chcfg5r1;
#[doc = "DFSDM_CHCFG6R1 register accessor: an alias for `Reg<DFSDM_CHCFG6R1_SPEC>`"]
pub type DFSDM_CHCFG6R1 = crate::Reg<dfsdm_chcfg6r1::DFSDM_CHCFG6R1_SPEC>;
#[doc = "DFSDM channel configuration 6 register 1"]
pub mod dfsdm_chcfg6r1;
#[doc = "DFSDM_CHCFG7R1 register accessor: an alias for `Reg<DFSDM_CHCFG7R1_SPEC>`"]
pub type DFSDM_CHCFG7R1 = crate::Reg<dfsdm_chcfg7r1::DFSDM_CHCFG7R1_SPEC>;
#[doc = "DFSDM channel configuration 7 register 1"]
pub mod dfsdm_chcfg7r1;
#[doc = "DFSDM_CHCFG0R2 register accessor: an alias for `Reg<DFSDM_CHCFG0R2_SPEC>`"]
pub type DFSDM_CHCFG0R2 = crate::Reg<dfsdm_chcfg0r2::DFSDM_CHCFG0R2_SPEC>;
#[doc = "DFSDM channel configuration 0 register 2"]
pub mod dfsdm_chcfg0r2;
#[doc = "DFSDM_CHCFG1R2 register accessor: an alias for `Reg<DFSDM_CHCFG1R2_SPEC>`"]
pub type DFSDM_CHCFG1R2 = crate::Reg<dfsdm_chcfg1r2::DFSDM_CHCFG1R2_SPEC>;
#[doc = "DFSDM channel configuration 1 register 2"]
pub mod dfsdm_chcfg1r2;
#[doc = "DFSDM_CHCFG2R2 register accessor: an alias for `Reg<DFSDM_CHCFG2R2_SPEC>`"]
pub type DFSDM_CHCFG2R2 = crate::Reg<dfsdm_chcfg2r2::DFSDM_CHCFG2R2_SPEC>;
#[doc = "DFSDM channel configuration 2 register 2"]
pub mod dfsdm_chcfg2r2;
#[doc = "DFSDM_CHCFG3R2 register accessor: an alias for `Reg<DFSDM_CHCFG3R2_SPEC>`"]
pub type DFSDM_CHCFG3R2 = crate::Reg<dfsdm_chcfg3r2::DFSDM_CHCFG3R2_SPEC>;
#[doc = "DFSDM channel configuration 3 register 2"]
pub mod dfsdm_chcfg3r2;
#[doc = "DFSDM_CHCFG4R2 register accessor: an alias for `Reg<DFSDM_CHCFG4R2_SPEC>`"]
pub type DFSDM_CHCFG4R2 = crate::Reg<dfsdm_chcfg4r2::DFSDM_CHCFG4R2_SPEC>;
#[doc = "DFSDM channel configuration 4 register 2"]
pub mod dfsdm_chcfg4r2;
#[doc = "DFSDM_CHCFG5R2 register accessor: an alias for `Reg<DFSDM_CHCFG5R2_SPEC>`"]
pub type DFSDM_CHCFG5R2 = crate::Reg<dfsdm_chcfg5r2::DFSDM_CHCFG5R2_SPEC>;
#[doc = "DFSDM channel configuration 5 register 2"]
pub mod dfsdm_chcfg5r2;
#[doc = "DFSDM_CHCFG6R2 register accessor: an alias for `Reg<DFSDM_CHCFG6R2_SPEC>`"]
pub type DFSDM_CHCFG6R2 = crate::Reg<dfsdm_chcfg6r2::DFSDM_CHCFG6R2_SPEC>;
#[doc = "DFSDM channel configuration 6 register 2"]
pub mod dfsdm_chcfg6r2;
#[doc = "DFSDM_CHCFG7R2 register accessor: an alias for `Reg<DFSDM_CHCFG7R2_SPEC>`"]
pub type DFSDM_CHCFG7R2 = crate::Reg<dfsdm_chcfg7r2::DFSDM_CHCFG7R2_SPEC>;
#[doc = "DFSDM channel configuration 7 register 2"]
pub mod dfsdm_chcfg7r2;
#[doc = "DFSDM_AWSCD0R register accessor: an alias for `Reg<DFSDM_AWSCD0R_SPEC>`"]
pub type DFSDM_AWSCD0R = crate::Reg<dfsdm_awscd0r::DFSDM_AWSCD0R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd0r;
#[doc = "DFSDM_AWSCD1R register accessor: an alias for `Reg<DFSDM_AWSCD1R_SPEC>`"]
pub type DFSDM_AWSCD1R = crate::Reg<dfsdm_awscd1r::DFSDM_AWSCD1R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd1r;
#[doc = "DFSDM_AWSCD2R register accessor: an alias for `Reg<DFSDM_AWSCD2R_SPEC>`"]
pub type DFSDM_AWSCD2R = crate::Reg<dfsdm_awscd2r::DFSDM_AWSCD2R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd2r;
#[doc = "DFSDM_AWSCD3R register accessor: an alias for `Reg<DFSDM_AWSCD3R_SPEC>`"]
pub type DFSDM_AWSCD3R = crate::Reg<dfsdm_awscd3r::DFSDM_AWSCD3R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd3r;
#[doc = "DFSDM_AWSCD4R register accessor: an alias for `Reg<DFSDM_AWSCD4R_SPEC>`"]
pub type DFSDM_AWSCD4R = crate::Reg<dfsdm_awscd4r::DFSDM_AWSCD4R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd4r;
#[doc = "DFSDM_AWSCD5R register accessor: an alias for `Reg<DFSDM_AWSCD5R_SPEC>`"]
pub type DFSDM_AWSCD5R = crate::Reg<dfsdm_awscd5r::DFSDM_AWSCD5R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd5r;
#[doc = "DFSDM_AWSCD6R register accessor: an alias for `Reg<DFSDM_AWSCD6R_SPEC>`"]
pub type DFSDM_AWSCD6R = crate::Reg<dfsdm_awscd6r::DFSDM_AWSCD6R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd6r;
#[doc = "DFSDM_AWSCD7R register accessor: an alias for `Reg<DFSDM_AWSCD7R_SPEC>`"]
pub type DFSDM_AWSCD7R = crate::Reg<dfsdm_awscd7r::DFSDM_AWSCD7R_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd7r;
#[doc = "DFSDM_CHWDAT0R register accessor: an alias for `Reg<DFSDM_CHWDAT0R_SPEC>`"]
pub type DFSDM_CHWDAT0R = crate::Reg<dfsdm_chwdat0r::DFSDM_CHWDAT0R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat0r;
#[doc = "DFSDM_CHWDAT1R register accessor: an alias for `Reg<DFSDM_CHWDAT1R_SPEC>`"]
pub type DFSDM_CHWDAT1R = crate::Reg<dfsdm_chwdat1r::DFSDM_CHWDAT1R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat1r;
#[doc = "DFSDM_CHWDAT2R register accessor: an alias for `Reg<DFSDM_CHWDAT2R_SPEC>`"]
pub type DFSDM_CHWDAT2R = crate::Reg<dfsdm_chwdat2r::DFSDM_CHWDAT2R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat2r;
#[doc = "DFSDM_CHWDAT3R register accessor: an alias for `Reg<DFSDM_CHWDAT3R_SPEC>`"]
pub type DFSDM_CHWDAT3R = crate::Reg<dfsdm_chwdat3r::DFSDM_CHWDAT3R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat3r;
#[doc = "DFSDM_CHWDAT4R register accessor: an alias for `Reg<DFSDM_CHWDAT4R_SPEC>`"]
pub type DFSDM_CHWDAT4R = crate::Reg<dfsdm_chwdat4r::DFSDM_CHWDAT4R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat4r;
#[doc = "DFSDM_CHWDAT5R register accessor: an alias for `Reg<DFSDM_CHWDAT5R_SPEC>`"]
pub type DFSDM_CHWDAT5R = crate::Reg<dfsdm_chwdat5r::DFSDM_CHWDAT5R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat5r;
#[doc = "DFSDM_CHWDAT6R register accessor: an alias for `Reg<DFSDM_CHWDAT6R_SPEC>`"]
pub type DFSDM_CHWDAT6R = crate::Reg<dfsdm_chwdat6r::DFSDM_CHWDAT6R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat6r;
#[doc = "DFSDM_CHWDAT7R register accessor: an alias for `Reg<DFSDM_CHWDAT7R_SPEC>`"]
pub type DFSDM_CHWDAT7R = crate::Reg<dfsdm_chwdat7r::DFSDM_CHWDAT7R_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat7r;
#[doc = "DFSDM_CHDATIN0R register accessor: an alias for `Reg<DFSDM_CHDATIN0R_SPEC>`"]
pub type DFSDM_CHDATIN0R = crate::Reg<dfsdm_chdatin0r::DFSDM_CHDATIN0R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin0r;
#[doc = "DFSDM_CHDATIN1R register accessor: an alias for `Reg<DFSDM_CHDATIN1R_SPEC>`"]
pub type DFSDM_CHDATIN1R = crate::Reg<dfsdm_chdatin1r::DFSDM_CHDATIN1R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin1r;
#[doc = "DFSDM_CHDATIN2R register accessor: an alias for `Reg<DFSDM_CHDATIN2R_SPEC>`"]
pub type DFSDM_CHDATIN2R = crate::Reg<dfsdm_chdatin2r::DFSDM_CHDATIN2R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin2r;
#[doc = "DFSDM_CHDATIN3R register accessor: an alias for `Reg<DFSDM_CHDATIN3R_SPEC>`"]
pub type DFSDM_CHDATIN3R = crate::Reg<dfsdm_chdatin3r::DFSDM_CHDATIN3R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin3r;
#[doc = "DFSDM_CHDATIN4R register accessor: an alias for `Reg<DFSDM_CHDATIN4R_SPEC>`"]
pub type DFSDM_CHDATIN4R = crate::Reg<dfsdm_chdatin4r::DFSDM_CHDATIN4R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin4r;
#[doc = "DFSDM_CHDATIN5R register accessor: an alias for `Reg<DFSDM_CHDATIN5R_SPEC>`"]
pub type DFSDM_CHDATIN5R = crate::Reg<dfsdm_chdatin5r::DFSDM_CHDATIN5R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin5r;
#[doc = "DFSDM_CHDATIN6R register accessor: an alias for `Reg<DFSDM_CHDATIN6R_SPEC>`"]
pub type DFSDM_CHDATIN6R = crate::Reg<dfsdm_chdatin6r::DFSDM_CHDATIN6R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin6r;
#[doc = "DFSDM_CHDATIN7R register accessor: an alias for `Reg<DFSDM_CHDATIN7R_SPEC>`"]
pub type DFSDM_CHDATIN7R = crate::Reg<dfsdm_chdatin7r::DFSDM_CHDATIN7R_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin7r;
#[doc = "DFSDM0_CR1 register accessor: an alias for `Reg<DFSDM0_CR1_SPEC>`"]
pub type DFSDM0_CR1 = crate::Reg<dfsdm0_cr1::DFSDM0_CR1_SPEC>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm0_cr1;
#[doc = "DFSDM1_CR1 register accessor: an alias for `Reg<DFSDM1_CR1_SPEC>`"]
pub type DFSDM1_CR1 = crate::Reg<dfsdm1_cr1::DFSDM1_CR1_SPEC>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm1_cr1;
#[doc = "DFSDM2_CR1 register accessor: an alias for `Reg<DFSDM2_CR1_SPEC>`"]
pub type DFSDM2_CR1 = crate::Reg<dfsdm2_cr1::DFSDM2_CR1_SPEC>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm2_cr1;
#[doc = "DFSDM3_CR1 register accessor: an alias for `Reg<DFSDM3_CR1_SPEC>`"]
pub type DFSDM3_CR1 = crate::Reg<dfsdm3_cr1::DFSDM3_CR1_SPEC>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm3_cr1;
#[doc = "DFSDM0_CR2 register accessor: an alias for `Reg<DFSDM0_CR2_SPEC>`"]
pub type DFSDM0_CR2 = crate::Reg<dfsdm0_cr2::DFSDM0_CR2_SPEC>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm0_cr2;
#[doc = "DFSDM1_CR2 register accessor: an alias for `Reg<DFSDM1_CR2_SPEC>`"]
pub type DFSDM1_CR2 = crate::Reg<dfsdm1_cr2::DFSDM1_CR2_SPEC>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm1_cr2;
#[doc = "DFSDM2_CR2 register accessor: an alias for `Reg<DFSDM2_CR2_SPEC>`"]
pub type DFSDM2_CR2 = crate::Reg<dfsdm2_cr2::DFSDM2_CR2_SPEC>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm2_cr2;
#[doc = "DFSDM3_CR2 register accessor: an alias for `Reg<DFSDM3_CR2_SPEC>`"]
pub type DFSDM3_CR2 = crate::Reg<dfsdm3_cr2::DFSDM3_CR2_SPEC>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm3_cr2;
#[doc = "DFSDM0_ISR register accessor: an alias for `Reg<DFSDM0_ISR_SPEC>`"]
pub type DFSDM0_ISR = crate::Reg<dfsdm0_isr::DFSDM0_ISR_SPEC>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm0_isr;
#[doc = "DFSDM1_ISR register accessor: an alias for `Reg<DFSDM1_ISR_SPEC>`"]
pub type DFSDM1_ISR = crate::Reg<dfsdm1_isr::DFSDM1_ISR_SPEC>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm1_isr;
#[doc = "DFSDM2_ISR register accessor: an alias for `Reg<DFSDM2_ISR_SPEC>`"]
pub type DFSDM2_ISR = crate::Reg<dfsdm2_isr::DFSDM2_ISR_SPEC>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm2_isr;
#[doc = "DFSDM3_ISR register accessor: an alias for `Reg<DFSDM3_ISR_SPEC>`"]
pub type DFSDM3_ISR = crate::Reg<dfsdm3_isr::DFSDM3_ISR_SPEC>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm3_isr;
#[doc = "DFSDM0_ICR register accessor: an alias for `Reg<DFSDM0_ICR_SPEC>`"]
pub type DFSDM0_ICR = crate::Reg<dfsdm0_icr::DFSDM0_ICR_SPEC>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm0_icr;
#[doc = "DFSDM1_ICR register accessor: an alias for `Reg<DFSDM1_ICR_SPEC>`"]
pub type DFSDM1_ICR = crate::Reg<dfsdm1_icr::DFSDM1_ICR_SPEC>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm1_icr;
#[doc = "DFSDM2_ICR register accessor: an alias for `Reg<DFSDM2_ICR_SPEC>`"]
pub type DFSDM2_ICR = crate::Reg<dfsdm2_icr::DFSDM2_ICR_SPEC>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm2_icr;
#[doc = "DFSDM3_ICR register accessor: an alias for `Reg<DFSDM3_ICR_SPEC>`"]
pub type DFSDM3_ICR = crate::Reg<dfsdm3_icr::DFSDM3_ICR_SPEC>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm3_icr;
#[doc = "DFSDM0_JCHGR register accessor: an alias for `Reg<DFSDM0_JCHGR_SPEC>`"]
pub type DFSDM0_JCHGR = crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGR_SPEC>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm0_jchgr;
#[doc = "DFSDM1_JCHGR register accessor: an alias for `Reg<DFSDM1_JCHGR_SPEC>`"]
pub type DFSDM1_JCHGR = crate::Reg<dfsdm1_jchgr::DFSDM1_JCHGR_SPEC>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm1_jchgr;
#[doc = "DFSDM2_JCHGR register accessor: an alias for `Reg<DFSDM2_JCHGR_SPEC>`"]
pub type DFSDM2_JCHGR = crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGR_SPEC>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm2_jchgr;
#[doc = "DFSDM3_JCHGR register accessor: an alias for `Reg<DFSDM3_JCHGR_SPEC>`"]
pub type DFSDM3_JCHGR = crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGR_SPEC>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm3_jchgr;
#[doc = "DFSDM0_FCR register accessor: an alias for `Reg<DFSDM0_FCR_SPEC>`"]
pub type DFSDM0_FCR = crate::Reg<dfsdm0_fcr::DFSDM0_FCR_SPEC>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm0_fcr;
#[doc = "DFSDM1_FCR register accessor: an alias for `Reg<DFSDM1_FCR_SPEC>`"]
pub type DFSDM1_FCR = crate::Reg<dfsdm1_fcr::DFSDM1_FCR_SPEC>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm1_fcr;
#[doc = "DFSDM2_FCR register accessor: an alias for `Reg<DFSDM2_FCR_SPEC>`"]
pub type DFSDM2_FCR = crate::Reg<dfsdm2_fcr::DFSDM2_FCR_SPEC>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm2_fcr;
#[doc = "DFSDM3_FCR register accessor: an alias for `Reg<DFSDM3_FCR_SPEC>`"]
pub type DFSDM3_FCR = crate::Reg<dfsdm3_fcr::DFSDM3_FCR_SPEC>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm3_fcr;
#[doc = "DFSDM0_JDATAR register accessor: an alias for `Reg<DFSDM0_JDATAR_SPEC>`"]
pub type DFSDM0_JDATAR = crate::Reg<dfsdm0_jdatar::DFSDM0_JDATAR_SPEC>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm0_jdatar;
#[doc = "DFSDM1_JDATAR register accessor: an alias for `Reg<DFSDM1_JDATAR_SPEC>`"]
pub type DFSDM1_JDATAR = crate::Reg<dfsdm1_jdatar::DFSDM1_JDATAR_SPEC>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm1_jdatar;
#[doc = "DFSDM2_JDATAR register accessor: an alias for `Reg<DFSDM2_JDATAR_SPEC>`"]
pub type DFSDM2_JDATAR = crate::Reg<dfsdm2_jdatar::DFSDM2_JDATAR_SPEC>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm2_jdatar;
#[doc = "DFSDM3_JDATAR register accessor: an alias for `Reg<DFSDM3_JDATAR_SPEC>`"]
pub type DFSDM3_JDATAR = crate::Reg<dfsdm3_jdatar::DFSDM3_JDATAR_SPEC>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm3_jdatar;
#[doc = "DFSDM0_RDATAR register accessor: an alias for `Reg<DFSDM0_RDATAR_SPEC>`"]
pub type DFSDM0_RDATAR = crate::Reg<dfsdm0_rdatar::DFSDM0_RDATAR_SPEC>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm0_rdatar;
#[doc = "DFSDM1_RDATAR register accessor: an alias for `Reg<DFSDM1_RDATAR_SPEC>`"]
pub type DFSDM1_RDATAR = crate::Reg<dfsdm1_rdatar::DFSDM1_RDATAR_SPEC>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm1_rdatar;
#[doc = "DFSDM2_RDATAR register accessor: an alias for `Reg<DFSDM2_RDATAR_SPEC>`"]
pub type DFSDM2_RDATAR = crate::Reg<dfsdm2_rdatar::DFSDM2_RDATAR_SPEC>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm2_rdatar;
#[doc = "DFSDM3_RDATAR register accessor: an alias for `Reg<DFSDM3_RDATAR_SPEC>`"]
pub type DFSDM3_RDATAR = crate::Reg<dfsdm3_rdatar::DFSDM3_RDATAR_SPEC>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm3_rdatar;
#[doc = "DFSDM0_AWHTR register accessor: an alias for `Reg<DFSDM0_AWHTR_SPEC>`"]
pub type DFSDM0_AWHTR = crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTR_SPEC>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm0_awhtr;
#[doc = "DFSDM1_AWHTR register accessor: an alias for `Reg<DFSDM1_AWHTR_SPEC>`"]
pub type DFSDM1_AWHTR = crate::Reg<dfsdm1_awhtr::DFSDM1_AWHTR_SPEC>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm1_awhtr;
#[doc = "DFSDM2_AWHTR register accessor: an alias for `Reg<DFSDM2_AWHTR_SPEC>`"]
pub type DFSDM2_AWHTR = crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTR_SPEC>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm2_awhtr;
#[doc = "DFSDM3_AWHTR register accessor: an alias for `Reg<DFSDM3_AWHTR_SPEC>`"]
pub type DFSDM3_AWHTR = crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTR_SPEC>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm3_awhtr;
#[doc = "DFSDM0_AWLTR register accessor: an alias for `Reg<DFSDM0_AWLTR_SPEC>`"]
pub type DFSDM0_AWLTR = crate::Reg<dfsdm0_awltr::DFSDM0_AWLTR_SPEC>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm0_awltr;
#[doc = "DFSDM1_AWLTR register accessor: an alias for `Reg<DFSDM1_AWLTR_SPEC>`"]
pub type DFSDM1_AWLTR = crate::Reg<dfsdm1_awltr::DFSDM1_AWLTR_SPEC>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm1_awltr;
#[doc = "DFSDM2_AWLTR register accessor: an alias for `Reg<DFSDM2_AWLTR_SPEC>`"]
pub type DFSDM2_AWLTR = crate::Reg<dfsdm2_awltr::DFSDM2_AWLTR_SPEC>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm2_awltr;
#[doc = "DFSDM3_AWLTR register accessor: an alias for `Reg<DFSDM3_AWLTR_SPEC>`"]
pub type DFSDM3_AWLTR = crate::Reg<dfsdm3_awltr::DFSDM3_AWLTR_SPEC>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm3_awltr;
#[doc = "DFSDM0_AWSR register accessor: an alias for `Reg<DFSDM0_AWSR_SPEC>`"]
pub type DFSDM0_AWSR = crate::Reg<dfsdm0_awsr::DFSDM0_AWSR_SPEC>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm0_awsr;
#[doc = "DFSDM1_AWSR register accessor: an alias for `Reg<DFSDM1_AWSR_SPEC>`"]
pub type DFSDM1_AWSR = crate::Reg<dfsdm1_awsr::DFSDM1_AWSR_SPEC>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm1_awsr;
#[doc = "DFSDM2_AWSR register accessor: an alias for `Reg<DFSDM2_AWSR_SPEC>`"]
pub type DFSDM2_AWSR = crate::Reg<dfsdm2_awsr::DFSDM2_AWSR_SPEC>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm2_awsr;
#[doc = "DFSDM3_AWSR register accessor: an alias for `Reg<DFSDM3_AWSR_SPEC>`"]
pub type DFSDM3_AWSR = crate::Reg<dfsdm3_awsr::DFSDM3_AWSR_SPEC>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm3_awsr;
#[doc = "DFSDM0_AWCFR register accessor: an alias for `Reg<DFSDM0_AWCFR_SPEC>`"]
pub type DFSDM0_AWCFR = crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFR_SPEC>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm0_awcfr;
#[doc = "DFSDM1_AWCFR register accessor: an alias for `Reg<DFSDM1_AWCFR_SPEC>`"]
pub type DFSDM1_AWCFR = crate::Reg<dfsdm1_awcfr::DFSDM1_AWCFR_SPEC>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm1_awcfr;
#[doc = "DFSDM2_AWCFR register accessor: an alias for `Reg<DFSDM2_AWCFR_SPEC>`"]
pub type DFSDM2_AWCFR = crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFR_SPEC>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm2_awcfr;
#[doc = "DFSDM3_AWCFR register accessor: an alias for `Reg<DFSDM3_AWCFR_SPEC>`"]
pub type DFSDM3_AWCFR = crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFR_SPEC>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm3_awcfr;
#[doc = "DFSDM0_EXMAX register accessor: an alias for `Reg<DFSDM0_EXMAX_SPEC>`"]
pub type DFSDM0_EXMAX = crate::Reg<dfsdm0_exmax::DFSDM0_EXMAX_SPEC>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm0_exmax;
#[doc = "DFSDM1_EXMAX register accessor: an alias for `Reg<DFSDM1_EXMAX_SPEC>`"]
pub type DFSDM1_EXMAX = crate::Reg<dfsdm1_exmax::DFSDM1_EXMAX_SPEC>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm1_exmax;
#[doc = "DFSDM2_EXMAX register accessor: an alias for `Reg<DFSDM2_EXMAX_SPEC>`"]
pub type DFSDM2_EXMAX = crate::Reg<dfsdm2_exmax::DFSDM2_EXMAX_SPEC>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm2_exmax;
#[doc = "DFSDM3_EXMAX register accessor: an alias for `Reg<DFSDM3_EXMAX_SPEC>`"]
pub type DFSDM3_EXMAX = crate::Reg<dfsdm3_exmax::DFSDM3_EXMAX_SPEC>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm3_exmax;
#[doc = "DFSDM0_EXMIN register accessor: an alias for `Reg<DFSDM0_EXMIN_SPEC>`"]
pub type DFSDM0_EXMIN = crate::Reg<dfsdm0_exmin::DFSDM0_EXMIN_SPEC>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm0_exmin;
#[doc = "DFSDM1_EXMIN register accessor: an alias for `Reg<DFSDM1_EXMIN_SPEC>`"]
pub type DFSDM1_EXMIN = crate::Reg<dfsdm1_exmin::DFSDM1_EXMIN_SPEC>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm1_exmin;
#[doc = "DFSDM2_EXMIN register accessor: an alias for `Reg<DFSDM2_EXMIN_SPEC>`"]
pub type DFSDM2_EXMIN = crate::Reg<dfsdm2_exmin::DFSDM2_EXMIN_SPEC>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm2_exmin;
#[doc = "DFSDM3_EXMIN register accessor: an alias for `Reg<DFSDM3_EXMIN_SPEC>`"]
pub type DFSDM3_EXMIN = crate::Reg<dfsdm3_exmin::DFSDM3_EXMIN_SPEC>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm3_exmin;
#[doc = "DFSDM0_CNVTIMR register accessor: an alias for `Reg<DFSDM0_CNVTIMR_SPEC>`"]
pub type DFSDM0_CNVTIMR = crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMR_SPEC>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm0_cnvtimr;
#[doc = "DFSDM1_CNVTIMR register accessor: an alias for `Reg<DFSDM1_CNVTIMR_SPEC>`"]
pub type DFSDM1_CNVTIMR = crate::Reg<dfsdm1_cnvtimr::DFSDM1_CNVTIMR_SPEC>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm1_cnvtimr;
#[doc = "DFSDM2_CNVTIMR register accessor: an alias for `Reg<DFSDM2_CNVTIMR_SPEC>`"]
pub type DFSDM2_CNVTIMR = crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMR_SPEC>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm2_cnvtimr;
#[doc = "DFSDM3_CNVTIMR register accessor: an alias for `Reg<DFSDM3_CNVTIMR_SPEC>`"]
pub type DFSDM3_CNVTIMR = crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMR_SPEC>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm3_cnvtimr;
