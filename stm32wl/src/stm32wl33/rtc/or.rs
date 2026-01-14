///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `ALARMOUTTYPE` reader - RTC_ALARM on PA8 output type
pub type ALARMOUTTYPE_R = crate::BitReader;
///Field `ALARMOUTTYPE` writer - RTC_ALARM on PA8 output type
pub type ALARMOUTTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_OUT_RMP` reader - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8.
pub type RTC_OUT_RMP_R = crate::BitReader;
///Field `RTC_OUT_RMP` writer - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8.
pub type RTC_OUT_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RTC_ALARM on PA8 output type
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8.
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RTC_OUT_RMP_R {
        RTC_OUT_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("alarmouttype", &self.alarmouttype())
            .field("rtc_out_rmp", &self.rtc_out_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - RTC_ALARM on PA8 output type
    #[inline(always)]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W<'_, ORrs> {
        ALARMOUTTYPE_W::new(self, 0)
    }
    ///Bit 1 - RTC_OUT remap Setting this bit allows to remap the RTC outputs on PA9 as follows: 0 : If OSEL/= '00' : RTC_ALARM is ouput on PA8 If OSEL= '00' and COE = 1 : RTC_CALIB is output on PA8 1 : If OSEL /= '00' and COE = 0 : RTC_ALARM is output on PA9 If OSEL = '00' and COE = 1: RTC_CALIB is output on PA9 If OSEL /= '00' and COE = 1: RTC_CALIB is output on PA9 and RTC_ALARM is output on PA8. Note: the RTC outputs are functional in DEEPSTOP mode only on PA8.
    #[inline(always)]
    pub fn rtc_out_rmp(&mut self) -> RTC_OUT_RMP_W<'_, ORrs> {
        RTC_OUT_RMP_W::new(self, 1)
    }
}
/**RTC_OR register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RTC:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
