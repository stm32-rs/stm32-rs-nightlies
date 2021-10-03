#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpioh_moder: crate::Reg<gpioh_moder::GPIOH_MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpioh_otyper: crate::Reg<gpioh_otyper::GPIOH_OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpioh_ospeedr: crate::Reg<gpioh_ospeedr::GPIOH_OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpioh_pupdr: crate::Reg<gpioh_pupdr::GPIOH_PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpioh_idr: crate::Reg<gpioh_idr::GPIOH_IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpioh_odr: crate::Reg<gpioh_odr::GPIOH_ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpioh_bsrr: crate::Reg<gpioh_bsrr::GPIOH_BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpioh_lckr: crate::Reg<gpioh_lckr::GPIOH_LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpioh_afrl: crate::Reg<gpioh_afrl::GPIOH_AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpioh_afrh: crate::Reg<gpioh_afrh::GPIOH_AFRH_SPEC>,
    #[doc = "0x28 - GPIO port bit reset register"]
    pub gpioh_brr: crate::Reg<gpioh_brr::GPIOH_BRR_SPEC>,
    _reserved11: [u8; 0x039c],
    #[doc = "0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
    pub gpioh_hwcfgr10: crate::Reg<gpioh_hwcfgr10::GPIOH_HWCFGR10_SPEC>,
    #[doc = "0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioh_hwcfgr9: crate::Reg<gpioh_hwcfgr9::GPIOH_HWCFGR9_SPEC>,
    #[doc = "0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
    pub gpioh_hwcfgr8: crate::Reg<gpioh_hwcfgr8::GPIOH_HWCFGR8_SPEC>,
    #[doc = "0x3d4 - GPIO hardware configuration register 7"]
    pub gpioh_hwcfgr7: crate::Reg<gpioh_hwcfgr7::GPIOH_HWCFGR7_SPEC>,
    #[doc = "0x3d8 - GPIO hardware configuration register 6"]
    pub gpioh_hwcfgr6: crate::Reg<gpioh_hwcfgr6::GPIOH_HWCFGR6_SPEC>,
    #[doc = "0x3dc - GPIO hardware configuration register 5"]
    pub gpioh_hwcfgr5: crate::Reg<gpioh_hwcfgr5::GPIOH_HWCFGR5_SPEC>,
    #[doc = "0x3e0 - GPIO hardware configuration register 4"]
    pub gpioh_hwcfgr4: crate::Reg<gpioh_hwcfgr4::GPIOH_HWCFGR4_SPEC>,
    #[doc = "0x3e4 - GPIO hardware configuration register 3"]
    pub gpioh_hwcfgr3: crate::Reg<gpioh_hwcfgr3::GPIOH_HWCFGR3_SPEC>,
    #[doc = "0x3e8 - GPIO hardware configuration register 2"]
    pub gpioh_hwcfgr2: crate::Reg<gpioh_hwcfgr2::GPIOH_HWCFGR2_SPEC>,
    #[doc = "0x3ec - GPIO hardware configuration register 1"]
    pub gpioh_hwcfgr1: crate::Reg<gpioh_hwcfgr1::GPIOH_HWCFGR1_SPEC>,
    #[doc = "0x3f0 - GPIO hardware configuration register 0"]
    pub gpioh_hwcfgr0: crate::Reg<gpioh_hwcfgr0::GPIOH_HWCFGR0_SPEC>,
    #[doc = "0x3f4 - GPIO version register"]
    pub gpioh_verr: crate::Reg<gpioh_verr::GPIOH_VERR_SPEC>,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpioh_ipidr: crate::Reg<gpioh_ipidr::GPIOH_IPIDR_SPEC>,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpioh_sidr: crate::Reg<gpioh_sidr::GPIOH_SIDR_SPEC>,
}
#[doc = "GPIOH_MODER register accessor: an alias for `Reg<GPIOH_MODER_SPEC>`"]
pub type GPIOH_MODER = crate::Reg<gpioh_moder::GPIOH_MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod gpioh_moder;
#[doc = "GPIOH_OTYPER register accessor: an alias for `Reg<GPIOH_OTYPER_SPEC>`"]
pub type GPIOH_OTYPER = crate::Reg<gpioh_otyper::GPIOH_OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod gpioh_otyper;
#[doc = "GPIOH_OSPEEDR register accessor: an alias for `Reg<GPIOH_OSPEEDR_SPEC>`"]
pub type GPIOH_OSPEEDR = crate::Reg<gpioh_ospeedr::GPIOH_OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod gpioh_ospeedr;
#[doc = "GPIOH_PUPDR register accessor: an alias for `Reg<GPIOH_PUPDR_SPEC>`"]
pub type GPIOH_PUPDR = crate::Reg<gpioh_pupdr::GPIOH_PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpioh_pupdr;
#[doc = "GPIOH_IDR register accessor: an alias for `Reg<GPIOH_IDR_SPEC>`"]
pub type GPIOH_IDR = crate::Reg<gpioh_idr::GPIOH_IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod gpioh_idr;
#[doc = "GPIOH_ODR register accessor: an alias for `Reg<GPIOH_ODR_SPEC>`"]
pub type GPIOH_ODR = crate::Reg<gpioh_odr::GPIOH_ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod gpioh_odr;
#[doc = "GPIOH_BSRR register accessor: an alias for `Reg<GPIOH_BSRR_SPEC>`"]
pub type GPIOH_BSRR = crate::Reg<gpioh_bsrr::GPIOH_BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod gpioh_bsrr;
#[doc = "GPIOH_LCKR register accessor: an alias for `Reg<GPIOH_LCKR_SPEC>`"]
pub type GPIOH_LCKR = crate::Reg<gpioh_lckr::GPIOH_LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpioh_lckr;
#[doc = "GPIOH_AFRL register accessor: an alias for `Reg<GPIOH_AFRL_SPEC>`"]
pub type GPIOH_AFRL = crate::Reg<gpioh_afrl::GPIOH_AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod gpioh_afrl;
#[doc = "GPIOH_AFRH register accessor: an alias for `Reg<GPIOH_AFRH_SPEC>`"]
pub type GPIOH_AFRH = crate::Reg<gpioh_afrh::GPIOH_AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod gpioh_afrh;
#[doc = "GPIOH_BRR register accessor: an alias for `Reg<GPIOH_BRR_SPEC>`"]
pub type GPIOH_BRR = crate::Reg<gpioh_brr::GPIOH_BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod gpioh_brr;
#[doc = "GPIOH_HWCFGR10 register accessor: an alias for `Reg<GPIOH_HWCFGR10_SPEC>`"]
pub type GPIOH_HWCFGR10 = crate::Reg<gpioh_hwcfgr10::GPIOH_HWCFGR10_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:"]
pub mod gpioh_hwcfgr10;
#[doc = "GPIOH_HWCFGR9 register accessor: an alias for `Reg<GPIOH_HWCFGR9_SPEC>`"]
pub type GPIOH_HWCFGR9 = crate::Reg<gpioh_hwcfgr9::GPIOH_HWCFGR9_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioh_hwcfgr9;
#[doc = "GPIOH_HWCFGR8 register accessor: an alias for `Reg<GPIOH_HWCFGR8_SPEC>`"]
pub type GPIOH_HWCFGR8 = crate::Reg<gpioh_hwcfgr8::GPIOH_HWCFGR8_SPEC>;
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:"]
pub mod gpioh_hwcfgr8;
#[doc = "GPIOH_HWCFGR7 register accessor: an alias for `Reg<GPIOH_HWCFGR7_SPEC>`"]
pub type GPIOH_HWCFGR7 = crate::Reg<gpioh_hwcfgr7::GPIOH_HWCFGR7_SPEC>;
#[doc = "GPIO hardware configuration register 7"]
pub mod gpioh_hwcfgr7;
#[doc = "GPIOH_HWCFGR6 register accessor: an alias for `Reg<GPIOH_HWCFGR6_SPEC>`"]
pub type GPIOH_HWCFGR6 = crate::Reg<gpioh_hwcfgr6::GPIOH_HWCFGR6_SPEC>;
#[doc = "GPIO hardware configuration register 6"]
pub mod gpioh_hwcfgr6;
#[doc = "GPIOH_HWCFGR5 register accessor: an alias for `Reg<GPIOH_HWCFGR5_SPEC>`"]
pub type GPIOH_HWCFGR5 = crate::Reg<gpioh_hwcfgr5::GPIOH_HWCFGR5_SPEC>;
#[doc = "GPIO hardware configuration register 5"]
pub mod gpioh_hwcfgr5;
#[doc = "GPIOH_HWCFGR4 register accessor: an alias for `Reg<GPIOH_HWCFGR4_SPEC>`"]
pub type GPIOH_HWCFGR4 = crate::Reg<gpioh_hwcfgr4::GPIOH_HWCFGR4_SPEC>;
#[doc = "GPIO hardware configuration register 4"]
pub mod gpioh_hwcfgr4;
#[doc = "GPIOH_HWCFGR3 register accessor: an alias for `Reg<GPIOH_HWCFGR3_SPEC>`"]
pub type GPIOH_HWCFGR3 = crate::Reg<gpioh_hwcfgr3::GPIOH_HWCFGR3_SPEC>;
#[doc = "GPIO hardware configuration register 3"]
pub mod gpioh_hwcfgr3;
#[doc = "GPIOH_HWCFGR2 register accessor: an alias for `Reg<GPIOH_HWCFGR2_SPEC>`"]
pub type GPIOH_HWCFGR2 = crate::Reg<gpioh_hwcfgr2::GPIOH_HWCFGR2_SPEC>;
#[doc = "GPIO hardware configuration register 2"]
pub mod gpioh_hwcfgr2;
#[doc = "GPIOH_HWCFGR1 register accessor: an alias for `Reg<GPIOH_HWCFGR1_SPEC>`"]
pub type GPIOH_HWCFGR1 = crate::Reg<gpioh_hwcfgr1::GPIOH_HWCFGR1_SPEC>;
#[doc = "GPIO hardware configuration register 1"]
pub mod gpioh_hwcfgr1;
#[doc = "GPIOH_HWCFGR0 register accessor: an alias for `Reg<GPIOH_HWCFGR0_SPEC>`"]
pub type GPIOH_HWCFGR0 = crate::Reg<gpioh_hwcfgr0::GPIOH_HWCFGR0_SPEC>;
#[doc = "GPIO hardware configuration register 0"]
pub mod gpioh_hwcfgr0;
#[doc = "GPIOH_VERR register accessor: an alias for `Reg<GPIOH_VERR_SPEC>`"]
pub type GPIOH_VERR = crate::Reg<gpioh_verr::GPIOH_VERR_SPEC>;
#[doc = "GPIO version register"]
pub mod gpioh_verr;
#[doc = "GPIOH_IPIDR register accessor: an alias for `Reg<GPIOH_IPIDR_SPEC>`"]
pub type GPIOH_IPIDR = crate::Reg<gpioh_ipidr::GPIOH_IPIDR_SPEC>;
#[doc = "GPIO identification register"]
pub mod gpioh_ipidr;
#[doc = "GPIOH_SIDR register accessor: an alias for `Reg<GPIOH_SIDR_SPEC>`"]
pub type GPIOH_SIDR = crate::Reg<gpioh_sidr::GPIOH_SIDR_SPEC>;
#[doc = "GPIO size identification register"]
pub mod gpioh_sidr;
