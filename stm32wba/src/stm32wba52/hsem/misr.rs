///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Field `MISF` reader - Masked non-secure interrupt semaphore x status bit after enable (mask) This bit is set by hardware and read only by software. This bit is cleared by software writing the corresponding HSEM_ICR bit. This bit is read as 0 when semaphore x status is masked in HSEM_IER bit x. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x cannot be accessed, read returns 0. When semaphore x PRIVx is disabled, bit x can be accessed with privileged and unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with privileged access. Unprivileged read returns 0.
pub type MISF_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Masked non-secure interrupt semaphore x status bit after enable (mask) This bit is set by hardware and read only by software. This bit is cleared by software writing the corresponding HSEM_ICR bit. This bit is read as 0 when semaphore x status is masked in HSEM_IER bit x. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x cannot be accessed, read returns 0. When semaphore x PRIVx is disabled, bit x can be accessed with privileged and unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with privileged access. Unprivileged read returns 0.
    #[inline(always)]
    pub fn misf(&self) -> MISF_R {
        MISF_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR").field("misf", &self.misf()).finish()
    }
}
/**HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HSEM:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
