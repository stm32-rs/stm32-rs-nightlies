///Register `IGROUPR7` reader
pub type R = crate::R<IGROUPR7rs>;
///Register `IGROUPR7` writer
pub type W = crate::W<IGROUPR7rs>;
///Field `IGROUPR7` reader - IGROUPR7
pub type IGROUPR7_R = crate::FieldReader<u32>;
///Field `IGROUPR7` writer - IGROUPR7
pub type IGROUPR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR7
    #[inline(always)]
    pub fn igroupr7(&self) -> IGROUPR7_R {
        IGROUPR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR7")
            .field("igroupr7", &self.igroupr7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR7
    #[inline(always)]
    pub fn igroupr7(&mut self) -> IGROUPR7_W<'_, IGROUPR7rs> {
        IGROUPR7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR7)*/
pub struct IGROUPR7rs;
impl crate::RegisterSpec for IGROUPR7rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr7::R`](R) reader structure
impl crate::Readable for IGROUPR7rs {}
///`write(|w| ..)` method takes [`igroupr7::W`](W) writer structure
impl crate::Writable for IGROUPR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR7 to value 0
impl crate::Resettable for IGROUPR7rs {}
