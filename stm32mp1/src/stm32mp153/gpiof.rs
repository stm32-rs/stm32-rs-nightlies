#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiof_moder: crate::Reg<gpiof_moder::GPIOF_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiof_otyper: crate::Reg<gpiof_otyper::GPIOF_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiof_ospeedr: crate::Reg<gpiof_ospeedr::GPIOF_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiof_pupdr: crate::Reg<gpiof_pupdr::GPIOF_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiof_idr: crate::Reg<gpiof_idr::GPIOF_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiof_odr: crate::Reg<gpiof_odr::GPIOF_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiof_bsrr: crate::Reg<gpiof_bsrr::GPIOF_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiof_lckr: crate::Reg<gpiof_lckr::GPIOF_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiof_afrl: crate::Reg<gpiof_afrl::GPIOF_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiof_afrh: crate::Reg<gpiof_afrh::GPIOF_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpiof_brr: crate::Reg<gpiof_brr::GPIOF_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpiof_hwcfgr10: crate::Reg<gpiof_hwcfgr10::GPIOF_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiof_hwcfgr9: crate::Reg<gpiof_hwcfgr9::GPIOF_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpiof_hwcfgr8: crate::Reg<gpiof_hwcfgr8::GPIOF_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpiof_hwcfgr7: crate::Reg<gpiof_hwcfgr7::GPIOF_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpiof_hwcfgr6: crate::Reg<gpiof_hwcfgr6::GPIOF_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpiof_hwcfgr5: crate::Reg<gpiof_hwcfgr5::GPIOF_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpiof_hwcfgr4: crate::Reg<gpiof_hwcfgr4::GPIOF_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpiof_hwcfgr3: crate::Reg<gpiof_hwcfgr3::GPIOF_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpiof_hwcfgr2: crate::Reg<gpiof_hwcfgr2::GPIOF_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpiof_hwcfgr1: crate::Reg<gpiof_hwcfgr1::GPIOF_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpiof_hwcfgr0: crate::Reg<gpiof_hwcfgr0::GPIOF_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiof_verr: crate::Reg<gpiof_verr::GPIOF_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiof_ipidr: crate::Reg<gpiof_ipidr::GPIOF_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiof_sidr: crate::Reg<gpiof_sidr::GPIOF_SIDR_SPEC>,
}
#[doc = "GPIOF_MODER register accessor: an alias for `Reg<GPIOF_MODER_SPEC>`"]
pub type GPIOF_MODER = crate::Reg<gpiof_moder::GPIOF_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpiof_moder;
#[doc = "GPIOF_OTYPER register accessor: an alias for `Reg<GPIOF_OTYPER_SPEC>`"]
pub type GPIOF_OTYPER = crate::Reg<gpiof_otyper::GPIOF_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpiof_otyper;
#[doc = "GPIOF_OSPEEDR register accessor: an alias for `Reg<GPIOF_OSPEEDR_SPEC>`"]
pub type GPIOF_OSPEEDR = crate::Reg<gpiof_ospeedr::GPIOF_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpiof_ospeedr;
#[doc = "GPIOF_PUPDR register accessor: an alias for `Reg<GPIOF_PUPDR_SPEC>`"]
pub type GPIOF_PUPDR = crate::Reg<gpiof_pupdr::GPIOF_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiof_pupdr;
#[doc = "GPIOF_IDR register accessor: an alias for `Reg<GPIOF_IDR_SPEC>`"]
pub type GPIOF_IDR = crate::Reg<gpiof_idr::GPIOF_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpiof_idr;
#[doc = "GPIOF_ODR register accessor: an alias for `Reg<GPIOF_ODR_SPEC>`"]
pub type GPIOF_ODR = crate::Reg<gpiof_odr::GPIOF_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpiof_odr;
#[doc = "GPIOF_BSRR register accessor: an alias for `Reg<GPIOF_BSRR_SPEC>`"]
pub type GPIOF_BSRR = crate::Reg<gpiof_bsrr::GPIOF_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpiof_bsrr;
#[doc = "GPIOF_LCKR register accessor: an alias for `Reg<GPIOF_LCKR_SPEC>`"]
pub type GPIOF_LCKR = crate::Reg<gpiof_lckr::GPIOF_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiof_lckr;
#[doc = "GPIOF_AFRL register accessor: an alias for `Reg<GPIOF_AFRL_SPEC>`"]
pub type GPIOF_AFRL = crate::Reg<gpiof_afrl::GPIOF_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpiof_afrl;
#[doc = "GPIOF_AFRH register accessor: an alias for `Reg<GPIOF_AFRH_SPEC>`"]
pub type GPIOF_AFRH = crate::Reg<gpiof_afrh::GPIOF_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpiof_afrh;
#[doc = "GPIOF_BRR register accessor: an alias for `Reg<GPIOF_BRR_SPEC>`"]
pub type GPIOF_BRR = crate::Reg<gpiof_brr::GPIOF_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpiof_brr;
#[doc = "GPIOF_HWCFGR10 register accessor: an alias for `Reg<GPIOF_HWCFGR10_SPEC>`"]
pub type GPIOF_HWCFGR10 = crate::Reg<gpiof_hwcfgr10::GPIOF_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpiof_hwcfgr10;
#[doc = "GPIOF_HWCFGR9 register accessor: an alias for `Reg<GPIOF_HWCFGR9_SPEC>`"]
pub type GPIOF_HWCFGR9 = crate::Reg<gpiof_hwcfgr9::GPIOF_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiof_hwcfgr9;
#[doc = "GPIOF_HWCFGR8 register accessor: an alias for `Reg<GPIOF_HWCFGR8_SPEC>`"]
pub type GPIOF_HWCFGR8 = crate::Reg<gpiof_hwcfgr8::GPIOF_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpiof_hwcfgr8;
#[doc = "GPIOF_HWCFGR7 register accessor: an alias for `Reg<GPIOF_HWCFGR7_SPEC>`"]
pub type GPIOF_HWCFGR7 = crate::Reg<gpiof_hwcfgr7::GPIOF_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpiof_hwcfgr7;
#[doc = "GPIOF_HWCFGR6 register accessor: an alias for `Reg<GPIOF_HWCFGR6_SPEC>`"]
pub type GPIOF_HWCFGR6 = crate::Reg<gpiof_hwcfgr6::GPIOF_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpiof_hwcfgr6;
#[doc = "GPIOF_HWCFGR5 register accessor: an alias for `Reg<GPIOF_HWCFGR5_SPEC>`"]
pub type GPIOF_HWCFGR5 = crate::Reg<gpiof_hwcfgr5::GPIOF_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpiof_hwcfgr5;
#[doc = "GPIOF_HWCFGR4 register accessor: an alias for `Reg<GPIOF_HWCFGR4_SPEC>`"]
pub type GPIOF_HWCFGR4 = crate::Reg<gpiof_hwcfgr4::GPIOF_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpiof_hwcfgr4;
#[doc = "GPIOF_HWCFGR3 register accessor: an alias for `Reg<GPIOF_HWCFGR3_SPEC>`"]
pub type GPIOF_HWCFGR3 = crate::Reg<gpiof_hwcfgr3::GPIOF_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpiof_hwcfgr3;
#[doc = "GPIOF_HWCFGR2 register accessor: an alias for `Reg<GPIOF_HWCFGR2_SPEC>`"]
pub type GPIOF_HWCFGR2 = crate::Reg<gpiof_hwcfgr2::GPIOF_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpiof_hwcfgr2;
#[doc = "GPIOF_HWCFGR1 register accessor: an alias for `Reg<GPIOF_HWCFGR1_SPEC>`"]
pub type GPIOF_HWCFGR1 = crate::Reg<gpiof_hwcfgr1::GPIOF_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpiof_hwcfgr1;
#[doc = "GPIOF_HWCFGR0 register accessor: an alias for `Reg<GPIOF_HWCFGR0_SPEC>`"]
pub type GPIOF_HWCFGR0 = crate::Reg<gpiof_hwcfgr0::GPIOF_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpiof_hwcfgr0;
#[doc = "GPIOF_VERR register accessor: an alias for `Reg<GPIOF_VERR_SPEC>`"]
pub type GPIOF_VERR = crate::Reg<gpiof_verr::GPIOF_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpiof_verr;
#[doc = "GPIOF_IPIDR register accessor: an alias for `Reg<GPIOF_IPIDR_SPEC>`"]
pub type GPIOF_IPIDR = crate::Reg<gpiof_ipidr::GPIOF_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpiof_ipidr;
#[doc = "GPIOF_SIDR register accessor: an alias for `Reg<GPIOF_SIDR_SPEC>`"]
pub type GPIOF_SIDR = crate::Reg<gpiof_sidr::GPIOF_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpiof_sidr;
