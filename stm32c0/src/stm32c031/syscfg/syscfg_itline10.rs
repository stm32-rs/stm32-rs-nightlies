#[doc = "Register `SYSCFG_ITLINE10` reader"]
pub type R = crate::R<SYSCFG_ITLINE10rs>;
#[doc = "Field `DMA1_CH2` reader - DMA1 channel 2 interrupt request pending"]
pub type DMA1_CH2_R = crate::BitReader;
#[doc = "Field `DMA1_CH3` reader - DMA1 channel 3 interrupt request pending"]
pub type DMA1_CH3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA1 channel 2 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch2(&self) -> DMA1_CH2_R {
        DMA1_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 channel 3 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch3(&self) -> DMA1_CH3_R {
        DMA1_CH3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 10 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE10rs;
impl crate::RegisterSpec for SYSCFG_ITLINE10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline10::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE10rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE10 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE10rs {
    const RESET_VALUE: u32 = 0;
}
