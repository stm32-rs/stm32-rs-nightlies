///Register `OLD1THLR` reader
pub type R = crate::R<OLD1THLRrs>;
///Register `OLD1THLR` writer
pub type W = crate::W<OLD1THLRrs>;
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
        f.debug_struct("OLD1THLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD low threshold value
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<'_, OLD1THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**MDF OLD1 low threshold register 1

You can [`read`](crate::Reg::read) this register and get [`old1thlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old1thlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:OLD1THLR)*/
pub struct OLD1THLRrs;
impl crate::RegisterSpec for OLD1THLRrs {
    type Ux = u32;
}
///`read()` method returns [`old1thlr::R`](R) reader structure
impl crate::Readable for OLD1THLRrs {}
///`write(|w| ..)` method takes [`old1thlr::W`](W) writer structure
impl crate::Writable for OLD1THLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD1THLR to value 0
impl crate::Resettable for OLD1THLRrs {}
