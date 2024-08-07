///Register `GICD_IGROUPR1` reader
pub type R = crate::R<GICD_IGROUPR1rs>;
///Register `GICD_IGROUPR1` writer
pub type W = crate::W<GICD_IGROUPR1rs>;
///Field `IGROUPR1` reader - IGROUPR1
pub type IGROUPR1_R = crate::FieldReader<u32>;
///Field `IGROUPR1` writer - IGROUPR1
pub type IGROUPR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR1
    #[inline(always)]
    pub fn igroupr1(&self) -> IGROUPR1_R {
        IGROUPR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IGROUPR1")
            .field("igroupr1", &self.igroupr1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR1
    #[inline(always)]
    #[must_use]
    pub fn igroupr1(&mut self) -> IGROUPR1_W<GICD_IGROUPR1rs> {
        IGROUPR1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_igroupr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_IGROUPR1)*/
pub struct GICD_IGROUPR1rs;
impl crate::RegisterSpec for GICD_IGROUPR1rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_igroupr1::R`](R) reader structure
impl crate::Readable for GICD_IGROUPR1rs {}
///`write(|w| ..)` method takes [`gicd_igroupr1::W`](W) writer structure
impl crate::Writable for GICD_IGROUPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_IGROUPR1 to value 0
impl crate::Resettable for GICD_IGROUPR1rs {
    const RESET_VALUE: u32 = 0;
}
