///Register `HSEM_C2MISR` reader
pub type R = crate::R<HSEM_C2MISRrs>;
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
        f.debug_struct("HSEM_C2MISR")
            .field("misf", &self.misf())
            .finish()
    }
}
/**HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`hsem_c2misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C2MISR)*/
pub struct HSEM_C2MISRrs;
impl crate::RegisterSpec for HSEM_C2MISRrs {
    type Ux = u32;
}
///`read()` method returns [`hsem_c2misr::R`](R) reader structure
impl crate::Readable for HSEM_C2MISRrs {}
///`reset()` method sets HSEM_C2MISR to value 0
impl crate::Resettable for HSEM_C2MISRrs {
    const RESET_VALUE: u32 = 0;
}
