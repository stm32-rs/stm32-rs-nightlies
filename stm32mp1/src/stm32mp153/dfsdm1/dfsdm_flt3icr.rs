#[doc = "Register `DFSDM_FLT3ICR` reader"]
pub struct R(crate::R<DFSDM_FLT3ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT3ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT3ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT3ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT3ICR` writer"]
pub struct W(crate::W<DFSDM_FLT3ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT3ICR_SPEC>;
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
impl From<crate::W<DFSDM_FLT3ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT3ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRJOVRF` reader - CLRJOVRF"]
pub struct CLRJOVRF_R(crate::FieldReader<bool, bool>);
impl CLRJOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRJOVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRJOVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRJOVRF` writer - CLRJOVRF"]
pub struct CLRJOVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRJOVRF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CLRROVRF` reader - CLRROVRF"]
pub struct CLRROVRF_R(crate::FieldReader<bool, bool>);
impl CLRROVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRROVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRROVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRROVRF` writer - CLRROVRF"]
pub struct CLRROVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRROVRF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CLRCKABF` reader - CLRCKABF"]
pub struct CLRCKABF_R(crate::FieldReader<u8, u8>);
impl CLRCKABF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLRCKABF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRCKABF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRCKABF` writer - CLRCKABF"]
pub struct CLRCKABF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCKABF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CLRSCDF` reader - CLRSCDF"]
pub struct CLRSCDF_R(crate::FieldReader<u8, u8>);
impl CLRSCDF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLRSCDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRSCDF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRSCDF` writer - CLRSCDF"]
pub struct CLRSCDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSCDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - CLRJOVRF"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLRROVRF"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CLRCKABF"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CLRSCDF"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - CLRJOVRF"]
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W {
        CLRJOVRF_W { w: self }
    }
    #[doc = "Bit 3 - CLRROVRF"]
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W {
        CLRROVRF_W { w: self }
    }
    #[doc = "Bits 16:23 - CLRCKABF"]
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W {
        CLRCKABF_W { w: self }
    }
    #[doc = "Bits 24:31 - CLRSCDF"]
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W {
        CLRSCDF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 3 interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3icr](index.html) module"]
pub struct DFSDM_FLT3ICR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT3ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt3icr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT3ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3icr::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT3ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_FLT3ICR to value 0"]
impl crate::Resettable for DFSDM_FLT3ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}