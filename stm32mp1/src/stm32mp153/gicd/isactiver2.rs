///Register `ISACTIVER2` reader
pub type R = crate::R<ISACTIVER2rs>;
///Register `ISACTIVER2` writer
pub type W = crate::W<ISACTIVER2rs>;
///Field `ISACTIVER2` reader - ISACTIVER2
pub type ISACTIVER2_R = crate::FieldReader<u32>;
///Field `ISACTIVER2` writer - ISACTIVER2
pub type ISACTIVER2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER2
    #[inline(always)]
    pub fn isactiver2(&self) -> ISACTIVER2_R {
        ISACTIVER2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER2")
            .field("isactiver2", &self.isactiver2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER2
    #[inline(always)]
    pub fn isactiver2(&mut self) -> ISACTIVER2_W<'_, ISACTIVER2rs> {
        ISACTIVER2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISACTIVER2)*/
pub struct ISACTIVER2rs;
impl crate::RegisterSpec for ISACTIVER2rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver2::R`](R) reader structure
impl crate::Readable for ISACTIVER2rs {}
///`write(|w| ..)` method takes [`isactiver2::W`](W) writer structure
impl crate::Writable for ISACTIVER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER2 to value 0
impl crate::Resettable for ISACTIVER2rs {}
