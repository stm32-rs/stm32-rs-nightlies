#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input capture 4 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TI4_RMP_A {
    #[doc = "0: TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping"]
    GPIO = 0,
    #[doc = "1: TIM2 TI4 is connected to COMP1_OUT"]
    COMP_1 = 1,
    #[doc = "2: TIM2 TI4 is connected to COMP2_OUT"]
    COMP_2 = 2,
    #[doc = "3: TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT"]
    COMP_12 = 3,
}
impl From<TI4_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TI4_RMP` reader - Input capture 4 remap"]
pub struct TI4_RMP_R(crate::FieldReader<u8, TI4_RMP_A>);
impl TI4_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI4_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI4_RMP_A {
        match self.bits {
            0 => TI4_RMP_A::GPIO,
            1 => TI4_RMP_A::COMP_1,
            2 => TI4_RMP_A::COMP_2,
            3 => TI4_RMP_A::COMP_12,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == TI4_RMP_A::GPIO
    }
    #[doc = "Checks if the value of the field is `COMP_1`"]
    #[inline(always)]
    pub fn is_comp_1(&self) -> bool {
        **self == TI4_RMP_A::COMP_1
    }
    #[doc = "Checks if the value of the field is `COMP_2`"]
    #[inline(always)]
    pub fn is_comp_2(&self) -> bool {
        **self == TI4_RMP_A::COMP_2
    }
    #[doc = "Checks if the value of the field is `COMP_12`"]
    #[inline(always)]
    pub fn is_comp_12(&self) -> bool {
        **self == TI4_RMP_A::COMP_12
    }
}
impl core::ops::Deref for TI4_RMP_R {
    type Target = crate::FieldReader<u8, TI4_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI4_RMP` writer - Input capture 4 remap"]
pub struct TI4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI4_RMP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI4_RMP_A::GPIO)
    }
    #[doc = "TIM2 TI4 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp_1(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP_1)
    }
    #[doc = "TIM2 TI4 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp_2(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP_2)
    }
    #[doc = "TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT"]
    #[inline(always)]
    pub fn comp_12(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP_12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "External trigger remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETR_RMP_A {
    #[doc = "0: TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping"]
    GPIO = 0,
    #[doc = "1: LSE internal clock is connected to TIM2_ETR input"]
    TIM2_ETR = 1,
}
impl From<ETR_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ETR_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETR_RMP` reader - External trigger remap"]
pub struct ETR_RMP_R(crate::FieldReader<bool, ETR_RMP_A>);
impl ETR_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETR_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETR_RMP_A {
        match self.bits {
            false => ETR_RMP_A::GPIO,
            true => ETR_RMP_A::TIM2_ETR,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == ETR_RMP_A::GPIO
    }
    #[doc = "Checks if the value of the field is `TIM2_ETR`"]
    #[inline(always)]
    pub fn is_tim2_etr(&self) -> bool {
        **self == ETR_RMP_A::TIM2_ETR
    }
}
impl core::ops::Deref for ETR_RMP_R {
    type Target = crate::FieldReader<bool, ETR_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_RMP` writer - External trigger remap"]
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETR_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(ETR_RMP_A::GPIO)
    }
    #[doc = "LSE internal clock is connected to TIM2_ETR input"]
    #[inline(always)]
    pub fn tim2_etr(self) -> &'a mut W {
        self.variant(ETR_RMP_A::TIM2_ETR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W {
        TI4_RMP_W { w: self }
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
