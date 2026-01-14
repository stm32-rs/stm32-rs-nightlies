///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC` reader - Semaphore x security attribute This bit is set and cleared by software.
pub type SEC_R = crate::FieldReader<u16>;
///Field `SEC` writer - Semaphore x security attribute This bit is set and cleared by software.
pub type SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Semaphore x security attribute This bit is set and cleared by software.
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR").field("sec", &self.sec()).finish()
    }
}
impl W {
    ///Bits 0:15 - Semaphore x security attribute This bit is set and cleared by software.
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 0)
    }
}
/**HSEM security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HSEM:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
