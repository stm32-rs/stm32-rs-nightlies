///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `LIF` reader - Line interrupt flag
pub type LIF_R = crate::BitReader;
///Field `FUWIF` reader - FIFO underrun warning interrupt flag
pub type FUWIF_R = crate::BitReader;
///Field `TERRIF` reader - Transfer error interrupt flag
pub type TERRIF_R = crate::BitReader;
///Field `RRIF` reader - Register reload interrupt flag
pub type RRIF_R = crate::BitReader;
///Field `FUIF` reader - FIFO underrun interrupt flag
pub type FUIF_R = crate::BitReader;
///Field `CRCIF` reader - CRC error interrupt flag
pub type CRCIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Line interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO underrun warning interrupt flag
    #[inline(always)]
    pub fn fuwif(&self) -> FUWIF_R {
        FUWIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Register reload interrupt flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - FIFO underrun interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC error interrupt flag
    #[inline(always)]
    pub fn crcif(&self) -> CRCIF_R {
        CRCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("lif", &self.lif())
            .field("fuwif", &self.fuwif())
            .field("terrif", &self.terrif())
            .field("rrif", &self.rrif())
            .field("fuif", &self.fuif())
            .field("crcif", &self.crcif())
            .finish()
    }
}
/**LTDC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
