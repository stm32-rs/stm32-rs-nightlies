#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiod_moder: crate::Reg<gpiod_moder::GPIOD_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiod_otyper: crate::Reg<gpiod_otyper::GPIOD_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiod_ospeedr: crate::Reg<gpiod_ospeedr::GPIOD_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiod_pupdr: crate::Reg<gpiod_pupdr::GPIOD_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiod_idr: crate::Reg<gpiod_idr::GPIOD_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiod_odr: crate::Reg<gpiod_odr::GPIOD_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiod_bsrr: crate::Reg<gpiod_bsrr::GPIOD_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiod_lckr: crate::Reg<gpiod_lckr::GPIOD_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiod_afrl: crate::Reg<gpiod_afrl::GPIOD_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiod_afrh: crate::Reg<gpiod_afrh::GPIOD_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiod_brr: crate::Reg<gpiod_brr::GPIOD_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiod_hwcfgr10: crate::Reg<gpiod_hwcfgr10::GPIOD_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiod_hwcfgr9: crate::Reg<gpiod_hwcfgr9::GPIOD_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiod_hwcfgr8: crate::Reg<gpiod_hwcfgr8::GPIOD_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiod_hwcfgr7: crate::Reg<gpiod_hwcfgr7::GPIOD_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiod_hwcfgr6: crate::Reg<gpiod_hwcfgr6::GPIOD_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiod_hwcfgr5: crate::Reg<gpiod_hwcfgr5::GPIOD_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiod_hwcfgr4: crate::Reg<gpiod_hwcfgr4::GPIOD_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiod_hwcfgr3: crate::Reg<gpiod_hwcfgr3::GPIOD_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiod_hwcfgr2: crate::Reg<gpiod_hwcfgr2::GPIOD_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiod_hwcfgr1: crate::Reg<gpiod_hwcfgr1::GPIOD_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiod_hwcfgr0: crate::Reg<gpiod_hwcfgr0::GPIOD_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiod_verr: crate::Reg<gpiod_verr::GPIOD_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiod_ipidr: crate::Reg<gpiod_ipidr::GPIOD_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiod_sidr: crate::Reg<gpiod_sidr::GPIOD_SIDR_SPEC>,
}
#[doc = "GPIOD_MODER register accessor: an alias for `Reg<GPIOD_MODER_SPEC>`"]
pub type GPIOD_MODER = crate::Reg<gpiod_moder::GPIOD_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpiod_moder;
#[doc = "GPIOD_OTYPER register accessor: an alias for `Reg<GPIOD_OTYPER_SPEC>`"]
pub type GPIOD_OTYPER = crate::Reg<gpiod_otyper::GPIOD_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpiod_otyper;
#[doc = "GPIOD_OSPEEDR register accessor: an alias for `Reg<GPIOD_OSPEEDR_SPEC>`"]
pub type GPIOD_OSPEEDR = crate::Reg<gpiod_ospeedr::GPIOD_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpiod_ospeedr;
#[doc = "GPIOD_PUPDR register accessor: an alias for `Reg<GPIOD_PUPDR_SPEC>`"]
pub type GPIOD_PUPDR = crate::Reg<gpiod_pupdr::GPIOD_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiod_pupdr;
#[doc = "GPIOD_IDR register accessor: an alias for `Reg<GPIOD_IDR_SPEC>`"]
pub type GPIOD_IDR = crate::Reg<gpiod_idr::GPIOD_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpiod_idr;
#[doc = "GPIOD_ODR register accessor: an alias for `Reg<GPIOD_ODR_SPEC>`"]
pub type GPIOD_ODR = crate::Reg<gpiod_odr::GPIOD_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpiod_odr;
#[doc = "GPIOD_BSRR register accessor: an alias for `Reg<GPIOD_BSRR_SPEC>`"]
pub type GPIOD_BSRR = crate::Reg<gpiod_bsrr::GPIOD_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiod_bsrr;
#[doc = "GPIOD_LCKR register accessor: an alias for `Reg<GPIOD_LCKR_SPEC>`"]
pub type GPIOD_LCKR = crate::Reg<gpiod_lckr::GPIOD_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiod_lckr;
#[doc = "GPIOD_AFRL register accessor: an alias for `Reg<GPIOD_AFRL_SPEC>`"]
pub type GPIOD_AFRL = crate::Reg<gpiod_afrl::GPIOD_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpiod_afrl;
#[doc = "GPIOD_AFRH register accessor: an alias for `Reg<GPIOD_AFRH_SPEC>`"]
pub type GPIOD_AFRH = crate::Reg<gpiod_afrh::GPIOD_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpiod_afrh;
#[doc = "GPIOD_BRR register accessor: an alias for `Reg<GPIOD_BRR_SPEC>`"]
pub type GPIOD_BRR = crate::Reg<gpiod_brr::GPIOD_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpiod_brr;
#[doc = "GPIOD_HWCFGR10 register accessor: an alias for `Reg<GPIOD_HWCFGR10_SPEC>`"]
pub type GPIOD_HWCFGR10 = crate::Reg<gpiod_hwcfgr10::GPIOD_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiod_hwcfgr10;
#[doc = "GPIOD_HWCFGR9 register accessor: an alias for `Reg<GPIOD_HWCFGR9_SPEC>`"]
pub type GPIOD_HWCFGR9 = crate::Reg<gpiod_hwcfgr9::GPIOD_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiod_hwcfgr9;
#[doc = "GPIOD_HWCFGR8 register accessor: an alias for `Reg<GPIOD_HWCFGR8_SPEC>`"]
pub type GPIOD_HWCFGR8 = crate::Reg<gpiod_hwcfgr8::GPIOD_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiod_hwcfgr8;
#[doc = "GPIOD_HWCFGR7 register accessor: an alias for `Reg<GPIOD_HWCFGR7_SPEC>`"]
pub type GPIOD_HWCFGR7 = crate::Reg<gpiod_hwcfgr7::GPIOD_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiod_hwcfgr7;
#[doc = "GPIOD_HWCFGR6 register accessor: an alias for `Reg<GPIOD_HWCFGR6_SPEC>`"]
pub type GPIOD_HWCFGR6 = crate::Reg<gpiod_hwcfgr6::GPIOD_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiod_hwcfgr6;
#[doc = "GPIOD_HWCFGR5 register accessor: an alias for `Reg<GPIOD_HWCFGR5_SPEC>`"]
pub type GPIOD_HWCFGR5 = crate::Reg<gpiod_hwcfgr5::GPIOD_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiod_hwcfgr5;
#[doc = "GPIOD_HWCFGR4 register accessor: an alias for `Reg<GPIOD_HWCFGR4_SPEC>`"]
pub type GPIOD_HWCFGR4 = crate::Reg<gpiod_hwcfgr4::GPIOD_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiod_hwcfgr4;
#[doc = "GPIOD_HWCFGR3 register accessor: an alias for `Reg<GPIOD_HWCFGR3_SPEC>`"]
pub type GPIOD_HWCFGR3 = crate::Reg<gpiod_hwcfgr3::GPIOD_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiod_hwcfgr3;
#[doc = "GPIOD_HWCFGR2 register accessor: an alias for `Reg<GPIOD_HWCFGR2_SPEC>`"]
pub type GPIOD_HWCFGR2 = crate::Reg<gpiod_hwcfgr2::GPIOD_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiod_hwcfgr2;
#[doc = "GPIOD_HWCFGR1 register accessor: an alias for `Reg<GPIOD_HWCFGR1_SPEC>`"]
pub type GPIOD_HWCFGR1 = crate::Reg<gpiod_hwcfgr1::GPIOD_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiod_hwcfgr1;
#[doc = "GPIOD_HWCFGR0 register accessor: an alias for `Reg<GPIOD_HWCFGR0_SPEC>`"]
pub type GPIOD_HWCFGR0 = crate::Reg<gpiod_hwcfgr0::GPIOD_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiod_hwcfgr0;
#[doc = "GPIOD_VERR register accessor: an alias for `Reg<GPIOD_VERR_SPEC>`"]
pub type GPIOD_VERR = crate::Reg<gpiod_verr::GPIOD_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpiod_verr;
#[doc = "GPIOD_IPIDR register accessor: an alias for `Reg<GPIOD_IPIDR_SPEC>`"]
pub type GPIOD_IPIDR = crate::Reg<gpiod_ipidr::GPIOD_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpiod_ipidr;
#[doc = "GPIOD_SIDR register accessor: an alias for `Reg<GPIOD_SIDR_SPEC>`"]
pub type GPIOD_SIDR = crate::Reg<gpiod_sidr::GPIOD_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpiod_sidr;
