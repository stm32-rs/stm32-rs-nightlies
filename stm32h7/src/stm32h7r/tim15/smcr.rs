///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TS` reader - TS\[0\]: Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See Section 68.4.2: TIM15/TIM16/TIM17 pins and internal signals for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (for example when SMS=000) to avoid wrong edge detections at the transition.
pub type TS_R = crate::FieldReader;
///Field `TS` writer - TS\[0\]: Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See Section 68.4.2: TIM15/TIM16/TIM17 pins and internal signals for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (for example when SMS=000) to avoid wrong edge detections at the transition.
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSM` reader - Master/slave mode
pub type MSM_R = crate::BitReader;
///Field `MSM` writer - Master/slave mode
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMS_1` reader - SMS\[3\]
pub type SMS_1_R = crate::BitReader;
///Field `SMS_1` writer - SMS\[3\]
pub type SMS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_1` reader - TS\[4:3\]
pub type TS_1_R = crate::FieldReader;
///Field `TS_1` writer - TS\[4:3\]
pub type TS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TS\[0\]: Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See Section 68.4.2: TIM15/TIM16/TIM17 pins and internal signals for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (for example when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_1(&self) -> SMS_1_R {
        SMS_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts_1(&self) -> TS_1_R {
        TS_1_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms", &self.sms())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("sms_1", &self.sms_1())
            .field("ts_1", &self.ts_1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Others: Reserved. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - TS\[0\]: Trigger selection This bit field selects the trigger input to be used to synchronize the counter. Others: Reserved See Section 68.4.2: TIM15/TIM16/TIM17 pins and internal signals for more details on tim_itrx meaning for each timer. Note: These bits must be changed only when they are not used (for example when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_1(&mut self) -> SMS_1_W<SMCRrs> {
        SMS_1_W::new(self, 16)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts_1(&mut self) -> TS_1_W<SMCRrs> {
        TS_1_W::new(self, 20)
    }
}
/**TIM15 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#TIM15:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}