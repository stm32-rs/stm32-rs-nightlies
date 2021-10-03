#[doc = "Register `AFRH` reader"]
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRH` writer"]
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL15_R(crate::FieldReader<u8, u8>);
impl AFSEL15_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL14_R(crate::FieldReader<u8, u8>);
impl AFSEL14_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL13_R(crate::FieldReader<u8, u8>);
impl AFSEL13_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL12_R(crate::FieldReader<u8, u8>);
impl AFSEL12_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL11_R(crate::FieldReader<u8, u8>);
impl AFSEL11_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL10_R(crate::FieldReader<u8, u8>);
impl AFSEL10_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL9_R(crate::FieldReader<u8, u8>);
impl AFSEL9_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL8_R(crate::FieldReader<u8, u8>);
impl AFSEL8_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub struct AFSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel15(&mut self) -> AFSEL15_W {
        AFSEL15_W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel14(&mut self) -> AFSEL14_W {
        AFSEL14_W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel13(&mut self) -> AFSEL13_W {
        AFSEL13_W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel12(&mut self) -> AFSEL12_W {
        AFSEL12_W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel11(&mut self) -> AFSEL11_W {
        AFSEL11_W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel10(&mut self) -> AFSEL10_W {
        AFSEL10_W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel9(&mut self) -> AFSEL9_W {
        AFSEL9_W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afsel8(&mut self) -> AFSEL8_W {
        AFSEL8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](index.html) module"]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrh::R](R) reader structure"]
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrh::W](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
