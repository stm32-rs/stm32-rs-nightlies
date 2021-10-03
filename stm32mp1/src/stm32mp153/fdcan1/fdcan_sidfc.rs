#[doc = "Register `FDCAN_SIDFC` reader"]
pub struct R(crate::R<FDCAN_SIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_SIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_SIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_SIDFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_SIDFC` writer"]
pub struct W(crate::W<FDCAN_SIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_SIDFC_SPEC>;
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
impl From<crate::W<FDCAN_SIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_SIDFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLSSA` reader - FLSSA"]
pub struct FLSSA_R(crate::FieldReader<u16, u16>);
impl FLSSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLSSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLSSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLSSA` writer - FLSSA"]
pub struct FLSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `LSS` reader - LSS"]
pub struct LSS_R(crate::FieldReader<u8, u8>);
impl LSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSS` writer - LSS"]
pub struct LSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - FLSSA"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - LSS"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - FLSSA"]
    #[inline(always)]
    pub fn flssa(&mut self) -> FLSSA_W {
        FLSSA_W { w: self }
    }
    #[doc = "Bits 16:23 - LSS"]
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W {
        LSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_sidfc](index.html) module"]
pub struct FDCAN_SIDFC_SPEC;
impl crate::RegisterSpec for FDCAN_SIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_sidfc::R](R) reader structure"]
impl crate::Readable for FDCAN_SIDFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_sidfc::W](W) writer structure"]
impl crate::Writable for FDCAN_SIDFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_SIDFC to value 0"]
impl crate::Resettable for FDCAN_SIDFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
