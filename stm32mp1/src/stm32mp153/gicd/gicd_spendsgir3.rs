#[doc = "Register `GICD_SPENDSGIR3` reader"]
pub struct R(crate::R<GICD_SPENDSGIR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPENDSGIR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPENDSGIR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPENDSGIR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_SPENDSGIR3` writer"]
pub struct W(crate::W<GICD_SPENDSGIR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SPENDSGIR3_SPEC>;
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
impl From<crate::W<GICD_SPENDSGIR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SPENDSGIR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGI_SET_PENDING0` reader - SGI_SET_PENDING0"]
pub struct SGI_SET_PENDING0_R(crate::FieldReader<u8, u8>);
impl SGI_SET_PENDING0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SGI_SET_PENDING0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGI_SET_PENDING0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGI_SET_PENDING0` writer - SGI_SET_PENDING0"]
pub struct SGI_SET_PENDING0_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_SET_PENDING0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SGI_SET_PENDING1` reader - SGI_SET_PENDING1"]
pub struct SGI_SET_PENDING1_R(crate::FieldReader<u8, u8>);
impl SGI_SET_PENDING1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SGI_SET_PENDING1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGI_SET_PENDING1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGI_SET_PENDING1` writer - SGI_SET_PENDING1"]
pub struct SGI_SET_PENDING1_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_SET_PENDING1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SGI_SET_PENDING2` reader - SGI_SET_PENDING2"]
pub struct SGI_SET_PENDING2_R(crate::FieldReader<u8, u8>);
impl SGI_SET_PENDING2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SGI_SET_PENDING2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGI_SET_PENDING2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGI_SET_PENDING2` writer - SGI_SET_PENDING2"]
pub struct SGI_SET_PENDING2_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_SET_PENDING2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SGI_SET_PENDING3` reader - SGI_SET_PENDING3"]
pub struct SGI_SET_PENDING3_R(crate::FieldReader<u8, u8>);
impl SGI_SET_PENDING3_R {
    pub(crate) fn new(bits: u8) -> Self {
        SGI_SET_PENDING3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGI_SET_PENDING3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGI_SET_PENDING3` writer - SGI_SET_PENDING3"]
pub struct SGI_SET_PENDING3_W<'a> {
    w: &'a mut W,
}
impl<'a> SGI_SET_PENDING3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SGI_SET_PENDING0"]
    #[inline(always)]
    pub fn sgi_set_pending0(&self) -> SGI_SET_PENDING0_R {
        SGI_SET_PENDING0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SGI_SET_PENDING1"]
    #[inline(always)]
    pub fn sgi_set_pending1(&self) -> SGI_SET_PENDING1_R {
        SGI_SET_PENDING1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SGI_SET_PENDING2"]
    #[inline(always)]
    pub fn sgi_set_pending2(&self) -> SGI_SET_PENDING2_R {
        SGI_SET_PENDING2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - SGI_SET_PENDING3"]
    #[inline(always)]
    pub fn sgi_set_pending3(&self) -> SGI_SET_PENDING3_R {
        SGI_SET_PENDING3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SGI_SET_PENDING0"]
    #[inline(always)]
    pub fn sgi_set_pending0(&mut self) -> SGI_SET_PENDING0_W {
        SGI_SET_PENDING0_W { w: self }
    }
    #[doc = "Bits 8:9 - SGI_SET_PENDING1"]
    #[inline(always)]
    pub fn sgi_set_pending1(&mut self) -> SGI_SET_PENDING1_W {
        SGI_SET_PENDING1_W { w: self }
    }
    #[doc = "Bits 16:17 - SGI_SET_PENDING2"]
    #[inline(always)]
    pub fn sgi_set_pending2(&mut self) -> SGI_SET_PENDING2_W {
        SGI_SET_PENDING2_W { w: self }
    }
    #[doc = "Bits 24:25 - SGI_SET_PENDING3"]
    #[inline(always)]
    pub fn sgi_set_pending3(&mut self) -> SGI_SET_PENDING3_W {
        SGI_SET_PENDING3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spendsgir3](index.html) module"]
pub struct GICD_SPENDSGIR3_SPEC;
impl crate::RegisterSpec for GICD_SPENDSGIR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spendsgir3::R](R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_spendsgir3::W](W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_SPENDSGIR3 to value 0"]
impl crate::Resettable for GICD_SPENDSGIR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
