#[doc = "Register `DMACTxDTPR` reader"]
pub struct R(crate::R<DMACTXDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTXDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTXDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTXDTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTxDTPR` writer"]
pub struct W(crate::W<DMACTXDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTXDTPR_SPEC>;
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
impl From<crate::W<DMACTXDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTXDTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer"]
pub struct TDT_R(crate::FieldReader<u32, u32>);
impl TDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        TDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer"]
pub struct TDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W {
        TDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_dtpr](index.html) module"]
pub struct DMACTXDTPR_SPEC;
impl crate::RegisterSpec for DMACTXDTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactx_dtpr::R](R) reader structure"]
impl crate::Readable for DMACTXDTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactx_dtpr::W](W) writer structure"]
impl crate::Writable for DMACTXDTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTxDTPR to value 0"]
impl crate::Resettable for DMACTXDTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
