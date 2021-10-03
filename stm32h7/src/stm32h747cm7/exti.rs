#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: crate::Reg<rtsr1::RTSR1_SPEC>,
    #[doc = "0x04 - EXTI falling trigger selection register"]
    pub ftsr1: crate::Reg<ftsr1::FTSR1_SPEC>,
    #[doc = "0x08 - EXTI software interrupt event register"]
    pub swier1: crate::Reg<swier1::SWIER1_SPEC>,
    #[doc = "0x0c - EXTI D3 pending mask register"]
    pub d3pmr1: crate::Reg<d3pmr1::D3PMR1_SPEC>,
    #[doc = "0x10 - EXTI D3 pending clear selection register low"]
    pub d3pcr1l: crate::Reg<d3pcr1l::D3PCR1L_SPEC>,
    #[doc = "0x14 - EXTI D3 pending clear selection register high"]
    pub d3pcr1h: crate::Reg<d3pcr1h::D3PCR1H_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - EXTI rising trigger selection register"]
    pub rtsr2: crate::Reg<rtsr2::RTSR2_SPEC>,
    #[doc = "0x24 - EXTI falling trigger selection register"]
    pub ftsr2: crate::Reg<ftsr2::FTSR2_SPEC>,
    #[doc = "0x28 - EXTI software interrupt event register"]
    pub swier2: crate::Reg<swier2::SWIER2_SPEC>,
    #[doc = "0x2c - EXTI D3 pending mask register"]
    pub d3pmr2: crate::Reg<d3pmr2::D3PMR2_SPEC>,
    #[doc = "0x30 - EXTI D3 pending clear selection register low"]
    pub d3pcr2l: crate::Reg<d3pcr2l::D3PCR2L_SPEC>,
    #[doc = "0x34 - EXTI D3 pending clear selection register high"]
    pub d3pcr2h: crate::Reg<d3pcr2h::D3PCR2H_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - EXTI rising trigger selection register"]
    pub rtsr3: crate::Reg<rtsr3::RTSR3_SPEC>,
    #[doc = "0x44 - EXTI falling trigger selection register"]
    pub ftsr3: crate::Reg<ftsr3::FTSR3_SPEC>,
    #[doc = "0x48 - EXTI software interrupt event register"]
    pub swier3: crate::Reg<swier3::SWIER3_SPEC>,
    #[doc = "0x4c - EXTI D3 pending mask register"]
    pub d3pmr3: crate::Reg<d3pmr3::D3PMR3_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x54 - EXTI D3 pending clear selection register high"]
    pub d3pcr3h: crate::Reg<d3pcr3h::D3PCR3H_SPEC>,
    _reserved17: [u8; 0x28],
    #[doc = "0x80 - EXTI interrupt mask register"]
    pub c1imr1: crate::Reg<c1imr1::C1IMR1_SPEC>,
    #[doc = "0x84 - EXTI event mask register"]
    pub c1emr1: crate::Reg<c1emr1::C1EMR1_SPEC>,
    #[doc = "0x88 - EXTI pending register"]
    pub c1pr1: crate::Reg<c1pr1::C1PR1_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x90 - EXTI interrupt mask register"]
    pub c1imr2: crate::Reg<c1imr2::C1IMR2_SPEC>,
    #[doc = "0x94 - EXTI event mask register"]
    pub c1emr2: crate::Reg<c1emr2::C1EMR2_SPEC>,
    #[doc = "0x98 - EXTI pending register"]
    pub c1pr2: crate::Reg<c1pr2::C1PR2_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0xa0 - EXTI interrupt mask register"]
    pub c1imr3: crate::Reg<c1imr3::C1IMR3_SPEC>,
    #[doc = "0xa4 - EXTI event mask register"]
    pub c1emr3: crate::Reg<c1emr3::C1EMR3_SPEC>,
    #[doc = "0xa8 - EXTI pending register"]
    pub c1pr3: crate::Reg<c1pr3::C1PR3_SPEC>,
    _reserved26: [u8; 0x14],
    #[doc = "0xc0 - CPU2 EXTI interrupt mask register"]
    pub c2imr1: crate::Reg<c2imr1::C2IMR1_SPEC>,
    #[doc = "0xc4 - CPU2 EXTI event mask register"]
    pub c2emr1: crate::Reg<c2emr1::C2EMR1_SPEC>,
    #[doc = "0xc8 - CPU2 EXTI pending register"]
    pub c2pr1: crate::Reg<c2pr1::C2PR1_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0xd0 - CPU2 EXTI interrupt mask register"]
    pub c2imr2: crate::Reg<c2imr2::C2IMR2_SPEC>,
    #[doc = "0xd4 - CPU2 EXTI event mask register"]
    pub c2emr2: crate::Reg<c2emr2::C2EMR2_SPEC>,
    #[doc = "0xd8 - CPU2 EXTI pending register"]
    pub c2pr2: crate::Reg<c2pr2::C2PR2_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0xe0 - CPU2 EXTI interrupt mask register"]
    pub c2imr3: crate::Reg<c2imr3::C2IMR3_SPEC>,
    #[doc = "0xe4 - CPU2 EXTI event mask register"]
    pub c2emr3: crate::Reg<c2emr3::C2EMR3_SPEC>,
    #[doc = "0xe8 - CPU2 EXTI pending register"]
    pub c2pr3: crate::Reg<c2pr3::C2PR3_SPEC>,
}
#[doc = "RTSR1 register accessor: an alias for `Reg<RTSR1_SPEC>`"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 register accessor: an alias for `Reg<FTSR1_SPEC>`"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 register accessor: an alias for `Reg<SWIER1_SPEC>`"]
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "D3PMR1 register accessor: an alias for `Reg<D3PMR1_SPEC>`"]
pub type D3PMR1 = crate::Reg<d3pmr1::D3PMR1_SPEC>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr1;
#[doc = "D3PCR1L register accessor: an alias for `Reg<D3PCR1L_SPEC>`"]
pub type D3PCR1L = crate::Reg<d3pcr1l::D3PCR1L_SPEC>;
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr1l;
#[doc = "D3PCR1H register accessor: an alias for `Reg<D3PCR1H_SPEC>`"]
pub type D3PCR1H = crate::Reg<d3pcr1h::D3PCR1H_SPEC>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr1h;
#[doc = "RTSR2 register accessor: an alias for `Reg<RTSR2_SPEC>`"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 register accessor: an alias for `Reg<FTSR2_SPEC>`"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 register accessor: an alias for `Reg<SWIER2_SPEC>`"]
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
#[doc = "EXTI software interrupt event register"]
pub mod swier2;
#[doc = "D3PMR2 register accessor: an alias for `Reg<D3PMR2_SPEC>`"]
pub type D3PMR2 = crate::Reg<d3pmr2::D3PMR2_SPEC>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr2;
#[doc = "D3PCR2L register accessor: an alias for `Reg<D3PCR2L_SPEC>`"]
pub type D3PCR2L = crate::Reg<d3pcr2l::D3PCR2L_SPEC>;
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr2l;
#[doc = "D3PCR2H register accessor: an alias for `Reg<D3PCR2H_SPEC>`"]
pub type D3PCR2H = crate::Reg<d3pcr2h::D3PCR2H_SPEC>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr2h;
#[doc = "RTSR3 register accessor: an alias for `Reg<RTSR3_SPEC>`"]
pub type RTSR3 = crate::Reg<rtsr3::RTSR3_SPEC>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr3;
#[doc = "FTSR3 register accessor: an alias for `Reg<FTSR3_SPEC>`"]
pub type FTSR3 = crate::Reg<ftsr3::FTSR3_SPEC>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr3;
#[doc = "SWIER3 register accessor: an alias for `Reg<SWIER3_SPEC>`"]
pub type SWIER3 = crate::Reg<swier3::SWIER3_SPEC>;
#[doc = "EXTI software interrupt event register"]
pub mod swier3;
#[doc = "D3PMR3 register accessor: an alias for `Reg<D3PMR3_SPEC>`"]
pub type D3PMR3 = crate::Reg<d3pmr3::D3PMR3_SPEC>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr3;
#[doc = "D3PCR3H register accessor: an alias for `Reg<D3PCR3H_SPEC>`"]
pub type D3PCR3H = crate::Reg<d3pcr3h::D3PCR3H_SPEC>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr3h;
#[doc = "C1IMR1 register accessor: an alias for `Reg<C1IMR1_SPEC>`"]
pub type C1IMR1 = crate::Reg<c1imr1::C1IMR1_SPEC>;
#[doc = "EXTI interrupt mask register"]
pub mod c1imr1;
#[doc = "C1EMR1 register accessor: an alias for `Reg<C1EMR1_SPEC>`"]
pub type C1EMR1 = crate::Reg<c1emr1::C1EMR1_SPEC>;
#[doc = "EXTI event mask register"]
pub mod c1emr1;
#[doc = "C1PR1 register accessor: an alias for `Reg<C1PR1_SPEC>`"]
pub type C1PR1 = crate::Reg<c1pr1::C1PR1_SPEC>;
#[doc = "EXTI pending register"]
pub mod c1pr1;
#[doc = "C1IMR2 register accessor: an alias for `Reg<C1IMR2_SPEC>`"]
pub type C1IMR2 = crate::Reg<c1imr2::C1IMR2_SPEC>;
#[doc = "EXTI interrupt mask register"]
pub mod c1imr2;
#[doc = "C1EMR2 register accessor: an alias for `Reg<C1EMR2_SPEC>`"]
pub type C1EMR2 = crate::Reg<c1emr2::C1EMR2_SPEC>;
#[doc = "EXTI event mask register"]
pub mod c1emr2;
#[doc = "C1PR2 register accessor: an alias for `Reg<C1PR2_SPEC>`"]
pub type C1PR2 = crate::Reg<c1pr2::C1PR2_SPEC>;
#[doc = "EXTI pending register"]
pub mod c1pr2;
#[doc = "C1IMR3 register accessor: an alias for `Reg<C1IMR3_SPEC>`"]
pub type C1IMR3 = crate::Reg<c1imr3::C1IMR3_SPEC>;
#[doc = "EXTI interrupt mask register"]
pub mod c1imr3;
#[doc = "C1EMR3 register accessor: an alias for `Reg<C1EMR3_SPEC>`"]
pub type C1EMR3 = crate::Reg<c1emr3::C1EMR3_SPEC>;
#[doc = "EXTI event mask register"]
pub mod c1emr3;
#[doc = "C1PR3 register accessor: an alias for `Reg<C1PR3_SPEC>`"]
pub type C1PR3 = crate::Reg<c1pr3::C1PR3_SPEC>;
#[doc = "EXTI pending register"]
pub mod c1pr3;
#[doc = "C2IMR1 register accessor: an alias for `Reg<C2IMR1_SPEC>`"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1_SPEC>;
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr1;
#[doc = "C2EMR1 register accessor: an alias for `Reg<C2EMR1_SPEC>`"]
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1_SPEC>;
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr1;
#[doc = "C2PR1 register accessor: an alias for `Reg<C2PR1_SPEC>`"]
pub type C2PR1 = crate::Reg<c2pr1::C2PR1_SPEC>;
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr1;
#[doc = "C2IMR2 register accessor: an alias for `Reg<C2IMR2_SPEC>`"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2_SPEC>;
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr2;
#[doc = "C2EMR2 register accessor: an alias for `Reg<C2EMR2_SPEC>`"]
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2_SPEC>;
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr2;
#[doc = "C2PR2 register accessor: an alias for `Reg<C2PR2_SPEC>`"]
pub type C2PR2 = crate::Reg<c2pr2::C2PR2_SPEC>;
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr2;
#[doc = "C2IMR3 register accessor: an alias for `Reg<C2IMR3_SPEC>`"]
pub type C2IMR3 = crate::Reg<c2imr3::C2IMR3_SPEC>;
#[doc = "CPU2 EXTI interrupt mask register"]
pub mod c2imr3;
#[doc = "C2EMR3 register accessor: an alias for `Reg<C2EMR3_SPEC>`"]
pub type C2EMR3 = crate::Reg<c2emr3::C2EMR3_SPEC>;
#[doc = "CPU2 EXTI event mask register"]
pub mod c2emr3;
#[doc = "C2PR3 register accessor: an alias for `Reg<C2PR3_SPEC>`"]
pub type C2PR3 = crate::Reg<c2pr3::C2PR3_SPEC>;
#[doc = "CPU2 EXTI pending register"]
pub mod c2pr3;
