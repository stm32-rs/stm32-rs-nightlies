///Register `STATISTICS_RESTART` reader
pub type R = crate::R<STATISTICS_RESTARTrs>;
///Register `STATISTICS_RESTART` writer
pub type W = crate::W<STATISTICS_RESTARTrs>;
///Field `CLR_MIN_MAX` reader - Write '1' to clear the minimum and maximum registers.
pub type CLR_MIN_MAX_R = crate::BitReader;
///Field `CLR_MIN_MAX` writer - Write '1' to clear the minimum and maximum registers.
pub type CLR_MIN_MAX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_AVR` reader - Write '1' to clear the AVERAGE_PERIOD_LENGTH register value.
pub type CLR_AVR_R = crate::BitReader;
///Field `CLR_AVR` writer - Write '1' to clear the AVERAGE_PERIOD_LENGTH register value.
pub type CLR_AVR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Write '1' to clear the minimum and maximum registers.
    #[inline(always)]
    pub fn clr_min_max(&self) -> CLR_MIN_MAX_R {
        CLR_MIN_MAX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write '1' to clear the AVERAGE_PERIOD_LENGTH register value.
    #[inline(always)]
    pub fn clr_avr(&self) -> CLR_AVR_R {
        CLR_AVR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATISTICS_RESTART")
            .field("clr_min_max", &self.clr_min_max())
            .field("clr_avr", &self.clr_avr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Write '1' to clear the minimum and maximum registers.
    #[inline(always)]
    pub fn clr_min_max(&mut self) -> CLR_MIN_MAX_W<'_, STATISTICS_RESTARTrs> {
        CLR_MIN_MAX_W::new(self, 0)
    }
    ///Bit 1 - Write '1' to clear the AVERAGE_PERIOD_LENGTH register value.
    #[inline(always)]
    pub fn clr_avr(&mut self) -> CLR_AVR_W<'_, STATISTICS_RESTARTrs> {
        CLR_AVR_W::new(self, 1)
    }
}
/**STATISTICS_RESTART register

You can [`read`](crate::Reg::read) this register and get [`statistics_restart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statistics_restart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:STATISTICS_RESTART)*/
pub struct STATISTICS_RESTARTrs;
impl crate::RegisterSpec for STATISTICS_RESTARTrs {
    type Ux = u32;
}
///`read()` method returns [`statistics_restart::R`](R) reader structure
impl crate::Readable for STATISTICS_RESTARTrs {}
///`write(|w| ..)` method takes [`statistics_restart::W`](W) writer structure
impl crate::Writable for STATISTICS_RESTARTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATISTICS_RESTART to value 0
impl crate::Resettable for STATISTICS_RESTARTrs {}
