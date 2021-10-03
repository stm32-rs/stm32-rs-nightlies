#[doc = "Register `ATCR2` reader"]
pub struct R(crate::R<ATCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCR2` writer"]
pub struct W(crate::W<ATCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR2_SPEC>;
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
impl From<crate::W<ATCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATOSEL1` reader - ATOSEL1"]
pub struct ATOSEL1_R(crate::FieldReader<u8, u8>);
impl ATOSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL1` writer - ATOSEL1"]
pub struct ATOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `ATOSEL2` reader - ATOSEL2"]
pub struct ATOSEL2_R(crate::FieldReader<u8, u8>);
impl ATOSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL2` writer - ATOSEL2"]
pub struct ATOSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `ATOSEL3` reader - ATOSEL3"]
pub struct ATOSEL3_R(crate::FieldReader<u8, u8>);
impl ATOSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL3` writer - ATOSEL3"]
pub struct ATOSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `ATOSEL4` reader - ATOSEL4"]
pub struct ATOSEL4_R(crate::FieldReader<u8, u8>);
impl ATOSEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL4` writer - ATOSEL4"]
pub struct ATOSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `ATOSEL5` reader - ATOSEL5"]
pub struct ATOSEL5_R(crate::FieldReader<u8, u8>);
impl ATOSEL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL5` writer - ATOSEL5"]
pub struct ATOSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `ATOSEL6` reader - ATOSEL6"]
pub struct ATOSEL6_R(crate::FieldReader<u8, u8>);
impl ATOSEL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL6` writer - ATOSEL6"]
pub struct ATOSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `ATOSEL7` reader - ATOSEL7"]
pub struct ATOSEL7_R(crate::FieldReader<u8, u8>);
impl ATOSEL7_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL7` writer - ATOSEL7"]
pub struct ATOSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | ((value as u32 & 0x07) << 26);
        self.w
    }
}
#[doc = "Field `ATOSEL8` reader - ATOSEL8"]
pub struct ATOSEL8_R(crate::FieldReader<u8, u8>);
impl ATOSEL8_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL8` writer - ATOSEL8"]
pub struct ATOSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - ATOSEL5"]
    #[inline(always)]
    pub fn atosel5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - ATOSEL6"]
    #[inline(always)]
    pub fn atosel6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - ATOSEL7"]
    #[inline(always)]
    pub fn atosel7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - ATOSEL8"]
    #[inline(always)]
    pub fn atosel8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W {
        ATOSEL1_W { w: self }
    }
    #[doc = "Bits 11:13 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W {
        ATOSEL2_W { w: self }
    }
    #[doc = "Bits 14:16 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W {
        ATOSEL3_W { w: self }
    }
    #[doc = "Bits 17:19 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W {
        ATOSEL4_W { w: self }
    }
    #[doc = "Bits 20:22 - ATOSEL5"]
    #[inline(always)]
    pub fn atosel5(&mut self) -> ATOSEL5_W {
        ATOSEL5_W { w: self }
    }
    #[doc = "Bits 23:25 - ATOSEL6"]
    #[inline(always)]
    pub fn atosel6(&mut self) -> ATOSEL6_W {
        ATOSEL6_W { w: self }
    }
    #[doc = "Bits 26:28 - ATOSEL7"]
    #[inline(always)]
    pub fn atosel7(&mut self) -> ATOSEL7_W {
        ATOSEL7_W { w: self }
    }
    #[doc = "Bits 29:31 - ATOSEL8"]
    #[inline(always)]
    pub fn atosel8(&mut self) -> ATOSEL8_W {
        ATOSEL8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP active tamper control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr2](index.html) module"]
pub struct ATCR2_SPEC;
impl crate::RegisterSpec for ATCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcr2::R](R) reader structure"]
impl crate::Readable for ATCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcr2::W](W) writer structure"]
impl crate::Writable for ATCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATCR2 to value 0"]
impl crate::Resettable for ATCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
