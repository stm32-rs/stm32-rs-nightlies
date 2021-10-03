#[doc = "Register `EXTICR3` reader"]
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR3` writer"]
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 11 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI11_A {
    #[doc = "0: Select PA11 as the source input for the EXTI11 external interrupt"]
    PA11 = 0,
    #[doc = "1: Select PB11 as the source input for the EXTI11 external interrupt"]
    PB11 = 1,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI11` reader - EXTI 11 configuration bits"]
pub struct EXTI11_R(crate::FieldReader<u8, EXTI11_A>);
impl EXTI11_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI11_A> {
        match self.bits {
            0 => Some(EXTI11_A::PA11),
            1 => Some(EXTI11_A::PB11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        **self == EXTI11_A::PA11
    }
    #[doc = "Checks if the value of the field is `PB11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        **self == EXTI11_A::PB11
    }
}
impl core::ops::Deref for EXTI11_R {
    type Target = crate::FieldReader<u8, EXTI11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI11` writer - EXTI 11 configuration bits"]
pub struct EXTI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(EXTI11_A::PA11)
    }
    #[doc = "Select PB11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(EXTI11_A::PB11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "EXTI 10 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI10_A {
    #[doc = "0: Select PA10 as the source input for the EXTI10 external interrupt"]
    PA10 = 0,
    #[doc = "1: Select PB10 as the source input for the EXTI10 external interrupt"]
    PB10 = 1,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI10` reader - EXTI 10 configuration bits"]
pub struct EXTI10_R(crate::FieldReader<u8, EXTI10_A>);
impl EXTI10_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI10_A> {
        match self.bits {
            0 => Some(EXTI10_A::PA10),
            1 => Some(EXTI10_A::PB10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        **self == EXTI10_A::PA10
    }
    #[doc = "Checks if the value of the field is `PB10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        **self == EXTI10_A::PB10
    }
}
impl core::ops::Deref for EXTI10_R {
    type Target = crate::FieldReader<u8, EXTI10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI10` writer - EXTI 10 configuration bits"]
pub struct EXTI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(EXTI10_A::PA10)
    }
    #[doc = "Select PB10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(EXTI10_A::PB10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "EXTI 9 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI9_A {
    #[doc = "0: Select PA9 as the source input for the EXTI9 external interrupt"]
    PA9 = 0,
    #[doc = "1: Select PB9 as the source input for the EXTI9 external interrupt"]
    PB9 = 1,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI9` reader - EXTI 9 configuration bits"]
pub struct EXTI9_R(crate::FieldReader<u8, EXTI9_A>);
impl EXTI9_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI9_A> {
        match self.bits {
            0 => Some(EXTI9_A::PA9),
            1 => Some(EXTI9_A::PB9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        **self == EXTI9_A::PA9
    }
    #[doc = "Checks if the value of the field is `PB9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        **self == EXTI9_A::PB9
    }
}
impl core::ops::Deref for EXTI9_R {
    type Target = crate::FieldReader<u8, EXTI9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI9` writer - EXTI 9 configuration bits"]
pub struct EXTI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(EXTI9_A::PA9)
    }
    #[doc = "Select PB9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(EXTI9_A::PB9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "EXTI 8 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI8_A {
    #[doc = "0: Select PA8 as the source input for the EXTI8 external interrupt"]
    PA8 = 0,
    #[doc = "1: Select PB8 as the source input for the EXTI8 external interrupt"]
    PB8 = 1,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI8` reader - EXTI 8 configuration bits"]
pub struct EXTI8_R(crate::FieldReader<u8, EXTI8_A>);
impl EXTI8_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI8_A> {
        match self.bits {
            0 => Some(EXTI8_A::PA8),
            1 => Some(EXTI8_A::PB8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        **self == EXTI8_A::PA8
    }
    #[doc = "Checks if the value of the field is `PB8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        **self == EXTI8_A::PB8
    }
}
impl core::ops::Deref for EXTI8_R {
    type Target = crate::FieldReader<u8, EXTI8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI8` writer - EXTI 8 configuration bits"]
pub struct EXTI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(EXTI8_A::PA8)
    }
    #[doc = "Select PB8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(EXTI8_A::PB8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:14 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W {
        EXTI11_W { w: self }
    }
    #[doc = "Bits 8:10 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W {
        EXTI10_W { w: self }
    }
    #[doc = "Bits 4:6 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W {
        EXTI9_W { w: self }
    }
    #[doc = "Bits 0:2 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W {
        EXTI8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](index.html) module"]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr3::R](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr3::W](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
