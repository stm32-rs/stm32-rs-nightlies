///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `ISC` reader - Non-secure Interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x cannot be accessed, write to this bit is discarded. When semaphore x PRIVx is disabled, bit x can be accessed with privileged and unprivileged access. When semaphore x PRIVx is enabled, bit x can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type ISC_R = crate::FieldReader<u16>;
///Field `ISC` writer - Non-secure Interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x cannot be accessed, write to this bit is discarded. When semaphore x PRIVx is disabled, bit x can be accessed with privileged and unprivileged access. When semaphore x PRIVx is enabled, bit x can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type ISC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Non-secure Interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x cannot be accessed, write to this bit is discarded. When semaphore x PRIVx is disabled, bit x can be accessed with privileged and unprivileged access. When semaphore x PRIVx is enabled, bit x can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR").field("isc", &self.isc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Non-secure Interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x SECx is disabled, bit x can be accessed with secure and non-secure access. When semaphore x SECx is enabled, bit x cannot be accessed, write to this bit is discarded. When semaphore x PRIVx is disabled, bit x can be accessed with privileged and unprivileged access. When semaphore x PRIVx is enabled, bit x can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn isc(&mut self) -> ISC_W<ICRrs> {
        ISC_W::new(self, 0)
    }
}
/**HSEM non-secure interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
