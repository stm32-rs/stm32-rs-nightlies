///Register `ITLINE11` reader
pub type R = crate::R<ITLINE11rs>;
///Field `DMAMUX` reader - DMAMUX interrupt request pending
pub type DMAMUX_R = crate::BitReader;
///Field `DMA_CH4` reader - DMA channel 5 interrupt request pending Note: Only applicable on STM32C071xx, reserved on the other products.
pub type DMA_CH4_R = crate::BitReader;
///Field `DMA_CH5` reader - DMA channel 5 interrupt request pending Note: Only applicable on STM32C071xx, reserved on the other products.
pub type DMA_CH5_R = crate::BitReader;
impl R {
    ///Bit 0 - DMAMUX interrupt request pending
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA channel 5 interrupt request pending Note: Only applicable on STM32C071xx, reserved on the other products.
    #[inline(always)]
    pub fn dma_ch4(&self) -> DMA_CH4_R {
        DMA_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA channel 5 interrupt request pending Note: Only applicable on STM32C071xx, reserved on the other products.
    #[inline(always)]
    pub fn dma_ch5(&self) -> DMA_CH5_R {
        DMA_CH5_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE11")
            .field("dmamux", &self.dmamux())
            .field("dma_ch4", &self.dma_ch4())
            .field("dma_ch5", &self.dma_ch5())
            .finish()
    }
}
/**SYSCFG interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`itline11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:ITLINE11)*/
pub struct ITLINE11rs;
impl crate::RegisterSpec for ITLINE11rs {
    type Ux = u32;
}
///`read()` method returns [`itline11::R`](R) reader structure
impl crate::Readable for ITLINE11rs {}
///`reset()` method sets ITLINE11 to value 0
impl crate::Resettable for ITLINE11rs {
    const RESET_VALUE: u32 = 0;
}
