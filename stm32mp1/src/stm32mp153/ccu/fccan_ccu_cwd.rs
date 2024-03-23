#[doc = "Register `FCCAN_CCU_CWD` reader"]
pub type R = crate::R<FCCAN_CCU_CWDrs>;
#[doc = "Register `FCCAN_CCU_CWD` writer"]
pub type W = crate::W<FCCAN_CCU_CWDrs>;
#[doc = "Field `WDC` reader - WDC"]
pub type WDC_R = crate::FieldReader<u16>;
#[doc = "Field `WDC` writer - WDC"]
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDV` reader - WDV"]
pub type WDV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<FCCAN_CCU_CWDrs> {
        WDC_W::new(self, 0)
    }
}
#[doc = "The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_cwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_cwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCCAN_CCU_CWDrs;
impl crate::RegisterSpec for FCCAN_CCU_CWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccan_ccu_cwd::R`](R) reader structure"]
impl crate::Readable for FCCAN_CCU_CWDrs {}
#[doc = "`write(|w| ..)` method takes [`fccan_ccu_cwd::W`](W) writer structure"]
impl crate::Writable for FCCAN_CCU_CWDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCAN_CCU_CWD to value 0"]
impl crate::Resettable for FCCAN_CCU_CWDrs {
    const RESET_VALUE: u32 = 0;
}
