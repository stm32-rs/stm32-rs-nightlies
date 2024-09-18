///Register `GICD_IGROUPR5` reader
pub type R = crate::R<GICD_IGROUPR5rs>;
///Register `GICD_IGROUPR5` writer
pub type W = crate::W<GICD_IGROUPR5rs>;
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
        f.debug_struct("GICD_IGROUPR5")
            .field("igroupr5", &self.igroupr5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - IGROUPR5
    #[inline(always)]
    #[must_use]
    pub fn igroupr5(&mut self) -> IGROUPR5_W<GICD_IGROUPR5rs> {
        IGROUPR5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_igroupr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_IGROUPR5)*/
pub struct GICD_IGROUPR5rs;
impl crate::RegisterSpec for GICD_IGROUPR5rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_igroupr5::R`](R) reader structure
impl crate::Readable for GICD_IGROUPR5rs {}
///`write(|w| ..)` method takes [`gicd_igroupr5::W`](W) writer structure
impl crate::Writable for GICD_IGROUPR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_IGROUPR5 to value 0
impl crate::Resettable for GICD_IGROUPR5rs {
    const RESET_VALUE: u32 = 0;
}
