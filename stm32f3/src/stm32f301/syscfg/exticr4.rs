#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 15 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI15_A {
    #[doc = "0: Select PA15 as the source input for the EXTI15 external interrupt"]
    PA15 = 0,
    #[doc = "1: Select PB15 as the source input for the EXTI15 external interrupt"]
    PB15 = 1,
    #[doc = "2: Select PC15 as the source input for the EXTI15 external interrupt"]
    PC15 = 2,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI15` reader - EXTI 15 configuration bits"]
pub struct EXTI15_R(crate::FieldReader<u8, EXTI15_A>);
impl EXTI15_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::PA15),
            1 => Some(EXTI15_A::PB15),
            2 => Some(EXTI15_A::PC15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        **self == EXTI15_A::PA15
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        **self == EXTI15_A::PB15
    }
    #[doc = "Checks if the value of the field is `PC15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        **self == EXTI15_A::PC15
    }
}
impl core::ops::Deref for EXTI15_R {
    type Target = crate::FieldReader<u8, EXTI15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI15` writer - EXTI 15 configuration bits"]
pub struct EXTI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::PA15)
    }
    #[doc = "Select PB15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::PB15)
    }
    #[doc = "Select PC15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::PC15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "EXTI 14 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI14_A {
    #[doc = "0: Select PA14 as the source input for the EXTI14 external interrupt"]
    PA14 = 0,
    #[doc = "1: Select PB14 as the source input for the EXTI14 external interrupt"]
    PB14 = 1,
    #[doc = "2: Select PC14 as the source input for the EXTI14 external interrupt"]
    PC14 = 2,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI14` reader - EXTI 14 configuration bits"]
pub struct EXTI14_R(crate::FieldReader<u8, EXTI14_A>);
impl EXTI14_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::PA14),
            1 => Some(EXTI14_A::PB14),
            2 => Some(EXTI14_A::PC14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        **self == EXTI14_A::PA14
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        **self == EXTI14_A::PB14
    }
    #[doc = "Checks if the value of the field is `PC14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        **self == EXTI14_A::PC14
    }
}
impl core::ops::Deref for EXTI14_R {
    type Target = crate::FieldReader<u8, EXTI14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI14` writer - EXTI 14 configuration bits"]
pub struct EXTI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::PA14)
    }
    #[doc = "Select PB14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::PB14)
    }
    #[doc = "Select PC14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::PC14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "EXTI 13 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI13_A {
    #[doc = "0: Select PA13 as the source input for the EXTI13 external interrupt"]
    PA13 = 0,
    #[doc = "1: Select PB13 as the source input for the EXTI13 external interrupt"]
    PB13 = 1,
    #[doc = "2: Select PC13 as the source input for the EXTI13 external interrupt"]
    PC13 = 2,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI13` reader - EXTI 13 configuration bits"]
pub struct EXTI13_R(crate::FieldReader<u8, EXTI13_A>);
impl EXTI13_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::PA13),
            1 => Some(EXTI13_A::PB13),
            2 => Some(EXTI13_A::PC13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        **self == EXTI13_A::PA13
    }
    #[doc = "Checks if the value of the field is `PB13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        **self == EXTI13_A::PB13
    }
    #[doc = "Checks if the value of the field is `PC13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        **self == EXTI13_A::PC13
    }
}
impl core::ops::Deref for EXTI13_R {
    type Target = crate::FieldReader<u8, EXTI13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI13` writer - EXTI 13 configuration bits"]
pub struct EXTI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::PA13)
    }
    #[doc = "Select PB13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_A::PB13)
    }
    #[doc = "Select PC13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::PC13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "EXTI 12 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI12_A {
    #[doc = "0: Select PA12 as the source input for the EXTI12 external interrupt"]
    PA12 = 0,
    #[doc = "1: Select PB12 as the source input for the EXTI12 external interrupt"]
    PB12 = 1,
    #[doc = "2: Select PC12 as the source input for the EXTI12 external interrupt"]
    PC12 = 2,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI12` reader - EXTI 12 configuration bits"]
pub struct EXTI12_R(crate::FieldReader<u8, EXTI12_A>);
impl EXTI12_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::PA12),
            1 => Some(EXTI12_A::PB12),
            2 => Some(EXTI12_A::PC12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        **self == EXTI12_A::PA12
    }
    #[doc = "Checks if the value of the field is `PB12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        **self == EXTI12_A::PB12
    }
    #[doc = "Checks if the value of the field is `PC12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        **self == EXTI12_A::PC12
    }
}
impl core::ops::Deref for EXTI12_R {
    type Target = crate::FieldReader<u8, EXTI12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI12` writer - EXTI 12 configuration bits"]
pub struct EXTI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::PA12)
    }
    #[doc = "Select PB12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_A::PB12)
    }
    #[doc = "Select PC12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_A::PC12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
