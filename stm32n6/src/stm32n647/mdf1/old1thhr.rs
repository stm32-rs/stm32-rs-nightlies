///Register `OLD1THHR` reader
pub type R = crate::R<OLD1THHRrs>;
///Register `OLD1THHR` writer
pub type W = crate::W<OLD1THHRrs>;
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
        f.debug_struct("OLD1THHR")
            .field("oldthh", &self.oldthh())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLDx high threshold value
    #[inline(always)]
    pub fn oldthh(&mut self) -> OLDTHH_W<'_, OLD1THHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
/**MDF OLD1 high threshold register 1

You can [`read`](crate::Reg::read) this register and get [`old1thhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old1thhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:OLD1THHR)*/
pub struct OLD1THHRrs;
impl crate::RegisterSpec for OLD1THHRrs {
    type Ux = u32;
}
///`read()` method returns [`old1thhr::R`](R) reader structure
impl crate::Readable for OLD1THHRrs {}
///`write(|w| ..)` method takes [`old1thhr::W`](W) writer structure
impl crate::Writable for OLD1THHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD1THHR to value 0
impl crate::Resettable for OLD1THHRrs {}
