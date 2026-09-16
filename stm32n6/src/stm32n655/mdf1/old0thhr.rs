///Register `OLD0THHR` reader
pub type R = crate::R<OLD0THHRrs>;
///Register `OLD0THHR` writer
pub type W = crate::W<OLD0THHRrs>;
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
        f.debug_struct("OLD0THHR")
            .field("oldthh", &self.oldthh())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLDx high threshold value
    #[inline(always)]
    pub fn oldthh(&mut self) -> OLDTHH_W<'_, OLD0THHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
/**MDF OLD0 high threshold register 0

You can [`read`](crate::Reg::read) this register and get [`old0thhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old0thhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MDF1:OLD0THHR)*/
pub struct OLD0THHRrs;
impl crate::RegisterSpec for OLD0THHRrs {
    type Ux = u32;
}
///`read()` method returns [`old0thhr::R`](R) reader structure
impl crate::Readable for OLD0THHRrs {}
///`write(|w| ..)` method takes [`old0thhr::W`](W) writer structure
impl crate::Writable for OLD0THHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD0THHR to value 0
impl crate::Resettable for OLD0THHRrs {}
