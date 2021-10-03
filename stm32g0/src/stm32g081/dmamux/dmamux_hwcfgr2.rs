#[doc = "Register `DMAMUX_HWCFGR2` reader"]
pub struct R(crate::R<DMAMUX_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMUX_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMUX_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMUX_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_DMA_EXT_REQ` reader - Number of DMA request trigger inputs"]
pub struct NUM_DMA_EXT_REQ_R(crate::FieldReader<u8, u8>);
impl NUM_DMA_EXT_REQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_DMA_EXT_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_DMA_EXT_REQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of DMA request trigger inputs"]
    #[inline(always)]
    pub fn num_dma_ext_req(&self) -> NUM_DMA_EXT_REQ_R {
        NUM_DMA_EXT_REQ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMAMUX hardware configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_hwcfgr2](index.html) module"]
pub struct DMAMUX_HWCFGR2_SPEC;
impl crate::RegisterSpec for DMAMUX_HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamux_hwcfgr2::R](R) reader structure"]
impl crate::Readable for DMAMUX_HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAMUX_HWCFGR2 to value 0x17"]
impl crate::Resettable for DMAMUX_HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
