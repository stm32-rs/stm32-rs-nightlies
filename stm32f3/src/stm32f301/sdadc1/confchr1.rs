#[doc = "Register `CONFCHR1` reader"]
pub struct R(crate::R<CONFCHR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFCHR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFCHR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFCHR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFCHR1` writer"]
pub struct W(crate::W<CONFCHR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFCHR1_SPEC>;
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
impl From<crate::W<CONFCHR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFCHR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFCH7` reader - CONFCH7"]
pub struct CONFCH7_R(crate::FieldReader<u8, u8>);
impl CONFCH7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH7` writer - CONFCH7"]
pub struct CONFCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CONFCH6` reader - CONFCH6"]
pub struct CONFCH6_R(crate::FieldReader<u8, u8>);
impl CONFCH6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH6` writer - CONFCH6"]
pub struct CONFCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CONFCH5` reader - CONFCH5"]
pub struct CONFCH5_R(crate::FieldReader<u8, u8>);
impl CONFCH5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH5` writer - CONFCH5"]
pub struct CONFCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `CONFCH4` reader - CONFCH4"]
pub struct CONFCH4_R(crate::FieldReader<u8, u8>);
impl CONFCH4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH4` writer - CONFCH4"]
pub struct CONFCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CONFCH3` reader - CONFCH3"]
pub struct CONFCH3_R(crate::FieldReader<u8, u8>);
impl CONFCH3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH3` writer - CONFCH3"]
pub struct CONFCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CONFCH2` reader - CONFCH2"]
pub struct CONFCH2_R(crate::FieldReader<u8, u8>);
impl CONFCH2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH2` writer - CONFCH2"]
pub struct CONFCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CONFCH1` reader - CONFCH1"]
pub struct CONFCH1_R(crate::FieldReader<u8, u8>);
impl CONFCH1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH1` writer - CONFCH1"]
pub struct CONFCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CONFCH0` reader - CONFCH0"]
pub struct CONFCH0_R(crate::FieldReader<u8, u8>);
impl CONFCH0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH0` writer - CONFCH0"]
pub struct CONFCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29 - CONFCH7"]
    #[inline(always)]
    pub fn confch7(&self) -> CONFCH7_R {
        CONFCH7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CONFCH6"]
    #[inline(always)]
    pub fn confch6(&self) -> CONFCH6_R {
        CONFCH6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - CONFCH5"]
    #[inline(always)]
    pub fn confch5(&self) -> CONFCH5_R {
        CONFCH5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CONFCH4"]
    #[inline(always)]
    pub fn confch4(&self) -> CONFCH4_R {
        CONFCH4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - CONFCH3"]
    #[inline(always)]
    pub fn confch3(&self) -> CONFCH3_R {
        CONFCH3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CONFCH2"]
    #[inline(always)]
    pub fn confch2(&self) -> CONFCH2_R {
        CONFCH2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CONFCH1"]
    #[inline(always)]
    pub fn confch1(&self) -> CONFCH1_R {
        CONFCH1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - CONFCH0"]
    #[inline(always)]
    pub fn confch0(&self) -> CONFCH0_R {
        CONFCH0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - CONFCH7"]
    #[inline(always)]
    pub fn confch7(&mut self) -> CONFCH7_W {
        CONFCH7_W { w: self }
    }
    #[doc = "Bits 24:25 - CONFCH6"]
    #[inline(always)]
    pub fn confch6(&mut self) -> CONFCH6_W {
        CONFCH6_W { w: self }
    }
    #[doc = "Bits 20:21 - CONFCH5"]
    #[inline(always)]
    pub fn confch5(&mut self) -> CONFCH5_W {
        CONFCH5_W { w: self }
    }
    #[doc = "Bits 16:17 - CONFCH4"]
    #[inline(always)]
    pub fn confch4(&mut self) -> CONFCH4_W {
        CONFCH4_W { w: self }
    }
    #[doc = "Bits 12:13 - CONFCH3"]
    #[inline(always)]
    pub fn confch3(&mut self) -> CONFCH3_W {
        CONFCH3_W { w: self }
    }
    #[doc = "Bits 8:9 - CONFCH2"]
    #[inline(always)]
    pub fn confch2(&mut self) -> CONFCH2_W {
        CONFCH2_W { w: self }
    }
    #[doc = "Bits 4:5 - CONFCH1"]
    #[inline(always)]
    pub fn confch1(&mut self) -> CONFCH1_W {
        CONFCH1_W { w: self }
    }
    #[doc = "Bits 0:1 - CONFCH0"]
    #[inline(always)]
    pub fn confch0(&mut self) -> CONFCH0_W {
        CONFCH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confchr1](index.html) module"]
pub struct CONFCHR1_SPEC;
impl crate::RegisterSpec for CONFCHR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confchr1::R](R) reader structure"]
impl crate::Readable for CONFCHR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confchr1::W](W) writer structure"]
impl crate::Writable for CONFCHR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFCHR1 to value 0"]
impl crate::Resettable for CONFCHR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
