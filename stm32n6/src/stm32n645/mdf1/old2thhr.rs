///Register `OLD2THHR` reader
pub type R = crate::R<OLD2THHRrs>;
///Register `OLD2THHR` writer
pub type W = crate::W<OLD2THHRrs>;
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
        f.debug_struct("OLD2THHR")
            .field("oldthh", &self.oldthh())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLDx high threshold value
    #[inline(always)]
    pub fn oldthh(&mut self) -> OLDTHH_W<'_, OLD2THHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
/**MDF OLD2 high threshold register 2

You can [`read`](crate::Reg::read) this register and get [`old2thhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old2thhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD2THHR)*/
pub struct OLD2THHRrs;
impl crate::RegisterSpec for OLD2THHRrs {
    type Ux = u32;
}
///`read()` method returns [`old2thhr::R`](R) reader structure
impl crate::Readable for OLD2THHRrs {}
///`write(|w| ..)` method takes [`old2thhr::W`](W) writer structure
impl crate::Writable for OLD2THHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD2THHR to value 0
impl crate::Resettable for OLD2THHRrs {}
