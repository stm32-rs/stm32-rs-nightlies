///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `IREF` reader - Interrupt rising edge flag
pub type IREF_R = crate::BitReader;
///Field `IHLF` reader - Interrupt high-level flag
pub type IHLF_R = crate::BitReader;
///Field `IFEF` reader - Interrupt falling edge flag
pub type IFEF_R = crate::BitReader;
impl R {
    ///Bit 0 - Interrupt rising edge flag
    #[inline(always)]
    pub fn iref(&self) -> IREF_R {
        IREF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt high-level flag
    #[inline(always)]
    pub fn ihlf(&self) -> IHLF_R {
        IHLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt falling edge flag
    #[inline(always)]
    pub fn ifef(&self) -> IFEF_R {
        IFEF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("iref", &self.iref())
            .field("ihlf", &self.ihlf())
            .field("ifef", &self.ifef())
            .finish()
    }
}
/**FMC controller interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FMC1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
