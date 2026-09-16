///Register `ISACTIVER1` reader
pub type R = crate::R<ISACTIVER1rs>;
///Register `ISACTIVER1` writer
pub type W = crate::W<ISACTIVER1rs>;
///Field `ISACTIVER1` reader - ISACTIVER1
pub type ISACTIVER1_R = crate::FieldReader<u32>;
///Field `ISACTIVER1` writer - ISACTIVER1
pub type ISACTIVER1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER1
    #[inline(always)]
    pub fn isactiver1(&self) -> ISACTIVER1_R {
        ISACTIVER1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER1")
            .field("isactiver1", &self.isactiver1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER1
    #[inline(always)]
    pub fn isactiver1(&mut self) -> ISACTIVER1_W<'_, ISACTIVER1rs> {
        ISACTIVER1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISACTIVER1)*/
pub struct ISACTIVER1rs;
impl crate::RegisterSpec for ISACTIVER1rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver1::R`](R) reader structure
impl crate::Readable for ISACTIVER1rs {}
///`write(|w| ..)` method takes [`isactiver1::W`](W) writer structure
impl crate::Writable for ISACTIVER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER1 to value 0
impl crate::Resettable for ISACTIVER1rs {}
