///Register `LTDC_ISR` reader
pub type R = crate::R<LTDC_ISRrs>;
///Field `LIF` reader - line interrupt flag
pub type LIF_R = crate::BitReader;
///Field `FUIF` reader - FIFO underrun interrupt flag
pub type FUIF_R = crate::BitReader;
///Field `TERRIF` reader - transfer error interrupt flag
pub type TERRIF_R = crate::BitReader;
///Field `RRIF` reader - register reload interrupt flag
pub type RRIF_R = crate::BitReader;
impl R {
    ///Bit 0 - line interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO underrun interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - transfer error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - register reload interrupt flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_ISR")
            .field("lif", &self.lif())
            .field("fuif", &self.fuif())
            .field("terrif", &self.terrif())
            .field("rrif", &self.rrif())
            .finish()
    }
}
/**LTDC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ltdc_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_ISR)*/
pub struct LTDC_ISRrs;
impl crate::RegisterSpec for LTDC_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_isr::R`](R) reader structure
impl crate::Readable for LTDC_ISRrs {}
///`reset()` method sets LTDC_ISR to value 0
impl crate::Resettable for LTDC_ISRrs {
    const RESET_VALUE: u32 = 0;
}
