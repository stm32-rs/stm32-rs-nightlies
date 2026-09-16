///Register `SISR` reader
pub type R = crate::R<SISRrs>;
///Field `SISF` reader - Secure interrupt semaphore x status bit before enable (mask) This bit is set by hardware and read only by software. Bit is cleared by software writing the corresponding HSEM_SCnICR bit x. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged read return 0 value.
pub type SISF_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Secure interrupt semaphore x status bit before enable (mask) This bit is set by hardware and read only by software. Bit is cleared by software writing the corresponding HSEM_SCnICR bit x. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged read return 0 value.
    #[inline(always)]
    pub fn sisf(&self) -> SISF_R {
        SISF_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SISR").field("sisf", &self.sisf()).finish()
    }
}
/**HSEM secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:SISR)*/
pub struct SISRrs;
impl crate::RegisterSpec for SISRrs {
    type Ux = u32;
}
///`read()` method returns [`sisr::R`](R) reader structure
impl crate::Readable for SISRrs {}
///`reset()` method sets SISR to value 0
impl crate::Resettable for SISRrs {}
