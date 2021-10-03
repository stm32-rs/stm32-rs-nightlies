#[doc = "Register `DMACRxDTPR` reader"]
pub struct R(crate::R<DMACRXDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRXDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRXDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRXDTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACRxDTPR` writer"]
pub struct W(crate::W<DMACRXDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRXDTPR_SPEC>;
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
impl From<crate::W<DMACRXDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRXDTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer"]
pub struct RDT_R(crate::FieldReader<u32, u32>);
impl RDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer"]
pub struct RDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W {
        RDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_dtpr](index.html) module"]
pub struct DMACRXDTPR_SPEC;
impl crate::RegisterSpec for DMACRXDTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacrx_dtpr::R](R) reader structure"]
impl crate::Readable for DMACRXDTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacrx_dtpr::W](W) writer structure"]
impl crate::Writable for DMACRXDTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACRxDTPR to value 0"]
impl crate::Resettable for DMACRXDTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
