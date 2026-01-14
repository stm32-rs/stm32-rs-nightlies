///Register `ITLINE9` reader
pub type R = crate::R<ITLINE9rs>;
/**DMA1_CH1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1_CH1 {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<DMA1_CH1> for bool {
    #[inline(always)]
    fn from(variant: DMA1_CH1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1_CH1` reader - DMA1_CH1
pub type DMA1_CH1_R = crate::BitReader<DMA1_CH1>;
impl DMA1_CH1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1_CH1 {
        match self.bits {
            false => DMA1_CH1::NotInterrupted,
            true => DMA1_CH1::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == DMA1_CH1::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == DMA1_CH1::Interrupted
    }
}
impl R {
    ///Bit 0 - DMA1_CH1
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
/**interrupt line 9 status register

You can [`read`](crate::Reg::read) this register and get [`itline9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#SYSCFG:ITLINE9)*/
pub struct ITLINE9rs;
impl crate::RegisterSpec for ITLINE9rs {
    type Ux = u32;
}
///`read()` method returns [`itline9::R`](R) reader structure
impl crate::Readable for ITLINE9rs {}
///`reset()` method sets ITLINE9 to value 0
impl crate::Resettable for ITLINE9rs {}
