#[doc = "Register `ITLINE9` reader"]
pub type R = crate::R<ITLINE9rs>;
#[doc = "Field `DMA1_CH1` reader - DMA1_CH1"]
pub type DMA1_CH1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA1_CH1"]
    #[inline(always)]
    pub fn dma1_ch1(&self) -> DMA1_CH1_R {
        DMA1_CH1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 9 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE9rs;
impl crate::RegisterSpec for ITLINE9rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline9::R`](R) reader structure"]
impl crate::Readable for ITLINE9rs {}
#[doc = "`reset()` method sets ITLINE9 to value 0"]
impl crate::Resettable for ITLINE9rs {
    const RESET_VALUE: u32 = 0;
}
