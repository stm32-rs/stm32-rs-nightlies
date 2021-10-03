#[doc = "Register `GPIOA_AFRL` reader"]
pub struct R(crate::R<GPIOA_AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOA_AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOA_AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOA_AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOA_AFRL` writer"]
pub struct W(crate::W<GPIOA_AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOA_AFRL_SPEC>;
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
impl From<crate::W<GPIOA_AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOA_AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFR0` reader - AFR0"]
pub struct AFR0_R(crate::FieldReader<u8, u8>);
impl AFR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR0` writer - AFR0"]
pub struct AFR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `AFR1` reader - AFR1"]
pub struct AFR1_R(crate::FieldReader<u8, u8>);
impl AFR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR1` writer - AFR1"]
pub struct AFR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `AFR2` reader - AFR2"]
pub struct AFR2_R(crate::FieldReader<u8, u8>);
impl AFR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR2` writer - AFR2"]
pub struct AFR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `AFR3` reader - AFR3"]
pub struct AFR3_R(crate::FieldReader<u8, u8>);
impl AFR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR3` writer - AFR3"]
pub struct AFR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `AFR4` reader - AFR4"]
pub struct AFR4_R(crate::FieldReader<u8, u8>);
impl AFR4_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR4` writer - AFR4"]
pub struct AFR4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `AFR5` reader - AFR5"]
pub struct AFR5_R(crate::FieldReader<u8, u8>);
impl AFR5_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR5` writer - AFR5"]
pub struct AFR5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `AFR6` reader - AFR6"]
pub struct AFR6_R(crate::FieldReader<u8, u8>);
impl AFR6_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR6` writer - AFR6"]
pub struct AFR6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `AFR7` reader - AFR7"]
pub struct AFR7_R(crate::FieldReader<u8, u8>);
impl AFR7_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFR7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR7` writer - AFR7"]
pub struct AFR7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - AFR0"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFR1"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AFR2"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AFR3"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AFR4"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AFR5"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AFR6"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AFR7"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFR0"]
    #[inline(always)]
    pub fn afr0(&mut self) -> AFR0_W {
        AFR0_W { w: self }
    }
    #[doc = "Bits 4:7 - AFR1"]
    #[inline(always)]
    pub fn afr1(&mut self) -> AFR1_W {
        AFR1_W { w: self }
    }
    #[doc = "Bits 8:11 - AFR2"]
    #[inline(always)]
    pub fn afr2(&mut self) -> AFR2_W {
        AFR2_W { w: self }
    }
    #[doc = "Bits 12:15 - AFR3"]
    #[inline(always)]
    pub fn afr3(&mut self) -> AFR3_W {
        AFR3_W { w: self }
    }
    #[doc = "Bits 16:19 - AFR4"]
    #[inline(always)]
    pub fn afr4(&mut self) -> AFR4_W {
        AFR4_W { w: self }
    }
    #[doc = "Bits 20:23 - AFR5"]
    #[inline(always)]
    pub fn afr5(&mut self) -> AFR5_W {
        AFR5_W { w: self }
    }
    #[doc = "Bits 24:27 - AFR6"]
    #[inline(always)]
    pub fn afr6(&mut self) -> AFR6_W {
        AFR6_W { w: self }
    }
    #[doc = "Bits 28:31 - AFR7"]
    #[inline(always)]
    pub fn afr7(&mut self) -> AFR7_W {
        AFR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_afrl](index.html) module"]
pub struct GPIOA_AFRL_SPEC;
impl crate::RegisterSpec for GPIOA_AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioa_afrl::R](R) reader structure"]
impl crate::Readable for GPIOA_AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioa_afrl::W](W) writer structure"]
impl crate::Writable for GPIOA_AFRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOA_AFRL to value 0"]
impl crate::Resettable for GPIOA_AFRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
