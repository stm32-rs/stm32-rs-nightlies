///Register `SICR` reader
pub type R = crate::R<SICRrs>;
///Register `SICR` writer
pub type W = crate::W<SICRrs>;
///Field `SISC` reader - Secure interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged write to this bit is discarded.
pub type SISC_R = crate::FieldReader<u16>;
///Field `SISC` writer - Secure interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged write to this bit is discarded.
pub type SISC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Secure interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn sisc(&self) -> SISC_R {
        SISC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SICR").field("sisc", &self.sisc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Secure interrupt semaphore x clear bit This bit is written by software, and is always read 0. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. Secure unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn sisc(&mut self) -> SISC_W<'_, SICRrs> {
        SISC_W::new(self, 0)
    }
}
/**HSEM secure interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`sicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#HSEM:SICR)*/
pub struct SICRrs;
impl crate::RegisterSpec for SICRrs {
    type Ux = u32;
}
///`read()` method returns [`sicr::R`](R) reader structure
impl crate::Readable for SICRrs {}
///`write(|w| ..)` method takes [`sicr::W`](W) writer structure
impl crate::Writable for SICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SICR to value 0
impl crate::Resettable for SICRrs {}
