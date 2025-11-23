///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `LIF` reader - Line Interrupt flag
pub type LIF_R = crate::BitReader;
///Field `FUIF` reader - FIFO Underrun Interrupt flag
pub type FUIF_R = crate::BitReader;
///Field `TERRIF` reader - Transfer Error interrupt flag
pub type TERRIF_R = crate::BitReader;
///Field `RRIF` reader - Register Reload Interrupt Flag
pub type RRIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Line Interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer Error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Register Reload Interrupt Flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rrif", &self.rrif())
            .field("terrif", &self.terrif())
            .field("fuif", &self.fuif())
            .field("lif", &self.lif())
            .finish()
    }
}
/**Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#LTDC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
