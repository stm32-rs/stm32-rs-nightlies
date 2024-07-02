///Register `GICD_ICENABLER5` reader
pub type R = crate::R<GICD_ICENABLER5rs>;
///Register `GICD_ICENABLER5` writer
pub type W = crate::W<GICD_ICENABLER5rs>;
///Field `ICENABLER5` reader - ICENABLER5
pub type ICENABLER5_R = crate::FieldReader<u32>;
///Field `ICENABLER5` writer - ICENABLER5
pub type ICENABLER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER5
    #[inline(always)]
    pub fn icenabler5(&self) -> ICENABLER5_R {
        ICENABLER5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICENABLER5")
            .field("icenabler5", &self.icenabler5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER5
    #[inline(always)]
    #[must_use]
    pub fn icenabler5(&mut self) -> ICENABLER5_W<GICD_ICENABLER5rs> {
        ICENABLER5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icenabler5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ICENABLER5)*/
pub struct GICD_ICENABLER5rs;
impl crate::RegisterSpec for GICD_ICENABLER5rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icenabler5::R`](R) reader structure
impl crate::Readable for GICD_ICENABLER5rs {}
///`write(|w| ..)` method takes [`gicd_icenabler5::W`](W) writer structure
impl crate::Writable for GICD_ICENABLER5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICENABLER5 to value 0
impl crate::Resettable for GICD_ICENABLER5rs {
    const RESET_VALUE: u32 = 0;
}
