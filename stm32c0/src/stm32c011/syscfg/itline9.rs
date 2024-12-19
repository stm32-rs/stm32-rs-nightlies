///Register `ITLINE9` reader
pub type R = crate::R<ITLINE9rs>;
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
        f.debug_struct("ITLINE9")
            .field("dma1_ch1", &self.dma1_ch1())
            .finish()
    }
}
/**SYSCFG interrupt line 9 status register

You can [`read`](crate::Reg::read) this register and get [`itline9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE9)*/
pub struct ITLINE9rs;
impl crate::RegisterSpec for ITLINE9rs {
    type Ux = u32;
}
///`read()` method returns [`itline9::R`](R) reader structure
impl crate::Readable for ITLINE9rs {}
///`reset()` method sets ITLINE9 to value 0
impl crate::Resettable for ITLINE9rs {
    const RESET_VALUE: u32 = 0;
}
