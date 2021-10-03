#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioe_moder: crate::Reg<gpioe_moder::GPIOE_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioe_otyper: crate::Reg<gpioe_otyper::GPIOE_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioe_ospeedr: crate::Reg<gpioe_ospeedr::GPIOE_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioe_pupdr: crate::Reg<gpioe_pupdr::GPIOE_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioe_idr: crate::Reg<gpioe_idr::GPIOE_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioe_odr: crate::Reg<gpioe_odr::GPIOE_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioe_bsrr: crate::Reg<gpioe_bsrr::GPIOE_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioe_lckr: crate::Reg<gpioe_lckr::GPIOE_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioe_afrl: crate::Reg<gpioe_afrl::GPIOE_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioe_afrh: crate::Reg<gpioe_afrh::GPIOE_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioe_brr: crate::Reg<gpioe_brr::GPIOE_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioe_hwcfgr10: crate::Reg<gpioe_hwcfgr10::GPIOE_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioe_hwcfgr9: crate::Reg<gpioe_hwcfgr9::GPIOE_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioe_hwcfgr8: crate::Reg<gpioe_hwcfgr8::GPIOE_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioe_hwcfgr7: crate::Reg<gpioe_hwcfgr7::GPIOE_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioe_hwcfgr6: crate::Reg<gpioe_hwcfgr6::GPIOE_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioe_hwcfgr5: crate::Reg<gpioe_hwcfgr5::GPIOE_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioe_hwcfgr4: crate::Reg<gpioe_hwcfgr4::GPIOE_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioe_hwcfgr3: crate::Reg<gpioe_hwcfgr3::GPIOE_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioe_hwcfgr2: crate::Reg<gpioe_hwcfgr2::GPIOE_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioe_hwcfgr1: crate::Reg<gpioe_hwcfgr1::GPIOE_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioe_hwcfgr0: crate::Reg<gpioe_hwcfgr0::GPIOE_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioe_verr: crate::Reg<gpioe_verr::GPIOE_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioe_ipidr: crate::Reg<gpioe_ipidr::GPIOE_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioe_sidr: crate::Reg<gpioe_sidr::GPIOE_SIDR_SPEC>,
}
#[doc = "GPIOE_MODER register accessor: an alias for `Reg<GPIOE_MODER_SPEC>`"]
pub type GPIOE_MODER = crate::Reg<gpioe_moder::GPIOE_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioe_moder;
#[doc = "GPIOE_OTYPER register accessor: an alias for `Reg<GPIOE_OTYPER_SPEC>`"]
pub type GPIOE_OTYPER = crate::Reg<gpioe_otyper::GPIOE_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioe_otyper;
#[doc = "GPIOE_OSPEEDR register accessor: an alias for `Reg<GPIOE_OSPEEDR_SPEC>`"]
pub type GPIOE_OSPEEDR = crate::Reg<gpioe_ospeedr::GPIOE_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioe_ospeedr;
#[doc = "GPIOE_PUPDR register accessor: an alias for `Reg<GPIOE_PUPDR_SPEC>`"]
pub type GPIOE_PUPDR = crate::Reg<gpioe_pupdr::GPIOE_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioe_pupdr;
#[doc = "GPIOE_IDR register accessor: an alias for `Reg<GPIOE_IDR_SPEC>`"]
pub type GPIOE_IDR = crate::Reg<gpioe_idr::GPIOE_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioe_idr;
#[doc = "GPIOE_ODR register accessor: an alias for `Reg<GPIOE_ODR_SPEC>`"]
pub type GPIOE_ODR = crate::Reg<gpioe_odr::GPIOE_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioe_odr;
#[doc = "GPIOE_BSRR register accessor: an alias for `Reg<GPIOE_BSRR_SPEC>`"]
pub type GPIOE_BSRR = crate::Reg<gpioe_bsrr::GPIOE_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioe_bsrr;
#[doc = "GPIOE_LCKR register accessor: an alias for `Reg<GPIOE_LCKR_SPEC>`"]
pub type GPIOE_LCKR = crate::Reg<gpioe_lckr::GPIOE_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioe_lckr;
#[doc = "GPIOE_AFRL register accessor: an alias for `Reg<GPIOE_AFRL_SPEC>`"]
pub type GPIOE_AFRL = crate::Reg<gpioe_afrl::GPIOE_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioe_afrl;
#[doc = "GPIOE_AFRH register accessor: an alias for `Reg<GPIOE_AFRH_SPEC>`"]
pub type GPIOE_AFRH = crate::Reg<gpioe_afrh::GPIOE_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioe_afrh;
#[doc = "GPIOE_BRR register accessor: an alias for `Reg<GPIOE_BRR_SPEC>`"]
pub type GPIOE_BRR = crate::Reg<gpioe_brr::GPIOE_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioe_brr;
#[doc = "GPIOE_HWCFGR10 register accessor: an alias for `Reg<GPIOE_HWCFGR10_SPEC>`"]
pub type GPIOE_HWCFGR10 = crate::Reg<gpioe_hwcfgr10::GPIOE_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioe_hwcfgr10;
#[doc = "GPIOE_HWCFGR9 register accessor: an alias for `Reg<GPIOE_HWCFGR9_SPEC>`"]
pub type GPIOE_HWCFGR9 = crate::Reg<gpioe_hwcfgr9::GPIOE_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioe_hwcfgr9;
#[doc = "GPIOE_HWCFGR8 register accessor: an alias for `Reg<GPIOE_HWCFGR8_SPEC>`"]
pub type GPIOE_HWCFGR8 = crate::Reg<gpioe_hwcfgr8::GPIOE_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioe_hwcfgr8;
#[doc = "GPIOE_HWCFGR7 register accessor: an alias for `Reg<GPIOE_HWCFGR7_SPEC>`"]
pub type GPIOE_HWCFGR7 = crate::Reg<gpioe_hwcfgr7::GPIOE_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioe_hwcfgr7;
#[doc = "GPIOE_HWCFGR6 register accessor: an alias for `Reg<GPIOE_HWCFGR6_SPEC>`"]
pub type GPIOE_HWCFGR6 = crate::Reg<gpioe_hwcfgr6::GPIOE_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioe_hwcfgr6;
#[doc = "GPIOE_HWCFGR5 register accessor: an alias for `Reg<GPIOE_HWCFGR5_SPEC>`"]
pub type GPIOE_HWCFGR5 = crate::Reg<gpioe_hwcfgr5::GPIOE_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioe_hwcfgr5;
#[doc = "GPIOE_HWCFGR4 register accessor: an alias for `Reg<GPIOE_HWCFGR4_SPEC>`"]
pub type GPIOE_HWCFGR4 = crate::Reg<gpioe_hwcfgr4::GPIOE_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioe_hwcfgr4;
#[doc = "GPIOE_HWCFGR3 register accessor: an alias for `Reg<GPIOE_HWCFGR3_SPEC>`"]
pub type GPIOE_HWCFGR3 = crate::Reg<gpioe_hwcfgr3::GPIOE_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioe_hwcfgr3;
#[doc = "GPIOE_HWCFGR2 register accessor: an alias for `Reg<GPIOE_HWCFGR2_SPEC>`"]
pub type GPIOE_HWCFGR2 = crate::Reg<gpioe_hwcfgr2::GPIOE_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioe_hwcfgr2;
#[doc = "GPIOE_HWCFGR1 register accessor: an alias for `Reg<GPIOE_HWCFGR1_SPEC>`"]
pub type GPIOE_HWCFGR1 = crate::Reg<gpioe_hwcfgr1::GPIOE_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioe_hwcfgr1;
#[doc = "GPIOE_HWCFGR0 register accessor: an alias for `Reg<GPIOE_HWCFGR0_SPEC>`"]
pub type GPIOE_HWCFGR0 = crate::Reg<gpioe_hwcfgr0::GPIOE_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioe_hwcfgr0;
#[doc = "GPIOE_VERR register accessor: an alias for `Reg<GPIOE_VERR_SPEC>`"]
pub type GPIOE_VERR = crate::Reg<gpioe_verr::GPIOE_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioe_verr;
#[doc = "GPIOE_IPIDR register accessor: an alias for `Reg<GPIOE_IPIDR_SPEC>`"]
pub type GPIOE_IPIDR = crate::Reg<gpioe_ipidr::GPIOE_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioe_ipidr;
#[doc = "GPIOE_SIDR register accessor: an alias for `Reg<GPIOE_SIDR_SPEC>`"]
pub type GPIOE_SIDR = crate::Reg<gpioe_sidr::GPIOE_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioe_sidr;
