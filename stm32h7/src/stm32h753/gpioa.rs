#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub moder: crate::Reg<moder::MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub otyper: crate::Reg<otyper::OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospeedr: crate::Reg<ospeedr::OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pupdr: crate::Reg<pupdr::PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub odr: crate::Reg<odr::ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bsrr: crate::Reg<bsrr::BSRR_SPEC>,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset.A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence.Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub lckr: crate::Reg<lckr::LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub afrl: crate::Reg<afrl::AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub afrh: crate::Reg<afrh::AFRH_SPEC>,
}
#[doc = "MODER register accessor: an alias for `Reg<MODER_SPEC>`"]
pub type MODER = crate::Reg<moder::MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod moder;
#[doc = "OTYPER register accessor: an alias for `Reg<OTYPER_SPEC>`"]
pub type OTYPER = crate::Reg<otyper::OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod otyper;
#[doc = "OSPEEDR register accessor: an alias for `Reg<OSPEEDR_SPEC>`"]
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod ospeedr;
#[doc = "PUPDR register accessor: an alias for `Reg<PUPDR_SPEC>`"]
pub type PUPDR = crate::Reg<pupdr::PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pupdr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod idr;
#[doc = "ODR register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod odr;
#[doc = "BSRR register accessor: an alias for `Reg<BSRR_SPEC>`"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "LCKR register accessor: an alias for `Reg<LCKR_SPEC>`"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset.A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence.Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod lckr;
#[doc = "AFRL register accessor: an alias for `Reg<AFRL_SPEC>`"]
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod afrl;
#[doc = "AFRH register accessor: an alias for `Reg<AFRH_SPEC>`"]
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod afrh;
