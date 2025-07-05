///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `ISE` reader - Non-secure Interrupt semaphore x enable bit This bit is read and written by software. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x is forced to 0 and cannot be accessed, write to this bit is discarded and a read returns 0. When semaphore x PRIVx is disabled, bit x can be accessed with privilege and unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with privileged access. Unprivileged write to this bit is discarded, unprivileged read returns 0.
pub type ISE_R = crate::FieldReader<u16>;
///Field `ISE` writer - Non-secure Interrupt semaphore x enable bit This bit is read and written by software. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x is forced to 0 and cannot be accessed, write to this bit is discarded and a read returns 0. When semaphore x PRIVx is disabled, bit x can be accessed with privilege and unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with privileged access. Unprivileged write to this bit is discarded, unprivileged read returns 0.
pub type ISE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Non-secure Interrupt semaphore x enable bit This bit is read and written by software. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x is forced to 0 and cannot be accessed, write to this bit is discarded and a read returns 0. When semaphore x PRIVx is disabled, bit x can be accessed with privilege and unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with privileged access. Unprivileged write to this bit is discarded, unprivileged read returns 0.
    #[inline(always)]
    pub fn ise(&self) -> ISE_R {
        ISE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER").field("ise", &self.ise()).finish()
    }
}
impl W {
    ///Bits 0:15 - Non-secure Interrupt semaphore x enable bit This bit is read and written by software. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x is forced to 0 and cannot be accessed, write to this bit is discarded and a read returns 0. When semaphore x PRIVx is disabled, bit x can be accessed with privilege and unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with privileged access. Unprivileged write to this bit is discarded, unprivileged read returns 0.
    #[inline(always)]
    pub fn ise(&mut self) -> ISE_W<IERrs> {
        ISE_W::new(self, 0)
    }
}
/**HSEM non-secure interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#HSEM:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
