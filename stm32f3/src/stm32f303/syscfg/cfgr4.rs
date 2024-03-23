#[doc = "Register `CFGR4` reader"]
pub type R = crate::R<CFGR4rs>;
#[doc = "Register `CFGR4` writer"]
pub type W = crate::W<CFGR4rs>;
#[doc = "Controls the Input trigger of ADC12 regular channel EXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT2_RMP {
    #[doc = "0: Trigger source is TIM3_CC3"]
    Tim1 = 0,
    #[doc = "1: rigger source is TIM20_TRGO"]
    Tim20 = 1,
}
impl From<ADC12_EXT2_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT2_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_EXT2_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT2"]
pub type ADC12_EXT2_RMP_R = crate::BitReader<ADC12_EXT2_RMP>;
impl ADC12_EXT2_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT2_RMP {
        match self.bits {
            false => ADC12_EXT2_RMP::Tim1,
            true => ADC12_EXT2_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM3_CC3"]
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC12_EXT2_RMP::Tim1
    }
    #[doc = "rigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT2_RMP::Tim20
    }
}
#[doc = "Field `ADC12_EXT2_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT2"]
pub type ADC12_EXT2_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT2_RMP>;
impl<'a, REG> ADC12_EXT2_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM3_CC3"]
    #[inline(always)]
    pub fn tim1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT2_RMP::Tim1)
    }
    #[doc = "rigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT2_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT3_RMP {
    #[doc = "0: Trigger source is TIM2_CC2"]
    Tim2 = 0,
    #[doc = "1: rigger source is TIM20_TRGO2"]
    Tim20 = 1,
}
impl From<ADC12_EXT3_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT3_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_EXT3_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT3"]
pub type ADC12_EXT3_RMP_R = crate::BitReader<ADC12_EXT3_RMP>;
impl ADC12_EXT3_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT3_RMP {
        match self.bits {
            false => ADC12_EXT3_RMP::Tim2,
            true => ADC12_EXT3_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM2_CC2"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_EXT3_RMP::Tim2
    }
    #[doc = "rigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT3_RMP::Tim20
    }
}
#[doc = "Field `ADC12_EXT3_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT3"]
pub type ADC12_EXT3_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT3_RMP>;
impl<'a, REG> ADC12_EXT3_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM2_CC2"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT3_RMP::Tim2)
    }
    #[doc = "rigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT3_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT5_RMP {
    #[doc = "0: Trigger source is TIM4_CC4"]
    Tim4 = 0,
    #[doc = "1: Trigger source is TIM20_CC1"]
    Tim20 = 1,
}
impl From<ADC12_EXT5_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT5_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_EXT5_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT5"]
pub type ADC12_EXT5_RMP_R = crate::BitReader<ADC12_EXT5_RMP>;
impl ADC12_EXT5_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT5_RMP {
        match self.bits {
            false => ADC12_EXT5_RMP::Tim4,
            true => ADC12_EXT5_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM4_CC4"]
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC12_EXT5_RMP::Tim4
    }
    #[doc = "Trigger source is TIM20_CC1"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT5_RMP::Tim20
    }
}
#[doc = "Field `ADC12_EXT5_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT5"]
pub type ADC12_EXT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT5_RMP>;
impl<'a, REG> ADC12_EXT5_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM4_CC4"]
    #[inline(always)]
    pub fn tim4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT5_RMP::Tim4)
    }
    #[doc = "Trigger source is TIM20_CC1"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT5_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT13_RMP {
    #[doc = "0: Trigger source is TIM6_TRGO"]
    Tim6 = 0,
    #[doc = "1: Trigger source is TIM20_CC2"]
    Tim20 = 1,
}
impl From<ADC12_EXT13_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT13_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_EXT13_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT13"]
pub type ADC12_EXT13_RMP_R = crate::BitReader<ADC12_EXT13_RMP>;
impl ADC12_EXT13_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT13_RMP {
        match self.bits {
            false => ADC12_EXT13_RMP::Tim6,
            true => ADC12_EXT13_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM6_TRGO"]
    #[inline(always)]
    pub fn is_tim6(&self) -> bool {
        *self == ADC12_EXT13_RMP::Tim6
    }
    #[doc = "Trigger source is TIM20_CC2"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT13_RMP::Tim20
    }
}
#[doc = "Field `ADC12_EXT13_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT13"]
pub type ADC12_EXT13_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT13_RMP>;
impl<'a, REG> ADC12_EXT13_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM6_TRGO"]
    #[inline(always)]
    pub fn tim6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT13_RMP::Tim6)
    }
    #[doc = "Trigger source is TIM20_CC2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT13_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_EXT15_RMP {
    #[doc = "0: Trigger source is TIM3_CC4"]
    Tim3 = 0,
    #[doc = "1: Trigger source is TIM20_CC3"]
    Tim20 = 1,
}
impl From<ADC12_EXT15_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT15_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_EXT15_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT15"]
pub type ADC12_EXT15_RMP_R = crate::BitReader<ADC12_EXT15_RMP>;
impl ADC12_EXT15_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_EXT15_RMP {
        match self.bits {
            false => ADC12_EXT15_RMP::Tim3,
            true => ADC12_EXT15_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM3_CC4"]
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_EXT15_RMP::Tim3
    }
    #[doc = "Trigger source is TIM20_CC3"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT15_RMP::Tim20
    }
}
#[doc = "Field `ADC12_EXT15_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT15"]
pub type ADC12_EXT15_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_EXT15_RMP>;
impl<'a, REG> ADC12_EXT15_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM3_CC4"]
    #[inline(always)]
    pub fn tim3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT15_RMP::Tim3)
    }
    #[doc = "Trigger source is TIM20_CC3"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_EXT15_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 injected channel JEXT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT3_RMP {
    #[doc = "0: Trigger source is TIM2_CC1"]
    Tim2 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO"]
    Tim20 = 1,
}
impl From<ADC12_JEXT3_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT3_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_JEXT3_RMP` reader - Controls the Input trigger of ADC12 injected channel JEXT3"]
pub type ADC12_JEXT3_RMP_R = crate::BitReader<ADC12_JEXT3_RMP>;
impl ADC12_JEXT3_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_JEXT3_RMP {
        match self.bits {
            false => ADC12_JEXT3_RMP::Tim2,
            true => ADC12_JEXT3_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM2_CC1"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_JEXT3_RMP::Tim2
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT3_RMP::Tim20
    }
}
#[doc = "Field `ADC12_JEXT3_RMP` writer - Controls the Input trigger of ADC12 injected channel JEXT3"]
pub type ADC12_JEXT3_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_JEXT3_RMP>;
impl<'a, REG> ADC12_JEXT3_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM2_CC1"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT3_RMP::Tim2)
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT3_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 injected channel JEXT6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT6_RMP {
    #[doc = "0: Trigger source is EXTI line 15"]
    Exti15 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO2"]
    Tim20 = 1,
}
impl From<ADC12_JEXT6_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT6_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_JEXT6_RMP` reader - Controls the Input trigger of ADC12 injected channel JEXT6"]
pub type ADC12_JEXT6_RMP_R = crate::BitReader<ADC12_JEXT6_RMP>;
impl ADC12_JEXT6_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_JEXT6_RMP {
        match self.bits {
            false => ADC12_JEXT6_RMP::Exti15,
            true => ADC12_JEXT6_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is EXTI line 15"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == ADC12_JEXT6_RMP::Exti15
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT6_RMP::Tim20
    }
}
#[doc = "Field `ADC12_JEXT6_RMP` writer - Controls the Input trigger of ADC12 injected channel JEXT6"]
pub type ADC12_JEXT6_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_JEXT6_RMP>;
impl<'a, REG> ADC12_JEXT6_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT6_RMP::Exti15)
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT6_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC12 injected channel JEXT13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC12_JEXT13_RMP {
    #[doc = "0: Trigger source is TIM3_CC1"]
    Tim3 = 0,
    #[doc = "1: Trigger source is TIM20_CC4"]
    Tim20 = 1,
}
impl From<ADC12_JEXT13_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT13_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12_JEXT13_RMP` reader - Controls the Input trigger of ADC12 injected channel JEXT13"]
pub type ADC12_JEXT13_RMP_R = crate::BitReader<ADC12_JEXT13_RMP>;
impl ADC12_JEXT13_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC12_JEXT13_RMP {
        match self.bits {
            false => ADC12_JEXT13_RMP::Tim3,
            true => ADC12_JEXT13_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM3_CC1"]
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_JEXT13_RMP::Tim3
    }
    #[doc = "Trigger source is TIM20_CC4"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT13_RMP::Tim20
    }
}
#[doc = "Field `ADC12_JEXT13_RMP` writer - Controls the Input trigger of ADC12 injected channel JEXT13"]
pub type ADC12_JEXT13_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC12_JEXT13_RMP>;
impl<'a, REG> ADC12_JEXT13_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM3_CC1"]
    #[inline(always)]
    pub fn tim3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT13_RMP::Tim3)
    }
    #[doc = "Trigger source is TIM20_CC4"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12_JEXT13_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC34 regular channel EXT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT5_RMP {
    #[doc = "0: Trigger source is EXTI line 2 when reset at 0"]
    Exti2 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO"]
    Tim20 = 1,
}
impl From<ADC34_EXT5_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT5_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC34_EXT5_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT5"]
pub type ADC34_EXT5_RMP_R = crate::BitReader<ADC34_EXT5_RMP>;
impl ADC34_EXT5_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_EXT5_RMP {
        match self.bits {
            false => ADC34_EXT5_RMP::Exti2,
            true => ADC34_EXT5_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is EXTI line 2 when reset at 0"]
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == ADC34_EXT5_RMP::Exti2
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT5_RMP::Tim20
    }
}
#[doc = "Field `ADC34_EXT5_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT5"]
pub type ADC34_EXT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_EXT5_RMP>;
impl<'a, REG> ADC34_EXT5_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is EXTI line 2 when reset at 0"]
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT5_RMP::Exti2)
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT5_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC34 regular channel EXT6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT6_RMP {
    #[doc = "0: Trigger source is TIM4_CC1"]
    Tim4 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO2"]
    Tim20 = 1,
}
impl From<ADC34_EXT6_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT6_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC34_EXT6_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT6"]
pub type ADC34_EXT6_RMP_R = crate::BitReader<ADC34_EXT6_RMP>;
impl ADC34_EXT6_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_EXT6_RMP {
        match self.bits {
            false => ADC34_EXT6_RMP::Tim4,
            true => ADC34_EXT6_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM4_CC1"]
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_EXT6_RMP::Tim4
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT6_RMP::Tim20
    }
}
#[doc = "Field `ADC34_EXT6_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT6"]
pub type ADC34_EXT6_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_EXT6_RMP>;
impl<'a, REG> ADC34_EXT6_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM4_CC1"]
    #[inline(always)]
    pub fn tim4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT6_RMP::Tim4)
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT6_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC34 regular channel EXT15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_EXT15_RMP {
    #[doc = "0: Trigger source is TIM2_CC1"]
    Tim2 = 0,
    #[doc = "1: Trigger source is TIM20_CC1"]
    Tim20 = 1,
}
impl From<ADC34_EXT15_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT15_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC34_EXT15_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT15"]
pub type ADC34_EXT15_RMP_R = crate::BitReader<ADC34_EXT15_RMP>;
impl ADC34_EXT15_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_EXT15_RMP {
        match self.bits {
            false => ADC34_EXT15_RMP::Tim2,
            true => ADC34_EXT15_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM2_CC1"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC34_EXT15_RMP::Tim2
    }
    #[doc = "Trigger source is TIM20_CC1"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT15_RMP::Tim20
    }
}
#[doc = "Field `ADC34_EXT15_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT15"]
pub type ADC34_EXT15_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_EXT15_RMP>;
impl<'a, REG> ADC34_EXT15_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM2_CC1"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT15_RMP::Tim2)
    }
    #[doc = "Trigger source is TIM20_CC1"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_EXT15_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC34 injected channel JEXT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT5_RMP {
    #[doc = "0: Trigger source is TIM4_CC3"]
    Tim4 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO"]
    Tim20 = 1,
}
impl From<ADC34_JEXT5_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT5_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC34_JEXT5_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT5"]
pub type ADC34_JEXT5_RMP_R = crate::BitReader<ADC34_JEXT5_RMP>;
impl ADC34_JEXT5_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_JEXT5_RMP {
        match self.bits {
            false => ADC34_JEXT5_RMP::Tim4,
            true => ADC34_JEXT5_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM4_CC3"]
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_JEXT5_RMP::Tim4
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT5_RMP::Tim20
    }
}
#[doc = "Field `ADC34_JEXT5_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT5"]
pub type ADC34_JEXT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_JEXT5_RMP>;
impl<'a, REG> ADC34_JEXT5_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM4_CC3"]
    #[inline(always)]
    pub fn tim4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT5_RMP::Tim4)
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT5_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC34 injected channel JEXT11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT11_RMP {
    #[doc = "0: Trigger source is TIM1_CC3"]
    Tim1 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO2"]
    Tim20 = 1,
}
impl From<ADC34_JEXT11_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT11_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC34_JEXT11_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT11"]
pub type ADC34_JEXT11_RMP_R = crate::BitReader<ADC34_JEXT11_RMP>;
impl ADC34_JEXT11_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_JEXT11_RMP {
        match self.bits {
            false => ADC34_JEXT11_RMP::Tim1,
            true => ADC34_JEXT11_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM1_CC3"]
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC34_JEXT11_RMP::Tim1
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT11_RMP::Tim20
    }
}
#[doc = "Field `ADC34_JEXT11_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT11"]
pub type ADC34_JEXT11_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_JEXT11_RMP>;
impl<'a, REG> ADC34_JEXT11_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM1_CC3"]
    #[inline(always)]
    pub fn tim1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT11_RMP::Tim1)
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT11_RMP::Tim20)
    }
}
#[doc = "Controls the Input trigger of ADC34 injected channel JEXT14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC34_JEXT14_RMP {
    #[doc = "0: Trigger source is TIM7_TRGO"]
    Tim7 = 0,
    #[doc = "1: Trigger source is TIM20_CC2"]
    Tim20 = 1,
}
impl From<ADC34_JEXT14_RMP> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT14_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC34_JEXT14_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT14"]
pub type ADC34_JEXT14_RMP_R = crate::BitReader<ADC34_JEXT14_RMP>;
impl ADC34_JEXT14_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC34_JEXT14_RMP {
        match self.bits {
            false => ADC34_JEXT14_RMP::Tim7,
            true => ADC34_JEXT14_RMP::Tim20,
        }
    }
    #[doc = "Trigger source is TIM7_TRGO"]
    #[inline(always)]
    pub fn is_tim7(&self) -> bool {
        *self == ADC34_JEXT14_RMP::Tim7
    }
    #[doc = "Trigger source is TIM20_CC2"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT14_RMP::Tim20
    }
}
#[doc = "Field `ADC34_JEXT14_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT14"]
pub type ADC34_JEXT14_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ADC34_JEXT14_RMP>;
impl<'a, REG> ADC34_JEXT14_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source is TIM7_TRGO"]
    #[inline(always)]
    pub fn tim7(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT14_RMP::Tim7)
    }
    #[doc = "Trigger source is TIM20_CC2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut crate::W<REG> {
        self.variant(ADC34_JEXT14_RMP::Tim20)
    }
}
impl R {
    #[doc = "Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2"]
    #[inline(always)]
    pub fn adc12_ext2_rmp(&self) -> ADC12_EXT2_RMP_R {
        ADC12_EXT2_RMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3"]
    #[inline(always)]
    pub fn adc12_ext3_rmp(&self) -> ADC12_EXT3_RMP_R {
        ADC12_EXT3_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5"]
    #[inline(always)]
    pub fn adc12_ext5_rmp(&self) -> ADC12_EXT5_RMP_R {
        ADC12_EXT5_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13"]
    #[inline(always)]
    pub fn adc12_ext13_rmp(&self) -> ADC12_EXT13_RMP_R {
        ADC12_EXT13_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15"]
    #[inline(always)]
    pub fn adc12_ext15_rmp(&self) -> ADC12_EXT15_RMP_R {
        ADC12_EXT15_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Controls the Input trigger of ADC12 injected channel JEXT3"]
    #[inline(always)]
    pub fn adc12_jext3_rmp(&self) -> ADC12_JEXT3_RMP_R {
        ADC12_JEXT3_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Controls the Input trigger of ADC12 injected channel JEXT6"]
    #[inline(always)]
    pub fn adc12_jext6_rmp(&self) -> ADC12_JEXT6_RMP_R {
        ADC12_JEXT6_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Controls the Input trigger of ADC12 injected channel JEXT13"]
    #[inline(always)]
    pub fn adc12_jext13_rmp(&self) -> ADC12_JEXT13_RMP_R {
        ADC12_JEXT13_RMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5"]
    #[inline(always)]
    pub fn adc34_ext5_rmp(&self) -> ADC34_EXT5_RMP_R {
        ADC34_EXT5_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6"]
    #[inline(always)]
    pub fn adc34_ext6_rmp(&self) -> ADC34_EXT6_RMP_R {
        ADC34_EXT6_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15"]
    #[inline(always)]
    pub fn adc34_ext15_rmp(&self) -> ADC34_EXT15_RMP_R {
        ADC34_EXT15_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5"]
    #[inline(always)]
    pub fn adc34_jext5_rmp(&self) -> ADC34_JEXT5_RMP_R {
        ADC34_JEXT5_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11"]
    #[inline(always)]
    pub fn adc34_jext11_rmp(&self) -> ADC34_JEXT11_RMP_R {
        ADC34_JEXT11_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14"]
    #[inline(always)]
    pub fn adc34_jext14_rmp(&self) -> ADC34_JEXT14_RMP_R {
        ADC34_JEXT14_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext2_rmp(&mut self) -> ADC12_EXT2_RMP_W<CFGR4rs> {
        ADC12_EXT2_RMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext3_rmp(&mut self) -> ADC12_EXT3_RMP_W<CFGR4rs> {
        ADC12_EXT3_RMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext5_rmp(&mut self) -> ADC12_EXT5_RMP_W<CFGR4rs> {
        ADC12_EXT5_RMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext13_rmp(&mut self) -> ADC12_EXT13_RMP_W<CFGR4rs> {
        ADC12_EXT13_RMP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_ext15_rmp(&mut self) -> ADC12_EXT15_RMP_W<CFGR4rs> {
        ADC12_EXT15_RMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Controls the Input trigger of ADC12 injected channel JEXT3"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_jext3_rmp(&mut self) -> ADC12_JEXT3_RMP_W<CFGR4rs> {
        ADC12_JEXT3_RMP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Controls the Input trigger of ADC12 injected channel JEXT6"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_jext6_rmp(&mut self) -> ADC12_JEXT6_RMP_W<CFGR4rs> {
        ADC12_JEXT6_RMP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Controls the Input trigger of ADC12 injected channel JEXT13"]
    #[inline(always)]
    #[must_use]
    pub fn adc12_jext13_rmp(&mut self) -> ADC12_JEXT13_RMP_W<CFGR4rs> {
        ADC12_JEXT13_RMP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5"]
    #[inline(always)]
    #[must_use]
    pub fn adc34_ext5_rmp(&mut self) -> ADC34_EXT5_RMP_W<CFGR4rs> {
        ADC34_EXT5_RMP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6"]
    #[inline(always)]
    #[must_use]
    pub fn adc34_ext6_rmp(&mut self) -> ADC34_EXT6_RMP_W<CFGR4rs> {
        ADC34_EXT6_RMP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15"]
    #[inline(always)]
    #[must_use]
    pub fn adc34_ext15_rmp(&mut self) -> ADC34_EXT15_RMP_W<CFGR4rs> {
        ADC34_EXT15_RMP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5"]
    #[inline(always)]
    #[must_use]
    pub fn adc34_jext5_rmp(&mut self) -> ADC34_JEXT5_RMP_W<CFGR4rs> {
        ADC34_JEXT5_RMP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11"]
    #[inline(always)]
    #[must_use]
    pub fn adc34_jext11_rmp(&mut self) -> ADC34_JEXT11_RMP_W<CFGR4rs> {
        ADC34_JEXT11_RMP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14"]
    #[inline(always)]
    #[must_use]
    pub fn adc34_jext14_rmp(&mut self) -> ADC34_JEXT14_RMP_W<CFGR4rs> {
        ADC34_JEXT14_RMP_W::new(self, 13)
    }
}
#[doc = "configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR4rs;
impl crate::RegisterSpec for CFGR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr4::R`](R) reader structure"]
impl crate::Readable for CFGR4rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr4::W`](W) writer structure"]
impl crate::Writable for CFGR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR4 to value 0"]
impl crate::Resettable for CFGR4rs {
    const RESET_VALUE: u32 = 0;
}
