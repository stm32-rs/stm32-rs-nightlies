///Register `IGROUPR5` reader
pub type R = crate::R<IGROUPR5rs>;
///Register `IGROUPR5` writer
pub type W = crate::W<IGROUPR5rs>;
///Field `IGROUPR5` reader - IGROUPR5
pub type IGROUPR5_R = crate::FieldReader<u32>;
///Field `IGROUPR5` writer - IGROUPR5
pub type IGROUPR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR5
    #[inline(always)]
    pub fn igroupr5(&self) -> IGROUPR5_R {
        IGROUPR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR5")
            .field("igroupr5", &self.igroupr5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR5
    #[inline(always)]
    pub fn igroupr5(&mut self) -> IGROUPR5_W<'_, IGROUPR5rs> {
        IGROUPR5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR5)*/
pub struct IGROUPR5rs;
impl crate::RegisterSpec for IGROUPR5rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr5::R`](R) reader structure
impl crate::Readable for IGROUPR5rs {}
///`write(|w| ..)` method takes [`igroupr5::W`](W) writer structure
impl crate::Writable for IGROUPR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR5 to value 0
impl crate::Resettable for IGROUPR5rs {}
