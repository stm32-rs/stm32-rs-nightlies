///Register `OLD5THLR` reader
pub type R = crate::R<OLD5THLRrs>;
///Register `OLD5THLR` writer
pub type W = crate::W<OLD5THLRrs>;
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
        f.debug_struct("OLD5THLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD low threshold value
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<'_, OLD5THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**MDF OLD5 low threshold register 5

You can [`read`](crate::Reg::read) this register and get [`old5thlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old5thlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:OLD5THLR)*/
pub struct OLD5THLRrs;
impl crate::RegisterSpec for OLD5THLRrs {
    type Ux = u32;
}
///`read()` method returns [`old5thlr::R`](R) reader structure
impl crate::Readable for OLD5THLRrs {}
///`write(|w| ..)` method takes [`old5thlr::W`](W) writer structure
impl crate::Writable for OLD5THLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD5THLR to value 0
impl crate::Resettable for OLD5THLRrs {}
