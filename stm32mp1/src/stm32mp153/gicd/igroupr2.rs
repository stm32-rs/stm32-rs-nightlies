///Register `IGROUPR2` reader
pub type R = crate::R<IGROUPR2rs>;
///Register `IGROUPR2` writer
pub type W = crate::W<IGROUPR2rs>;
///Field `IGROUPR2` reader - IGROUPR2
pub type IGROUPR2_R = crate::FieldReader<u32>;
///Field `IGROUPR2` writer - IGROUPR2
pub type IGROUPR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR2
    #[inline(always)]
    pub fn igroupr2(&self) -> IGROUPR2_R {
        IGROUPR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR2")
            .field("igroupr2", &self.igroupr2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR2
    #[inline(always)]
    pub fn igroupr2(&mut self) -> IGROUPR2_W<'_, IGROUPR2rs> {
        IGROUPR2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:IGROUPR2)*/
pub struct IGROUPR2rs;
impl crate::RegisterSpec for IGROUPR2rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr2::R`](R) reader structure
impl crate::Readable for IGROUPR2rs {}
///`write(|w| ..)` method takes [`igroupr2::W`](W) writer structure
impl crate::Writable for IGROUPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR2 to value 0
impl crate::Resettable for IGROUPR2rs {}
