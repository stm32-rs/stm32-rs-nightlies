///Register `SYSCFG_ITLINE10` reader
pub type R = crate::R<SYSCFG_ITLINE10rs>;
///Field `DMA1_CH2` reader - DMA1 channel 2 interrupt request pending
pub type DMA1_CH2_R = crate::BitReader;
///Field `DMA1_CH3` reader - DMA1 channel 3 interrupt request pending
pub type DMA1_CH3_R = crate::BitReader;
impl R {
    ///Bit 0 - DMA1 channel 2 interrupt request pending
    #[inline(always)]
    pub fn dma1_ch2(&self) -> DMA1_CH2_R {
        DMA1_CH2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA1 channel 3 interrupt request pending
    #[inline(always)]
    pub fn dma1_ch3(&self) -> DMA1_CH3_R {
        DMA1_CH3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE10")
            .field("dma1_ch2", &self.dma1_ch2())
            .field("dma1_ch3", &self.dma1_ch3())
            .finish()
    }
}
/**SYSCFG interrupt line 10 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE10)*/
pub struct SYSCFG_ITLINE10rs;
impl crate::RegisterSpec for SYSCFG_ITLINE10rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline10::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE10rs {}
///`reset()` method sets SYSCFG_ITLINE10 to value 0
impl crate::Resettable for SYSCFG_ITLINE10rs {}
