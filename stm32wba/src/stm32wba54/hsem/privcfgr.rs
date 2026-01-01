///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `PRIV` reader - Semaphore x privilege attribute This bit is set and cleared by software. When semaphore x SECx is disabled, bit x can be write accessed with secure privileged and non-secure privileged access. When semaphore x SECx is enabled, bit x can only be write accessed with secure privilege access. Non-secure privileged write access is discarded. Both secure and non-secure read return the register bit x value
pub type PRIV_R = crate::FieldReader<u16>;
///Field `PRIV` writer - Semaphore x privilege attribute This bit is set and cleared by software. When semaphore x SECx is disabled, bit x can be write accessed with secure privileged and non-secure privileged access. When semaphore x SECx is enabled, bit x can only be write accessed with secure privilege access. Non-secure privileged write access is discarded. Both secure and non-secure read return the register bit x value
pub type PRIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Semaphore x privilege attribute This bit is set and cleared by software. When semaphore x SECx is disabled, bit x can be write accessed with secure privileged and non-secure privileged access. When semaphore x SECx is enabled, bit x can only be write accessed with secure privilege access. Non-secure privileged write access is discarded. Both secure and non-secure read return the register bit x value
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Semaphore x privilege attribute This bit is set and cleared by software. When semaphore x SECx is disabled, bit x can be write accessed with secure privileged and non-secure privileged access. When semaphore x SECx is enabled, bit x can only be write accessed with secure privilege access. Non-secure privileged write access is discarded. Both secure and non-secure read return the register bit x value
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 0)
    }
}
/**HSEM privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#HSEM:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
