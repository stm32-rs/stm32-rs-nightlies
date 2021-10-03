#[doc = "Register `FLTCR` reader"]
pub struct R(crate::R<FLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTCR` writer"]
pub struct W(crate::W<FLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCR_SPEC>;
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
impl From<crate::W<FLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMPFREQ` reader - TAMPFREQ"]
pub struct TAMPFREQ_R(crate::FieldReader<u8, u8>);
impl TAMPFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPFREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPFREQ` writer - TAMPFREQ"]
pub struct TAMPFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `TAMPFLT` reader - TAMPFLT"]
pub struct TAMPFLT_R(crate::FieldReader<u8, u8>);
impl TAMPFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPFLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPFLT` writer - TAMPFLT"]
pub struct TAMPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `TAMPPRCH` reader - TAMPPRCH"]
pub struct TAMPPRCH_R(crate::FieldReader<u8, u8>);
impl TAMPPRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPPRCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPPRCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPRCH` writer - TAMPPRCH"]
pub struct TAMPPRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `TAMPPUDIS` reader - TAMPPUDIS"]
pub struct TAMPPUDIS_R(crate::FieldReader<bool, bool>);
impl TAMPPUDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPPUDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPPUDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPUDIS` writer - TAMPPUDIS"]
pub struct TAMPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPUDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W {
        TAMPPRCH_W { w: self }
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W {
        TAMPPUDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltcr](index.html) module"]
pub struct FLTCR_SPEC;
impl crate::RegisterSpec for FLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltcr::R](R) reader structure"]
impl crate::Readable for FLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltcr::W](W) writer structure"]
impl crate::Writable for FLTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FLTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
