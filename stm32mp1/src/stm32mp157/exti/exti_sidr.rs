///Register `EXTI_SIDR` reader
pub type R = crate::R<EXTI_SIDRrs>;
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**EXTI size ID register

You can [`read`](crate::Reg::read) this register and get [`exti_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:EXTI_SIDR)*/
pub struct EXTI_SIDRrs;
impl crate::RegisterSpec for EXTI_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`exti_sidr::R`](R) reader structure
impl crate::Readable for EXTI_SIDRrs {}
///`reset()` method sets EXTI_SIDR to value 0xa3c5_dd01
impl crate::Resettable for EXTI_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
