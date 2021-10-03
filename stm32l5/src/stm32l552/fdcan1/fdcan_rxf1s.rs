#[doc = "Register `FDCAN_RXF1S` reader"]
pub struct R(crate::R<FDCAN_RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF1S` writer"]
pub struct W(crate::W<FDCAN_RXF1S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF1S_SPEC>;
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
impl From<crate::W<FDCAN_RXF1S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF1S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level"]
pub struct F1FL_R(crate::FieldReader<u8, u8>);
impl F1FL_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1FL` writer - Rx FIFO 1 Fill Level"]
pub struct F1FL_W<'a> {
    w: &'a mut W,
}
impl<'a> F1FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index"]
pub struct F1GI_R(crate::FieldReader<u8, u8>);
impl F1GI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index"]
pub struct F1PI_R(crate::FieldReader<u8, u8>);
impl F1PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1F` reader - Rx FIFO 1 Full"]
pub struct F1F_R(crate::FieldReader<bool, bool>);
impl F1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        F1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub struct RF1L_R(crate::FieldReader<bool, bool>);
impl RF1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&mut self) -> F1FL_W {
        F1FL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1s](index.html) module"]
pub struct FDCAN_RXF1S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1s::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1s::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF1S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FDCAN_RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
