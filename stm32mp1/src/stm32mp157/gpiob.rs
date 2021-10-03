#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiob_moder: crate::Reg<gpiob_moder::GPIOB_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiob_otyper: crate::Reg<gpiob_otyper::GPIOB_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiob_ospeedr: crate::Reg<gpiob_ospeedr::GPIOB_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiob_pupdr: crate::Reg<gpiob_pupdr::GPIOB_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiob_idr: crate::Reg<gpiob_idr::GPIOB_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiob_odr: crate::Reg<gpiob_odr::GPIOB_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiob_bsrr: crate::Reg<gpiob_bsrr::GPIOB_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiob_lckr: crate::Reg<gpiob_lckr::GPIOB_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiob_afrl: crate::Reg<gpiob_afrl::GPIOB_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiob_afrh: crate::Reg<gpiob_afrh::GPIOB_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiob_brr: crate::Reg<gpiob_brr::GPIOB_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiob_hwcfgr10: crate::Reg<gpiob_hwcfgr10::GPIOB_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiob_hwcfgr9: crate::Reg<gpiob_hwcfgr9::GPIOB_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiob_hwcfgr8: crate::Reg<gpiob_hwcfgr8::GPIOB_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiob_hwcfgr7: crate::Reg<gpiob_hwcfgr7::GPIOB_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiob_hwcfgr6: crate::Reg<gpiob_hwcfgr6::GPIOB_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiob_hwcfgr5: crate::Reg<gpiob_hwcfgr5::GPIOB_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiob_hwcfgr4: crate::Reg<gpiob_hwcfgr4::GPIOB_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiob_hwcfgr3: crate::Reg<gpiob_hwcfgr3::GPIOB_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiob_hwcfgr2: crate::Reg<gpiob_hwcfgr2::GPIOB_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiob_hwcfgr1: crate::Reg<gpiob_hwcfgr1::GPIOB_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiob_hwcfgr0: crate::Reg<gpiob_hwcfgr0::GPIOB_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiob_verr: crate::Reg<gpiob_verr::GPIOB_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiob_ipidr: crate::Reg<gpiob_ipidr::GPIOB_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiob_sidr: crate::Reg<gpiob_sidr::GPIOB_SIDR_SPEC>,
}
#[doc = "GPIOB_MODER register accessor: an alias for `Reg<GPIOB_MODER_SPEC>`"]
pub type GPIOB_MODER = crate::Reg<gpiob_moder::GPIOB_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpiob_moder;
#[doc = "GPIOB_OTYPER register accessor: an alias for `Reg<GPIOB_OTYPER_SPEC>`"]
pub type GPIOB_OTYPER = crate::Reg<gpiob_otyper::GPIOB_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpiob_otyper;
#[doc = "GPIOB_OSPEEDR register accessor: an alias for `Reg<GPIOB_OSPEEDR_SPEC>`"]
pub type GPIOB_OSPEEDR = crate::Reg<gpiob_ospeedr::GPIOB_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpiob_ospeedr;
#[doc = "GPIOB_PUPDR register accessor: an alias for `Reg<GPIOB_PUPDR_SPEC>`"]
pub type GPIOB_PUPDR = crate::Reg<gpiob_pupdr::GPIOB_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiob_pupdr;
#[doc = "GPIOB_IDR register accessor: an alias for `Reg<GPIOB_IDR_SPEC>`"]
pub type GPIOB_IDR = crate::Reg<gpiob_idr::GPIOB_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpiob_idr;
#[doc = "GPIOB_ODR register accessor: an alias for `Reg<GPIOB_ODR_SPEC>`"]
pub type GPIOB_ODR = crate::Reg<gpiob_odr::GPIOB_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpiob_odr;
#[doc = "GPIOB_BSRR register accessor: an alias for `Reg<GPIOB_BSRR_SPEC>`"]
pub type GPIOB_BSRR = crate::Reg<gpiob_bsrr::GPIOB_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiob_bsrr;
#[doc = "GPIOB_LCKR register accessor: an alias for `Reg<GPIOB_LCKR_SPEC>`"]
pub type GPIOB_LCKR = crate::Reg<gpiob_lckr::GPIOB_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiob_lckr;
#[doc = "GPIOB_AFRL register accessor: an alias for `Reg<GPIOB_AFRL_SPEC>`"]
pub type GPIOB_AFRL = crate::Reg<gpiob_afrl::GPIOB_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpiob_afrl;
#[doc = "GPIOB_AFRH register accessor: an alias for `Reg<GPIOB_AFRH_SPEC>`"]
pub type GPIOB_AFRH = crate::Reg<gpiob_afrh::GPIOB_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpiob_afrh;
#[doc = "GPIOB_BRR register accessor: an alias for `Reg<GPIOB_BRR_SPEC>`"]
pub type GPIOB_BRR = crate::Reg<gpiob_brr::GPIOB_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpiob_brr;
#[doc = "GPIOB_HWCFGR10 register accessor: an alias for `Reg<GPIOB_HWCFGR10_SPEC>`"]
pub type GPIOB_HWCFGR10 = crate::Reg<gpiob_hwcfgr10::GPIOB_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiob_hwcfgr10;
#[doc = "GPIOB_HWCFGR9 register accessor: an alias for `Reg<GPIOB_HWCFGR9_SPEC>`"]
pub type GPIOB_HWCFGR9 = crate::Reg<gpiob_hwcfgr9::GPIOB_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiob_hwcfgr9;
#[doc = "GPIOB_HWCFGR8 register accessor: an alias for `Reg<GPIOB_HWCFGR8_SPEC>`"]
pub type GPIOB_HWCFGR8 = crate::Reg<gpiob_hwcfgr8::GPIOB_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiob_hwcfgr8;
#[doc = "GPIOB_HWCFGR7 register accessor: an alias for `Reg<GPIOB_HWCFGR7_SPEC>`"]
pub type GPIOB_HWCFGR7 = crate::Reg<gpiob_hwcfgr7::GPIOB_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiob_hwcfgr7;
#[doc = "GPIOB_HWCFGR6 register accessor: an alias for `Reg<GPIOB_HWCFGR6_SPEC>`"]
pub type GPIOB_HWCFGR6 = crate::Reg<gpiob_hwcfgr6::GPIOB_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiob_hwcfgr6;
#[doc = "GPIOB_HWCFGR5 register accessor: an alias for `Reg<GPIOB_HWCFGR5_SPEC>`"]
pub type GPIOB_HWCFGR5 = crate::Reg<gpiob_hwcfgr5::GPIOB_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiob_hwcfgr5;
#[doc = "GPIOB_HWCFGR4 register accessor: an alias for `Reg<GPIOB_HWCFGR4_SPEC>`"]
pub type GPIOB_HWCFGR4 = crate::Reg<gpiob_hwcfgr4::GPIOB_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiob_hwcfgr4;
#[doc = "GPIOB_HWCFGR3 register accessor: an alias for `Reg<GPIOB_HWCFGR3_SPEC>`"]
pub type GPIOB_HWCFGR3 = crate::Reg<gpiob_hwcfgr3::GPIOB_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiob_hwcfgr3;
#[doc = "GPIOB_HWCFGR2 register accessor: an alias for `Reg<GPIOB_HWCFGR2_SPEC>`"]
pub type GPIOB_HWCFGR2 = crate::Reg<gpiob_hwcfgr2::GPIOB_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiob_hwcfgr2;
#[doc = "GPIOB_HWCFGR1 register accessor: an alias for `Reg<GPIOB_HWCFGR1_SPEC>`"]
pub type GPIOB_HWCFGR1 = crate::Reg<gpiob_hwcfgr1::GPIOB_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiob_hwcfgr1;
#[doc = "GPIOB_HWCFGR0 register accessor: an alias for `Reg<GPIOB_HWCFGR0_SPEC>`"]
pub type GPIOB_HWCFGR0 = crate::Reg<gpiob_hwcfgr0::GPIOB_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiob_hwcfgr0;
#[doc = "GPIOB_VERR register accessor: an alias for `Reg<GPIOB_VERR_SPEC>`"]
pub type GPIOB_VERR = crate::Reg<gpiob_verr::GPIOB_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpiob_verr;
#[doc = "GPIOB_IPIDR register accessor: an alias for `Reg<GPIOB_IPIDR_SPEC>`"]
pub type GPIOB_IPIDR = crate::Reg<gpiob_ipidr::GPIOB_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpiob_ipidr;
#[doc = "GPIOB_SIDR register accessor: an alias for `Reg<GPIOB_SIDR_SPEC>`"]
pub type GPIOB_SIDR = crate::Reg<gpiob_sidr::GPIOB_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpiob_sidr;
