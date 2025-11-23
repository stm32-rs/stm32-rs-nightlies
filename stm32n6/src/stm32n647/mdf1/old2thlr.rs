///Register `OLD2THLR` reader
pub type R = crate::R<OLD2THLRrs>;
///Register `OLD2THLR` writer
pub type W = crate::W<OLD2THLRrs>;
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
        f.debug_struct("OLD2THLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD low threshold value
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<'_, OLD2THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**MDF OLD2 low threshold register 2

You can [`read`](crate::Reg::read) this register and get [`old2thlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old2thlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:OLD2THLR)*/
pub struct OLD2THLRrs;
impl crate::RegisterSpec for OLD2THLRrs {
    type Ux = u32;
}
///`read()` method returns [`old2thlr::R`](R) reader structure
impl crate::Readable for OLD2THLRrs {}
///`write(|w| ..)` method takes [`old2thlr::W`](W) writer structure
impl crate::Writable for OLD2THLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD2THLR to value 0
impl crate::Resettable for OLD2THLRrs {}
