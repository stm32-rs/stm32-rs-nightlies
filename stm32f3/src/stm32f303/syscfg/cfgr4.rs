///Register `CFGR4` reader
pub type R = crate::R<CFGR4rs>;
///Register `CFGR4` writer
pub type W = crate::W<CFGR4rs>;
/**Controls the Input trigger of ADC12 regular channel EXT2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT2_RMP {
    ///0: Trigger source is TIM3_CC3
    Tim1 = 0,
    ///1: rigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC12_EXT2_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT2_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_EXT2_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT2
pub type ADC12_EXT2_RMP_R = crate::BitReader<ADC12_EXT2_RMP>;
impl ADC12_EXT2_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT2_RMP {
        match self.bits {
            false => ADC12_EXT2_RMP::Tim1,
            true => ADC12_EXT2_RMP::Tim20,
        }
    }
    ///Trigger source is TIM3_CC3
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC12_EXT2_RMP::Tim1
    }
    ///rigger source is TIM20_TRGO
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT2_RMP::Tim20
    }
}
///Field `ADC12_EXT2_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT2
pub type ADC12_EXT2_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT2_RMP>;
impl<'a, REG> ADC12_EXT2_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM3_CC3
    #[inline(always)]
    pub fn tim1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT2_RMP::Tim1)
    }
    ///rigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT2_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 regular channel EXT3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT3_RMP {
    ///0: Trigger source is TIM2_CC2
    Tim2 = 0,
    ///1: rigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC12_EXT3_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT3_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_EXT3_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT3
pub type ADC12_EXT3_RMP_R = crate::BitReader<ADC12_EXT3_RMP>;
impl ADC12_EXT3_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT3_RMP {
        match self.bits {
            false => ADC12_EXT3_RMP::Tim2,
            true => ADC12_EXT3_RMP::Tim20,
        }
    }
    ///Trigger source is TIM2_CC2
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_EXT3_RMP::Tim2
    }
    ///rigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT3_RMP::Tim20
    }
}
///Field `ADC12_EXT3_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT3
pub type ADC12_EXT3_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT3_RMP>;
impl<'a, REG> ADC12_EXT3_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM2_CC2
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT3_RMP::Tim2)
    }
    ///rigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT3_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 regular channel EXT5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT5_RMP {
    ///0: Trigger source is TIM4_CC4
    Tim4 = 0,
    ///1: Trigger source is TIM20_CC1
    Tim20 = 1,
}
impl From<ADC12_EXT5_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT5_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_EXT5_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT5
pub type ADC12_EXT5_RMP_R = crate::BitReader<ADC12_EXT5_RMP>;
impl ADC12_EXT5_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT5_RMP {
        match self.bits {
            false => ADC12_EXT5_RMP::Tim4,
            true => ADC12_EXT5_RMP::Tim20,
        }
    }
    ///Trigger source is TIM4_CC4
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC12_EXT5_RMP::Tim4
    }
    ///Trigger source is TIM20_CC1
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT5_RMP::Tim20
    }
}
///Field `ADC12_EXT5_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT5
pub type ADC12_EXT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT5_RMP>;
impl<'a, REG> ADC12_EXT5_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM4_CC4
    #[inline(always)]
    pub fn tim4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT5_RMP::Tim4)
    }
    ///Trigger source is TIM20_CC1
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT5_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 regular channel EXT13

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT13_RMP {
    ///0: Trigger source is TIM6_TRGO
    Tim6 = 0,
    ///1: Trigger source is TIM20_CC2
    Tim20 = 1,
}
impl From<ADC12_EXT13_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT13_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_EXT13_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT13
pub type ADC12_EXT13_RMP_R = crate::BitReader<ADC12_EXT13_RMP>;
impl ADC12_EXT13_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT13_RMP {
        match self.bits {
            false => ADC12_EXT13_RMP::Tim6,
            true => ADC12_EXT13_RMP::Tim20,
        }
    }
    ///Trigger source is TIM6_TRGO
    #[inline(always)]
    pub fn is_tim6(&self) -> bool {
        *self == ADC12_EXT13_RMP::Tim6
    }
    ///Trigger source is TIM20_CC2
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT13_RMP::Tim20
    }
}
///Field `ADC12_EXT13_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT13
pub type ADC12_EXT13_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT13_RMP>;
impl<'a, REG> ADC12_EXT13_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM6_TRGO
    #[inline(always)]
    pub fn tim6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT13_RMP::Tim6)
    }
    ///Trigger source is TIM20_CC2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT13_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 regular channel EXT15

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT15_RMP {
    ///0: Trigger source is TIM3_CC4
    Tim3 = 0,
    ///1: Trigger source is TIM20_CC3
    Tim20 = 1,
}
impl From<ADC12_EXT15_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT15_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_EXT15_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT15
pub type ADC12_EXT15_RMP_R = crate::BitReader<ADC12_EXT15_RMP>;
impl ADC12_EXT15_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT15_RMP {
        match self.bits {
            false => ADC12_EXT15_RMP::Tim3,
            true => ADC12_EXT15_RMP::Tim20,
        }
    }
    ///Trigger source is TIM3_CC4
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_EXT15_RMP::Tim3
    }
    ///Trigger source is TIM20_CC3
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT15_RMP::Tim20
    }
}
///Field `ADC12_EXT15_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT15
pub type ADC12_EXT15_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT15_RMP>;
impl<'a, REG> ADC12_EXT15_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM3_CC4
    #[inline(always)]
    pub fn tim3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT15_RMP::Tim3)
    }
    ///Trigger source is TIM20_CC3
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT15_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 injected channel EXTI3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT3_RMP {
    ///0: Trigger source is TIM2_CC1
    Tim2 = 0,
    ///1: Trigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC12_JEXT3_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT3_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_JEXT3_RMP` reader - Controls the Input trigger of ADC12 injected channel EXTI3
pub type ADC12_JEXT3_RMP_R = crate::BitReader<ADC12_JEXT3_RMP>;
impl ADC12_JEXT3_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_JEXT3_RMP {
        match self.bits {
            false => ADC12_JEXT3_RMP::Tim2,
            true => ADC12_JEXT3_RMP::Tim20,
        }
    }
    ///Trigger source is TIM2_CC1
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_JEXT3_RMP::Tim2
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT3_RMP::Tim20
    }
}
///Field `ADC12_JEXT3_RMP` writer - Controls the Input trigger of ADC12 injected channel EXTI3
pub type ADC12_JEXT3_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_JEXT3_RMP>;
impl<'a, REG> ADC12_JEXT3_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM2_CC1
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT3_RMP::Tim2)
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT3_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 injected channel EXTI6

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT6_RMP {
    ///0: Trigger source is EXTI line 15
    Exti15 = 0,
    ///1: Trigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC12_JEXT6_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT6_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_JEXT6_RMP` reader - Controls the Input trigger of ADC12 injected channel EXTI6
pub type ADC12_JEXT6_RMP_R = crate::BitReader<ADC12_JEXT6_RMP>;
impl ADC12_JEXT6_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_JEXT6_RMP {
        match self.bits {
            false => ADC12_JEXT6_RMP::Exti15,
            true => ADC12_JEXT6_RMP::Tim20,
        }
    }
    ///Trigger source is EXTI line 15
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == ADC12_JEXT6_RMP::Exti15
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT6_RMP::Tim20
    }
}
///Field `ADC12_JEXT6_RMP` writer - Controls the Input trigger of ADC12 injected channel EXTI6
pub type ADC12_JEXT6_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_JEXT6_RMP>;
impl<'a, REG> ADC12_JEXT6_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT6_RMP::Exti15)
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT6_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC12 injected channel EXTI13

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT13_RMP {
    ///0: Trigger source is TIM3_CC1
    Tim3 = 0,
    ///1: Trigger source is TIM20_CC4
    Tim20 = 1,
}
impl From<ADC12_JEXT13_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT13_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC12_JEXT13_RMP` reader - Controls the Input trigger of ADC12 injected channel EXTI13
pub type ADC12_JEXT13_RMP_R = crate::BitReader<ADC12_JEXT13_RMP>;
impl ADC12_JEXT13_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_JEXT13_RMP {
        match self.bits {
            false => ADC12_JEXT13_RMP::Tim3,
            true => ADC12_JEXT13_RMP::Tim20,
        }
    }
    ///Trigger source is TIM3_CC1
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_JEXT13_RMP::Tim3
    }
    ///Trigger source is TIM20_CC4
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT13_RMP::Tim20
    }
}
///Field `ADC12_JEXT13_RMP` writer - Controls the Input trigger of ADC12 injected channel EXTI13
pub type ADC12_JEXT13_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_JEXT13_RMP>;
impl<'a, REG> ADC12_JEXT13_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM3_CC1
    #[inline(always)]
    pub fn tim3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT13_RMP::Tim3)
    }
    ///Trigger source is TIM20_CC4
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT13_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC34 regular channel EXT5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT5_RMP {
    ///0: Trigger source is EXTI line 2 when reset at 0
    Exti2 = 0,
    ///1: Trigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC34_EXT5_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT5_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC34_EXT5_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT5
pub type ADC34_EXT5_RMP_R = crate::BitReader<ADC34_EXT5_RMP>;
impl ADC34_EXT5_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_EXT5_RMP {
        match self.bits {
            false => ADC34_EXT5_RMP::Exti2,
            true => ADC34_EXT5_RMP::Tim20,
        }
    }
    ///Trigger source is EXTI line 2 when reset at 0
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == ADC34_EXT5_RMP::Exti2
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT5_RMP::Tim20
    }
}
///Field `ADC34_EXT5_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT5
pub type ADC34_EXT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_EXT5_RMP>;
impl<'a, REG> ADC34_EXT5_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is EXTI line 2 when reset at 0
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT5_RMP::Exti2)
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT5_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC34 regular channel EXT6

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT6_RMP {
    ///0: Trigger source is TIM4_CC1
    Tim4 = 0,
    ///1: Trigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC34_EXT6_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT6_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC34_EXT6_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT6
pub type ADC34_EXT6_RMP_R = crate::BitReader<ADC34_EXT6_RMP>;
impl ADC34_EXT6_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_EXT6_RMP {
        match self.bits {
            false => ADC34_EXT6_RMP::Tim4,
            true => ADC34_EXT6_RMP::Tim20,
        }
    }
    ///Trigger source is TIM4_CC1
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_EXT6_RMP::Tim4
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT6_RMP::Tim20
    }
}
///Field `ADC34_EXT6_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT6
pub type ADC34_EXT6_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_EXT6_RMP>;
impl<'a, REG> ADC34_EXT6_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM4_CC1
    #[inline(always)]
    pub fn tim4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT6_RMP::Tim4)
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT6_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC34 regular channel EXT15

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT15_RMP {
    ///0: Trigger source is TIM2_CC1
    Tim2 = 0,
    ///1: Trigger source is TIM20_CC1
    Tim20 = 1,
}
impl From<ADC34_EXT15_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT15_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC34_EXT15_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT15
pub type ADC34_EXT15_RMP_R = crate::BitReader<ADC34_EXT15_RMP>;
impl ADC34_EXT15_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_EXT15_RMP {
        match self.bits {
            false => ADC34_EXT15_RMP::Tim2,
            true => ADC34_EXT15_RMP::Tim20,
        }
    }
    ///Trigger source is TIM2_CC1
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC34_EXT15_RMP::Tim2
    }
    ///Trigger source is TIM20_CC1
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT15_RMP::Tim20
    }
}
///Field `ADC34_EXT15_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT15
pub type ADC34_EXT15_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_EXT15_RMP>;
impl<'a, REG> ADC34_EXT15_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM2_CC1
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT15_RMP::Tim2)
    }
    ///Trigger source is TIM20_CC1
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT15_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC34 injected channel JEXT5

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT5_RMP {
    ///0: Trigger source is TIM4_CC3
    Tim4 = 0,
    ///1: Trigger source is TIM20_TRGO
    Tim20 = 1,
}
impl From<ADC34_JEXT5_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT5_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC34_JEXT5_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT5
pub type ADC34_JEXT5_RMP_R = crate::BitReader<ADC34_JEXT5_RMP>;
impl ADC34_JEXT5_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_JEXT5_RMP {
        match self.bits {
            false => ADC34_JEXT5_RMP::Tim4,
            true => ADC34_JEXT5_RMP::Tim20,
        }
    }
    ///Trigger source is TIM4_CC3
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_JEXT5_RMP::Tim4
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT5_RMP::Tim20
    }
}
///Field `ADC34_JEXT5_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT5
pub type ADC34_JEXT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_JEXT5_RMP>;
impl<'a, REG> ADC34_JEXT5_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM4_CC3
    #[inline(always)]
    pub fn tim4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT5_RMP::Tim4)
    }
    ///Trigger source is TIM20_TRGO
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT5_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC34 injected channel JEXT11

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT11_RMP {
    ///0: Trigger source is TIM1_CC3
    Tim1 = 0,
    ///1: Trigger source is TIM20_TRGO2
    Tim20 = 1,
}
impl From<ADC34_JEXT11_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT11_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC34_JEXT11_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT11
pub type ADC34_JEXT11_RMP_R = crate::BitReader<ADC34_JEXT11_RMP>;
impl ADC34_JEXT11_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_JEXT11_RMP {
        match self.bits {
            false => ADC34_JEXT11_RMP::Tim1,
            true => ADC34_JEXT11_RMP::Tim20,
        }
    }
    ///Trigger source is TIM1_CC3
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC34_JEXT11_RMP::Tim1
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT11_RMP::Tim20
    }
}
///Field `ADC34_JEXT11_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT11
pub type ADC34_JEXT11_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_JEXT11_RMP>;
impl<'a, REG> ADC34_JEXT11_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM1_CC3
    #[inline(always)]
    pub fn tim1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT11_RMP::Tim1)
    }
    ///Trigger source is TIM20_TRGO2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT11_RMP::Tim20)
    }
}
/**Controls the Input trigger of ADC34 injected channel JEXT14

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT14_RMP {
    ///0: Trigger source is TIM7_TRGO
    Tim7 = 0,
    ///1: Trigger source is TIM20_CC2
    Tim20 = 1,
}
impl From<ADC34_JEXT14_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT14_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC34_JEXT14_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT14
pub type ADC34_JEXT14_RMP_R = crate::BitReader<ADC34_JEXT14_RMP>;
impl ADC34_JEXT14_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_JEXT14_RMP {
        match self.bits {
            false => ADC34_JEXT14_RMP::Tim7,
            true => ADC34_JEXT14_RMP::Tim20,
        }
    }
    ///Trigger source is TIM7_TRGO
    #[inline(always)]
    pub fn is_tim7(&self) -> bool {
        *self == ADC34_JEXT14_RMP::Tim7
    }
    ///Trigger source is TIM20_CC2
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT14_RMP::Tim20
    }
}
///Field `ADC34_JEXT14_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT14
pub type ADC34_JEXT14_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_JEXT14_RMP>;
impl<'a, REG> ADC34_JEXT14_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger source is TIM7_TRGO
    #[inline(always)]
    pub fn tim7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT14_RMP::Tim7)
    }
    ///Trigger source is TIM20_CC2
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT14_RMP::Tim20)
    }
}
impl R {
    ///Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2
    #[inline(always)]
    pub fn adc12_ext2_rmp(&self) -> ADC12_EXT2_RMP_R {
        ADC12_EXT2_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3
    #[inline(always)]
    pub fn adc12_ext3_rmp(&self) -> ADC12_EXT3_RMP_R {
        ADC12_EXT3_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5
    #[inline(always)]
    pub fn adc12_ext5_rmp(&self) -> ADC12_EXT5_RMP_R {
        ADC12_EXT5_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13
    #[inline(always)]
    pub fn adc12_ext13_rmp(&self) -> ADC12_EXT13_RMP_R {
        ADC12_EXT13_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15
    #[inline(always)]
    pub fn adc12_ext15_rmp(&self) -> ADC12_EXT15_RMP_R {
        ADC12_EXT15_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Controls the Input trigger of ADC12 injected channel EXTI3
    #[inline(always)]
    pub fn adc12_jext3_rmp(&self) -> ADC12_JEXT3_RMP_R {
        ADC12_JEXT3_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Controls the Input trigger of ADC12 injected channel EXTI6
    #[inline(always)]
    pub fn adc12_jext6_rmp(&self) -> ADC12_JEXT6_RMP_R {
        ADC12_JEXT6_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Controls the Input trigger of ADC12 injected channel EXTI13
    #[inline(always)]
    pub fn adc12_jext13_rmp(&self) -> ADC12_JEXT13_RMP_R {
        ADC12_JEXT13_RMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5
    #[inline(always)]
    pub fn adc34_ext5_rmp(&self) -> ADC34_EXT5_RMP_R {
        ADC34_EXT5_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6
    #[inline(always)]
    pub fn adc34_ext6_rmp(&self) -> ADC34_EXT6_RMP_R {
        ADC34_EXT6_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15
    #[inline(always)]
    pub fn adc34_ext15_rmp(&self) -> ADC34_EXT15_RMP_R {
        ADC34_EXT15_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5
    #[inline(always)]
    pub fn adc34_jext5_rmp(&self) -> ADC34_JEXT5_RMP_R {
        ADC34_JEXT5_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11
    #[inline(always)]
    pub fn adc34_jext11_rmp(&self) -> ADC34_JEXT11_RMP_R {
        ADC34_JEXT11_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14
    #[inline(always)]
    pub fn adc34_jext14_rmp(&self) -> ADC34_JEXT14_RMP_R {
        ADC34_JEXT14_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR4")
            .field("adc12_ext2_rmp", &self.adc12_ext2_rmp())
            .field("adc12_ext3_rmp", &self.adc12_ext3_rmp())
            .field("adc12_ext5_rmp", &self.adc12_ext5_rmp())
            .field("adc12_ext13_rmp", &self.adc12_ext13_rmp())
            .field("adc12_ext15_rmp", &self.adc12_ext15_rmp())
            .field("adc12_jext3_rmp", &self.adc12_jext3_rmp())
            .field("adc12_jext6_rmp", &self.adc12_jext6_rmp())
            .field("adc12_jext13_rmp", &self.adc12_jext13_rmp())
            .field("adc34_ext5_rmp", &self.adc34_ext5_rmp())
            .field("adc34_ext6_rmp", &self.adc34_ext6_rmp())
            .field("adc34_ext15_rmp", &self.adc34_ext15_rmp())
            .field("adc34_jext5_rmp", &self.adc34_jext5_rmp())
            .field("adc34_jext11_rmp", &self.adc34_jext11_rmp())
            .field("adc34_jext14_rmp", &self.adc34_jext14_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2
    #[inline(always)]
    pub fn adc12_ext2_rmp(&mut self) -> ADC12_EXT2_RMP_W<'_, CFGR4rs> {
        ADC12_EXT2_RMP_W::new(self, 0)
    }
    ///Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3
    #[inline(always)]
    pub fn adc12_ext3_rmp(&mut self) -> ADC12_EXT3_RMP_W<'_, CFGR4rs> {
        ADC12_EXT3_RMP_W::new(self, 1)
    }
    ///Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5
    #[inline(always)]
    pub fn adc12_ext5_rmp(&mut self) -> ADC12_EXT5_RMP_W<'_, CFGR4rs> {
        ADC12_EXT5_RMP_W::new(self, 2)
    }
    ///Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13
    #[inline(always)]
    pub fn adc12_ext13_rmp(&mut self) -> ADC12_EXT13_RMP_W<'_, CFGR4rs> {
        ADC12_EXT13_RMP_W::new(self, 3)
    }
    ///Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15
    #[inline(always)]
    pub fn adc12_ext15_rmp(&mut self) -> ADC12_EXT15_RMP_W<'_, CFGR4rs> {
        ADC12_EXT15_RMP_W::new(self, 4)
    }
    ///Bit 5 - Controls the Input trigger of ADC12 injected channel EXTI3
    #[inline(always)]
    pub fn adc12_jext3_rmp(&mut self) -> ADC12_JEXT3_RMP_W<'_, CFGR4rs> {
        ADC12_JEXT3_RMP_W::new(self, 5)
    }
    ///Bit 6 - Controls the Input trigger of ADC12 injected channel EXTI6
    #[inline(always)]
    pub fn adc12_jext6_rmp(&mut self) -> ADC12_JEXT6_RMP_W<'_, CFGR4rs> {
        ADC12_JEXT6_RMP_W::new(self, 6)
    }
    ///Bit 7 - Controls the Input trigger of ADC12 injected channel EXTI13
    #[inline(always)]
    pub fn adc12_jext13_rmp(&mut self) -> ADC12_JEXT13_RMP_W<'_, CFGR4rs> {
        ADC12_JEXT13_RMP_W::new(self, 7)
    }
    ///Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5
    #[inline(always)]
    pub fn adc34_ext5_rmp(&mut self) -> ADC34_EXT5_RMP_W<'_, CFGR4rs> {
        ADC34_EXT5_RMP_W::new(self, 8)
    }
    ///Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6
    #[inline(always)]
    pub fn adc34_ext6_rmp(&mut self) -> ADC34_EXT6_RMP_W<'_, CFGR4rs> {
        ADC34_EXT6_RMP_W::new(self, 9)
    }
    ///Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15
    #[inline(always)]
    pub fn adc34_ext15_rmp(&mut self) -> ADC34_EXT15_RMP_W<'_, CFGR4rs> {
        ADC34_EXT15_RMP_W::new(self, 10)
    }
    ///Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5
    #[inline(always)]
    pub fn adc34_jext5_rmp(&mut self) -> ADC34_JEXT5_RMP_W<'_, CFGR4rs> {
        ADC34_JEXT5_RMP_W::new(self, 11)
    }
    ///Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11
    #[inline(always)]
    pub fn adc34_jext11_rmp(&mut self) -> ADC34_JEXT11_RMP_W<'_, CFGR4rs> {
        ADC34_JEXT11_RMP_W::new(self, 12)
    }
    ///Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14
    #[inline(always)]
    pub fn adc34_jext14_rmp(&mut self) -> ADC34_JEXT14_RMP_W<'_, CFGR4rs> {
        ADC34_JEXT14_RMP_W::new(self, 13)
    }
}
/**SYSCFG configuration register 4

You can [`read`](crate::Reg::read) this register and get [`cfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#SYSCFG:CFGR4)*/
pub struct CFGR4rs;
impl crate::RegisterSpec for CFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr4::R`](R) reader structure
impl crate::Readable for CFGR4rs {}
///`write(|w| ..)` method takes [`cfgr4::W`](W) writer structure
impl crate::Writable for CFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR4 to value 0
impl crate::Resettable for CFGR4rs {}
