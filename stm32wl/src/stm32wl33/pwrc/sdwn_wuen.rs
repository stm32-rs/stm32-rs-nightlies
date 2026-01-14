///Register `SDWN_WUEN` reader
pub type R = crate::R<SDWN_WUENrs>;
///Register `SDWN_WUEN` writer
pub type W = crate::W<SDWN_WUENrs>;
///Field `WUEN` reader - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled
pub type WUEN_R = crate::BitReader;
///Field `WUEN` writer - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled
pub type WUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled
    #[inline(always)]
    pub fn wuen(&self) -> WUEN_R {
        WUEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDWN_WUEN")
            .field("wuen", &self.wuen())
            .finish()
    }
}
impl W {
    ///Bit 0 - WUEN PB0 I/O WakeUp from shutdown Enable When this bit is set the PB0 wakeup from shutdown is enabled so that a rising or falling edge on PB0 (depending on SDWN_WUPOL..WUPOL bit) will trigger a CPU wakeup. It is cleared by a PORESETn. - 0: PB0 wakeup from shutdown disabled - 1: PB0 wakeup from shutdown enabled
    #[inline(always)]
    pub fn wuen(&mut self) -> WUEN_W<'_, SDWN_WUENrs> {
        WUEN_W::new(self, 0)
    }
}
/**SDWN_WUEN register

You can [`read`](crate::Reg::read) this register and get [`sdwn_wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:SDWN_WUEN)*/
pub struct SDWN_WUENrs;
impl crate::RegisterSpec for SDWN_WUENrs {
    type Ux = u32;
}
///`read()` method returns [`sdwn_wuen::R`](R) reader structure
impl crate::Readable for SDWN_WUENrs {}
///`write(|w| ..)` method takes [`sdwn_wuen::W`](W) writer structure
impl crate::Writable for SDWN_WUENrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDWN_WUEN to value 0
impl crate::Resettable for SDWN_WUENrs {}
