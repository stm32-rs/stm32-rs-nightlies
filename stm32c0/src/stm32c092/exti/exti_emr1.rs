///Register `EXTI_EMR1` reader
pub type R = crate::R<EXTI_EMR1rs>;
///Register `EXTI_EMR1` writer
pub type W = crate::W<EXTI_EMR1rs>;
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM0 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM0> for bool {
    #[inline(always)]
    fn from(variant: EM0) -> Self {
        variant as u8 != 0
    }
}
///Field `EM0` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM0_R = crate::BitReader<EM0>;
impl EM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM0 {
        match self.bits {
            false => EM0::B0x0,
            true => EM0::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM0::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM0::B0x1
    }
}
///Field `EM0` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG, EM0>;
impl<'a, REG> EM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM0::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM0::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM1 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM1> for bool {
    #[inline(always)]
    fn from(variant: EM1) -> Self {
        variant as u8 != 0
    }
}
///Field `EM1` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM1_R = crate::BitReader<EM1>;
impl EM1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM1 {
        match self.bits {
            false => EM1::B0x0,
            true => EM1::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM1::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM1::B0x1
    }
}
///Field `EM1` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM1_W<'a, REG> = crate::BitWriter<'a, REG, EM1>;
impl<'a, REG> EM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM1::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM1::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM2 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM2> for bool {
    #[inline(always)]
    fn from(variant: EM2) -> Self {
        variant as u8 != 0
    }
}
///Field `EM2` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM2_R = crate::BitReader<EM2>;
impl EM2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM2 {
        match self.bits {
            false => EM2::B0x0,
            true => EM2::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM2::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM2::B0x1
    }
}
///Field `EM2` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM2_W<'a, REG> = crate::BitWriter<'a, REG, EM2>;
impl<'a, REG> EM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM2::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM2::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM3 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM3> for bool {
    #[inline(always)]
    fn from(variant: EM3) -> Self {
        variant as u8 != 0
    }
}
///Field `EM3` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM3_R = crate::BitReader<EM3>;
impl EM3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM3 {
        match self.bits {
            false => EM3::B0x0,
            true => EM3::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM3::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM3::B0x1
    }
}
///Field `EM3` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM3_W<'a, REG> = crate::BitWriter<'a, REG, EM3>;
impl<'a, REG> EM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM3::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM3::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM4 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM4> for bool {
    #[inline(always)]
    fn from(variant: EM4) -> Self {
        variant as u8 != 0
    }
}
///Field `EM4` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM4_R = crate::BitReader<EM4>;
impl EM4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM4 {
        match self.bits {
            false => EM4::B0x0,
            true => EM4::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM4::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM4::B0x1
    }
}
///Field `EM4` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM4_W<'a, REG> = crate::BitWriter<'a, REG, EM4>;
impl<'a, REG> EM4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM4::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM4::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM5 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM5> for bool {
    #[inline(always)]
    fn from(variant: EM5) -> Self {
        variant as u8 != 0
    }
}
///Field `EM5` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM5_R = crate::BitReader<EM5>;
impl EM5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM5 {
        match self.bits {
            false => EM5::B0x0,
            true => EM5::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM5::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM5::B0x1
    }
}
///Field `EM5` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM5_W<'a, REG> = crate::BitWriter<'a, REG, EM5>;
impl<'a, REG> EM5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM5::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM5::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM6 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM6> for bool {
    #[inline(always)]
    fn from(variant: EM6) -> Self {
        variant as u8 != 0
    }
}
///Field `EM6` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM6_R = crate::BitReader<EM6>;
impl EM6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM6 {
        match self.bits {
            false => EM6::B0x0,
            true => EM6::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM6::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM6::B0x1
    }
}
///Field `EM6` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM6_W<'a, REG> = crate::BitWriter<'a, REG, EM6>;
impl<'a, REG> EM6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM6::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM6::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM7 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM7> for bool {
    #[inline(always)]
    fn from(variant: EM7) -> Self {
        variant as u8 != 0
    }
}
///Field `EM7` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM7_R = crate::BitReader<EM7>;
impl EM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM7 {
        match self.bits {
            false => EM7::B0x0,
            true => EM7::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM7::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM7::B0x1
    }
}
///Field `EM7` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM7_W<'a, REG> = crate::BitWriter<'a, REG, EM7>;
impl<'a, REG> EM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM7::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM7::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM8 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM8> for bool {
    #[inline(always)]
    fn from(variant: EM8) -> Self {
        variant as u8 != 0
    }
}
///Field `EM8` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM8_R = crate::BitReader<EM8>;
impl EM8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM8 {
        match self.bits {
            false => EM8::B0x0,
            true => EM8::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM8::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM8::B0x1
    }
}
///Field `EM8` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM8_W<'a, REG> = crate::BitWriter<'a, REG, EM8>;
impl<'a, REG> EM8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM8::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM8::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM9 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM9> for bool {
    #[inline(always)]
    fn from(variant: EM9) -> Self {
        variant as u8 != 0
    }
}
///Field `EM9` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM9_R = crate::BitReader<EM9>;
impl EM9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM9 {
        match self.bits {
            false => EM9::B0x0,
            true => EM9::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM9::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM9::B0x1
    }
}
///Field `EM9` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM9_W<'a, REG> = crate::BitWriter<'a, REG, EM9>;
impl<'a, REG> EM9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM9::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM9::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM10 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM10> for bool {
    #[inline(always)]
    fn from(variant: EM10) -> Self {
        variant as u8 != 0
    }
}
///Field `EM10` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM10_R = crate::BitReader<EM10>;
impl EM10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM10 {
        match self.bits {
            false => EM10::B0x0,
            true => EM10::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM10::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM10::B0x1
    }
}
///Field `EM10` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM10_W<'a, REG> = crate::BitWriter<'a, REG, EM10>;
impl<'a, REG> EM10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM10::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM10::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM11 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM11> for bool {
    #[inline(always)]
    fn from(variant: EM11) -> Self {
        variant as u8 != 0
    }
}
///Field `EM11` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM11_R = crate::BitReader<EM11>;
impl EM11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM11 {
        match self.bits {
            false => EM11::B0x0,
            true => EM11::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM11::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM11::B0x1
    }
}
///Field `EM11` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM11_W<'a, REG> = crate::BitWriter<'a, REG, EM11>;
impl<'a, REG> EM11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM11::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM11::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM12 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM12> for bool {
    #[inline(always)]
    fn from(variant: EM12) -> Self {
        variant as u8 != 0
    }
}
///Field `EM12` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM12_R = crate::BitReader<EM12>;
impl EM12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM12 {
        match self.bits {
            false => EM12::B0x0,
            true => EM12::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM12::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM12::B0x1
    }
}
///Field `EM12` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM12_W<'a, REG> = crate::BitWriter<'a, REG, EM12>;
impl<'a, REG> EM12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM12::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM12::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM13 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM13> for bool {
    #[inline(always)]
    fn from(variant: EM13) -> Self {
        variant as u8 != 0
    }
}
///Field `EM13` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM13_R = crate::BitReader<EM13>;
impl EM13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM13 {
        match self.bits {
            false => EM13::B0x0,
            true => EM13::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM13::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM13::B0x1
    }
}
///Field `EM13` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM13_W<'a, REG> = crate::BitWriter<'a, REG, EM13>;
impl<'a, REG> EM13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM13::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM13::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM14 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM14> for bool {
    #[inline(always)]
    fn from(variant: EM14) -> Self {
        variant as u8 != 0
    }
}
///Field `EM14` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM14_R = crate::BitReader<EM14>;
impl EM14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM14 {
        match self.bits {
            false => EM14::B0x0,
            true => EM14::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM14::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM14::B0x1
    }
}
///Field `EM14` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM14_W<'a, REG> = crate::BitWriter<'a, REG, EM14>;
impl<'a, REG> EM14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM14::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM14::B0x1)
    }
}
/**CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM15 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM15> for bool {
    #[inline(always)]
    fn from(variant: EM15) -> Self {
        variant as u8 != 0
    }
}
///Field `EM15` reader - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM15_R = crate::BitReader<EM15>;
impl EM15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM15 {
        match self.bits {
            false => EM15::B0x0,
            true => EM15::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM15::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM15::B0x1
    }
}
///Field `EM15` writer - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM15_W<'a, REG> = crate::BitWriter<'a, REG, EM15>;
impl<'a, REG> EM15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM15::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM15::B0x1)
    }
}
/**CPU wakeup with event generation mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM19 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM19> for bool {
    #[inline(always)]
    fn from(variant: EM19) -> Self {
        variant as u8 != 0
    }
}
///Field `EM19` reader - CPU wakeup with event generation mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM19_R = crate::BitReader<EM19>;
impl EM19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM19 {
        match self.bits {
            false => EM19::B0x0,
            true => EM19::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM19::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM19::B0x1
    }
}
///Field `EM19` writer - CPU wakeup with event generation mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM19_W<'a, REG> = crate::BitWriter<'a, REG, EM19>;
impl<'a, REG> EM19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM19::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM19::B0x1)
    }
}
/**CPU wakeup with event generation mask on line 23 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM23 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM23> for bool {
    #[inline(always)]
    fn from(variant: EM23) -> Self {
        variant as u8 != 0
    }
}
///Field `EM23` reader - CPU wakeup with event generation mask on line 23 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM23_R = crate::BitReader<EM23>;
impl EM23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM23 {
        match self.bits {
            false => EM23::B0x0,
            true => EM23::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM23::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM23::B0x1
    }
}
///Field `EM23` writer - CPU wakeup with event generation mask on line 23 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM23_W<'a, REG> = crate::BitWriter<'a, REG, EM23>;
impl<'a, REG> EM23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM23::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM23::B0x1)
    }
}
/**CPU wakeup with event generation mask on line 25 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM25 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM25> for bool {
    #[inline(always)]
    fn from(variant: EM25) -> Self {
        variant as u8 != 0
    }
}
///Field `EM25` reader - CPU wakeup with event generation mask on line 25 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM25_R = crate::BitReader<EM25>;
impl EM25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM25 {
        match self.bits {
            false => EM25::B0x0,
            true => EM25::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM25::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM25::B0x1
    }
}
///Field `EM25` writer - CPU wakeup with event generation mask on line 25 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM25_W<'a, REG> = crate::BitWriter<'a, REG, EM25>;
impl<'a, REG> EM25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM25::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM25::B0x1)
    }
}
/**CPU wakeup with event generation mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM31 {
    ///0: wakeup with event generation masked
    B0x0 = 0,
    ///1: wakeup with event generation unmasked
    B0x1 = 1,
}
impl From<EM31> for bool {
    #[inline(always)]
    fn from(variant: EM31) -> Self {
        variant as u8 != 0
    }
}
///Field `EM31` reader - CPU wakeup with event generation mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM31_R = crate::BitReader<EM31>;
impl EM31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM31 {
        match self.bits {
            false => EM31::B0x0,
            true => EM31::B0x1,
        }
    }
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EM31::B0x0
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EM31::B0x1
    }
}
///Field `EM31` writer - CPU wakeup with event generation mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
pub type EM31_W<'a, REG> = crate::BitWriter<'a, REG, EM31>;
impl<'a, REG> EM31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with event generation masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM31::B0x0)
    }
    ///wakeup with event generation unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM31::B0x1)
    }
}
impl R {
    ///Bit 0 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - CPU wakeup with event generation mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - CPU wakeup with event generation mask on line 23 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CPU wakeup with event generation mask on line 25 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - CPU wakeup with event generation mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em31(&self) -> EM31_R {
        EM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EMR1")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("em4", &self.em4())
            .field("em5", &self.em5())
            .field("em6", &self.em6())
            .field("em7", &self.em7())
            .field("em8", &self.em8())
            .field("em9", &self.em9())
            .field("em10", &self.em10())
            .field("em11", &self.em11())
            .field("em12", &self.em12())
            .field("em13", &self.em13())
            .field("em14", &self.em14())
            .field("em15", &self.em15())
            .field("em19", &self.em19())
            .field("em23", &self.em23())
            .field("em25", &self.em25())
            .field("em31", &self.em31())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<'_, EXTI_EMR1rs> {
        EM0_W::new(self, 0)
    }
    ///Bit 1 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<'_, EXTI_EMR1rs> {
        EM1_W::new(self, 1)
    }
    ///Bit 2 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<'_, EXTI_EMR1rs> {
        EM2_W::new(self, 2)
    }
    ///Bit 3 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<'_, EXTI_EMR1rs> {
        EM3_W::new(self, 3)
    }
    ///Bit 4 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<'_, EXTI_EMR1rs> {
        EM4_W::new(self, 4)
    }
    ///Bit 5 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<'_, EXTI_EMR1rs> {
        EM5_W::new(self, 5)
    }
    ///Bit 6 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<'_, EXTI_EMR1rs> {
        EM6_W::new(self, 6)
    }
    ///Bit 7 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<'_, EXTI_EMR1rs> {
        EM7_W::new(self, 7)
    }
    ///Bit 8 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<'_, EXTI_EMR1rs> {
        EM8_W::new(self, 8)
    }
    ///Bit 9 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<'_, EXTI_EMR1rs> {
        EM9_W::new(self, 9)
    }
    ///Bit 10 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<'_, EXTI_EMR1rs> {
        EM10_W::new(self, 10)
    }
    ///Bit 11 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<'_, EXTI_EMR1rs> {
        EM11_W::new(self, 11)
    }
    ///Bit 12 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<'_, EXTI_EMR1rs> {
        EM12_W::new(self, 12)
    }
    ///Bit 13 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<'_, EXTI_EMR1rs> {
        EM13_W::new(self, 13)
    }
    ///Bit 14 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<'_, EXTI_EMR1rs> {
        EM14_W::new(self, 14)
    }
    ///Bit 15 - CPU wakeup with event generation mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<'_, EXTI_EMR1rs> {
        EM15_W::new(self, 15)
    }
    ///Bit 19 - CPU wakeup with event generation mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<'_, EXTI_EMR1rs> {
        EM19_W::new(self, 19)
    }
    ///Bit 23 - CPU wakeup with event generation mask on line 23 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em23(&mut self) -> EM23_W<'_, EXTI_EMR1rs> {
        EM23_W::new(self, 23)
    }
    ///Bit 25 - CPU wakeup with event generation mask on line 25 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em25(&mut self) -> EM25_W<'_, EXTI_EMR1rs> {
        EM25_W::new(self, 25)
    }
    ///Bit 31 - CPU wakeup with event generation mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with event generation on the corresponding line.
    #[inline(always)]
    pub fn em31(&mut self) -> EM31_W<'_, EXTI_EMR1rs> {
        EM31_W::new(self, 31)
    }
}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`exti_emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#EXTI:EXTI_EMR1)*/
pub struct EXTI_EMR1rs;
impl crate::RegisterSpec for EXTI_EMR1rs {
    type Ux = u32;
}
///`read()` method returns [`exti_emr1::R`](R) reader structure
impl crate::Readable for EXTI_EMR1rs {}
///`write(|w| ..)` method takes [`exti_emr1::W`](W) writer structure
impl crate::Writable for EXTI_EMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_EMR1 to value 0
impl crate::Resettable for EXTI_EMR1rs {}
