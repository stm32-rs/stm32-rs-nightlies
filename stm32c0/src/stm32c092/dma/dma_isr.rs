///Register `DMA_ISR` reader
pub type R = crate::R<DMA_ISRrs>;
/**Global interrupt flag for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1 {
    ///0: No TE, HT, or TC event
    B0x0 = 0,
    ///1: A TE, HT, or TC event occurred.
    B0x1 = 1,
}
impl From<GIF1> for bool {
    #[inline(always)]
    fn from(variant: GIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF1` reader - Global interrupt flag for channel 1
pub type GIF1_R = crate::BitReader<GIF1>;
impl GIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GIF1 {
        match self.bits {
            false => GIF1::B0x0,
            true => GIF1::B0x1,
        }
    }
    ///No TE, HT, or TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF1::B0x0
    }
    ///A TE, HT, or TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF1::B0x1
    }
}
/**Transfer complete (TC) flag for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1 {
    ///0: No TC event
    B0x0 = 0,
    ///1: A TC event occurred.
    B0x1 = 1,
}
impl From<TCIF1> for bool {
    #[inline(always)]
    fn from(variant: TCIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF1` reader - Transfer complete (TC) flag for channel 1
pub type TCIF1_R = crate::BitReader<TCIF1>;
impl TCIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF1 {
        match self.bits {
            false => TCIF1::B0x0,
            true => TCIF1::B0x1,
        }
    }
    ///No TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF1::B0x0
    }
    ///A TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF1::B0x1
    }
}
/**Half transfer (HT) flag for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1 {
    ///0: No HT event
    B0x0 = 0,
    ///1: An HT event occurred.
    B0x1 = 1,
}
impl From<HTIF1> for bool {
    #[inline(always)]
    fn from(variant: HTIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF1` reader - Half transfer (HT) flag for channel 1
pub type HTIF1_R = crate::BitReader<HTIF1>;
impl HTIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF1 {
        match self.bits {
            false => HTIF1::B0x0,
            true => HTIF1::B0x1,
        }
    }
    ///No HT event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF1::B0x0
    }
    ///An HT event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF1::B0x1
    }
}
/**Transfer error (TE) flag for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1 {
    ///0: No TE event
    B0x0 = 0,
    ///1: A TE event occurred.
    B0x1 = 1,
}
impl From<TEIF1> for bool {
    #[inline(always)]
    fn from(variant: TEIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF1` reader - Transfer error (TE) flag for channel 1
pub type TEIF1_R = crate::BitReader<TEIF1>;
impl TEIF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF1 {
        match self.bits {
            false => TEIF1::B0x0,
            true => TEIF1::B0x1,
        }
    }
    ///No TE event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF1::B0x0
    }
    ///A TE event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF1::B0x1
    }
}
/**Global interrupt flag for channel 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF2 {
    ///0: No TE, HT, or TC event
    B0x0 = 0,
    ///1: A TE, HT, or TC event occurred.
    B0x1 = 1,
}
impl From<GIF2> for bool {
    #[inline(always)]
    fn from(variant: GIF2) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF2` reader - Global interrupt flag for channel 2
pub type GIF2_R = crate::BitReader<GIF2>;
impl GIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GIF2 {
        match self.bits {
            false => GIF2::B0x0,
            true => GIF2::B0x1,
        }
    }
    ///No TE, HT, or TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF2::B0x0
    }
    ///A TE, HT, or TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF2::B0x1
    }
}
/**Transfer complete (TC) flag for channel 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF2 {
    ///0: No TC event
    B0x0 = 0,
    ///1: A TC event occurred.
    B0x1 = 1,
}
impl From<TCIF2> for bool {
    #[inline(always)]
    fn from(variant: TCIF2) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF2` reader - Transfer complete (TC) flag for channel 2
pub type TCIF2_R = crate::BitReader<TCIF2>;
impl TCIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF2 {
        match self.bits {
            false => TCIF2::B0x0,
            true => TCIF2::B0x1,
        }
    }
    ///No TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF2::B0x0
    }
    ///A TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF2::B0x1
    }
}
/**Half transfer (HT) flag for channel 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF2 {
    ///0: No HT event
    B0x0 = 0,
    ///1: An HT event occurred.
    B0x1 = 1,
}
impl From<HTIF2> for bool {
    #[inline(always)]
    fn from(variant: HTIF2) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF2` reader - Half transfer (HT) flag for channel 2
pub type HTIF2_R = crate::BitReader<HTIF2>;
impl HTIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF2 {
        match self.bits {
            false => HTIF2::B0x0,
            true => HTIF2::B0x1,
        }
    }
    ///No HT event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF2::B0x0
    }
    ///An HT event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF2::B0x1
    }
}
/**Transfer error (TE) flag for channel 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF2 {
    ///0: No TE event
    B0x0 = 0,
    ///1: A TE event occurred.
    B0x1 = 1,
}
impl From<TEIF2> for bool {
    #[inline(always)]
    fn from(variant: TEIF2) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF2` reader - Transfer error (TE) flag for channel 2
pub type TEIF2_R = crate::BitReader<TEIF2>;
impl TEIF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF2 {
        match self.bits {
            false => TEIF2::B0x0,
            true => TEIF2::B0x1,
        }
    }
    ///No TE event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF2::B0x0
    }
    ///A TE event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF2::B0x1
    }
}
/**Global interrupt flag for channel 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF3 {
    ///0: No TE, HT, or TC event
    B0x0 = 0,
    ///1: A TE, HT, or TC event occurred.
    B0x1 = 1,
}
impl From<GIF3> for bool {
    #[inline(always)]
    fn from(variant: GIF3) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF3` reader - Global interrupt flag for channel 3
pub type GIF3_R = crate::BitReader<GIF3>;
impl GIF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GIF3 {
        match self.bits {
            false => GIF3::B0x0,
            true => GIF3::B0x1,
        }
    }
    ///No TE, HT, or TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF3::B0x0
    }
    ///A TE, HT, or TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF3::B0x1
    }
}
/**Transfer complete (TC) flag for channel 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF3 {
    ///0: No TC event
    B0x0 = 0,
    ///1: A TC event occurred.
    B0x1 = 1,
}
impl From<TCIF3> for bool {
    #[inline(always)]
    fn from(variant: TCIF3) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF3` reader - Transfer complete (TC) flag for channel 3
pub type TCIF3_R = crate::BitReader<TCIF3>;
impl TCIF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF3 {
        match self.bits {
            false => TCIF3::B0x0,
            true => TCIF3::B0x1,
        }
    }
    ///No TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF3::B0x0
    }
    ///A TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF3::B0x1
    }
}
/**Half transfer (HT) flag for channel 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF3 {
    ///0: No HT event
    B0x0 = 0,
    ///1: An HT event occurred.
    B0x1 = 1,
}
impl From<HTIF3> for bool {
    #[inline(always)]
    fn from(variant: HTIF3) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF3` reader - Half transfer (HT) flag for channel 3
pub type HTIF3_R = crate::BitReader<HTIF3>;
impl HTIF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF3 {
        match self.bits {
            false => HTIF3::B0x0,
            true => HTIF3::B0x1,
        }
    }
    ///No HT event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF3::B0x0
    }
    ///An HT event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF3::B0x1
    }
}
/**Transfer error (TE) flag for channel 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF3 {
    ///0: No TE event
    B0x0 = 0,
    ///1: A TE event occurred.
    B0x1 = 1,
}
impl From<TEIF3> for bool {
    #[inline(always)]
    fn from(variant: TEIF3) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF3` reader - Transfer error (TE) flag for channel 3
pub type TEIF3_R = crate::BitReader<TEIF3>;
impl TEIF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF3 {
        match self.bits {
            false => TEIF3::B0x0,
            true => TEIF3::B0x1,
        }
    }
    ///No TE event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF3::B0x0
    }
    ///A TE event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF3::B0x1
    }
}
/**global interrupt flag for channel 4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF4 {
    ///0: No TE, HT, or TC event
    B0x0 = 0,
    ///1: A TE, HT, or TC event occurred.
    B0x1 = 1,
}
impl From<GIF4> for bool {
    #[inline(always)]
    fn from(variant: GIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF4` reader - global interrupt flag for channel 4
pub type GIF4_R = crate::BitReader<GIF4>;
impl GIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GIF4 {
        match self.bits {
            false => GIF4::B0x0,
            true => GIF4::B0x1,
        }
    }
    ///No TE, HT, or TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF4::B0x0
    }
    ///A TE, HT, or TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF4::B0x1
    }
}
/**Transfer complete (TC) flag for channel 4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF4 {
    ///0: No TC event
    B0x0 = 0,
    ///1: A TC event occurred.
    B0x1 = 1,
}
impl From<TCIF4> for bool {
    #[inline(always)]
    fn from(variant: TCIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF4` reader - Transfer complete (TC) flag for channel 4
pub type TCIF4_R = crate::BitReader<TCIF4>;
impl TCIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF4 {
        match self.bits {
            false => TCIF4::B0x0,
            true => TCIF4::B0x1,
        }
    }
    ///No TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF4::B0x0
    }
    ///A TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF4::B0x1
    }
}
/**Half transfer (HT) flag for channel 4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF4 {
    ///0: No HT event
    B0x0 = 0,
    ///1: An HT event occurred.
    B0x1 = 1,
}
impl From<HTIF4> for bool {
    #[inline(always)]
    fn from(variant: HTIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF4` reader - Half transfer (HT) flag for channel 4
pub type HTIF4_R = crate::BitReader<HTIF4>;
impl HTIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF4 {
        match self.bits {
            false => HTIF4::B0x0,
            true => HTIF4::B0x1,
        }
    }
    ///No HT event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF4::B0x0
    }
    ///An HT event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF4::B0x1
    }
}
/**Transfer error (TE) flag for channel 4

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF4 {
    ///0: No TE event
    B0x0 = 0,
    ///1: A TE event occurred.
    B0x1 = 1,
}
impl From<TEIF4> for bool {
    #[inline(always)]
    fn from(variant: TEIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF4` reader - Transfer error (TE) flag for channel 4
pub type TEIF4_R = crate::BitReader<TEIF4>;
impl TEIF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF4 {
        match self.bits {
            false => TEIF4::B0x0,
            true => TEIF4::B0x1,
        }
    }
    ///No TE event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF4::B0x0
    }
    ///A TE event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF4::B0x1
    }
}
/**global interrupt flag for channel 5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF5 {
    ///0: No TE, HT, or TC event
    B0x0 = 0,
    ///1: A TE, HT, or TC event occurred.
    B0x1 = 1,
}
impl From<GIF5> for bool {
    #[inline(always)]
    fn from(variant: GIF5) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF5` reader - global interrupt flag for channel 5
pub type GIF5_R = crate::BitReader<GIF5>;
impl GIF5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GIF5 {
        match self.bits {
            false => GIF5::B0x0,
            true => GIF5::B0x1,
        }
    }
    ///No TE, HT, or TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF5::B0x0
    }
    ///A TE, HT, or TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF5::B0x1
    }
}
/**Transfer complete (TC) flag for channel 5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF5 {
    ///0: No TC event
    B0x0 = 0,
    ///1: A TC event occurred.
    B0x1 = 1,
}
impl From<TCIF5> for bool {
    #[inline(always)]
    fn from(variant: TCIF5) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF5` reader - Transfer complete (TC) flag for channel 5
pub type TCIF5_R = crate::BitReader<TCIF5>;
impl TCIF5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF5 {
        match self.bits {
            false => TCIF5::B0x0,
            true => TCIF5::B0x1,
        }
    }
    ///No TC event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF5::B0x0
    }
    ///A TC event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF5::B0x1
    }
}
/**Half transfer (HT) flag for channel 5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF5 {
    ///0: No HT event
    B0x0 = 0,
    ///1: An HT event occurred.
    B0x1 = 1,
}
impl From<HTIF5> for bool {
    #[inline(always)]
    fn from(variant: HTIF5) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF5` reader - Half transfer (HT) flag for channel 5
pub type HTIF5_R = crate::BitReader<HTIF5>;
impl HTIF5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF5 {
        match self.bits {
            false => HTIF5::B0x0,
            true => HTIF5::B0x1,
        }
    }
    ///No HT event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF5::B0x0
    }
    ///An HT event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF5::B0x1
    }
}
/**Transfer error (TE) flag for channel 5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF5 {
    ///0: No TE event
    B0x0 = 0,
    ///1: A TE event occurred.
    B0x1 = 1,
}
impl From<TEIF5> for bool {
    #[inline(always)]
    fn from(variant: TEIF5) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF5` reader - Transfer error (TE) flag for channel 5
pub type TEIF5_R = crate::BitReader<TEIF5>;
impl TEIF5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF5 {
        match self.bits {
            false => TEIF5::B0x0,
            true => TEIF5::B0x1,
        }
    }
    ///No TE event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF5::B0x0
    }
    ///A TE event occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF5::B0x1
    }
}
impl R {
    ///Bit 0 - Global interrupt flag for channel 1
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete (TC) flag for channel 1
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half transfer (HT) flag for channel 1
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer error (TE) flag for channel 1
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Global interrupt flag for channel 2
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transfer complete (TC) flag for channel 2
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Half transfer (HT) flag for channel 2
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transfer error (TE) flag for channel 2
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Global interrupt flag for channel 3
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete (TC) flag for channel 3
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Half transfer (HT) flag for channel 3
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Transfer error (TE) flag for channel 3
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - global interrupt flag for channel 4
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Transfer complete (TC) flag for channel 4
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Half transfer (HT) flag for channel 4
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transfer error (TE) flag for channel 4
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - global interrupt flag for channel 5
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete (TC) flag for channel 5
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Half transfer (HT) flag for channel 5
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transfer error (TE) flag for channel 5
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_ISR")
            .field("gif1", &self.gif1())
            .field("tcif1", &self.tcif1())
            .field("htif1", &self.htif1())
            .field("teif1", &self.teif1())
            .field("gif2", &self.gif2())
            .field("tcif2", &self.tcif2())
            .field("htif2", &self.htif2())
            .field("teif2", &self.teif2())
            .field("gif3", &self.gif3())
            .field("tcif3", &self.tcif3())
            .field("htif3", &self.htif3())
            .field("teif3", &self.teif3())
            .field("gif4", &self.gif4())
            .field("tcif4", &self.tcif4())
            .field("htif4", &self.htif4())
            .field("teif4", &self.teif4())
            .field("gif5", &self.gif5())
            .field("tcif5", &self.tcif5())
            .field("htif5", &self.htif5())
            .field("teif5", &self.teif5())
            .finish()
    }
}
/**DMA interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dma_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMA:DMA_ISR)*/
pub struct DMA_ISRrs;
impl crate::RegisterSpec for DMA_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`dma_isr::R`](R) reader structure
impl crate::Readable for DMA_ISRrs {}
///`reset()` method sets DMA_ISR to value 0
impl crate::Resettable for DMA_ISRrs {}
