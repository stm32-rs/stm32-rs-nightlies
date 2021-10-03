#[doc = "Register `OTG_FS_HPTXFSIZ` reader"]
pub struct R(crate::R<OTG_FS_HPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_HPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_HPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_HPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_HPTXFSIZ` writer"]
pub struct W(crate::W<OTG_FS_HPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_HPTXFSIZ_SPEC>;
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
impl From<crate::W<OTG_FS_HPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_HPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub struct PTXSA_R(crate::FieldReader<u16, u16>);
impl PTXSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        PTXSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub struct PTXSA_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PTXFSIZ` reader - Host periodic TxFIFO depth"]
pub struct PTXFSIZ_R(crate::FieldReader<u16, u16>);
impl PTXFSIZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        PTXFSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFSIZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFSIZ` writer - Host periodic TxFIFO depth"]
pub struct PTXFSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W {
        PTXSA_W { w: self }
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W {
        PTXFSIZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hptxfsiz](index.html) module"]
pub struct OTG_FS_HPTXFSIZ_SPEC;
impl crate::RegisterSpec for OTG_FS_HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_hptxfsiz::R](R) reader structure"]
impl crate::Readable for OTG_FS_HPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_hptxfsiz::W](W) writer structure"]
impl crate::Writable for OTG_FS_HPTXFSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for OTG_FS_HPTXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0600
    }
}
