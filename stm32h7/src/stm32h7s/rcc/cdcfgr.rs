///Register `CDCFGR` reader
pub type R = crate::R<CDCFGRrs>;
///Register `CDCFGR` writer
pub type W = crate::W<CDCFGRrs>;
///Field `CPRE` reader - CPU domain core prescaler
pub type CPRE_R = crate::FieldReader;
///Field `CPRE` writer - CPU domain core prescaler
pub type CPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - CPU domain core prescaler
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDCFGR")
            .field("cpre", &self.cpre())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - CPU domain core prescaler
    #[inline(always)]
    pub fn cpre(&mut self) -> CPRE_W<'_, CDCFGRrs> {
        CPRE_W::new(self, 0)
    }
}
/**RCC CPU domain clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cdcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:CDCFGR)*/
pub struct CDCFGRrs;
impl crate::RegisterSpec for CDCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cdcfgr::R`](R) reader structure
impl crate::Readable for CDCFGRrs {}
///`write(|w| ..)` method takes [`cdcfgr::W`](W) writer structure
impl crate::Writable for CDCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CDCFGR to value 0
impl crate::Resettable for CDCFGRrs {}
