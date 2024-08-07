///Register `HSEM_C1MISR` reader
pub type R = crate::R<HSEM_C1MISRrs>;
///Field `MISF` reader - MISF
pub type MISF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - MISF
    #[inline(always)]
    pub fn misf(&self) -> MISF_R {
        MISF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C1MISR")
            .field("misf", &self.misf())
            .finish()
    }
}
/**HSEM i1terrupt status register

You can [`read`](crate::Reg::read) this register and get [`hsem_c1misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:HSEM_C1MISR)*/
pub struct HSEM_C1MISRrs;
impl crate::RegisterSpec for HSEM_C1MISRrs {
    type Ux = u32;
}
///`read()` method returns [`hsem_c1misr::R`](R) reader structure
impl crate::Readable for HSEM_C1MISRrs {}
///`reset()` method sets HSEM_C1MISR to value 0
impl crate::Resettable for HSEM_C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
