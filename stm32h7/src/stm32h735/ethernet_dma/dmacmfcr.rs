#[doc = "Register `DMACMFCR` reader"]
pub struct R(crate::R<DMACMFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACMFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACMFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACMFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACMFCR` writer"]
pub struct W(crate::W<DMACMFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACMFCR_SPEC>;
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
impl From<crate::W<DMACMFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACMFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter"]
pub struct MFCO_R(crate::FieldReader<bool, bool>);
impl MFCO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MFCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFCO` writer - Overflow status of the MFC Counter"]
pub struct MFCO_W<'a> {
    w: &'a mut W,
}
impl<'a> MFCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MFC` reader - Dropped Packet Counters"]
pub struct MFC_R(crate::FieldReader<u16, u16>);
impl MFC_R {
    pub(crate) fn new(bits: u16) -> Self {
        MFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFC` writer - Dropped Packet Counters"]
pub struct MFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&mut self) -> MFCO_W {
        MFCO_W { w: self }
    }
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W {
        MFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacmfcr](index.html) module"]
pub struct DMACMFCR_SPEC;
impl crate::RegisterSpec for DMACMFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacmfcr::R](R) reader structure"]
impl crate::Readable for DMACMFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacmfcr::W](W) writer structure"]
impl crate::Writable for DMACMFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACMFCR to value 0"]
impl crate::Resettable for DMACMFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
