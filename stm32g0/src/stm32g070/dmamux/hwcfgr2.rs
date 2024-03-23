#[doc = "Register `HWCFGR2` reader"]
pub type R = crate::R<HWCFGR2rs>;
#[doc = "Field `NUM_DMA_EXT_REQ` reader - Number of DMA request trigger inputs"]
pub type NUM_DMA_EXT_REQ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of DMA request trigger inputs"]
    #[inline(always)]
    pub fn num_dma_ext_req(&self) -> NUM_DMA_EXT_REQ_R {
        NUM_DMA_EXT_REQ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMAMUX hardware configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr2::R`](R) reader structure"]
impl crate::Readable for HWCFGR2rs {}
#[doc = "`reset()` method sets HWCFGR2 to value 0x17"]
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x17;
}
