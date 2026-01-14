///Register `IGROUPR4` reader
pub type R = crate::R<IGROUPR4rs>;
///Register `IGROUPR4` writer
pub type W = crate::W<IGROUPR4rs>;
///Field `IGROUPR4` reader - IGROUPR4
pub type IGROUPR4_R = crate::FieldReader<u32>;
///Field `IGROUPR4` writer - IGROUPR4
pub type IGROUPR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR4
    #[inline(always)]
    pub fn igroupr4(&self) -> IGROUPR4_R {
        IGROUPR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR4")
            .field("igroupr4", &self.igroupr4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR4
    #[inline(always)]
    pub fn igroupr4(&mut self) -> IGROUPR4_W<'_, IGROUPR4rs> {
        IGROUPR4_W::new(self, 0)
    }
}
/**For interrupts ID = x*32 to ID = x*32+31

You can [`read`](crate::Reg::read) this register and get [`igroupr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:IGROUPR4)*/
pub struct IGROUPR4rs;
impl crate::RegisterSpec for IGROUPR4rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr4::R`](R) reader structure
impl crate::Readable for IGROUPR4rs {}
///`write(|w| ..)` method takes [`igroupr4::W`](W) writer structure
impl crate::Writable for IGROUPR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR4 to value 0
impl crate::Resettable for IGROUPR4rs {}
