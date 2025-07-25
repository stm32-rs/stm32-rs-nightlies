///Register `WDGRR` reader
pub type R = crate::R<WDGRRrs>;
///Register `WDGRR` writer
pub type W = crate::W<WDGRRrs>;
///Field `RELOAD` reader - reload value
pub type RELOAD_R = crate::FieldReader<u16>;
///Field `RELOAD` writer - reload value
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - reload value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDGRR")
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - reload value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<WDGRRrs> {
        RELOAD_W::new(self, 0)
    }
}
/**GFXTIM watchdog reload register

You can [`read`](crate::Reg::read) this register and get [`wdgrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdgrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GFXTIM:WDGRR)*/
pub struct WDGRRrs;
impl crate::RegisterSpec for WDGRRrs {
    type Ux = u32;
}
///`read()` method returns [`wdgrr::R`](R) reader structure
impl crate::Readable for WDGRRrs {}
///`write(|w| ..)` method takes [`wdgrr::W`](W) writer structure
impl crate::Writable for WDGRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDGRR to value 0
impl crate::Resettable for WDGRRrs {}
