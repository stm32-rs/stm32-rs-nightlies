///Register `OLD3THLR` reader
pub type R = crate::R<OLD3THLRrs>;
///Register `OLD3THLR` writer
pub type W = crate::W<OLD3THLRrs>;
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
        f.debug_struct("OLD3THLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD low threshold value
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<'_, OLD3THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**MDF OLD3 low threshold register 3

You can [`read`](crate::Reg::read) this register and get [`old3thlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old3thlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:OLD3THLR)*/
pub struct OLD3THLRrs;
impl crate::RegisterSpec for OLD3THLRrs {
    type Ux = u32;
}
///`read()` method returns [`old3thlr::R`](R) reader structure
impl crate::Readable for OLD3THLRrs {}
///`write(|w| ..)` method takes [`old3thlr::W`](W) writer structure
impl crate::Writable for OLD3THLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD3THLR to value 0
impl crate::Resettable for OLD3THLRrs {}
