///Register `ISACTIVER7` reader
pub type R = crate::R<ISACTIVER7rs>;
///Register `ISACTIVER7` writer
pub type W = crate::W<ISACTIVER7rs>;
///Field `ISACTIVER7` reader - ISACTIVER7
pub type ISACTIVER7_R = crate::FieldReader<u32>;
///Field `ISACTIVER7` writer - ISACTIVER7
pub type ISACTIVER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER7
    #[inline(always)]
    pub fn isactiver7(&self) -> ISACTIVER7_R {
        ISACTIVER7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER7")
            .field("isactiver7", &self.isactiver7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER7
    #[inline(always)]
    pub fn isactiver7(&mut self) -> ISACTIVER7_W<'_, ISACTIVER7rs> {
        ISACTIVER7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISACTIVER7)*/
pub struct ISACTIVER7rs;
impl crate::RegisterSpec for ISACTIVER7rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver7::R`](R) reader structure
impl crate::Readable for ISACTIVER7rs {}
///`write(|w| ..)` method takes [`isactiver7::W`](W) writer structure
impl crate::Writable for ISACTIVER7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER7 to value 0
impl crate::Resettable for ISACTIVER7rs {}
