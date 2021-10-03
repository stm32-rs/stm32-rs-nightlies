#[doc = "Register `FDCAN_XIDFC` reader"]
pub struct R(crate::R<FDCAN_XIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_XIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_XIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_XIDFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_XIDFC` writer"]
pub struct W(crate::W<FDCAN_XIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_XIDFC_SPEC>;
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
impl From<crate::W<FDCAN_XIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_XIDFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLESA` reader - FLESA"]
pub struct FLESA_R(crate::FieldReader<u16, u16>);
impl FLESA_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLESA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLESA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLESA` writer - FLESA"]
pub struct FLESA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLESA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `LSE` reader - LSE"]
pub struct LSE_R(crate::FieldReader<u8, u8>);
impl LSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSE` writer - LSE"]
pub struct LSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - FLESA"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - LSE"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - FLESA"]
    #[inline(always)]
    pub fn flesa(&mut self) -> FLESA_W {
        FLESA_W { w: self }
    }
    #[doc = "Bits 16:23 - LSE"]
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W {
        LSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_xidfc](index.html) module"]
pub struct FDCAN_XIDFC_SPEC;
impl crate::RegisterSpec for FDCAN_XIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_xidfc::R](R) reader structure"]
impl crate::Readable for FDCAN_XIDFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_xidfc::W](W) writer structure"]
impl crate::Writable for FDCAN_XIDFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_XIDFC to value 0"]
impl crate::Resettable for FDCAN_XIDFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
