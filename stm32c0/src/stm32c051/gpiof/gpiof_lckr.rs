///Register `GPIOF_LCKR` reader
pub type R = crate::R<GPIOF_LCKRrs>;
///Register `GPIOF_LCKR` writer
pub type W = crate::W<GPIOF_LCKRrs>;
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK0 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK0> for bool {
    #[inline(always)]
    fn from(variant: LCK0) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK0` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK0_R = crate::BitReader<LCK0>;
impl LCK0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK0 {
        match self.bits {
            false => LCK0::B0x0,
            true => LCK0::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK0::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK0::B0x1
    }
}
///Field `LCK0` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG, LCK0>;
impl<'a, REG> LCK0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK1 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK1> for bool {
    #[inline(always)]
    fn from(variant: LCK1) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK1` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK1_R = crate::BitReader<LCK1>;
impl LCK1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK1 {
        match self.bits {
            false => LCK1::B0x0,
            true => LCK1::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK1::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK1::B0x1
    }
}
///Field `LCK1` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK1_W<'a, REG> = crate::BitWriter<'a, REG, LCK1>;
impl<'a, REG> LCK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK1::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK1::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK2 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK2> for bool {
    #[inline(always)]
    fn from(variant: LCK2) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK2` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK2_R = crate::BitReader<LCK2>;
impl LCK2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK2 {
        match self.bits {
            false => LCK2::B0x0,
            true => LCK2::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK2::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK2::B0x1
    }
}
///Field `LCK2` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK2_W<'a, REG> = crate::BitWriter<'a, REG, LCK2>;
impl<'a, REG> LCK2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK2::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK2::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK3 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK3> for bool {
    #[inline(always)]
    fn from(variant: LCK3) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK3` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK3_R = crate::BitReader<LCK3>;
impl LCK3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK3 {
        match self.bits {
            false => LCK3::B0x0,
            true => LCK3::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK3::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK3::B0x1
    }
}
///Field `LCK3` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG, LCK3>;
impl<'a, REG> LCK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK3::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK3::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK4 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK4> for bool {
    #[inline(always)]
    fn from(variant: LCK4) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK4` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK4_R = crate::BitReader<LCK4>;
impl LCK4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK4 {
        match self.bits {
            false => LCK4::B0x0,
            true => LCK4::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK4::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK4::B0x1
    }
}
///Field `LCK4` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK4_W<'a, REG> = crate::BitWriter<'a, REG, LCK4>;
impl<'a, REG> LCK4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK4::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK4::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK5 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK5> for bool {
    #[inline(always)]
    fn from(variant: LCK5) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK5` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK5_R = crate::BitReader<LCK5>;
impl LCK5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK5 {
        match self.bits {
            false => LCK5::B0x0,
            true => LCK5::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK5::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK5::B0x1
    }
}
///Field `LCK5` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK5_W<'a, REG> = crate::BitWriter<'a, REG, LCK5>;
impl<'a, REG> LCK5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK5::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK5::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK6 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK6> for bool {
    #[inline(always)]
    fn from(variant: LCK6) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK6` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK6_R = crate::BitReader<LCK6>;
impl LCK6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK6 {
        match self.bits {
            false => LCK6::B0x0,
            true => LCK6::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK6::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK6::B0x1
    }
}
///Field `LCK6` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK6_W<'a, REG> = crate::BitWriter<'a, REG, LCK6>;
impl<'a, REG> LCK6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK6::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK6::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK7 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK7> for bool {
    #[inline(always)]
    fn from(variant: LCK7) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK7` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK7_R = crate::BitReader<LCK7>;
impl LCK7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK7 {
        match self.bits {
            false => LCK7::B0x0,
            true => LCK7::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK7::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK7::B0x1
    }
}
///Field `LCK7` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK7_W<'a, REG> = crate::BitWriter<'a, REG, LCK7>;
impl<'a, REG> LCK7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK7::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK7::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK8 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK8> for bool {
    #[inline(always)]
    fn from(variant: LCK8) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK8` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK8_R = crate::BitReader<LCK8>;
impl LCK8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK8 {
        match self.bits {
            false => LCK8::B0x0,
            true => LCK8::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK8::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK8::B0x1
    }
}
///Field `LCK8` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK8_W<'a, REG> = crate::BitWriter<'a, REG, LCK8>;
impl<'a, REG> LCK8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK8::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK8::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK9 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK9> for bool {
    #[inline(always)]
    fn from(variant: LCK9) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK9` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK9_R = crate::BitReader<LCK9>;
impl LCK9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK9 {
        match self.bits {
            false => LCK9::B0x0,
            true => LCK9::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK9::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK9::B0x1
    }
}
///Field `LCK9` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK9_W<'a, REG> = crate::BitWriter<'a, REG, LCK9>;
impl<'a, REG> LCK9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK9::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK9::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK10 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK10> for bool {
    #[inline(always)]
    fn from(variant: LCK10) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK10` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK10_R = crate::BitReader<LCK10>;
impl LCK10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK10 {
        match self.bits {
            false => LCK10::B0x0,
            true => LCK10::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK10::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK10::B0x1
    }
}
///Field `LCK10` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK10_W<'a, REG> = crate::BitWriter<'a, REG, LCK10>;
impl<'a, REG> LCK10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK10::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK10::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK11 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK11> for bool {
    #[inline(always)]
    fn from(variant: LCK11) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK11` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK11_R = crate::BitReader<LCK11>;
impl LCK11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK11 {
        match self.bits {
            false => LCK11::B0x0,
            true => LCK11::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK11::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK11::B0x1
    }
}
///Field `LCK11` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK11_W<'a, REG> = crate::BitWriter<'a, REG, LCK11>;
impl<'a, REG> LCK11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK11::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK11::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK12 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK12> for bool {
    #[inline(always)]
    fn from(variant: LCK12) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK12` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK12_R = crate::BitReader<LCK12>;
impl LCK12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK12 {
        match self.bits {
            false => LCK12::B0x0,
            true => LCK12::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK12::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK12::B0x1
    }
}
///Field `LCK12` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK12_W<'a, REG> = crate::BitWriter<'a, REG, LCK12>;
impl<'a, REG> LCK12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK12::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK12::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK13 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK13> for bool {
    #[inline(always)]
    fn from(variant: LCK13) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK13` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK13_R = crate::BitReader<LCK13>;
impl LCK13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK13 {
        match self.bits {
            false => LCK13::B0x0,
            true => LCK13::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK13::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK13::B0x1
    }
}
///Field `LCK13` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK13_W<'a, REG> = crate::BitWriter<'a, REG, LCK13>;
impl<'a, REG> LCK13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK13::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK13::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK14 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK14> for bool {
    #[inline(always)]
    fn from(variant: LCK14) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK14` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK14_R = crate::BitReader<LCK14>;
impl LCK14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK14 {
        match self.bits {
            false => LCK14::B0x0,
            true => LCK14::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK14::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK14::B0x1
    }
}
///Field `LCK14` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK14_W<'a, REG> = crate::BitWriter<'a, REG, LCK14>;
impl<'a, REG> LCK14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK14::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK14::B0x1)
    }
}
/**Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK15 {
    ///0: Port configuration not locked
    B0x0 = 0,
    ///1: Port configuration locked
    B0x1 = 1,
}
impl From<LCK15> for bool {
    #[inline(always)]
    fn from(variant: LCK15) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK15` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK15_R = crate::BitReader<LCK15>;
impl LCK15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK15 {
        match self.bits {
            false => LCK15::B0x0,
            true => LCK15::B0x1,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCK15::B0x0
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCK15::B0x1
    }
}
///Field `LCK15` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK15_W<'a, REG> = crate::BitWriter<'a, REG, LCK15>;
impl<'a, REG> LCK15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK15::B0x0)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK15::B0x1)
    }
}
/**Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] RD LCKR RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKK {
    ///0: Port configuration lock key not active
    B0x0 = 0,
    ///1: Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset.
    B0x1 = 1,
}
impl From<LCKK> for bool {
    #[inline(always)]
    fn from(variant: LCKK) -> Self {
        variant as u8 != 0
    }
}
///Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] RD LCKR RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
pub type LCKK_R = crate::BitReader<LCKK>;
impl LCKK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCKK {
        match self.bits {
            false => LCKK::B0x0,
            true => LCKK::B0x1,
        }
    }
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LCKK::B0x0
    }
    ///Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LCKK::B0x1
    }
}
///Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] RD LCKR RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG, LCKK>;
impl<'a, REG> LCKK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK::B0x0)
    }
    ///Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK::B0x1)
    }
}
impl R {
    ///Bit 0 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] RD LCKR RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOF_LCKR")
            .field("lck0", &self.lck0())
            .field("lck1", &self.lck1())
            .field("lck2", &self.lck2())
            .field("lck3", &self.lck3())
            .field("lck4", &self.lck4())
            .field("lck5", &self.lck5())
            .field("lck6", &self.lck6())
            .field("lck7", &self.lck7())
            .field("lck8", &self.lck8())
            .field("lck9", &self.lck9())
            .field("lck10", &self.lck10())
            .field("lck11", &self.lck11())
            .field("lck12", &self.lck12())
            .field("lck13", &self.lck13())
            .field("lck14", &self.lck14())
            .field("lck15", &self.lck15())
            .field("lckk", &self.lckk())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W<'_, GPIOF_LCKRrs> {
        LCK0_W::new(self, 0)
    }
    ///Bit 1 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W<'_, GPIOF_LCKRrs> {
        LCK1_W::new(self, 1)
    }
    ///Bit 2 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W<'_, GPIOF_LCKRrs> {
        LCK2_W::new(self, 2)
    }
    ///Bit 3 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W<'_, GPIOF_LCKRrs> {
        LCK3_W::new(self, 3)
    }
    ///Bit 4 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W<'_, GPIOF_LCKRrs> {
        LCK4_W::new(self, 4)
    }
    ///Bit 5 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W<'_, GPIOF_LCKRrs> {
        LCK5_W::new(self, 5)
    }
    ///Bit 6 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W<'_, GPIOF_LCKRrs> {
        LCK6_W::new(self, 6)
    }
    ///Bit 7 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W<'_, GPIOF_LCKRrs> {
        LCK7_W::new(self, 7)
    }
    ///Bit 8 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W<'_, GPIOF_LCKRrs> {
        LCK8_W::new(self, 8)
    }
    ///Bit 9 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W<'_, GPIOF_LCKRrs> {
        LCK9_W::new(self, 9)
    }
    ///Bit 10 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W<'_, GPIOF_LCKRrs> {
        LCK10_W::new(self, 10)
    }
    ///Bit 11 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W<'_, GPIOF_LCKRrs> {
        LCK11_W::new(self, 11)
    }
    ///Bit 12 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W<'_, GPIOF_LCKRrs> {
        LCK12_W::new(self, 12)
    }
    ///Bit 13 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W<'_, GPIOF_LCKRrs> {
        LCK13_W::new(self, 13)
    }
    ///Bit 14 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W<'_, GPIOF_LCKRrs> {
        LCK14_W::new(self, 14)
    }
    ///Bit 15 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W<'_, GPIOF_LCKRrs> {
        LCK15_W::new(self, 15)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] RD LCKR RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\] must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<'_, GPIOF_LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`gpiof_lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOF:GPIOF_LCKR)*/
pub struct GPIOF_LCKRrs;
impl crate::RegisterSpec for GPIOF_LCKRrs {
    type Ux = u32;
}
///`read()` method returns [`gpiof_lckr::R`](R) reader structure
impl crate::Readable for GPIOF_LCKRrs {}
///`write(|w| ..)` method takes [`gpiof_lckr::W`](W) writer structure
impl crate::Writable for GPIOF_LCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOF_LCKR to value 0
impl crate::Resettable for GPIOF_LCKRrs {}
