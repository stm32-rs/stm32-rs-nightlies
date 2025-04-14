///Register `ITLINE10` reader
pub type R = crate::R<ITLINE10rs>;
/**DMA1_CH1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1_CH2 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<DMA1_CH2> for bool {
    #[inline(always)]
    fn from(variant: DMA1_CH2) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1_CH2` reader - DMA1_CH1
pub type DMA1_CH2_R = crate::BitReader<DMA1_CH2>;
impl DMA1_CH2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1_CH2 {
        match self.bits {
            false => DMA1_CH2::NotInterrupted,
            true => DMA1_CH2::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == DMA1_CH2::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == DMA1_CH2::Interrupted
    }
}
///Field `DMA1_CH3` reader - DMA1_CH3
pub use DMA1_CH2_R as DMA1_CH3_R;
impl R {
    ///Bit 0 - DMA1_CH1
    #[inline(always)]
    pub fn dma1_ch2(&self) -> DMA1_CH2_R {
        DMA1_CH2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA1_CH3
    #[inline(always)]
    pub fn dma1_ch3(&self) -> DMA1_CH3_R {
        DMA1_CH3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE10")
            .field("dma1_ch2", &self.dma1_ch2())
            .field("dma1_ch3", &self.dma1_ch3())
            .finish()
    }
}
/**interrupt line 10 status register

You can [`read`](crate::Reg::read) this register and get [`itline10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE10)*/
pub struct ITLINE10rs;
impl crate::RegisterSpec for ITLINE10rs {
    type Ux = u32;
}
///`read()` method returns [`itline10::R`](R) reader structure
impl crate::Readable for ITLINE10rs {}
///`reset()` method sets ITLINE10 to value 0
impl crate::Resettable for ITLINE10rs {}
