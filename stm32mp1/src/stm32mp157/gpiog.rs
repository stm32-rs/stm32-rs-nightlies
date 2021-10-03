#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiog_moder: crate::Reg<gpiog_moder::GPIOG_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiog_otyper: crate::Reg<gpiog_otyper::GPIOG_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiog_ospeedr: crate::Reg<gpiog_ospeedr::GPIOG_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiog_pupdr: crate::Reg<gpiog_pupdr::GPIOG_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiog_idr: crate::Reg<gpiog_idr::GPIOG_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiog_odr: crate::Reg<gpiog_odr::GPIOG_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiog_bsrr: crate::Reg<gpiog_bsrr::GPIOG_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiog_lckr: crate::Reg<gpiog_lckr::GPIOG_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiog_afrl: crate::Reg<gpiog_afrl::GPIOG_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiog_afrh: crate::Reg<gpiog_afrh::GPIOG_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiog_brr: crate::Reg<gpiog_brr::GPIOG_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiog_hwcfgr10: crate::Reg<gpiog_hwcfgr10::GPIOG_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiog_hwcfgr9: crate::Reg<gpiog_hwcfgr9::GPIOG_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiog_hwcfgr8: crate::Reg<gpiog_hwcfgr8::GPIOG_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiog_hwcfgr7: crate::Reg<gpiog_hwcfgr7::GPIOG_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiog_hwcfgr6: crate::Reg<gpiog_hwcfgr6::GPIOG_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiog_hwcfgr5: crate::Reg<gpiog_hwcfgr5::GPIOG_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiog_hwcfgr4: crate::Reg<gpiog_hwcfgr4::GPIOG_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiog_hwcfgr3: crate::Reg<gpiog_hwcfgr3::GPIOG_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiog_hwcfgr2: crate::Reg<gpiog_hwcfgr2::GPIOG_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiog_hwcfgr1: crate::Reg<gpiog_hwcfgr1::GPIOG_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiog_hwcfgr0: crate::Reg<gpiog_hwcfgr0::GPIOG_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiog_verr: crate::Reg<gpiog_verr::GPIOG_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiog_ipidr: crate::Reg<gpiog_ipidr::GPIOG_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiog_sidr: crate::Reg<gpiog_sidr::GPIOG_SIDR_SPEC>,
}
#[doc = "GPIOG_MODER register accessor: an alias for `Reg<GPIOG_MODER_SPEC>`"]
pub type GPIOG_MODER = crate::Reg<gpiog_moder::GPIOG_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpiog_moder;
#[doc = "GPIOG_OTYPER register accessor: an alias for `Reg<GPIOG_OTYPER_SPEC>`"]
pub type GPIOG_OTYPER = crate::Reg<gpiog_otyper::GPIOG_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpiog_otyper;
#[doc = "GPIOG_OSPEEDR register accessor: an alias for `Reg<GPIOG_OSPEEDR_SPEC>`"]
pub type GPIOG_OSPEEDR = crate::Reg<gpiog_ospeedr::GPIOG_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpiog_ospeedr;
#[doc = "GPIOG_PUPDR register accessor: an alias for `Reg<GPIOG_PUPDR_SPEC>`"]
pub type GPIOG_PUPDR = crate::Reg<gpiog_pupdr::GPIOG_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiog_pupdr;
#[doc = "GPIOG_IDR register accessor: an alias for `Reg<GPIOG_IDR_SPEC>`"]
pub type GPIOG_IDR = crate::Reg<gpiog_idr::GPIOG_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpiog_idr;
#[doc = "GPIOG_ODR register accessor: an alias for `Reg<GPIOG_ODR_SPEC>`"]
pub type GPIOG_ODR = crate::Reg<gpiog_odr::GPIOG_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpiog_odr;
#[doc = "GPIOG_BSRR register accessor: an alias for `Reg<GPIOG_BSRR_SPEC>`"]
pub type GPIOG_BSRR = crate::Reg<gpiog_bsrr::GPIOG_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiog_bsrr;
#[doc = "GPIOG_LCKR register accessor: an alias for `Reg<GPIOG_LCKR_SPEC>`"]
pub type GPIOG_LCKR = crate::Reg<gpiog_lckr::GPIOG_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiog_lckr;
#[doc = "GPIOG_AFRL register accessor: an alias for `Reg<GPIOG_AFRL_SPEC>`"]
pub type GPIOG_AFRL = crate::Reg<gpiog_afrl::GPIOG_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpiog_afrl;
#[doc = "GPIOG_AFRH register accessor: an alias for `Reg<GPIOG_AFRH_SPEC>`"]
pub type GPIOG_AFRH = crate::Reg<gpiog_afrh::GPIOG_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpiog_afrh;
#[doc = "GPIOG_BRR register accessor: an alias for `Reg<GPIOG_BRR_SPEC>`"]
pub type GPIOG_BRR = crate::Reg<gpiog_brr::GPIOG_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpiog_brr;
#[doc = "GPIOG_HWCFGR10 register accessor: an alias for `Reg<GPIOG_HWCFGR10_SPEC>`"]
pub type GPIOG_HWCFGR10 = crate::Reg<gpiog_hwcfgr10::GPIOG_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiog_hwcfgr10;
#[doc = "GPIOG_HWCFGR9 register accessor: an alias for `Reg<GPIOG_HWCFGR9_SPEC>`"]
pub type GPIOG_HWCFGR9 = crate::Reg<gpiog_hwcfgr9::GPIOG_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiog_hwcfgr9;
#[doc = "GPIOG_HWCFGR8 register accessor: an alias for `Reg<GPIOG_HWCFGR8_SPEC>`"]
pub type GPIOG_HWCFGR8 = crate::Reg<gpiog_hwcfgr8::GPIOG_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiog_hwcfgr8;
#[doc = "GPIOG_HWCFGR7 register accessor: an alias for `Reg<GPIOG_HWCFGR7_SPEC>`"]
pub type GPIOG_HWCFGR7 = crate::Reg<gpiog_hwcfgr7::GPIOG_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiog_hwcfgr7;
#[doc = "GPIOG_HWCFGR6 register accessor: an alias for `Reg<GPIOG_HWCFGR6_SPEC>`"]
pub type GPIOG_HWCFGR6 = crate::Reg<gpiog_hwcfgr6::GPIOG_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiog_hwcfgr6;
#[doc = "GPIOG_HWCFGR5 register accessor: an alias for `Reg<GPIOG_HWCFGR5_SPEC>`"]
pub type GPIOG_HWCFGR5 = crate::Reg<gpiog_hwcfgr5::GPIOG_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiog_hwcfgr5;
#[doc = "GPIOG_HWCFGR4 register accessor: an alias for `Reg<GPIOG_HWCFGR4_SPEC>`"]
pub type GPIOG_HWCFGR4 = crate::Reg<gpiog_hwcfgr4::GPIOG_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiog_hwcfgr4;
#[doc = "GPIOG_HWCFGR3 register accessor: an alias for `Reg<GPIOG_HWCFGR3_SPEC>`"]
pub type GPIOG_HWCFGR3 = crate::Reg<gpiog_hwcfgr3::GPIOG_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiog_hwcfgr3;
#[doc = "GPIOG_HWCFGR2 register accessor: an alias for `Reg<GPIOG_HWCFGR2_SPEC>`"]
pub type GPIOG_HWCFGR2 = crate::Reg<gpiog_hwcfgr2::GPIOG_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiog_hwcfgr2;
#[doc = "GPIOG_HWCFGR1 register accessor: an alias for `Reg<GPIOG_HWCFGR1_SPEC>`"]
pub type GPIOG_HWCFGR1 = crate::Reg<gpiog_hwcfgr1::GPIOG_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiog_hwcfgr1;
#[doc = "GPIOG_HWCFGR0 register accessor: an alias for `Reg<GPIOG_HWCFGR0_SPEC>`"]
pub type GPIOG_HWCFGR0 = crate::Reg<gpiog_hwcfgr0::GPIOG_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiog_hwcfgr0;
#[doc = "GPIOG_VERR register accessor: an alias for `Reg<GPIOG_VERR_SPEC>`"]
pub type GPIOG_VERR = crate::Reg<gpiog_verr::GPIOG_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpiog_verr;
#[doc = "GPIOG_IPIDR register accessor: an alias for `Reg<GPIOG_IPIDR_SPEC>`"]
pub type GPIOG_IPIDR = crate::Reg<gpiog_ipidr::GPIOG_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpiog_ipidr;
#[doc = "GPIOG_SIDR register accessor: an alias for `Reg<GPIOG_SIDR_SPEC>`"]
pub type GPIOG_SIDR = crate::Reg<gpiog_sidr::GPIOG_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpiog_sidr;
