///Register `IGROUPR8` reader
pub type R = crate::R<IGROUPR8rs>;
///Register `IGROUPR8` writer
pub type W = crate::W<IGROUPR8rs>;
///Field `IGROUPR8` reader - IGROUPR8
pub type IGROUPR8_R = crate::FieldReader<u32>;
///Field `IGROUPR8` writer - IGROUPR8
pub type IGROUPR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR8
    #[inline(always)]
    pub fn igroupr8(&self) -> IGROUPR8_R {
        IGROUPR8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR8")
            .field("igroupr8", &self.igroupr8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR8
    #[inline(always)]
    pub fn igroupr8(&mut self) -> IGROUPR8_W<'_, IGROUPR8rs> {
        IGROUPR8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:IGROUPR8)*/
pub struct IGROUPR8rs;
impl crate::RegisterSpec for IGROUPR8rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr8::R`](R) reader structure
impl crate::Readable for IGROUPR8rs {}
///`write(|w| ..)` method takes [`igroupr8::W`](W) writer structure
impl crate::Writable for IGROUPR8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR8 to value 0
impl crate::Resettable for IGROUPR8rs {}
