///Register `GICD_IGROUPR6` reader
pub type R = crate::R<GICD_IGROUPR6rs>;
///Register `GICD_IGROUPR6` writer
pub type W = crate::W<GICD_IGROUPR6rs>;
///Field `IGROUPR6` reader - IGROUPR6
pub type IGROUPR6_R = crate::FieldReader<u32>;
///Field `IGROUPR6` writer - IGROUPR6
pub type IGROUPR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - IGROUPR6
    #[inline(always)]
    pub fn igroupr6(&self) -> IGROUPR6_R {
        IGROUPR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IGROUPR6")
            .field("igroupr6", &self.igroupr6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR6
    #[inline(always)]
    #[must_use]
    pub fn igroupr6(&mut self) -> IGROUPR6_W<GICD_IGROUPR6rs> {
        IGROUPR6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_igroupr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_IGROUPR6)*/
pub struct GICD_IGROUPR6rs;
impl crate::RegisterSpec for GICD_IGROUPR6rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_igroupr6::R`](R) reader structure
impl crate::Readable for GICD_IGROUPR6rs {}
///`write(|w| ..)` method takes [`gicd_igroupr6::W`](W) writer structure
impl crate::Writable for GICD_IGROUPR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_IGROUPR6 to value 0
impl crate::Resettable for GICD_IGROUPR6rs {
    const RESET_VALUE: u32 = 0;
}
