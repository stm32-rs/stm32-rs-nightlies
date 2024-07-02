///Register `GICD_ICENABLER2` reader
pub type R = crate::R<GICD_ICENABLER2rs>;
///Register `GICD_ICENABLER2` writer
pub type W = crate::W<GICD_ICENABLER2rs>;
///Field `ICENABLER2` reader - ICENABLER2
pub type ICENABLER2_R = crate::FieldReader<u32>;
///Field `ICENABLER2` writer - ICENABLER2
pub type ICENABLER2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER2
    #[inline(always)]
    pub fn icenabler2(&self) -> ICENABLER2_R {
        ICENABLER2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICENABLER2")
            .field("icenabler2", &self.icenabler2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER2
    #[inline(always)]
    #[must_use]
    pub fn icenabler2(&mut self) -> ICENABLER2_W<GICD_ICENABLER2rs> {
        ICENABLER2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icenabler2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ICENABLER2)*/
pub struct GICD_ICENABLER2rs;
impl crate::RegisterSpec for GICD_ICENABLER2rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icenabler2::R`](R) reader structure
impl crate::Readable for GICD_ICENABLER2rs {}
///`write(|w| ..)` method takes [`gicd_icenabler2::W`](W) writer structure
impl crate::Writable for GICD_ICENABLER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICENABLER2 to value 0
impl crate::Resettable for GICD_ICENABLER2rs {
    const RESET_VALUE: u32 = 0;
}
