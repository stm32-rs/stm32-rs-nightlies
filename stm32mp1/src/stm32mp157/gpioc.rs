#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioc_moder: crate::Reg<gpioc_moder::GPIOC_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioc_otyper: crate::Reg<gpioc_otyper::GPIOC_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioc_ospeedr: crate::Reg<gpioc_ospeedr::GPIOC_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioc_pupdr: crate::Reg<gpioc_pupdr::GPIOC_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioc_idr: crate::Reg<gpioc_idr::GPIOC_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioc_odr: crate::Reg<gpioc_odr::GPIOC_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioc_bsrr: crate::Reg<gpioc_bsrr::GPIOC_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioc_lckr: crate::Reg<gpioc_lckr::GPIOC_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioc_afrl: crate::Reg<gpioc_afrl::GPIOC_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioc_afrh: crate::Reg<gpioc_afrh::GPIOC_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioc_brr: crate::Reg<gpioc_brr::GPIOC_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioc_hwcfgr10: crate::Reg<gpioc_hwcfgr10::GPIOC_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioc_hwcfgr9: crate::Reg<gpioc_hwcfgr9::GPIOC_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioc_hwcfgr8: crate::Reg<gpioc_hwcfgr8::GPIOC_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioc_hwcfgr7: crate::Reg<gpioc_hwcfgr7::GPIOC_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioc_hwcfgr6: crate::Reg<gpioc_hwcfgr6::GPIOC_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioc_hwcfgr5: crate::Reg<gpioc_hwcfgr5::GPIOC_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioc_hwcfgr4: crate::Reg<gpioc_hwcfgr4::GPIOC_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioc_hwcfgr3: crate::Reg<gpioc_hwcfgr3::GPIOC_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioc_hwcfgr2: crate::Reg<gpioc_hwcfgr2::GPIOC_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioc_hwcfgr1: crate::Reg<gpioc_hwcfgr1::GPIOC_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioc_hwcfgr0: crate::Reg<gpioc_hwcfgr0::GPIOC_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioc_verr: crate::Reg<gpioc_verr::GPIOC_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioc_ipidr: crate::Reg<gpioc_ipidr::GPIOC_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioc_sidr: crate::Reg<gpioc_sidr::GPIOC_SIDR_SPEC>,
}
#[doc = "GPIOC_MODER register accessor: an alias for `Reg<GPIOC_MODER_SPEC>`"]
pub type GPIOC_MODER = crate::Reg<gpioc_moder::GPIOC_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioc_moder;
#[doc = "GPIOC_OTYPER register accessor: an alias for `Reg<GPIOC_OTYPER_SPEC>`"]
pub type GPIOC_OTYPER = crate::Reg<gpioc_otyper::GPIOC_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioc_otyper;
#[doc = "GPIOC_OSPEEDR register accessor: an alias for `Reg<GPIOC_OSPEEDR_SPEC>`"]
pub type GPIOC_OSPEEDR = crate::Reg<gpioc_ospeedr::GPIOC_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioc_ospeedr;
#[doc = "GPIOC_PUPDR register accessor: an alias for `Reg<GPIOC_PUPDR_SPEC>`"]
pub type GPIOC_PUPDR = crate::Reg<gpioc_pupdr::GPIOC_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioc_pupdr;
#[doc = "GPIOC_IDR register accessor: an alias for `Reg<GPIOC_IDR_SPEC>`"]
pub type GPIOC_IDR = crate::Reg<gpioc_idr::GPIOC_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioc_idr;
#[doc = "GPIOC_ODR register accessor: an alias for `Reg<GPIOC_ODR_SPEC>`"]
pub type GPIOC_ODR = crate::Reg<gpioc_odr::GPIOC_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioc_odr;
#[doc = "GPIOC_BSRR register accessor: an alias for `Reg<GPIOC_BSRR_SPEC>`"]
pub type GPIOC_BSRR = crate::Reg<gpioc_bsrr::GPIOC_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioc_bsrr;
#[doc = "GPIOC_LCKR register accessor: an alias for `Reg<GPIOC_LCKR_SPEC>`"]
pub type GPIOC_LCKR = crate::Reg<gpioc_lckr::GPIOC_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioc_lckr;
#[doc = "GPIOC_AFRL register accessor: an alias for `Reg<GPIOC_AFRL_SPEC>`"]
pub type GPIOC_AFRL = crate::Reg<gpioc_afrl::GPIOC_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioc_afrl;
#[doc = "GPIOC_AFRH register accessor: an alias for `Reg<GPIOC_AFRH_SPEC>`"]
pub type GPIOC_AFRH = crate::Reg<gpioc_afrh::GPIOC_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioc_afrh;
#[doc = "GPIOC_BRR register accessor: an alias for `Reg<GPIOC_BRR_SPEC>`"]
pub type GPIOC_BRR = crate::Reg<gpioc_brr::GPIOC_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioc_brr;
#[doc = "GPIOC_HWCFGR10 register accessor: an alias for `Reg<GPIOC_HWCFGR10_SPEC>`"]
pub type GPIOC_HWCFGR10 = crate::Reg<gpioc_hwcfgr10::GPIOC_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioc_hwcfgr10;
#[doc = "GPIOC_HWCFGR9 register accessor: an alias for `Reg<GPIOC_HWCFGR9_SPEC>`"]
pub type GPIOC_HWCFGR9 = crate::Reg<gpioc_hwcfgr9::GPIOC_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioc_hwcfgr9;
#[doc = "GPIOC_HWCFGR8 register accessor: an alias for `Reg<GPIOC_HWCFGR8_SPEC>`"]
pub type GPIOC_HWCFGR8 = crate::Reg<gpioc_hwcfgr8::GPIOC_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioc_hwcfgr8;
#[doc = "GPIOC_HWCFGR7 register accessor: an alias for `Reg<GPIOC_HWCFGR7_SPEC>`"]
pub type GPIOC_HWCFGR7 = crate::Reg<gpioc_hwcfgr7::GPIOC_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioc_hwcfgr7;
#[doc = "GPIOC_HWCFGR6 register accessor: an alias for `Reg<GPIOC_HWCFGR6_SPEC>`"]
pub type GPIOC_HWCFGR6 = crate::Reg<gpioc_hwcfgr6::GPIOC_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioc_hwcfgr6;
#[doc = "GPIOC_HWCFGR5 register accessor: an alias for `Reg<GPIOC_HWCFGR5_SPEC>`"]
pub type GPIOC_HWCFGR5 = crate::Reg<gpioc_hwcfgr5::GPIOC_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioc_hwcfgr5;
#[doc = "GPIOC_HWCFGR4 register accessor: an alias for `Reg<GPIOC_HWCFGR4_SPEC>`"]
pub type GPIOC_HWCFGR4 = crate::Reg<gpioc_hwcfgr4::GPIOC_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioc_hwcfgr4;
#[doc = "GPIOC_HWCFGR3 register accessor: an alias for `Reg<GPIOC_HWCFGR3_SPEC>`"]
pub type GPIOC_HWCFGR3 = crate::Reg<gpioc_hwcfgr3::GPIOC_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioc_hwcfgr3;
#[doc = "GPIOC_HWCFGR2 register accessor: an alias for `Reg<GPIOC_HWCFGR2_SPEC>`"]
pub type GPIOC_HWCFGR2 = crate::Reg<gpioc_hwcfgr2::GPIOC_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioc_hwcfgr2;
#[doc = "GPIOC_HWCFGR1 register accessor: an alias for `Reg<GPIOC_HWCFGR1_SPEC>`"]
pub type GPIOC_HWCFGR1 = crate::Reg<gpioc_hwcfgr1::GPIOC_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioc_hwcfgr1;
#[doc = "GPIOC_HWCFGR0 register accessor: an alias for `Reg<GPIOC_HWCFGR0_SPEC>`"]
pub type GPIOC_HWCFGR0 = crate::Reg<gpioc_hwcfgr0::GPIOC_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioc_hwcfgr0;
#[doc = "GPIOC_VERR register accessor: an alias for `Reg<GPIOC_VERR_SPEC>`"]
pub type GPIOC_VERR = crate::Reg<gpioc_verr::GPIOC_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioc_verr;
#[doc = "GPIOC_IPIDR register accessor: an alias for `Reg<GPIOC_IPIDR_SPEC>`"]
pub type GPIOC_IPIDR = crate::Reg<gpioc_ipidr::GPIOC_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioc_ipidr;
#[doc = "GPIOC_SIDR register accessor: an alias for `Reg<GPIOC_SIDR_SPEC>`"]
pub type GPIOC_SIDR = crate::Reg<gpioc_sidr::GPIOC_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioc_sidr;
