///Register `OLD0THLR` reader
pub type R = crate::R<OLD0THLRrs>;
///Register `OLD0THLR` writer
pub type W = crate::W<OLD0THLRrs>;
///Field `OLDTHL` reader - OLD low threshold value
pub type OLDTHL_R = crate::FieldReader<u32>;
///Field `OLDTHL` writer - OLD low threshold value
pub type OLDTHL_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - OLD low threshold value
    #[inline(always)]
    pub fn oldthl(&self) -> OLDTHL_R {
        OLDTHL_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OLD0THLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD low threshold value
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<'_, OLD0THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**MDF OLD0 low threshold register 0

You can [`read`](crate::Reg::read) this register and get [`old0thlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old0thlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD0THLR)*/
pub struct OLD0THLRrs;
impl crate::RegisterSpec for OLD0THLRrs {
    type Ux = u32;
}
///`read()` method returns [`old0thlr::R`](R) reader structure
impl crate::Readable for OLD0THLRrs {}
///`write(|w| ..)` method takes [`old0thlr::W`](W) writer structure
impl crate::Writable for OLD0THLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD0THLR to value 0
impl crate::Resettable for OLD0THLRrs {}
