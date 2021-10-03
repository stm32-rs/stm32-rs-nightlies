#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP18` reader - SMP18"]
pub struct SMP18_R(crate::FieldReader<u8, u8>);
impl SMP18_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP18` writer - SMP18"]
pub struct SMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `SMP17` reader - SMP17"]
pub struct SMP17_R(crate::FieldReader<u8, u8>);
impl SMP17_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP17` writer - SMP17"]
pub struct SMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SMP16` reader - SMP16"]
pub struct SMP16_R(crate::FieldReader<u8, u8>);
impl SMP16_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP16` writer - SMP16"]
pub struct SMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `SMP15` reader - SMP15"]
pub struct SMP15_R(crate::FieldReader<u8, u8>);
impl SMP15_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP15` writer - SMP15"]
pub struct SMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SMP14` reader - SMP14"]
pub struct SMP14_R(crate::FieldReader<u8, u8>);
impl SMP14_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP14` writer - SMP14"]
pub struct SMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SMP13` reader - SMP13"]
pub struct SMP13_R(crate::FieldReader<u8, u8>);
impl SMP13_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP13` writer - SMP13"]
pub struct SMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `SMP12` reader - SMP12"]
pub struct SMP12_R(crate::FieldReader<u8, u8>);
impl SMP12_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP12` writer - SMP12"]
pub struct SMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `SMP11` reader - SMP11"]
pub struct SMP11_R(crate::FieldReader<u8, u8>);
impl SMP11_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP11` writer - SMP11"]
pub struct SMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `SMP10` reader - SMP10"]
pub struct SMP10_R(crate::FieldReader<u8, u8>);
impl SMP10_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP10` writer - SMP10"]
pub struct SMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W {
        SMP18_W { w: self }
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W {
        SMP17_W { w: self }
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W {
        SMP16_W { w: self }
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W {
        SMP15_W { w: self }
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W {
        SMP14_W { w: self }
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W {
        SMP13_W { w: self }
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W {
        SMP12_W { w: self }
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W {
        SMP11_W { w: self }
    }
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W {
        SMP10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
