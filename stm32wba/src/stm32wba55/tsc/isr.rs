///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `EOAF` reader - End of acquisition flag This bit is set by hardware when the acquisition of all enabled group is complete (all GxS bits of all enabled analog I/O groups are set or when a max count error is detected). It is cleared by software writing 1 to the bit EOAIC of the TSC_ICR register.
pub type EOAF_R = crate::BitReader;
///Field `MCEF` reader - Max count error flag This bit is set by hardware as soon as an analog I/O group counter reaches the max count value specified. It is cleared by software writing 1 to the bit MCEIC of the TSC_ICR register.
pub type MCEF_R = crate::BitReader;
impl R {
    ///Bit 0 - End of acquisition flag This bit is set by hardware when the acquisition of all enabled group is complete (all GxS bits of all enabled analog I/O groups are set or when a max count error is detected). It is cleared by software writing 1 to the bit EOAIC of the TSC_ICR register.
    #[inline(always)]
    pub fn eoaf(&self) -> EOAF_R {
        EOAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error flag This bit is set by hardware as soon as an analog I/O group counter reaches the max count value specified. It is cleared by software writing 1 to the bit MCEIC of the TSC_ICR register.
    #[inline(always)]
    pub fn mcef(&self) -> MCEF_R {
        MCEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("eoaf", &self.eoaf())
            .field("mcef", &self.mcef())
            .finish()
    }
}
/**TSC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TSC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
