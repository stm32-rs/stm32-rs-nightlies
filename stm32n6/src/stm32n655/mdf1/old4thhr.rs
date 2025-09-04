///Register `OLD4THHR` reader
pub type R = crate::R<OLD4THHRrs>;
///Register `OLD4THHR` writer
pub type W = crate::W<OLD4THHRrs>;
///Field `OLDTHH` reader - OLDx high threshold value
pub type OLDTHH_R = crate::FieldReader<u32>;
///Field `OLDTHH` writer - OLDx high threshold value
pub type OLDTHH_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - OLDx high threshold value
    #[inline(always)]
    pub fn oldthh(&self) -> OLDTHH_R {
        OLDTHH_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OLD4THHR")
            .field("oldthh", &self.oldthh())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLDx high threshold value
    #[inline(always)]
    pub fn oldthh(&mut self) -> OLDTHH_W<OLD4THHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
/**MDF OLD4 high threshold register 4

You can [`read`](crate::Reg::read) this register and get [`old4thhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old4thhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:OLD4THHR)*/
pub struct OLD4THHRrs;
impl crate::RegisterSpec for OLD4THHRrs {
    type Ux = u32;
}
///`read()` method returns [`old4thhr::R`](R) reader structure
impl crate::Readable for OLD4THHRrs {}
///`write(|w| ..)` method takes [`old4thhr::W`](W) writer structure
impl crate::Writable for OLD4THHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD4THHR to value 0
impl crate::Resettable for OLD4THHRrs {}
