#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 7 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI7_A {
    #[doc = "0: Select PA7 as the source input for the EXTI7 external interrupt"]
    PA7 = 0,
    #[doc = "1: Select PB7 as the source input for the EXTI7 external interrupt"]
    PB7 = 1,
    #[doc = "2: Select PC7 as the source input for the EXTI7 external interrupt"]
    PC7 = 2,
}
impl From<EXTI7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub struct EXTI7_R(crate::FieldReader<u8, EXTI7_A>);
impl EXTI7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI7_A> {
        match self.bits {
            0 => Some(EXTI7_A::PA7),
            1 => Some(EXTI7_A::PB7),
            2 => Some(EXTI7_A::PC7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        **self == EXTI7_A::PA7
    }
    #[doc = "Checks if the value of the field is `PB7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        **self == EXTI7_A::PB7
    }
    #[doc = "Checks if the value of the field is `PC7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        **self == EXTI7_A::PC7
    }
}
impl core::ops::Deref for EXTI7_R {
    type Target = crate::FieldReader<u8, EXTI7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub struct EXTI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(EXTI7_A::PA7)
    }
    #[doc = "Select PB7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(EXTI7_A::PB7)
    }
    #[doc = "Select PC7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(EXTI7_A::PC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "EXTI 6 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI6_A {
    #[doc = "0: Select PA6 as the source input for the EXTI6 external interrupt"]
    PA6 = 0,
    #[doc = "1: Select PB6 as the source input for the EXTI6 external interrupt"]
    PB6 = 1,
    #[doc = "2: Select PC6 as the source input for the EXTI6 external interrupt"]
    PC6 = 2,
}
impl From<EXTI6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub struct EXTI6_R(crate::FieldReader<u8, EXTI6_A>);
impl EXTI6_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI6_A> {
        match self.bits {
            0 => Some(EXTI6_A::PA6),
            1 => Some(EXTI6_A::PB6),
            2 => Some(EXTI6_A::PC6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        **self == EXTI6_A::PA6
    }
    #[doc = "Checks if the value of the field is `PB6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        **self == EXTI6_A::PB6
    }
    #[doc = "Checks if the value of the field is `PC6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        **self == EXTI6_A::PC6
    }
}
impl core::ops::Deref for EXTI6_R {
    type Target = crate::FieldReader<u8, EXTI6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub struct EXTI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(EXTI6_A::PA6)
    }
    #[doc = "Select PB6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(EXTI6_A::PB6)
    }
    #[doc = "Select PC6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(EXTI6_A::PC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "EXTI 5 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI5_A {
    #[doc = "0: Select PA5 as the source input for the EXTI5 external interrupt"]
    PA5 = 0,
    #[doc = "1: Select PB5 as the source input for the EXTI5 external interrupt"]
    PB5 = 1,
    #[doc = "2: Select PC5 as the source input for the EXTI5 external interrupt"]
    PC5 = 2,
}
impl From<EXTI5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub struct EXTI5_R(crate::FieldReader<u8, EXTI5_A>);
impl EXTI5_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI5_A> {
        match self.bits {
            0 => Some(EXTI5_A::PA5),
            1 => Some(EXTI5_A::PB5),
            2 => Some(EXTI5_A::PC5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        **self == EXTI5_A::PA5
    }
    #[doc = "Checks if the value of the field is `PB5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        **self == EXTI5_A::PB5
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        **self == EXTI5_A::PC5
    }
}
impl core::ops::Deref for EXTI5_R {
    type Target = crate::FieldReader<u8, EXTI5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub struct EXTI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(EXTI5_A::PA5)
    }
    #[doc = "Select PB5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(EXTI5_A::PB5)
    }
    #[doc = "Select PC5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(EXTI5_A::PC5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "EXTI 4 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI4_A {
    #[doc = "0: Select PA4 as the source input for the EXTI4 external interrupt"]
    PA4 = 0,
    #[doc = "1: Select PB4 as the source input for the EXTI4 external interrupt"]
    PB4 = 1,
    #[doc = "2: Select PC4 as the source input for the EXTI4 external interrupt"]
    PC4 = 2,
}
impl From<EXTI4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub struct EXTI4_R(crate::FieldReader<u8, EXTI4_A>);
impl EXTI4_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI4_A> {
        match self.bits {
            0 => Some(EXTI4_A::PA4),
            1 => Some(EXTI4_A::PB4),
            2 => Some(EXTI4_A::PC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        **self == EXTI4_A::PA4
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        **self == EXTI4_A::PB4
    }
    #[doc = "Checks if the value of the field is `PC4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        **self == EXTI4_A::PC4
    }
}
impl core::ops::Deref for EXTI4_R {
    type Target = crate::FieldReader<u8, EXTI4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub struct EXTI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(EXTI4_A::PA4)
    }
    #[doc = "Select PB4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(EXTI4_A::PB4)
    }
    #[doc = "Select PC4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(EXTI4_A::PC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W {
        EXTI7_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W {
        EXTI6_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W {
        EXTI5_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W {
        EXTI4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
