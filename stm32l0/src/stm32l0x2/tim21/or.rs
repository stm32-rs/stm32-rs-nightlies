#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETR_RMP` reader - Timer21 ETR remap"]
pub struct ETR_RMP_R(crate::FieldReader<u8, u8>);
impl ETR_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETR_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_RMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_RMP` writer - Timer21 ETR remap"]
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TI1_RMP` reader - Timer21 TI1"]
pub struct TI1_RMP_R(crate::FieldReader<u8, u8>);
impl TI1_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI1_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI1_RMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1_RMP` writer - Timer21 TI1"]
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Timer21 TI2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI2_RMP_A {
    #[doc = "0: TIM2x TI2 input connected to GPIO"]
    GPIO = 0,
    #[doc = "1: TIM2x TI2 input connected to COMP2_OUT"]
    COMP2_OUT = 1,
}
impl From<TI2_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TI2_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI2_RMP` reader - Timer21 TI2"]
pub struct TI2_RMP_R(crate::FieldReader<bool, TI2_RMP_A>);
impl TI2_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI2_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI2_RMP_A {
        match self.bits {
            false => TI2_RMP_A::GPIO,
            true => TI2_RMP_A::COMP2_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == TI2_RMP_A::GPIO
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        **self == TI2_RMP_A::COMP2_OUT
    }
}
impl core::ops::Deref for TI2_RMP_R {
    type Target = crate::FieldReader<bool, TI2_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI2_RMP` writer - Timer21 TI2"]
pub struct TI2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI2_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI2_RMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TIM2x TI2 input connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI2_RMP_A::GPIO)
    }
    #[doc = "TIM2x TI2 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI2_RMP_A::COMP2_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer21 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Timer21 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Timer21 TI2"]
    #[inline(always)]
    pub fn ti2_rmp(&self) -> TI2_RMP_R {
        TI2_RMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer21 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Bits 2:4 - Timer21 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
    #[doc = "Bit 5 - Timer21 TI2"]
    #[inline(always)]
    pub fn ti2_rmp(&mut self) -> TI2_RMP_W {
        TI2_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM21 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
