#[doc = "Register `FDCAN_RXF1C` reader"]
pub struct R(crate::R<FDCAN_RXF1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF1C` writer"]
pub struct W(crate::W<FDCAN_RXF1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF1C_SPEC>;
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
impl From<crate::W<FDCAN_RXF1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1SA` reader - Rx FIFO 1 Start Address"]
pub struct F1SA_R(crate::FieldReader<u16, u16>);
impl F1SA_R {
    pub(crate) fn new(bits: u16) -> Self {
        F1SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1SA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1SA` writer - Rx FIFO 1 Start Address"]
pub struct F1SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F1SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `F1S` reader - Rx FIFO 1 Size"]
pub struct F1S_R(crate::FieldReader<u8, u8>);
impl F1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1S` writer - Rx FIFO 1 Size"]
pub struct F1S_W<'a> {
    w: &'a mut W,
}
impl<'a> F1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `F1WM` reader - Rx FIFO 1 Watermark"]
pub struct F1WM_R(crate::FieldReader<u8, u8>);
impl F1WM_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1WM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1WM` writer - Rx FIFO 1 Watermark"]
pub struct F1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&mut self) -> F1SA_W {
        F1SA_W { w: self }
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&mut self) -> F1S_W {
        F1S_W { w: self }
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&mut self) -> F1WM_W {
        F1WM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1c](index.html) module"]
pub struct FDCAN_RXF1C_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1c::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1c::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF1C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_RXF1C to value 0"]
impl crate::Resettable for FDCAN_RXF1C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
