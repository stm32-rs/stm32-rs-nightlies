#[doc = "Register `OTG_FS_HNPTXFSIZ_Host` reader"]
pub struct R(crate::R<OTG_FS_HNPTXFSIZ_HOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_HNPTXFSIZ_HOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_HNPTXFSIZ_HOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_HNPTXFSIZ_HOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_HNPTXFSIZ_Host` writer"]
pub struct W(crate::W<OTG_FS_HNPTXFSIZ_HOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_HNPTXFSIZ_HOST_SPEC>;
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
impl From<crate::W<OTG_FS_HNPTXFSIZ_HOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_HNPTXFSIZ_HOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPTXFSA` reader - Non-periodic transmit RAM start address"]
pub struct NPTXFSA_R(crate::FieldReader<u16, u16>);
impl NPTXFSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFSA` writer - Non-periodic transmit RAM start address"]
pub struct NPTXFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `NPTXFD` reader - Non-periodic TxFIFO depth"]
pub struct NPTXFD_R(crate::FieldReader<u16, u16>);
impl NPTXFD_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFD` writer - Non-periodic TxFIFO depth"]
pub struct NPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W {
        NPTXFSA_W { w: self }
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W {
        NPTXFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS Host non-periodic transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hnptxfsiz_host](index.html) module"]
pub struct OTG_FS_HNPTXFSIZ_HOST_SPEC;
impl crate::RegisterSpec for OTG_FS_HNPTXFSIZ_HOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_hnptxfsiz_host::R](R) reader structure"]
impl crate::Readable for OTG_FS_HNPTXFSIZ_HOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_hnptxfsiz_host::W](W) writer structure"]
impl crate::Writable for OTG_FS_HNPTXFSIZ_HOST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_HNPTXFSIZ_Host to value 0x0200"]
impl crate::Resettable for OTG_FS_HNPTXFSIZ_HOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
