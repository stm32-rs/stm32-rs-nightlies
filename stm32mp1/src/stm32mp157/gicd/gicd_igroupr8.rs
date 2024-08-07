///Register `GICD_IGROUPR8` reader
pub type R = crate::R<GICD_IGROUPR8rs>;
///Register `GICD_IGROUPR8` writer
pub type W = crate::W<GICD_IGROUPR8rs>;
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
        f.debug_struct("GICD_IGROUPR8")
            .field("igroupr8", &self.igroupr8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR8
    #[inline(always)]
    #[must_use]
    pub fn igroupr8(&mut self) -> IGROUPR8_W<GICD_IGROUPR8rs> {
        IGROUPR8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_igroupr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_IGROUPR8)*/
pub struct GICD_IGROUPR8rs;
impl crate::RegisterSpec for GICD_IGROUPR8rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_igroupr8::R`](R) reader structure
impl crate::Readable for GICD_IGROUPR8rs {}
///`write(|w| ..)` method takes [`gicd_igroupr8::W`](W) writer structure
impl crate::Writable for GICD_IGROUPR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_IGROUPR8 to value 0
impl crate::Resettable for GICD_IGROUPR8rs {
    const RESET_VALUE: u32 = 0;
}
