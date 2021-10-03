#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiok_moder: crate::Reg<gpiok_moder::GPIOK_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiok_otyper: crate::Reg<gpiok_otyper::GPIOK_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiok_ospeedr: crate::Reg<gpiok_ospeedr::GPIOK_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiok_pupdr: crate::Reg<gpiok_pupdr::GPIOK_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiok_idr: crate::Reg<gpiok_idr::GPIOK_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiok_odr: crate::Reg<gpiok_odr::GPIOK_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiok_bsrr: crate::Reg<gpiok_bsrr::GPIOK_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiok_lckr: crate::Reg<gpiok_lckr::GPIOK_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiok_afrl: crate::Reg<gpiok_afrl::GPIOK_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiok_afrh: crate::Reg<gpiok_afrh::GPIOK_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiok_brr: crate::Reg<gpiok_brr::GPIOK_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiok_hwcfgr10: crate::Reg<gpiok_hwcfgr10::GPIOK_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiok_hwcfgr9: crate::Reg<gpiok_hwcfgr9::GPIOK_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiok_hwcfgr8: crate::Reg<gpiok_hwcfgr8::GPIOK_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiok_hwcfgr7: crate::Reg<gpiok_hwcfgr7::GPIOK_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiok_hwcfgr6: crate::Reg<gpiok_hwcfgr6::GPIOK_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiok_hwcfgr5: crate::Reg<gpiok_hwcfgr5::GPIOK_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiok_hwcfgr4: crate::Reg<gpiok_hwcfgr4::GPIOK_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiok_hwcfgr3: crate::Reg<gpiok_hwcfgr3::GPIOK_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiok_hwcfgr2: crate::Reg<gpiok_hwcfgr2::GPIOK_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiok_hwcfgr1: crate::Reg<gpiok_hwcfgr1::GPIOK_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiok_hwcfgr0: crate::Reg<gpiok_hwcfgr0::GPIOK_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiok_verr: crate::Reg<gpiok_verr::GPIOK_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiok_ipidr: crate::Reg<gpiok_ipidr::GPIOK_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiok_sidr: crate::Reg<gpiok_sidr::GPIOK_SIDR_SPEC>,
}
#[doc = "GPIOK_MODER register accessor: an alias for `Reg<GPIOK_MODER_SPEC>`"]
pub type GPIOK_MODER = crate::Reg<gpiok_moder::GPIOK_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpiok_moder;
#[doc = "GPIOK_OTYPER register accessor: an alias for `Reg<GPIOK_OTYPER_SPEC>`"]
pub type GPIOK_OTYPER = crate::Reg<gpiok_otyper::GPIOK_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpiok_otyper;
#[doc = "GPIOK_OSPEEDR register accessor: an alias for `Reg<GPIOK_OSPEEDR_SPEC>`"]
pub type GPIOK_OSPEEDR = crate::Reg<gpiok_ospeedr::GPIOK_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpiok_ospeedr;
#[doc = "GPIOK_PUPDR register accessor: an alias for `Reg<GPIOK_PUPDR_SPEC>`"]
pub type GPIOK_PUPDR = crate::Reg<gpiok_pupdr::GPIOK_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiok_pupdr;
#[doc = "GPIOK_IDR register accessor: an alias for `Reg<GPIOK_IDR_SPEC>`"]
pub type GPIOK_IDR = crate::Reg<gpiok_idr::GPIOK_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpiok_idr;
#[doc = "GPIOK_ODR register accessor: an alias for `Reg<GPIOK_ODR_SPEC>`"]
pub type GPIOK_ODR = crate::Reg<gpiok_odr::GPIOK_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpiok_odr;
#[doc = "GPIOK_BSRR register accessor: an alias for `Reg<GPIOK_BSRR_SPEC>`"]
pub type GPIOK_BSRR = crate::Reg<gpiok_bsrr::GPIOK_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiok_bsrr;
#[doc = "GPIOK_LCKR register accessor: an alias for `Reg<GPIOK_LCKR_SPEC>`"]
pub type GPIOK_LCKR = crate::Reg<gpiok_lckr::GPIOK_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiok_lckr;
#[doc = "GPIOK_AFRL register accessor: an alias for `Reg<GPIOK_AFRL_SPEC>`"]
pub type GPIOK_AFRL = crate::Reg<gpiok_afrl::GPIOK_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpiok_afrl;
#[doc = "GPIOK_AFRH register accessor: an alias for `Reg<GPIOK_AFRH_SPEC>`"]
pub type GPIOK_AFRH = crate::Reg<gpiok_afrh::GPIOK_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpiok_afrh;
#[doc = "GPIOK_BRR register accessor: an alias for `Reg<GPIOK_BRR_SPEC>`"]
pub type GPIOK_BRR = crate::Reg<gpiok_brr::GPIOK_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpiok_brr;
#[doc = "GPIOK_HWCFGR10 register accessor: an alias for `Reg<GPIOK_HWCFGR10_SPEC>`"]
pub type GPIOK_HWCFGR10 = crate::Reg<gpiok_hwcfgr10::GPIOK_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiok_hwcfgr10;
#[doc = "GPIOK_HWCFGR9 register accessor: an alias for `Reg<GPIOK_HWCFGR9_SPEC>`"]
pub type GPIOK_HWCFGR9 = crate::Reg<gpiok_hwcfgr9::GPIOK_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiok_hwcfgr9;
#[doc = "GPIOK_HWCFGR8 register accessor: an alias for `Reg<GPIOK_HWCFGR8_SPEC>`"]
pub type GPIOK_HWCFGR8 = crate::Reg<gpiok_hwcfgr8::GPIOK_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiok_hwcfgr8;
#[doc = "GPIOK_HWCFGR7 register accessor: an alias for `Reg<GPIOK_HWCFGR7_SPEC>`"]
pub type GPIOK_HWCFGR7 = crate::Reg<gpiok_hwcfgr7::GPIOK_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiok_hwcfgr7;
#[doc = "GPIOK_HWCFGR6 register accessor: an alias for `Reg<GPIOK_HWCFGR6_SPEC>`"]
pub type GPIOK_HWCFGR6 = crate::Reg<gpiok_hwcfgr6::GPIOK_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiok_hwcfgr6;
#[doc = "GPIOK_HWCFGR5 register accessor: an alias for `Reg<GPIOK_HWCFGR5_SPEC>`"]
pub type GPIOK_HWCFGR5 = crate::Reg<gpiok_hwcfgr5::GPIOK_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiok_hwcfgr5;
#[doc = "GPIOK_HWCFGR4 register accessor: an alias for `Reg<GPIOK_HWCFGR4_SPEC>`"]
pub type GPIOK_HWCFGR4 = crate::Reg<gpiok_hwcfgr4::GPIOK_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiok_hwcfgr4;
#[doc = "GPIOK_HWCFGR3 register accessor: an alias for `Reg<GPIOK_HWCFGR3_SPEC>`"]
pub type GPIOK_HWCFGR3 = crate::Reg<gpiok_hwcfgr3::GPIOK_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiok_hwcfgr3;
#[doc = "GPIOK_HWCFGR2 register accessor: an alias for `Reg<GPIOK_HWCFGR2_SPEC>`"]
pub type GPIOK_HWCFGR2 = crate::Reg<gpiok_hwcfgr2::GPIOK_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiok_hwcfgr2;
#[doc = "GPIOK_HWCFGR1 register accessor: an alias for `Reg<GPIOK_HWCFGR1_SPEC>`"]
pub type GPIOK_HWCFGR1 = crate::Reg<gpiok_hwcfgr1::GPIOK_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiok_hwcfgr1;
#[doc = "GPIOK_HWCFGR0 register accessor: an alias for `Reg<GPIOK_HWCFGR0_SPEC>`"]
pub type GPIOK_HWCFGR0 = crate::Reg<gpiok_hwcfgr0::GPIOK_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiok_hwcfgr0;
#[doc = "GPIOK_VERR register accessor: an alias for `Reg<GPIOK_VERR_SPEC>`"]
pub type GPIOK_VERR = crate::Reg<gpiok_verr::GPIOK_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpiok_verr;
#[doc = "GPIOK_IPIDR register accessor: an alias for `Reg<GPIOK_IPIDR_SPEC>`"]
pub type GPIOK_IPIDR = crate::Reg<gpiok_ipidr::GPIOK_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpiok_ipidr;
#[doc = "GPIOK_SIDR register accessor: an alias for `Reg<GPIOK_SIDR_SPEC>`"]
pub type GPIOK_SIDR = crate::Reg<gpiok_sidr::GPIOK_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpiok_sidr;
