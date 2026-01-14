///Register `DBPCR` reader
pub type R = crate::R<DBPCRrs>;
///Register `DBPCR` writer
pub type W = crate::W<DBPCRrs>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBPCR").field("dbp", &self.dbp()).finish()
    }
}
impl W {
    ///Bit 0 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, DBPCRrs> {
        DBP_W::new(self, 0)
    }
}
/**PWR disable backup protection control register

You can [`read`](crate::Reg::read) this register and get [`dbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#PWR:DBPCR)*/
pub struct DBPCRrs;
impl crate::RegisterSpec for DBPCRrs {
    type Ux = u32;
}
///`read()` method returns [`dbpcr::R`](R) reader structure
impl crate::Readable for DBPCRrs {}
///`write(|w| ..)` method takes [`dbpcr::W`](W) writer structure
impl crate::Writable for DBPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBPCR to value 0
impl crate::Resettable for DBPCRrs {}
