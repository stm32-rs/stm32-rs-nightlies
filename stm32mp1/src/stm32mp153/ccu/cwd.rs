///Register `CWD` reader
pub type R = crate::R<CWDrs>;
///Register `CWD` writer
pub type W = crate::W<CWDrs>;
///Field `WDC` reader - WDC
pub type WDC_R = crate::FieldReader<u16>;
///Field `WDC` writer - WDC
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `WDV` reader - WDV
pub type WDV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - WDC
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - WDV
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWD")
            .field("wdc", &self.wdc())
            .field("wdv", &self.wdv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - WDC
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W<'_, CWDrs> {
        WDC_W::new(self, 0)
    }
}
/**The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.

You can [`read`](crate::Reg::read) this register and get [`cwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CCU:CWD)*/
pub struct CWDrs;
impl crate::RegisterSpec for CWDrs {
    type Ux = u32;
}
///`read()` method returns [`cwd::R`](R) reader structure
impl crate::Readable for CWDrs {}
///`write(|w| ..)` method takes [`cwd::W`](W) writer structure
impl crate::Writable for CWDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWD to value 0
impl crate::Resettable for CWDrs {}
