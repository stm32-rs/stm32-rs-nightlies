///Register `SYSCFG_ITLINE9` reader
pub type R = crate::R<SYSCFG_ITLINE9rs>;
///Field `DMA1_CH1` reader - DMA1 channel 1interrupt request pending
pub type DMA1_CH1_R = crate::BitReader;
impl R {
    ///Bit 0 - DMA1 channel 1interrupt request pending
    #[inline(always)]
    pub fn dma1_ch1(&self) -> DMA1_CH1_R {
        DMA1_CH1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE9")
            .field("dma1_ch1", &self.dma1_ch1())
            .finish()
    }
}
/**SYSCFG interrupt line 9 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SYSCFG:SYSCFG_ITLINE9)*/
pub struct SYSCFG_ITLINE9rs;
impl crate::RegisterSpec for SYSCFG_ITLINE9rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline9::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE9rs {}
///`reset()` method sets SYSCFG_ITLINE9 to value 0
impl crate::Resettable for SYSCFG_ITLINE9rs {}
