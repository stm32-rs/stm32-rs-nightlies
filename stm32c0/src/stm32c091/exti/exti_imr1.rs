///Register `EXTI_IMR1` reader
pub type R = crate::R<EXTI_IMR1rs>;
///Register `EXTI_IMR1` writer
pub type W = crate::W<EXTI_IMR1rs>;
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM0 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM0> for bool {
    #[inline(always)]
    fn from(variant: IM0) -> Self {
        variant as u8 != 0
    }
}
///Field `IM0` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM0_R = crate::BitReader<IM0>;
impl IM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM0 {
        match self.bits {
            false => IM0::B0x0,
            true => IM0::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM0::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM0::B0x1
    }
}
///Field `IM0` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM0_W<'a, REG> = crate::BitWriter<'a, REG, IM0>;
impl<'a, REG> IM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM0::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM0::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM1 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM1> for bool {
    #[inline(always)]
    fn from(variant: IM1) -> Self {
        variant as u8 != 0
    }
}
///Field `IM1` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM1_R = crate::BitReader<IM1>;
impl IM1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM1 {
        match self.bits {
            false => IM1::B0x0,
            true => IM1::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM1::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM1::B0x1
    }
}
///Field `IM1` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM1_W<'a, REG> = crate::BitWriter<'a, REG, IM1>;
impl<'a, REG> IM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM1::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM1::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM2 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM2> for bool {
    #[inline(always)]
    fn from(variant: IM2) -> Self {
        variant as u8 != 0
    }
}
///Field `IM2` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM2_R = crate::BitReader<IM2>;
impl IM2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM2 {
        match self.bits {
            false => IM2::B0x0,
            true => IM2::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM2::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM2::B0x1
    }
}
///Field `IM2` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM2_W<'a, REG> = crate::BitWriter<'a, REG, IM2>;
impl<'a, REG> IM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM2::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM2::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM3 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM3> for bool {
    #[inline(always)]
    fn from(variant: IM3) -> Self {
        variant as u8 != 0
    }
}
///Field `IM3` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM3_R = crate::BitReader<IM3>;
impl IM3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM3 {
        match self.bits {
            false => IM3::B0x0,
            true => IM3::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM3::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM3::B0x1
    }
}
///Field `IM3` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM3_W<'a, REG> = crate::BitWriter<'a, REG, IM3>;
impl<'a, REG> IM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM3::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM3::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM4 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM4> for bool {
    #[inline(always)]
    fn from(variant: IM4) -> Self {
        variant as u8 != 0
    }
}
///Field `IM4` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM4_R = crate::BitReader<IM4>;
impl IM4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM4 {
        match self.bits {
            false => IM4::B0x0,
            true => IM4::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM4::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM4::B0x1
    }
}
///Field `IM4` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM4_W<'a, REG> = crate::BitWriter<'a, REG, IM4>;
impl<'a, REG> IM4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM4::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM4::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM5 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM5> for bool {
    #[inline(always)]
    fn from(variant: IM5) -> Self {
        variant as u8 != 0
    }
}
///Field `IM5` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM5_R = crate::BitReader<IM5>;
impl IM5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM5 {
        match self.bits {
            false => IM5::B0x0,
            true => IM5::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM5::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM5::B0x1
    }
}
///Field `IM5` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM5_W<'a, REG> = crate::BitWriter<'a, REG, IM5>;
impl<'a, REG> IM5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM5::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM5::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM6 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM6> for bool {
    #[inline(always)]
    fn from(variant: IM6) -> Self {
        variant as u8 != 0
    }
}
///Field `IM6` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM6_R = crate::BitReader<IM6>;
impl IM6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM6 {
        match self.bits {
            false => IM6::B0x0,
            true => IM6::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM6::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM6::B0x1
    }
}
///Field `IM6` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM6_W<'a, REG> = crate::BitWriter<'a, REG, IM6>;
impl<'a, REG> IM6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM6::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM6::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM7 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM7> for bool {
    #[inline(always)]
    fn from(variant: IM7) -> Self {
        variant as u8 != 0
    }
}
///Field `IM7` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM7_R = crate::BitReader<IM7>;
impl IM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM7 {
        match self.bits {
            false => IM7::B0x0,
            true => IM7::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM7::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM7::B0x1
    }
}
///Field `IM7` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM7_W<'a, REG> = crate::BitWriter<'a, REG, IM7>;
impl<'a, REG> IM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM7::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM7::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM8 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM8> for bool {
    #[inline(always)]
    fn from(variant: IM8) -> Self {
        variant as u8 != 0
    }
}
///Field `IM8` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM8_R = crate::BitReader<IM8>;
impl IM8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM8 {
        match self.bits {
            false => IM8::B0x0,
            true => IM8::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM8::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM8::B0x1
    }
}
///Field `IM8` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM8_W<'a, REG> = crate::BitWriter<'a, REG, IM8>;
impl<'a, REG> IM8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM8::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM8::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM9 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM9> for bool {
    #[inline(always)]
    fn from(variant: IM9) -> Self {
        variant as u8 != 0
    }
}
///Field `IM9` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM9_R = crate::BitReader<IM9>;
impl IM9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM9 {
        match self.bits {
            false => IM9::B0x0,
            true => IM9::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM9::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM9::B0x1
    }
}
///Field `IM9` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM9_W<'a, REG> = crate::BitWriter<'a, REG, IM9>;
impl<'a, REG> IM9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM9::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM9::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM10 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM10> for bool {
    #[inline(always)]
    fn from(variant: IM10) -> Self {
        variant as u8 != 0
    }
}
///Field `IM10` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM10_R = crate::BitReader<IM10>;
impl IM10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM10 {
        match self.bits {
            false => IM10::B0x0,
            true => IM10::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM10::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM10::B0x1
    }
}
///Field `IM10` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM10_W<'a, REG> = crate::BitWriter<'a, REG, IM10>;
impl<'a, REG> IM10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM10::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM10::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM11 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM11> for bool {
    #[inline(always)]
    fn from(variant: IM11) -> Self {
        variant as u8 != 0
    }
}
///Field `IM11` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM11_R = crate::BitReader<IM11>;
impl IM11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM11 {
        match self.bits {
            false => IM11::B0x0,
            true => IM11::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM11::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM11::B0x1
    }
}
///Field `IM11` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM11_W<'a, REG> = crate::BitWriter<'a, REG, IM11>;
impl<'a, REG> IM11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM11::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM11::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM12 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM12> for bool {
    #[inline(always)]
    fn from(variant: IM12) -> Self {
        variant as u8 != 0
    }
}
///Field `IM12` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM12_R = crate::BitReader<IM12>;
impl IM12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM12 {
        match self.bits {
            false => IM12::B0x0,
            true => IM12::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM12::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM12::B0x1
    }
}
///Field `IM12` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM12_W<'a, REG> = crate::BitWriter<'a, REG, IM12>;
impl<'a, REG> IM12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM12::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM12::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM13 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM13> for bool {
    #[inline(always)]
    fn from(variant: IM13) -> Self {
        variant as u8 != 0
    }
}
///Field `IM13` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM13_R = crate::BitReader<IM13>;
impl IM13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM13 {
        match self.bits {
            false => IM13::B0x0,
            true => IM13::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM13::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM13::B0x1
    }
}
///Field `IM13` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM13_W<'a, REG> = crate::BitWriter<'a, REG, IM13>;
impl<'a, REG> IM13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM13::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM13::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM14 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM14> for bool {
    #[inline(always)]
    fn from(variant: IM14) -> Self {
        variant as u8 != 0
    }
}
///Field `IM14` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM14_R = crate::BitReader<IM14>;
impl IM14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM14 {
        match self.bits {
            false => IM14::B0x0,
            true => IM14::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM14::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM14::B0x1
    }
}
///Field `IM14` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM14_W<'a, REG> = crate::BitWriter<'a, REG, IM14>;
impl<'a, REG> IM14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM14::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM14::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM15 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM15> for bool {
    #[inline(always)]
    fn from(variant: IM15) -> Self {
        variant as u8 != 0
    }
}
///Field `IM15` reader - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM15_R = crate::BitReader<IM15>;
impl IM15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM15 {
        match self.bits {
            false => IM15::B0x0,
            true => IM15::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM15::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM15::B0x1
    }
}
///Field `IM15` writer - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM15_W<'a, REG> = crate::BitWriter<'a, REG, IM15>;
impl<'a, REG> IM15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM15::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM15::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM19 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM19> for bool {
    #[inline(always)]
    fn from(variant: IM19) -> Self {
        variant as u8 != 0
    }
}
///Field `IM19` reader - CPU wakeup with interrupt mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM19_R = crate::BitReader<IM19>;
impl IM19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM19 {
        match self.bits {
            false => IM19::B0x0,
            true => IM19::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM19::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM19::B0x1
    }
}
///Field `IM19` writer - CPU wakeup with interrupt mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM19_W<'a, REG> = crate::BitWriter<'a, REG, IM19>;
impl<'a, REG> IM19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM19::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM19::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line 23 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM23 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM23> for bool {
    #[inline(always)]
    fn from(variant: IM23) -> Self {
        variant as u8 != 0
    }
}
///Field `IM23` reader - CPU wakeup with interrupt mask on line 23 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM23_R = crate::BitReader<IM23>;
impl IM23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM23 {
        match self.bits {
            false => IM23::B0x0,
            true => IM23::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM23::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM23::B0x1
    }
}
///Field `IM23` writer - CPU wakeup with interrupt mask on line 23 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM23_W<'a, REG> = crate::BitWriter<'a, REG, IM23>;
impl<'a, REG> IM23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM23::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM23::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line 25 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM25 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM25> for bool {
    #[inline(always)]
    fn from(variant: IM25) -> Self {
        variant as u8 != 0
    }
}
///Field `IM25` reader - CPU wakeup with interrupt mask on line 25 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM25_R = crate::BitReader<IM25>;
impl IM25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM25 {
        match self.bits {
            false => IM25::B0x0,
            true => IM25::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM25::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM25::B0x1
    }
}
///Field `IM25` writer - CPU wakeup with interrupt mask on line 25 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM25_W<'a, REG> = crate::BitWriter<'a, REG, IM25>;
impl<'a, REG> IM25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM25::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM25::B0x1)
    }
}
/**CPU wakeup with interrupt mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM31 {
    ///0: wakeup with interrupt masked
    B0x0 = 0,
    ///1: wakeup with interrupt unmasked
    B0x1 = 1,
}
impl From<IM31> for bool {
    #[inline(always)]
    fn from(variant: IM31) -> Self {
        variant as u8 != 0
    }
}
///Field `IM31` reader - CPU wakeup with interrupt mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM31_R = crate::BitReader<IM31>;
impl IM31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IM31 {
        match self.bits {
            false => IM31::B0x0,
            true => IM31::B0x1,
        }
    }
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IM31::B0x0
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IM31::B0x1
    }
}
///Field `IM31` writer - CPU wakeup with interrupt mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
pub type IM31_W<'a, REG> = crate::BitWriter<'a, REG, IM31>;
impl<'a, REG> IM31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///wakeup with interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM31::B0x0)
    }
    ///wakeup with interrupt unmasked
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM31::B0x1)
    }
}
impl R {
    ///Bit 0 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - CPU wakeup with interrupt mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - CPU wakeup with interrupt mask on line 23 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CPU wakeup with interrupt mask on line 25 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - CPU wakeup with interrupt mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_IMR1")
            .field("im0", &self.im0())
            .field("im1", &self.im1())
            .field("im2", &self.im2())
            .field("im3", &self.im3())
            .field("im4", &self.im4())
            .field("im5", &self.im5())
            .field("im6", &self.im6())
            .field("im7", &self.im7())
            .field("im8", &self.im8())
            .field("im9", &self.im9())
            .field("im10", &self.im10())
            .field("im11", &self.im11())
            .field("im12", &self.im12())
            .field("im13", &self.im13())
            .field("im14", &self.im14())
            .field("im15", &self.im15())
            .field("im19", &self.im19())
            .field("im23", &self.im23())
            .field("im25", &self.im25())
            .field("im31", &self.im31())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W<'_, EXTI_IMR1rs> {
        IM0_W::new(self, 0)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W<'_, EXTI_IMR1rs> {
        IM1_W::new(self, 1)
    }
    ///Bit 2 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W<'_, EXTI_IMR1rs> {
        IM2_W::new(self, 2)
    }
    ///Bit 3 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W<'_, EXTI_IMR1rs> {
        IM3_W::new(self, 3)
    }
    ///Bit 4 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W<'_, EXTI_IMR1rs> {
        IM4_W::new(self, 4)
    }
    ///Bit 5 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W<'_, EXTI_IMR1rs> {
        IM5_W::new(self, 5)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W<'_, EXTI_IMR1rs> {
        IM6_W::new(self, 6)
    }
    ///Bit 7 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W<'_, EXTI_IMR1rs> {
        IM7_W::new(self, 7)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W<'_, EXTI_IMR1rs> {
        IM8_W::new(self, 8)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W<'_, EXTI_IMR1rs> {
        IM9_W::new(self, 9)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W<'_, EXTI_IMR1rs> {
        IM10_W::new(self, 10)
    }
    ///Bit 11 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W<'_, EXTI_IMR1rs> {
        IM11_W::new(self, 11)
    }
    ///Bit 12 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W<'_, EXTI_IMR1rs> {
        IM12_W::new(self, 12)
    }
    ///Bit 13 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W<'_, EXTI_IMR1rs> {
        IM13_W::new(self, 13)
    }
    ///Bit 14 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W<'_, EXTI_IMR1rs> {
        IM14_W::new(self, 14)
    }
    ///Bit 15 - CPU wakeup with interrupt mask on line x (x = 15 to 0) Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W<'_, EXTI_IMR1rs> {
        IM15_W::new(self, 15)
    }
    ///Bit 19 - CPU wakeup with interrupt mask on line 19 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W<'_, EXTI_IMR1rs> {
        IM19_W::new(self, 19)
    }
    ///Bit 23 - CPU wakeup with interrupt mask on line 23 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W<'_, EXTI_IMR1rs> {
        IM23_W::new(self, 23)
    }
    ///Bit 25 - CPU wakeup with interrupt mask on line 25 Setting/clearing each bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im25(&mut self) -> IM25_W<'_, EXTI_IMR1rs> {
        IM25_W::new(self, 25)
    }
    ///Bit 31 - CPU wakeup with interrupt mask on line 31 Setting/clearing this bit unmasks/masks the CPU wakeup with interrupt, by an event on the corresponding line.
    #[inline(always)]
    pub fn im31(&mut self) -> IM31_W<'_, EXTI_IMR1rs> {
        IM31_W::new(self, 31)
    }
}
/**EXTI CPU wakeup with interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`exti_imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#EXTI:EXTI_IMR1)*/
pub struct EXTI_IMR1rs;
impl crate::RegisterSpec for EXTI_IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`exti_imr1::R`](R) reader structure
impl crate::Readable for EXTI_IMR1rs {}
///`write(|w| ..)` method takes [`exti_imr1::W`](W) writer structure
impl crate::Writable for EXTI_IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_IMR1 to value 0xfff8_0000
impl crate::Resettable for EXTI_IMR1rs {
    const RESET_VALUE: u32 = 0xfff8_0000;
}
