#[doc = "Register `GPIOB_AFRH` reader"]
pub struct R(crate::R<GPIOB_AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOB_AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOB_AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOB_AFRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOB_AFRH` writer"]
pub struct W(crate::W<GPIOB_AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOB_AFRH_SPEC>;
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
impl From<crate::W<GPIOB_AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOB_AFRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFR8` reader - AFR8"]
pub struct AFR8_R(crate::FieldReader<u8, u8>);
impl AFR8_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR8` writer - AFR8"]
pub struct AFR8_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `AFR9` reader - AFR9"]
pub struct AFR9_R(crate::FieldReader<u8, u8>);
impl AFR9_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR9` writer - AFR9"]
pub struct AFR9_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `AFR10` reader - AFR10"]
pub struct AFR10_R(crate::FieldReader<u8, u8>);
impl AFR10_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR10` writer - AFR10"]
pub struct AFR10_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `AFR11` reader - AFR11"]
pub struct AFR11_R(crate::FieldReader<u8, u8>);
impl AFR11_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR11` writer - AFR11"]
pub struct AFR11_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `AFR12` reader - AFR12"]
pub struct AFR12_R(crate::FieldReader<u8, u8>);
impl AFR12_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR12` writer - AFR12"]
pub struct AFR12_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `AFR13` reader - AFR13"]
pub struct AFR13_R(crate::FieldReader<u8, u8>);
impl AFR13_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR13` writer - AFR13"]
pub struct AFR13_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `AFR14` reader - AFR14"]
pub struct AFR14_R(crate::FieldReader<u8, u8>);
impl AFR14_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR14` writer - AFR14"]
pub struct AFR14_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `AFR15` reader - AFR15"]
pub struct AFR15_R(crate::FieldReader<u8, u8>);
impl AFR15_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR15` writer - AFR15"]
pub struct AFR15_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - AFR8"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFR9"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AFR10"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AFR11"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AFR12"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AFR13"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AFR14"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AFR15"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFR8"]
    #[inline(always)]
    pub fn afr8(&mut self) -> AFR8_W {
        AFR8_W { w: self }
    }
    #[doc = "Bits 4:7 - AFR9"]
    #[inline(always)]
    pub fn afr9(&mut self) -> AFR9_W {
        AFR9_W { w: self }
    }
    #[doc = "Bits 8:11 - AFR10"]
    #[inline(always)]
    pub fn afr10(&mut self) -> AFR10_W {
        AFR10_W { w: self }
    }
    #[doc = "Bits 12:15 - AFR11"]
    #[inline(always)]
    pub fn afr11(&mut self) -> AFR11_W {
        AFR11_W { w: self }
    }
    #[doc = "Bits 16:19 - AFR12"]
    #[inline(always)]
    pub fn afr12(&mut self) -> AFR12_W {
        AFR12_W { w: self }
    }
    #[doc = "Bits 20:23 - AFR13"]
    #[inline(always)]
    pub fn afr13(&mut self) -> AFR13_W {
        AFR13_W { w: self }
    }
    #[doc = "Bits 24:27 - AFR14"]
    #[inline(always)]
    pub fn afr14(&mut self) -> AFR14_W {
        AFR14_W { w: self }
    }
    #[doc = "Bits 28:31 - AFR15"]
    #[inline(always)]
    pub fn afr15(&mut self) -> AFR15_W {
        AFR15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_afrh](index.html) module"]
pub struct GPIOB_AFRH_SPEC;
impl crate::RegisterSpec for GPIOB_AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiob_afrh::R](R) reader structure"]
impl crate::Readable for GPIOB_AFRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiob_afrh::W](W) writer structure"]
impl crate::Writable for GPIOB_AFRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOB_AFRH to value 0"]
impl crate::Resettable for GPIOB_AFRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
