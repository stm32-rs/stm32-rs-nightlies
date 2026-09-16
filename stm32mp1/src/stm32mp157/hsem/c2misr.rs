///Register `C2MISR` reader
pub type R = crate::R<C2MISRrs>;
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
        f.debug_struct("C2MISR")
            .field("misf", &self.misf())
            .finish()
    }
}
/**HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:C2MISR)*/
pub struct C2MISRrs;
impl crate::RegisterSpec for C2MISRrs {
    type Ux = u32;
}
///`read()` method returns [`c2misr::R`](R) reader structure
impl crate::Readable for C2MISRrs {}
///`reset()` method sets C2MISR to value 0
impl crate::Resettable for C2MISRrs {}
