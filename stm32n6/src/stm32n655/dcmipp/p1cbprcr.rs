///Register `P1CBPRCR` reader
pub type R = crate::R<P1CBPRCRrs>;
///Field `ENABLE` reader - Current status of enable bit
pub type ENABLE_R = crate::BitReader;
///Field `STRENGTH` reader - Current strength (aggressiveness) of the bad pixel detection:
pub type STRENGTH_R = crate::FieldReader;
impl R {
    ///Bit 0 - Current status of enable bit
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Current strength (aggressiveness) of the bad pixel detection:
    #[inline(always)]
    pub fn strength(&self) -> STRENGTH_R {
        STRENGTH_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CBPRCR")
            .field("enable", &self.enable())
            .field("strength", &self.strength())
            .finish()
    }
}
/**DCMIPP Pipe1 current bad pixel removal register

You can [`read`](crate::Reg::read) this register and get [`p1cbprcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CBPRCR)*/
pub struct P1CBPRCRrs;
impl crate::RegisterSpec for P1CBPRCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cbprcr::R`](R) reader structure
impl crate::Readable for P1CBPRCRrs {}
///`reset()` method sets P1CBPRCR to value 0
impl crate::Resettable for P1CBPRCRrs {}
