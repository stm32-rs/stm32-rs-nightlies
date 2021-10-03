#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioj_moder: crate::Reg<gpioj_moder::GPIOJ_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioj_otyper: crate::Reg<gpioj_otyper::GPIOJ_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioj_ospeedr: crate::Reg<gpioj_ospeedr::GPIOJ_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioj_pupdr: crate::Reg<gpioj_pupdr::GPIOJ_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioj_idr: crate::Reg<gpioj_idr::GPIOJ_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioj_odr: crate::Reg<gpioj_odr::GPIOJ_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioj_bsrr: crate::Reg<gpioj_bsrr::GPIOJ_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioj_lckr: crate::Reg<gpioj_lckr::GPIOJ_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioj_afrl: crate::Reg<gpioj_afrl::GPIOJ_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioj_afrh: crate::Reg<gpioj_afrh::GPIOJ_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioj_brr: crate::Reg<gpioj_brr::GPIOJ_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioj_hwcfgr10: crate::Reg<gpioj_hwcfgr10::GPIOJ_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioj_hwcfgr9: crate::Reg<gpioj_hwcfgr9::GPIOJ_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioj_hwcfgr8: crate::Reg<gpioj_hwcfgr8::GPIOJ_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioj_hwcfgr7: crate::Reg<gpioj_hwcfgr7::GPIOJ_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioj_hwcfgr6: crate::Reg<gpioj_hwcfgr6::GPIOJ_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioj_hwcfgr5: crate::Reg<gpioj_hwcfgr5::GPIOJ_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioj_hwcfgr4: crate::Reg<gpioj_hwcfgr4::GPIOJ_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioj_hwcfgr3: crate::Reg<gpioj_hwcfgr3::GPIOJ_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioj_hwcfgr2: crate::Reg<gpioj_hwcfgr2::GPIOJ_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioj_hwcfgr1: crate::Reg<gpioj_hwcfgr1::GPIOJ_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioj_hwcfgr0: crate::Reg<gpioj_hwcfgr0::GPIOJ_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioj_verr: crate::Reg<gpioj_verr::GPIOJ_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioj_ipidr: crate::Reg<gpioj_ipidr::GPIOJ_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioj_sidr: crate::Reg<gpioj_sidr::GPIOJ_SIDR_SPEC>,
}
#[doc = "GPIOJ_MODER register accessor: an alias for `Reg<GPIOJ_MODER_SPEC>`"]
pub type GPIOJ_MODER = crate::Reg<gpioj_moder::GPIOJ_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioj_moder;
#[doc = "GPIOJ_OTYPER register accessor: an alias for `Reg<GPIOJ_OTYPER_SPEC>`"]
pub type GPIOJ_OTYPER = crate::Reg<gpioj_otyper::GPIOJ_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioj_otyper;
#[doc = "GPIOJ_OSPEEDR register accessor: an alias for `Reg<GPIOJ_OSPEEDR_SPEC>`"]
pub type GPIOJ_OSPEEDR = crate::Reg<gpioj_ospeedr::GPIOJ_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioj_ospeedr;
#[doc = "GPIOJ_PUPDR register accessor: an alias for `Reg<GPIOJ_PUPDR_SPEC>`"]
pub type GPIOJ_PUPDR = crate::Reg<gpioj_pupdr::GPIOJ_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioj_pupdr;
#[doc = "GPIOJ_IDR register accessor: an alias for `Reg<GPIOJ_IDR_SPEC>`"]
pub type GPIOJ_IDR = crate::Reg<gpioj_idr::GPIOJ_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioj_idr;
#[doc = "GPIOJ_ODR register accessor: an alias for `Reg<GPIOJ_ODR_SPEC>`"]
pub type GPIOJ_ODR = crate::Reg<gpioj_odr::GPIOJ_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioj_odr;
#[doc = "GPIOJ_BSRR register accessor: an alias for `Reg<GPIOJ_BSRR_SPEC>`"]
pub type GPIOJ_BSRR = crate::Reg<gpioj_bsrr::GPIOJ_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioj_bsrr;
#[doc = "GPIOJ_LCKR register accessor: an alias for `Reg<GPIOJ_LCKR_SPEC>`"]
pub type GPIOJ_LCKR = crate::Reg<gpioj_lckr::GPIOJ_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioj_lckr;
#[doc = "GPIOJ_AFRL register accessor: an alias for `Reg<GPIOJ_AFRL_SPEC>`"]
pub type GPIOJ_AFRL = crate::Reg<gpioj_afrl::GPIOJ_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioj_afrl;
#[doc = "GPIOJ_AFRH register accessor: an alias for `Reg<GPIOJ_AFRH_SPEC>`"]
pub type GPIOJ_AFRH = crate::Reg<gpioj_afrh::GPIOJ_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioj_afrh;
#[doc = "GPIOJ_BRR register accessor: an alias for `Reg<GPIOJ_BRR_SPEC>`"]
pub type GPIOJ_BRR = crate::Reg<gpioj_brr::GPIOJ_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioj_brr;
#[doc = "GPIOJ_HWCFGR10 register accessor: an alias for `Reg<GPIOJ_HWCFGR10_SPEC>`"]
pub type GPIOJ_HWCFGR10 = crate::Reg<gpioj_hwcfgr10::GPIOJ_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioj_hwcfgr10;
#[doc = "GPIOJ_HWCFGR9 register accessor: an alias for `Reg<GPIOJ_HWCFGR9_SPEC>`"]
pub type GPIOJ_HWCFGR9 = crate::Reg<gpioj_hwcfgr9::GPIOJ_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioj_hwcfgr9;
#[doc = "GPIOJ_HWCFGR8 register accessor: an alias for `Reg<GPIOJ_HWCFGR8_SPEC>`"]
pub type GPIOJ_HWCFGR8 = crate::Reg<gpioj_hwcfgr8::GPIOJ_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioj_hwcfgr8;
#[doc = "GPIOJ_HWCFGR7 register accessor: an alias for `Reg<GPIOJ_HWCFGR7_SPEC>`"]
pub type GPIOJ_HWCFGR7 = crate::Reg<gpioj_hwcfgr7::GPIOJ_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioj_hwcfgr7;
#[doc = "GPIOJ_HWCFGR6 register accessor: an alias for `Reg<GPIOJ_HWCFGR6_SPEC>`"]
pub type GPIOJ_HWCFGR6 = crate::Reg<gpioj_hwcfgr6::GPIOJ_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioj_hwcfgr6;
#[doc = "GPIOJ_HWCFGR5 register accessor: an alias for `Reg<GPIOJ_HWCFGR5_SPEC>`"]
pub type GPIOJ_HWCFGR5 = crate::Reg<gpioj_hwcfgr5::GPIOJ_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioj_hwcfgr5;
#[doc = "GPIOJ_HWCFGR4 register accessor: an alias for `Reg<GPIOJ_HWCFGR4_SPEC>`"]
pub type GPIOJ_HWCFGR4 = crate::Reg<gpioj_hwcfgr4::GPIOJ_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioj_hwcfgr4;
#[doc = "GPIOJ_HWCFGR3 register accessor: an alias for `Reg<GPIOJ_HWCFGR3_SPEC>`"]
pub type GPIOJ_HWCFGR3 = crate::Reg<gpioj_hwcfgr3::GPIOJ_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioj_hwcfgr3;
#[doc = "GPIOJ_HWCFGR2 register accessor: an alias for `Reg<GPIOJ_HWCFGR2_SPEC>`"]
pub type GPIOJ_HWCFGR2 = crate::Reg<gpioj_hwcfgr2::GPIOJ_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioj_hwcfgr2;
#[doc = "GPIOJ_HWCFGR1 register accessor: an alias for `Reg<GPIOJ_HWCFGR1_SPEC>`"]
pub type GPIOJ_HWCFGR1 = crate::Reg<gpioj_hwcfgr1::GPIOJ_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioj_hwcfgr1;
#[doc = "GPIOJ_HWCFGR0 register accessor: an alias for `Reg<GPIOJ_HWCFGR0_SPEC>`"]
pub type GPIOJ_HWCFGR0 = crate::Reg<gpioj_hwcfgr0::GPIOJ_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioj_hwcfgr0;
#[doc = "GPIOJ_VERR register accessor: an alias for `Reg<GPIOJ_VERR_SPEC>`"]
pub type GPIOJ_VERR = crate::Reg<gpioj_verr::GPIOJ_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioj_verr;
#[doc = "GPIOJ_IPIDR register accessor: an alias for `Reg<GPIOJ_IPIDR_SPEC>`"]
pub type GPIOJ_IPIDR = crate::Reg<gpioj_ipidr::GPIOJ_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioj_ipidr;
#[doc = "GPIOJ_SIDR register accessor: an alias for `Reg<GPIOJ_SIDR_SPEC>`"]
pub type GPIOJ_SIDR = crate::Reg<gpioj_sidr::GPIOJ_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioj_sidr;
