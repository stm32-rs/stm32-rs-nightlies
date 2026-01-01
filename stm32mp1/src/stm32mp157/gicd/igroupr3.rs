///Register `IGROUPR3` reader
pub type R = crate::R<IGROUPR3rs>;
///Register `IGROUPR3` writer
pub type W = crate::W<IGROUPR3rs>;
///Field `IGROUPR3` reader - IGROUPR3
pub type IGROUPR3_R = crate::FieldReader<u32>;
///Field `IGROUPR3` writer - IGROUPR3
pub type IGROUPR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR3
    #[inline(always)]
    pub fn igroupr3(&self) -> IGROUPR3_R {
        IGROUPR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR3")
            .field("igroupr3", &self.igroupr3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR3
    #[inline(always)]
    pub fn igroupr3(&mut self) -> IGROUPR3_W<'_, IGROUPR3rs> {
        IGROUPR3_W::new(self, 0)
    }
}
/**For interrupts ID = x*32 to ID = x*32+31

You can [`read`](crate::Reg::read) this register and get [`igroupr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR3)*/
pub struct IGROUPR3rs;
impl crate::RegisterSpec for IGROUPR3rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr3::R`](R) reader structure
impl crate::Readable for IGROUPR3rs {}
///`write(|w| ..)` method takes [`igroupr3::W`](W) writer structure
impl crate::Writable for IGROUPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR3 to value 0
impl crate::Resettable for IGROUPR3rs {}
