///Register `MSISR` reader
pub type R = crate::R<MSISRrs>;
///Field `SMISF` reader - Secure masked interrupt semaphore x status bit after enable (mask) This bit is set by hardware and read only by software. Bit is cleared by software writing the corresponding HSEM_SCnICR bit x. Bit is read as 0 when semaphore x status is masked in HSEM_SCnIER bit x. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged read return 0 value.
pub type SMISF_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Secure masked interrupt semaphore x status bit after enable (mask) This bit is set by hardware and read only by software. Bit is cleared by software writing the corresponding HSEM_SCnICR bit x. Bit is read as 0 when semaphore x status is masked in HSEM_SCnIER bit x. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged read return 0 value.
    #[inline(always)]
    pub fn smisf(&self) -> SMISF_R {
        SMISF_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSISR")
            .field("smisf", &self.smisf())
            .finish()
    }
}
/**HSEM secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`msisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HSEM:MSISR)*/
pub struct MSISRrs;
impl crate::RegisterSpec for MSISRrs {
    type Ux = u32;
}
///`read()` method returns [`msisr::R`](R) reader structure
impl crate::Readable for MSISRrs {}
///`reset()` method sets MSISR to value 0
impl crate::Resettable for MSISRrs {}
