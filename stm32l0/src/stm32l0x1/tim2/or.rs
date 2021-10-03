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
#[doc = "Timer2 ETR remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETR_RMP_A {
    #[doc = "7: TIM2 ETR input is connected to COMP1_OUT"]
    COMP1_OUT = 7,
    #[doc = "6: TIM2 ETR input is connected to COMP2_OUT"]
    COMP2_OUT = 6,
    #[doc = "5: TIM2 ETR input is connected to LSE"]
    LSE = 5,
    #[doc = "3: TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    HSI = 3,
}
impl From<ETR_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ETR_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETR_RMP` reader - Timer2 ETR remap"]
pub struct ETR_RMP_R(crate::FieldReader<u8, ETR_RMP_A>);
impl ETR_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETR_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETR_RMP_A> {
        match self.bits {
            7 => Some(ETR_RMP_A::COMP1_OUT),
            6 => Some(ETR_RMP_A::COMP2_OUT),
            5 => Some(ETR_RMP_A::LSE),
            3 => Some(ETR_RMP_A::HSI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        **self == ETR_RMP_A::COMP1_OUT
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        **self == ETR_RMP_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == ETR_RMP_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == ETR_RMP_A::HSI
    }
}
impl core::ops::Deref for ETR_RMP_R {
    type Target = crate::FieldReader<u8, ETR_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_RMP` writer - Timer2 ETR remap"]
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETR_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2 ETR input is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::COMP1_OUT)
    }
    #[doc = "TIM2 ETR input is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::COMP2_OUT)
    }
    #[doc = "TIM2 ETR input is connected to LSE"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(ETR_RMP_A::LSE)
    }
    #[doc = "TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(ETR_RMP_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Internal trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TI4_RMP_A {
    #[doc = "1: TIM2 TI4 input connected to COMP2_OUT"]
    COMP2_OUT = 1,
    #[doc = "2: TIM2 TI4 input connected to COMP1_OUT"]
    COMP1_OUT = 2,
}
impl From<TI4_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TI4_RMP` reader - Internal trigger"]
pub struct TI4_RMP_R(crate::FieldReader<u8, TI4_RMP_A>);
impl TI4_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI4_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TI4_RMP_A> {
        match self.bits {
            1 => Some(TI4_RMP_A::COMP2_OUT),
            2 => Some(TI4_RMP_A::COMP1_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        **self == TI4_RMP_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        **self == TI4_RMP_A::COMP1_OUT
    }
}
impl core::ops::Deref for TI4_RMP_R {
    type Target = crate::FieldReader<u8, TI4_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI4_RMP` writer - Internal trigger"]
pub struct TI4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI4_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2 TI4 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP2_OUT)
    }
    #[doc = "TIM2 TI4 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP1_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer2 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Internal trigger"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer2 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Bits 3:4 - Internal trigger"]
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W {
        TI4_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
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
