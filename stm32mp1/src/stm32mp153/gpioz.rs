#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioz_moder: crate::Reg<gpioz_moder::GPIOZ_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioz_otyper: crate::Reg<gpioz_otyper::GPIOZ_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioz_ospeedr: crate::Reg<gpioz_ospeedr::GPIOZ_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioz_pupdr: crate::Reg<gpioz_pupdr::GPIOZ_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioz_idr: crate::Reg<gpioz_idr::GPIOZ_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioz_odr: crate::Reg<gpioz_odr::GPIOZ_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioz_bsrr: crate::Reg<gpioz_bsrr::GPIOZ_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioz_lckr: crate::Reg<gpioz_lckr::GPIOZ_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioz_afrl: crate::Reg<gpioz_afrl::GPIOZ_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioz_afrh: crate::Reg<gpioz_afrh::GPIOZ_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioz_brr: crate::Reg<gpioz_brr::GPIOZ_BRR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded."]
    pub gpioz_seccfgr: crate::Reg<gpioz_seccfgr::GPIOZ_SECCFGR_SPEC>,
    _reserved12: [u8; 0x0394],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioz_hwcfgr10: crate::Reg<gpioz_hwcfgr10::GPIOZ_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioz_hwcfgr9: crate::Reg<gpioz_hwcfgr9::GPIOZ_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioz_hwcfgr8: crate::Reg<gpioz_hwcfgr8::GPIOZ_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioz_hwcfgr7: crate::Reg<gpioz_hwcfgr7::GPIOZ_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioz_hwcfgr6: crate::Reg<gpioz_hwcfgr6::GPIOZ_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioz_hwcfgr5: crate::Reg<gpioz_hwcfgr5::GPIOZ_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioz_hwcfgr4: crate::Reg<gpioz_hwcfgr4::GPIOZ_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioz_hwcfgr3: crate::Reg<gpioz_hwcfgr3::GPIOZ_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioz_hwcfgr2: crate::Reg<gpioz_hwcfgr2::GPIOZ_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioz_hwcfgr1: crate::Reg<gpioz_hwcfgr1::GPIOZ_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioz_hwcfgr0: crate::Reg<gpioz_hwcfgr0::GPIOZ_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioz_verr: crate::Reg<gpioz_verr::GPIOZ_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioz_ipidr: crate::Reg<gpioz_ipidr::GPIOZ_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioz_sidr: crate::Reg<gpioz_sidr::GPIOZ_SIDR_SPEC>,
}
#[doc = "GPIOZ_MODER register accessor: an alias for `Reg<GPIOZ_MODER_SPEC>`"]
pub type GPIOZ_MODER = crate::Reg<gpioz_moder::GPIOZ_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioz_moder;
#[doc = "GPIOZ_OTYPER register accessor: an alias for `Reg<GPIOZ_OTYPER_SPEC>`"]
pub type GPIOZ_OTYPER = crate::Reg<gpioz_otyper::GPIOZ_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioz_otyper;
#[doc = "GPIOZ_OSPEEDR register accessor: an alias for `Reg<GPIOZ_OSPEEDR_SPEC>`"]
pub type GPIOZ_OSPEEDR = crate::Reg<gpioz_ospeedr::GPIOZ_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioz_ospeedr;
#[doc = "GPIOZ_PUPDR register accessor: an alias for `Reg<GPIOZ_PUPDR_SPEC>`"]
pub type GPIOZ_PUPDR = crate::Reg<gpioz_pupdr::GPIOZ_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioz_pupdr;
#[doc = "GPIOZ_IDR register accessor: an alias for `Reg<GPIOZ_IDR_SPEC>`"]
pub type GPIOZ_IDR = crate::Reg<gpioz_idr::GPIOZ_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioz_idr;
#[doc = "GPIOZ_ODR register accessor: an alias for `Reg<GPIOZ_ODR_SPEC>`"]
pub type GPIOZ_ODR = crate::Reg<gpioz_odr::GPIOZ_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioz_odr;
#[doc = "GPIOZ_BSRR register accessor: an alias for `Reg<GPIOZ_BSRR_SPEC>`"]
pub type GPIOZ_BSRR = crate::Reg<gpioz_bsrr::GPIOZ_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioz_bsrr;
#[doc = "GPIOZ_LCKR register accessor: an alias for `Reg<GPIOZ_LCKR_SPEC>`"]
pub type GPIOZ_LCKR = crate::Reg<gpioz_lckr::GPIOZ_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioz_lckr;
#[doc = "GPIOZ_AFRL register accessor: an alias for `Reg<GPIOZ_AFRL_SPEC>`"]
pub type GPIOZ_AFRL = crate::Reg<gpioz_afrl::GPIOZ_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioz_afrl;
#[doc = "GPIOZ_AFRH register accessor: an alias for `Reg<GPIOZ_AFRH_SPEC>`"]
pub type GPIOZ_AFRH = crate::Reg<gpioz_afrh::GPIOZ_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioz_afrh;
#[doc = "GPIOZ_BRR register accessor: an alias for `Reg<GPIOZ_BRR_SPEC>`"]
pub type GPIOZ_BRR = crate::Reg<gpioz_brr::GPIOZ_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioz_brr;
#[doc = "GPIOZ_SECCFGR register accessor: an alias for `Reg<GPIOZ_SECCFGR_SPEC>`"]
pub type GPIOZ_SECCFGR = crate::Reg<gpioz_seccfgr::GPIOZ_SECCFGR_SPEC>;
#[doc = "This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded."]
pub mod gpioz_seccfgr;
#[doc = "GPIOZ_HWCFGR10 register accessor: an alias for `Reg<GPIOZ_HWCFGR10_SPEC>`"]
pub type GPIOZ_HWCFGR10 = crate::Reg<gpioz_hwcfgr10::GPIOZ_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioz_hwcfgr10;
#[doc = "GPIOZ_HWCFGR9 register accessor: an alias for `Reg<GPIOZ_HWCFGR9_SPEC>`"]
pub type GPIOZ_HWCFGR9 = crate::Reg<gpioz_hwcfgr9::GPIOZ_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioz_hwcfgr9;
#[doc = "GPIOZ_HWCFGR8 register accessor: an alias for `Reg<GPIOZ_HWCFGR8_SPEC>`"]
pub type GPIOZ_HWCFGR8 = crate::Reg<gpioz_hwcfgr8::GPIOZ_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioz_hwcfgr8;
#[doc = "GPIOZ_HWCFGR7 register accessor: an alias for `Reg<GPIOZ_HWCFGR7_SPEC>`"]
pub type GPIOZ_HWCFGR7 = crate::Reg<gpioz_hwcfgr7::GPIOZ_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioz_hwcfgr7;
#[doc = "GPIOZ_HWCFGR6 register accessor: an alias for `Reg<GPIOZ_HWCFGR6_SPEC>`"]
pub type GPIOZ_HWCFGR6 = crate::Reg<gpioz_hwcfgr6::GPIOZ_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioz_hwcfgr6;
#[doc = "GPIOZ_HWCFGR5 register accessor: an alias for `Reg<GPIOZ_HWCFGR5_SPEC>`"]
pub type GPIOZ_HWCFGR5 = crate::Reg<gpioz_hwcfgr5::GPIOZ_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioz_hwcfgr5;
#[doc = "GPIOZ_HWCFGR4 register accessor: an alias for `Reg<GPIOZ_HWCFGR4_SPEC>`"]
pub type GPIOZ_HWCFGR4 = crate::Reg<gpioz_hwcfgr4::GPIOZ_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioz_hwcfgr4;
#[doc = "GPIOZ_HWCFGR3 register accessor: an alias for `Reg<GPIOZ_HWCFGR3_SPEC>`"]
pub type GPIOZ_HWCFGR3 = crate::Reg<gpioz_hwcfgr3::GPIOZ_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioz_hwcfgr3;
#[doc = "GPIOZ_HWCFGR2 register accessor: an alias for `Reg<GPIOZ_HWCFGR2_SPEC>`"]
pub type GPIOZ_HWCFGR2 = crate::Reg<gpioz_hwcfgr2::GPIOZ_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioz_hwcfgr2;
#[doc = "GPIOZ_HWCFGR1 register accessor: an alias for `Reg<GPIOZ_HWCFGR1_SPEC>`"]
pub type GPIOZ_HWCFGR1 = crate::Reg<gpioz_hwcfgr1::GPIOZ_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioz_hwcfgr1;
#[doc = "GPIOZ_HWCFGR0 register accessor: an alias for `Reg<GPIOZ_HWCFGR0_SPEC>`"]
pub type GPIOZ_HWCFGR0 = crate::Reg<gpioz_hwcfgr0::GPIOZ_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioz_hwcfgr0;
#[doc = "GPIOZ_VERR register accessor: an alias for `Reg<GPIOZ_VERR_SPEC>`"]
pub type GPIOZ_VERR = crate::Reg<gpioz_verr::GPIOZ_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioz_verr;
#[doc = "GPIOZ_IPIDR register accessor: an alias for `Reg<GPIOZ_IPIDR_SPEC>`"]
pub type GPIOZ_IPIDR = crate::Reg<gpioz_ipidr::GPIOZ_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioz_ipidr;
#[doc = "GPIOZ_SIDR register accessor: an alias for `Reg<GPIOZ_SIDR_SPEC>`"]
pub type GPIOZ_SIDR = crate::Reg<gpioz_sidr::GPIOZ_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioz_sidr;
