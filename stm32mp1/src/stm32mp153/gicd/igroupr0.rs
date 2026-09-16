///Register `IGROUPR0` reader
pub type R = crate::R<IGROUPR0rs>;
///Register `IGROUPR0` writer
pub type W = crate::W<IGROUPR0rs>;
///Field `IGROUPR0` reader - IGROUPR0
pub type IGROUPR0_R = crate::FieldReader<u32>;
///Field `IGROUPR0` writer - IGROUPR0
pub type IGROUPR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR0
    #[inline(always)]
    pub fn igroupr0(&self) -> IGROUPR0_R {
        IGROUPR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IGROUPR0")
            .field("igroupr0", &self.igroupr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR0
    #[inline(always)]
    pub fn igroupr0(&mut self) -> IGROUPR0_W<'_, IGROUPR0rs> {
        IGROUPR0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:IGROUPR0)*/
pub struct IGROUPR0rs;
impl crate::RegisterSpec for IGROUPR0rs {
    type Ux = u32;
}
///`read()` method returns [`igroupr0::R`](R) reader structure
impl crate::Readable for IGROUPR0rs {}
///`write(|w| ..)` method takes [`igroupr0::W`](W) writer structure
impl crate::Writable for IGROUPR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IGROUPR0 to value 0
impl crate::Resettable for IGROUPR0rs {}
