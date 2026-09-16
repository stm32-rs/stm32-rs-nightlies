///Register `ITLINE11` reader
pub type R = crate::R<ITLINE11rs>;
/**DMAMUX

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<DMAMUX> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAMUX` reader - DMAMUX
pub type DMAMUX_R = crate::BitReader<DMAMUX>;
impl DMAMUX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAMUX {
        match self.bits {
            false => DMAMUX::NotInterrupted,
            true => DMAMUX::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == DMAMUX::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == DMAMUX::Interrupted
    }
}
///Field `DMA1_CH4` reader - DMA1_CH4
pub use DMAMUX_R as DMA1_CH4_R;
///Field `DMA1_CH5` reader - DMA1_CH5
pub use DMAMUX_R as DMA1_CH5_R;
///Field `DMA1_CH6` reader - DMA1_CH6
pub use DMAMUX_R as DMA1_CH6_R;
///Field `DMA1_CH7` reader - DMA1_CH7
pub use DMAMUX_R as DMA1_CH7_R;
///Field `DMA2_CH1` reader - DMA2_CH1
pub use DMAMUX_R as DMA2_CH1_R;
///Field `DMA2_CH2` reader - DMA2_CH2
pub use DMAMUX_R as DMA2_CH2_R;
///Field `DMA2_CH3` reader - DMA2_CH3
pub use DMAMUX_R as DMA2_CH3_R;
///Field `DMA2_CH4` reader - DMA2_CH4
pub use DMAMUX_R as DMA2_CH4_R;
///Field `DMA2_CH5` reader - DMA2_CH5
pub use DMAMUX_R as DMA2_CH5_R;
impl R {
    ///Bit 0 - DMAMUX
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA1_CH4
    #[inline(always)]
    pub fn dma1_ch4(&self) -> DMA1_CH4_R {
        DMA1_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA1_CH5
    #[inline(always)]
    pub fn dma1_ch5(&self) -> DMA1_CH5_R {
        DMA1_CH5_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA1_CH6
    #[inline(always)]
    pub fn dma1_ch6(&self) -> DMA1_CH6_R {
        DMA1_CH6_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DMA1_CH7
    #[inline(always)]
    pub fn dma1_ch7(&self) -> DMA1_CH7_R {
        DMA1_CH7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA2_CH1
    #[inline(always)]
    pub fn dma2_ch1(&self) -> DMA2_CH1_R {
        DMA2_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA2_CH2
    #[inline(always)]
    pub fn dma2_ch2(&self) -> DMA2_CH2_R {
        DMA2_CH2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA2_CH3
    #[inline(always)]
    pub fn dma2_ch3(&self) -> DMA2_CH3_R {
        DMA2_CH3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA2_CH4
    #[inline(always)]
    pub fn dma2_ch4(&self) -> DMA2_CH4_R {
        DMA2_CH4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA2_CH5
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
/**interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`itline11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#SYSCFG:ITLINE11)*/
pub struct ITLINE11rs;
impl crate::RegisterSpec for ITLINE11rs {
    type Ux = u32;
}
///`read()` method returns [`itline11::R`](R) reader structure
impl crate::Readable for ITLINE11rs {}
///`reset()` method sets ITLINE11 to value 0
impl crate::Resettable for ITLINE11rs {}
