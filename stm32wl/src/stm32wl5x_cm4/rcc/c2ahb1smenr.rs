#[doc = "Register `C2AHB1SMENR` reader"]
pub struct R(crate::R<C2AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB1SMENR` writer"]
pub struct W(crate::W<C2AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB1SMENR_SPEC>;
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
impl From<crate::W<C2AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCSMEN` reader - CRC clock enable during CPU2 CSleep mode."]
pub struct CRCSMEN_R(crate::FieldReader<bool, bool>);
impl CRCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCSMEN` writer - CRC clock enable during CPU2 CSleep mode."]
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during CPU2 CSleep mode."]
pub struct DMAMUX1SMEN_R(crate::FieldReader<bool, bool>);
impl DMAMUX1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during CPU2 CSleep mode."]
pub struct DMAMUX1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DMA2SMEN` reader - DMA2 clock enable during CPU2 CSleep mode."]
pub struct DMA2SMEN_R(crate::FieldReader<bool, bool>);
impl DMA2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2SMEN` writer - DMA2 clock enable during CPU2 CSleep mode."]
pub struct DMA2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 clock enable during CPU2 CSleep mode."]
pub struct DMA1SMEN_R(crate::FieldReader<bool, bool>);
impl DMA1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1SMEN` writer - DMA1 clock enable during CPU2 CSleep mode."]
pub struct DMA1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - CRC clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - CRC clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W {
        DMAMUX1SMEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W {
        DMA2SMEN_W { w: self }
    }
    #[doc = "Bit 0 - DMA1 clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W {
        DMA1SMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep modes register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb1smenr](index.html) module"]
pub struct C2AHB1SMENR_SPEC;
impl crate::RegisterSpec for C2AHB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb1smenr::R](R) reader structure"]
impl crate::Readable for C2AHB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb1smenr::W](W) writer structure"]
impl crate::Writable for C2AHB1SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB1SMENR to value 0x1007"]
impl crate::Resettable for C2AHB1SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1007
    }
}
