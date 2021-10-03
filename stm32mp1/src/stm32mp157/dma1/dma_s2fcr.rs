#[doc = "Register `DMA_S2FCR` reader"]
pub struct R(crate::R<DMA_S2FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S2FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S2FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S2FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_S2FCR` writer"]
pub struct W(crate::W<DMA_S2FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S2FCR_SPEC>;
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
impl From<crate::W<DMA_S2FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S2FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FTH"]
pub struct FTH_R(crate::FieldReader<u8, u8>);
impl FTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTH` writer - FTH"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DMDIS` reader - DMDIS"]
pub struct DMDIS_R(crate::FieldReader<bool, bool>);
impl DMDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMDIS` writer - DMDIS"]
pub struct DMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDIS_W<'a> {
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
#[doc = "Field `FS` reader - FS"]
pub struct FS_R(crate::FieldReader<u8, u8>);
impl FS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIE` reader - FEIE"]
pub struct FEIE_R(crate::FieldReader<bool, bool>);
impl FEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIE` writer - FEIE"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FTH"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - DMDIS"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - FS"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 7 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FTH"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
    #[doc = "Bit 2 - DMDIS"]
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W {
        DMDIS_W { w: self }
    }
    #[doc = "Bit 7 - FEIE"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA stream 2 FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2fcr](index.html) module"]
pub struct DMA_S2FCR_SPEC;
impl crate::RegisterSpec for DMA_S2FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_s2fcr::R](R) reader structure"]
impl crate::Readable for DMA_S2FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_s2fcr::W](W) writer structure"]
impl crate::Writable for DMA_S2FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_S2FCR to value 0x21"]
impl crate::Resettable for DMA_S2FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
