#[doc = "Reader of register CFGR4"]
pub type R = crate::R<u32, super::CFGR4>;
#[doc = "Writer for register CFGR4"]
pub type W = crate::W<u32, super::CFGR4>;
#[doc = "Register CFGR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_EXT2_RMP_A {
    #[doc = "0: Trigger source is TIM3_CC3"]
    TIM1 = 0,
    #[doc = "1: rigger source is TIM20_TRGO"]
    TIM20 = 1,
}
impl From<ADC12_EXT2_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT2_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_EXT2_RMP`"]
pub type ADC12_EXT2_RMP_R = crate::R<bool, ADC12_EXT2_RMP_A>;
impl ADC12_EXT2_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT2_RMP_A {
        match self.bits {
            false => ADC12_EXT2_RMP_A::TIM1,
            true => ADC12_EXT2_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM1`"]
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC12_EXT2_RMP_A::TIM1
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT2_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_EXT2_RMP`"]
pub struct ADC12_EXT2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_EXT2_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_EXT2_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM3_CC3"]
    #[inline(always)]
    pub fn tim1(self) -> &'a mut W {
        self.variant(ADC12_EXT2_RMP_A::TIM1)
    }
    #[doc = "rigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT2_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_EXT3_RMP_A {
    #[doc = "0: Trigger source is TIM2_CC2"]
    TIM2 = 0,
    #[doc = "1: rigger source is TIM20_TRGO2"]
    TIM20 = 1,
}
impl From<ADC12_EXT3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_EXT3_RMP`"]
pub type ADC12_EXT3_RMP_R = crate::R<bool, ADC12_EXT3_RMP_A>;
impl ADC12_EXT3_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT3_RMP_A {
        match self.bits {
            false => ADC12_EXT3_RMP_A::TIM2,
            true => ADC12_EXT3_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM2`"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_EXT3_RMP_A::TIM2
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT3_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_EXT3_RMP`"]
pub struct ADC12_EXT3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_EXT3_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_EXT3_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM2_CC2"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut W {
        self.variant(ADC12_EXT3_RMP_A::TIM2)
    }
    #[doc = "rigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT3_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_EXT5_RMP_A {
    #[doc = "0: Trigger source is TIM4_CC4"]
    TIM4 = 0,
    #[doc = "1: Trigger source is TIM20_CC1"]
    TIM20 = 1,
}
impl From<ADC12_EXT5_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT5_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_EXT5_RMP`"]
pub type ADC12_EXT5_RMP_R = crate::R<bool, ADC12_EXT5_RMP_A>;
impl ADC12_EXT5_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT5_RMP_A {
        match self.bits {
            false => ADC12_EXT5_RMP_A::TIM4,
            true => ADC12_EXT5_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM4`"]
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC12_EXT5_RMP_A::TIM4
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT5_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_EXT5_RMP`"]
pub struct ADC12_EXT5_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_EXT5_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_EXT5_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM4_CC4"]
    #[inline(always)]
    pub fn tim4(self) -> &'a mut W {
        self.variant(ADC12_EXT5_RMP_A::TIM4)
    }
    #[doc = "Trigger source is TIM20_CC1"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT5_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_EXT13_RMP_A {
    #[doc = "0: Trigger source is TIM6_TRGO"]
    TIM6 = 0,
    #[doc = "1: Trigger source is TIM20_CC2"]
    TIM20 = 1,
}
impl From<ADC12_EXT13_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT13_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_EXT13_RMP`"]
pub type ADC12_EXT13_RMP_R = crate::R<bool, ADC12_EXT13_RMP_A>;
impl ADC12_EXT13_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT13_RMP_A {
        match self.bits {
            false => ADC12_EXT13_RMP_A::TIM6,
            true => ADC12_EXT13_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM6`"]
    #[inline(always)]
    pub fn is_tim6(&self) -> bool {
        *self == ADC12_EXT13_RMP_A::TIM6
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT13_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_EXT13_RMP`"]
pub struct ADC12_EXT13_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_EXT13_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_EXT13_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM6_TRGO"]
    #[inline(always)]
    pub fn tim6(self) -> &'a mut W {
        self.variant(ADC12_EXT13_RMP_A::TIM6)
    }
    #[doc = "Trigger source is TIM20_CC2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT13_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 regular channel EXT15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_EXT15_RMP_A {
    #[doc = "0: Trigger source is TIM3_CC4"]
    TIM3 = 0,
    #[doc = "1: Trigger source is TIM20_CC3"]
    TIM20 = 1,
}
impl From<ADC12_EXT15_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_EXT15_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_EXT15_RMP`"]
pub type ADC12_EXT15_RMP_R = crate::R<bool, ADC12_EXT15_RMP_A>;
impl ADC12_EXT15_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_EXT15_RMP_A {
        match self.bits {
            false => ADC12_EXT15_RMP_A::TIM3,
            true => ADC12_EXT15_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM3`"]
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_EXT15_RMP_A::TIM3
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_EXT15_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_EXT15_RMP`"]
pub struct ADC12_EXT15_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_EXT15_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_EXT15_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM3_CC4"]
    #[inline(always)]
    pub fn tim3(self) -> &'a mut W {
        self.variant(ADC12_EXT15_RMP_A::TIM3)
    }
    #[doc = "Trigger source is TIM20_CC3"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_EXT15_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 injected channel JEXT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_JEXT3_RMP_A {
    #[doc = "0: Trigger source is TIM2_CC1"]
    TIM2 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO"]
    TIM20 = 1,
}
impl From<ADC12_JEXT3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_JEXT3_RMP`"]
pub type ADC12_JEXT3_RMP_R = crate::R<bool, ADC12_JEXT3_RMP_A>;
impl ADC12_JEXT3_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_JEXT3_RMP_A {
        match self.bits {
            false => ADC12_JEXT3_RMP_A::TIM2,
            true => ADC12_JEXT3_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM2`"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC12_JEXT3_RMP_A::TIM2
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT3_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_JEXT3_RMP`"]
pub struct ADC12_JEXT3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_JEXT3_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_JEXT3_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM2_CC1"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut W {
        self.variant(ADC12_JEXT3_RMP_A::TIM2)
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_JEXT3_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 injected channel JEXT6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_JEXT6_RMP_A {
    #[doc = "0: Trigger source is EXTI line 15"]
    EXTI15 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO2"]
    TIM20 = 1,
}
impl From<ADC12_JEXT6_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT6_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_JEXT6_RMP`"]
pub type ADC12_JEXT6_RMP_R = crate::R<bool, ADC12_JEXT6_RMP_A>;
impl ADC12_JEXT6_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_JEXT6_RMP_A {
        match self.bits {
            false => ADC12_JEXT6_RMP_A::EXTI15,
            true => ADC12_JEXT6_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `EXTI15`"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == ADC12_JEXT6_RMP_A::EXTI15
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT6_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_JEXT6_RMP`"]
pub struct ADC12_JEXT6_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_JEXT6_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_JEXT6_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(ADC12_JEXT6_RMP_A::EXTI15)
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_JEXT6_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC12 injected channel JEXT13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12_JEXT13_RMP_A {
    #[doc = "0: Trigger source is TIM3_CC1"]
    TIM3 = 0,
    #[doc = "1: Trigger source is TIM20_CC4"]
    TIM20 = 1,
}
impl From<ADC12_JEXT13_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12_JEXT13_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12_JEXT13_RMP`"]
pub type ADC12_JEXT13_RMP_R = crate::R<bool, ADC12_JEXT13_RMP_A>;
impl ADC12_JEXT13_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12_JEXT13_RMP_A {
        match self.bits {
            false => ADC12_JEXT13_RMP_A::TIM3,
            true => ADC12_JEXT13_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM3`"]
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == ADC12_JEXT13_RMP_A::TIM3
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC12_JEXT13_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC12_JEXT13_RMP`"]
pub struct ADC12_JEXT13_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12_JEXT13_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12_JEXT13_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM3_CC1"]
    #[inline(always)]
    pub fn tim3(self) -> &'a mut W {
        self.variant(ADC12_JEXT13_RMP_A::TIM3)
    }
    #[doc = "Trigger source is TIM20_CC4"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC12_JEXT13_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC34 regular channel EXT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC34_EXT5_RMP_A {
    #[doc = "0: Trigger source is EXTI line 2 when reset at 0"]
    EXTI2 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO"]
    TIM20 = 1,
}
impl From<ADC34_EXT5_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT5_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC34_EXT5_RMP`"]
pub type ADC34_EXT5_RMP_R = crate::R<bool, ADC34_EXT5_RMP_A>;
impl ADC34_EXT5_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC34_EXT5_RMP_A {
        match self.bits {
            false => ADC34_EXT5_RMP_A::EXTI2,
            true => ADC34_EXT5_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `EXTI2`"]
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == ADC34_EXT5_RMP_A::EXTI2
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT5_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC34_EXT5_RMP`"]
pub struct ADC34_EXT5_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34_EXT5_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34_EXT5_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is EXTI line 2 when reset at 0"]
    #[inline(always)]
    pub fn exti2(self) -> &'a mut W {
        self.variant(ADC34_EXT5_RMP_A::EXTI2)
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_EXT5_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC34 regular channel EXT6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC34_EXT6_RMP_A {
    #[doc = "0: Trigger source is TIM4_CC1"]
    TIM4 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO2"]
    TIM20 = 1,
}
impl From<ADC34_EXT6_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT6_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC34_EXT6_RMP`"]
pub type ADC34_EXT6_RMP_R = crate::R<bool, ADC34_EXT6_RMP_A>;
impl ADC34_EXT6_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC34_EXT6_RMP_A {
        match self.bits {
            false => ADC34_EXT6_RMP_A::TIM4,
            true => ADC34_EXT6_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM4`"]
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_EXT6_RMP_A::TIM4
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT6_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC34_EXT6_RMP`"]
pub struct ADC34_EXT6_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34_EXT6_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34_EXT6_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM4_CC1"]
    #[inline(always)]
    pub fn tim4(self) -> &'a mut W {
        self.variant(ADC34_EXT6_RMP_A::TIM4)
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_EXT6_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC34 regular channel EXT15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC34_EXT15_RMP_A {
    #[doc = "0: Trigger source is TIM2_CC1"]
    TIM2 = 0,
    #[doc = "1: Trigger source is TIM20_CC1"]
    TIM20 = 1,
}
impl From<ADC34_EXT15_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_EXT15_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC34_EXT15_RMP`"]
pub type ADC34_EXT15_RMP_R = crate::R<bool, ADC34_EXT15_RMP_A>;
impl ADC34_EXT15_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC34_EXT15_RMP_A {
        match self.bits {
            false => ADC34_EXT15_RMP_A::TIM2,
            true => ADC34_EXT15_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM2`"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == ADC34_EXT15_RMP_A::TIM2
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_EXT15_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC34_EXT15_RMP`"]
pub struct ADC34_EXT15_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34_EXT15_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34_EXT15_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM2_CC1"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut W {
        self.variant(ADC34_EXT15_RMP_A::TIM2)
    }
    #[doc = "Trigger source is TIM20_CC1"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_EXT15_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC34 injected channel JEXT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC34_JEXT5_RMP_A {
    #[doc = "0: Trigger source is TIM4_CC3"]
    TIM4 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO"]
    TIM20 = 1,
}
impl From<ADC34_JEXT5_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT5_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC34_JEXT5_RMP`"]
pub type ADC34_JEXT5_RMP_R = crate::R<bool, ADC34_JEXT5_RMP_A>;
impl ADC34_JEXT5_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC34_JEXT5_RMP_A {
        match self.bits {
            false => ADC34_JEXT5_RMP_A::TIM4,
            true => ADC34_JEXT5_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM4`"]
    #[inline(always)]
    pub fn is_tim4(&self) -> bool {
        *self == ADC34_JEXT5_RMP_A::TIM4
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT5_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC34_JEXT5_RMP`"]
pub struct ADC34_JEXT5_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34_JEXT5_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34_JEXT5_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM4_CC3"]
    #[inline(always)]
    pub fn tim4(self) -> &'a mut W {
        self.variant(ADC34_JEXT5_RMP_A::TIM4)
    }
    #[doc = "Trigger source is TIM20_TRGO"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_JEXT5_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC34 injected channel JEXT11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC34_JEXT11_RMP_A {
    #[doc = "0: Trigger source is TIM1_CC3"]
    TIM1 = 0,
    #[doc = "1: Trigger source is TIM20_TRGO2"]
    TIM20 = 1,
}
impl From<ADC34_JEXT11_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT11_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC34_JEXT11_RMP`"]
pub type ADC34_JEXT11_RMP_R = crate::R<bool, ADC34_JEXT11_RMP_A>;
impl ADC34_JEXT11_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC34_JEXT11_RMP_A {
        match self.bits {
            false => ADC34_JEXT11_RMP_A::TIM1,
            true => ADC34_JEXT11_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM1`"]
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == ADC34_JEXT11_RMP_A::TIM1
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT11_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC34_JEXT11_RMP`"]
pub struct ADC34_JEXT11_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34_JEXT11_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34_JEXT11_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM1_CC3"]
    #[inline(always)]
    pub fn tim1(self) -> &'a mut W {
        self.variant(ADC34_JEXT11_RMP_A::TIM1)
    }
    #[doc = "Trigger source is TIM20_TRGO2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_JEXT11_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Controls the Input trigger of ADC34 injected channel JEXT14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC34_JEXT14_RMP_A {
    #[doc = "0: Trigger source is TIM7_TRGO"]
    TIM7 = 0,
    #[doc = "1: Trigger source is TIM20_CC2"]
    TIM20 = 1,
}
impl From<ADC34_JEXT14_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC34_JEXT14_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC34_JEXT14_RMP`"]
pub type ADC34_JEXT14_RMP_R = crate::R<bool, ADC34_JEXT14_RMP_A>;
impl ADC34_JEXT14_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC34_JEXT14_RMP_A {
        match self.bits {
            false => ADC34_JEXT14_RMP_A::TIM7,
            true => ADC34_JEXT14_RMP_A::TIM20,
        }
    }
    #[doc = "Checks if the value of the field is `TIM7`"]
    #[inline(always)]
    pub fn is_tim7(&self) -> bool {
        *self == ADC34_JEXT14_RMP_A::TIM7
    }
    #[doc = "Checks if the value of the field is `TIM20`"]
    #[inline(always)]
    pub fn is_tim20(&self) -> bool {
        *self == ADC34_JEXT14_RMP_A::TIM20
    }
}
#[doc = "Write proxy for field `ADC34_JEXT14_RMP`"]
pub struct ADC34_JEXT14_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34_JEXT14_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34_JEXT14_RMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger source is TIM7_TRGO"]
    #[inline(always)]
    pub fn tim7(self) -> &'a mut W {
        self.variant(ADC34_JEXT14_RMP_A::TIM7)
    }
    #[doc = "Trigger source is TIM20_CC2"]
    #[inline(always)]
    pub fn tim20(self) -> &'a mut W {
        self.variant(ADC34_JEXT14_RMP_A::TIM20)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2"]
    #[inline(always)]
    pub fn adc12_ext2_rmp(&self) -> ADC12_EXT2_RMP_R {
        ADC12_EXT2_RMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3"]
    #[inline(always)]
    pub fn adc12_ext3_rmp(&self) -> ADC12_EXT3_RMP_R {
        ADC12_EXT3_RMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5"]
    #[inline(always)]
    pub fn adc12_ext5_rmp(&self) -> ADC12_EXT5_RMP_R {
        ADC12_EXT5_RMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13"]
    #[inline(always)]
    pub fn adc12_ext13_rmp(&self) -> ADC12_EXT13_RMP_R {
        ADC12_EXT13_RMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15"]
    #[inline(always)]
    pub fn adc12_ext15_rmp(&self) -> ADC12_EXT15_RMP_R {
        ADC12_EXT15_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Controls the Input trigger of ADC12 injected channel JEXT3"]
    #[inline(always)]
    pub fn adc12_jext3_rmp(&self) -> ADC12_JEXT3_RMP_R {
        ADC12_JEXT3_RMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controls the Input trigger of ADC12 injected channel JEXT6"]
    #[inline(always)]
    pub fn adc12_jext6_rmp(&self) -> ADC12_JEXT6_RMP_R {
        ADC12_JEXT6_RMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controls the Input trigger of ADC12 injected channel JEXT13"]
    #[inline(always)]
    pub fn adc12_jext13_rmp(&self) -> ADC12_JEXT13_RMP_R {
        ADC12_JEXT13_RMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5"]
    #[inline(always)]
    pub fn adc34_ext5_rmp(&self) -> ADC34_EXT5_RMP_R {
        ADC34_EXT5_RMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6"]
    #[inline(always)]
    pub fn adc34_ext6_rmp(&self) -> ADC34_EXT6_RMP_R {
        ADC34_EXT6_RMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15"]
    #[inline(always)]
    pub fn adc34_ext15_rmp(&self) -> ADC34_EXT15_RMP_R {
        ADC34_EXT15_RMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5"]
    #[inline(always)]
    pub fn adc34_jext5_rmp(&self) -> ADC34_JEXT5_RMP_R {
        ADC34_JEXT5_RMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11"]
    #[inline(always)]
    pub fn adc34_jext11_rmp(&self) -> ADC34_JEXT11_RMP_R {
        ADC34_JEXT11_RMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14"]
    #[inline(always)]
    pub fn adc34_jext14_rmp(&self) -> ADC34_JEXT14_RMP_R {
        ADC34_JEXT14_RMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2"]
    #[inline(always)]
    pub fn adc12_ext2_rmp(&mut self) -> ADC12_EXT2_RMP_W {
        ADC12_EXT2_RMP_W { w: self }
    }
    #[doc = "Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3"]
    #[inline(always)]
    pub fn adc12_ext3_rmp(&mut self) -> ADC12_EXT3_RMP_W {
        ADC12_EXT3_RMP_W { w: self }
    }
    #[doc = "Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5"]
    #[inline(always)]
    pub fn adc12_ext5_rmp(&mut self) -> ADC12_EXT5_RMP_W {
        ADC12_EXT5_RMP_W { w: self }
    }
    #[doc = "Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13"]
    #[inline(always)]
    pub fn adc12_ext13_rmp(&mut self) -> ADC12_EXT13_RMP_W {
        ADC12_EXT13_RMP_W { w: self }
    }
    #[doc = "Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15"]
    #[inline(always)]
    pub fn adc12_ext15_rmp(&mut self) -> ADC12_EXT15_RMP_W {
        ADC12_EXT15_RMP_W { w: self }
    }
    #[doc = "Bit 5 - Controls the Input trigger of ADC12 injected channel JEXT3"]
    #[inline(always)]
    pub fn adc12_jext3_rmp(&mut self) -> ADC12_JEXT3_RMP_W {
        ADC12_JEXT3_RMP_W { w: self }
    }
    #[doc = "Bit 6 - Controls the Input trigger of ADC12 injected channel JEXT6"]
    #[inline(always)]
    pub fn adc12_jext6_rmp(&mut self) -> ADC12_JEXT6_RMP_W {
        ADC12_JEXT6_RMP_W { w: self }
    }
    #[doc = "Bit 7 - Controls the Input trigger of ADC12 injected channel JEXT13"]
    #[inline(always)]
    pub fn adc12_jext13_rmp(&mut self) -> ADC12_JEXT13_RMP_W {
        ADC12_JEXT13_RMP_W { w: self }
    }
    #[doc = "Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5"]
    #[inline(always)]
    pub fn adc34_ext5_rmp(&mut self) -> ADC34_EXT5_RMP_W {
        ADC34_EXT5_RMP_W { w: self }
    }
    #[doc = "Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6"]
    #[inline(always)]
    pub fn adc34_ext6_rmp(&mut self) -> ADC34_EXT6_RMP_W {
        ADC34_EXT6_RMP_W { w: self }
    }
    #[doc = "Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15"]
    #[inline(always)]
    pub fn adc34_ext15_rmp(&mut self) -> ADC34_EXT15_RMP_W {
        ADC34_EXT15_RMP_W { w: self }
    }
    #[doc = "Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5"]
    #[inline(always)]
    pub fn adc34_jext5_rmp(&mut self) -> ADC34_JEXT5_RMP_W {
        ADC34_JEXT5_RMP_W { w: self }
    }
    #[doc = "Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11"]
    #[inline(always)]
    pub fn adc34_jext11_rmp(&mut self) -> ADC34_JEXT11_RMP_W {
        ADC34_JEXT11_RMP_W { w: self }
    }
    #[doc = "Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14"]
    #[inline(always)]
    pub fn adc34_jext14_rmp(&mut self) -> ADC34_JEXT14_RMP_W {
        ADC34_JEXT14_RMP_W { w: self }
    }
}
