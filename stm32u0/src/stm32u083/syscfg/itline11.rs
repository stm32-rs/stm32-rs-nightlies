///Register `ITLINE11` reader
pub type R = crate::R<ITLINE11rs>;
///Field `DMAMUX` reader - DMAMUX interrupt request pending
pub type DMAMUX_R = crate::BitReader;
///Field `DMA1_CH4` reader - DMA1 channel 4 interrupt request pending
pub type DMA1_CH4_R = crate::BitReader;
///Field `DMA1_CH5` reader - DMA1 channel 5 interrupt request pending
pub type DMA1_CH5_R = crate::BitReader;
///Field `DMA1_CH6` reader - DMA1 channel 6 interrupt request pending
pub type DMA1_CH6_R = crate::BitReader;
///Field `DMA1_CH7` reader - DMA1 channel 7 interrupt request pending
pub type DMA1_CH7_R = crate::BitReader;
///Field `DMA2_CH1` reader - DMA2 channel 1 interrupt request pending
pub type DMA2_CH1_R = crate::BitReader;
///Field `DMA2_CH2` reader - DMA2 channel 2 interrupt request pending
pub type DMA2_CH2_R = crate::BitReader;
///Field `DMA2_CH3` reader - DMA2 channel 3 interrupt request pending
pub type DMA2_CH3_R = crate::BitReader;
///Field `DMA2_CH4` reader - DMA2 channel 4 interrupt request pending
pub type DMA2_CH4_R = crate::BitReader;
///Field `DMA2_CH5` reader - DMA2 channel 5 interrupt request pending
pub type DMA2_CH5_R = crate::BitReader;
impl R {
    ///Bit 0 - DMAMUX interrupt request pending
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA1 channel 4 interrupt request pending
    #[inline(always)]
    pub fn dma1_ch4(&self) -> DMA1_CH4_R {
        DMA1_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA1 channel 5 interrupt request pending
    #[inline(always)]
    pub fn dma1_ch5(&self) -> DMA1_CH5_R {
        DMA1_CH5_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA1 channel 6 interrupt request pending
    #[inline(always)]
    pub fn dma1_ch6(&self) -> DMA1_CH6_R {
        DMA1_CH6_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DMA1 channel 7 interrupt request pending
    #[inline(always)]
    pub fn dma1_ch7(&self) -> DMA1_CH7_R {
        DMA1_CH7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA2 channel 1 interrupt request pending
    #[inline(always)]
    pub fn dma2_ch1(&self) -> DMA2_CH1_R {
        DMA2_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA2 channel 2 interrupt request pending
    #[inline(always)]
    pub fn dma2_ch2(&self) -> DMA2_CH2_R {
        DMA2_CH2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA2 channel 3 interrupt request pending
    #[inline(always)]
    pub fn dma2_ch3(&self) -> DMA2_CH3_R {
        DMA2_CH3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA2 channel 4 interrupt request pending
    #[inline(always)]
    pub fn dma2_ch4(&self) -> DMA2_CH4_R {
        DMA2_CH4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA2 channel 5 interrupt request pending
    #[inline(always)]
    pub fn dma2_ch5(&self) -> DMA2_CH5_R {
        DMA2_CH5_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE11")
            .field("dmamux", &self.dmamux())
            .field("dma1_ch4", &self.dma1_ch4())
            .field("dma1_ch5", &self.dma1_ch5())
            .field("dma1_ch6", &self.dma1_ch6())
            .field("dma1_ch7", &self.dma1_ch7())
            .field("dma2_ch1", &self.dma2_ch1())
            .field("dma2_ch2", &self.dma2_ch2())
            .field("dma2_ch3", &self.dma2_ch3())
            .field("dma2_ch4", &self.dma2_ch4())
            .field("dma2_ch5", &self.dma2_ch5())
            .finish()
    }
}
/**SYSCFG interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`itline11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:ITLINE11)*/
pub struct ITLINE11rs;
impl crate::RegisterSpec for ITLINE11rs {
    type Ux = u32;
}
///`read()` method returns [`itline11::R`](R) reader structure
impl crate::Readable for ITLINE11rs {}
///`reset()` method sets ITLINE11 to value 0
impl crate::Resettable for ITLINE11rs {}
