#[doc = "Register `ITLINE9` reader"]
pub struct R(crate::R<ITLINE9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA1_CH1` reader - DMA1_CH1"]
pub struct DMA1_CH1_R(crate::FieldReader<bool, bool>);
impl DMA1_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA1_CH1"]
    #[inline(always)]
    pub fn dma1_ch1(&self) -> DMA1_CH1_R {
        DMA1_CH1_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 9 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline9](index.html) module"]
pub struct ITLINE9_SPEC;
impl crate::RegisterSpec for ITLINE9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline9::R](R) reader structure"]
impl crate::Readable for ITLINE9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE9 to value 0"]
impl crate::Resettable for ITLINE9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
