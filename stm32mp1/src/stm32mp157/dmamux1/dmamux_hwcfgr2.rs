#[doc = "Register `DMAMUX_HWCFGR2` reader"]
pub type R = crate::R<DMAMUX_HWCFGR2rs>;
#[doc = "Field `NUM_DMA_EXT_REQ` reader - NUM_DMA_EXT_REQ"]
pub type NUM_DMA_EXT_REQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NUM_DMA_EXT_REQ"]
    #[inline(always)]
    pub fn num_dma_ext_req(&self) -> NUM_DMA_EXT_REQ_R {
        NUM_DMA_EXT_REQ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMAMUX hardware configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_HWCFGR2rs;
impl crate::RegisterSpec for DMAMUX_HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_hwcfgr2::R`](R) reader structure"]
impl crate::Readable for DMAMUX_HWCFGR2rs {}
#[doc = "`reset()` method sets DMAMUX_HWCFGR2 to value 0x08"]
impl crate::Resettable for DMAMUX_HWCFGR2rs {
    const RESET_VALUE: u32 = 0x08;
}
