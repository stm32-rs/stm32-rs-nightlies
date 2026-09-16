///Register `JCHGR` reader
pub type R = crate::R<JCHGRrs>;
///Register `JCHGR` writer
pub type W = crate::W<JCHGRrs>;
///Field `JCHG` reader - Injected channel group selection
pub type JCHG_R = crate::FieldReader<u16>;
///Field `JCHG` writer - Injected channel group selection
pub type JCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JCHGR").field("jchg", &self.jchg()).finish()
    }
}
impl W {
    ///Bits 0:8 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&mut self) -> JCHG_W<'_, JCHGRrs> {
        JCHG_W::new(self, 0)
    }
}
/**injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`jchgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JCHGR)*/
pub struct JCHGRrs;
impl crate::RegisterSpec for JCHGRrs {
    type Ux = u32;
}
///`read()` method returns [`jchgr::R`](R) reader structure
impl crate::Readable for JCHGRrs {}
///`write(|w| ..)` method takes [`jchgr::W`](W) writer structure
impl crate::Writable for JCHGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCHGR to value 0x01
impl crate::Resettable for JCHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
