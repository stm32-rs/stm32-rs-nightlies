///Register `LCCRR` reader
pub type R = crate::R<LCCRRrs>;
///Register `LCCRR` writer
pub type W = crate::W<LCCRRrs>;
///Field `RELOAD` reader - reload value Reload value of the line clock counter.
pub type RELOAD_R = crate::FieldReader<u32>;
///Field `RELOAD` writer - reload value Reload value of the line clock counter.
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - reload value Reload value of the line clock counter.
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCCRR")
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - reload value Reload value of the line clock counter.
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, LCCRRrs> {
        RELOAD_W::new(self, 0)
    }
}
/**GFXTIM line clock counter reload register

You can [`read`](crate::Reg::read) this register and get [`lccrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GFXTIM:LCCRR)*/
pub struct LCCRRrs;
impl crate::RegisterSpec for LCCRRrs {
    type Ux = u32;
}
///`read()` method returns [`lccrr::R`](R) reader structure
impl crate::Readable for LCCRRrs {}
///`write(|w| ..)` method takes [`lccrr::W`](W) writer structure
impl crate::Writable for LCCRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCCRR to value 0
impl crate::Resettable for LCCRRrs {}
