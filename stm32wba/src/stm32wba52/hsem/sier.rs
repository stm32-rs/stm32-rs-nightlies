///Register `SIER` reader
pub type R = crate::R<SIERrs>;
///Register `SIER` writer
pub type W = crate::W<SIERrs>;
///Field `SISE` reader - Secure interrupt semaphore x enable bit This bit is read and written by software. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. secure unprivileged write to this bit is discarded, secure unprivileged read return 0 value.
pub type SISE_R = crate::FieldReader<u16>;
///Field `SISE` writer - Secure interrupt semaphore x enable bit This bit is read and written by software. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. secure unprivileged write to this bit is discarded, secure unprivileged read return 0 value.
pub type SISE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Secure interrupt semaphore x enable bit This bit is read and written by software. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. secure unprivileged write to this bit is discarded, secure unprivileged read return 0 value.
    #[inline(always)]
    pub fn sise(&self) -> SISE_R {
        SISE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIER").field("sise", &self.sise()).finish()
    }
}
impl W {
    ///Bits 0:15 - Secure interrupt semaphore x enable bit This bit is read and written by software. When semaphore x PRIVx is disabled, bit x can be accessed with secure privilege and secure unprivileged access. When semaphore x PRIVx is enabled, bit x can be accessed only with secure privilege access. secure unprivileged write to this bit is discarded, secure unprivileged read return 0 value.
    #[inline(always)]
    pub fn sise(&mut self) -> SISE_W<'_, SIERrs> {
        SISE_W::new(self, 0)
    }
}
/**HSEM secure interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`sier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HSEM:SIER)*/
pub struct SIERrs;
impl crate::RegisterSpec for SIERrs {
    type Ux = u32;
}
///`read()` method returns [`sier::R`](R) reader structure
impl crate::Readable for SIERrs {}
///`write(|w| ..)` method takes [`sier::W`](W) writer structure
impl crate::Writable for SIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIER to value 0
impl crate::Resettable for SIERrs {}
