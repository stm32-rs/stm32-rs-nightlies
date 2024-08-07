///Register `DDRPERFM_ISR` reader
pub type R = crate::R<DDRPERFM_ISRrs>;
///Field `OVFF` reader - OVFF
pub type OVFF_R = crate::BitReader;
impl R {
    ///Bit 0 - OVFF
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRPERFM_ISR")
            .field("ovff", &self.ovff())
            .finish()
    }
}
/**DDRPERFM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_ISR)*/
pub struct DDRPERFM_ISRrs;
impl crate::RegisterSpec for DDRPERFM_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_isr::R`](R) reader structure
impl crate::Readable for DDRPERFM_ISRrs {}
///`reset()` method sets DDRPERFM_ISR to value 0
impl crate::Resettable for DDRPERFM_ISRrs {
    const RESET_VALUE: u32 = 0;
}
