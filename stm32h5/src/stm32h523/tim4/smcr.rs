///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - SMS\[2:0\]: Slave mode selection
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - SMS\[2:0\]: Slave mode selection
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Field `OCCS` reader - OCREF clear selection
pub type OCCS_R = crate::BitReader;
///Field `OCCS` writer - OCREF clear selection
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS` reader - TS\[2:0\]: Trigger selection
pub type TS_R = crate::FieldReader;
///Field `TS` writer - TS\[2:0\]: Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///External clock enable
pub use crate::stm32h523::tim3::smcr::ECE;
///Field `ECE` reader - External clock enable
pub use crate::stm32h523::tim3::smcr::ECE_R;
///Field `ECE` writer - External clock enable
pub use crate::stm32h523::tim3::smcr::ECE_W;
///External trigger filter
pub use crate::stm32h523::tim3::smcr::ETF;
///Field `ETF` reader - External trigger filter
pub use crate::stm32h523::tim3::smcr::ETF_R;
///Field `ETF` writer - External trigger filter
pub use crate::stm32h523::tim3::smcr::ETF_W;
///External trigger polarity
pub use crate::stm32h523::tim3::smcr::ETP;
///External trigger prescaler
pub use crate::stm32h523::tim3::smcr::ETPS;
///Field `ETPS` reader - External trigger prescaler
pub use crate::stm32h523::tim3::smcr::ETPS_R;
///Field `ETPS` writer - External trigger prescaler
pub use crate::stm32h523::tim3::smcr::ETPS_W;
///Field `ETP` reader - External trigger polarity
pub use crate::stm32h523::tim3::smcr::ETP_R;
///Field `ETP` writer - External trigger polarity
pub use crate::stm32h523::tim3::smcr::ETP_W;
///Master/Slave mode
pub use crate::stm32h523::tim3::smcr::MSM;
///Field `MSM` reader - Master/Slave mode
pub use crate::stm32h523::tim3::smcr::MSM_R;
///Field `MSM` writer - Master/Slave mode
pub use crate::stm32h523::tim3::smcr::MSM_W;
///Field `SMS_3` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS2` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS2_R = crate::FieldReader;
///Field `TS2` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
///SMS preload enable
pub use crate::stm32h523::tim3::smcr::SMSPE;
///Field `SMSPE` reader - SMS preload enable
pub use crate::stm32h523::tim3::smcr::SMSPE_R;
///Field `SMSPE` writer - SMS preload enable
pub use crate::stm32h523::tim3::smcr::SMSPE_W;
///SMS preload source
pub use crate::stm32h523::tim3::smcr::SMSPS;
///Field `SMSPS` reader - SMS preload source
pub use crate::stm32h523::tim3::smcr::SMSPS_R;
///Field `SMSPS` writer - SMS preload source
pub use crate::stm32h523::tim3::smcr::SMSPS_W;
impl R {
    ///Bits 0:2 - SMS\[2:0\]: Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - SMS preload enable
    #[inline(always)]
    pub fn smspe(&self) -> SMSPE_R {
        SMSPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SMS preload source
    #[inline(always)]
    pub fn smsps(&self) -> SMSPS_R {
        SMSPS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms", &self.sms())
            .field("occs", &self.occs())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .field("sms_3", &self.sms_3())
            .field("ts2", &self.ts2())
            .field("smspe", &self.smspe())
            .field("smsps", &self.smsps())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS\[2:0\]: Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W<'_, SMCRrs> {
        OCCS_W::new(self, 3)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<'_, SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<'_, SMCRrs> {
        TS2_W::new(self, 20)
    }
    ///Bit 24 - SMS preload enable
    #[inline(always)]
    pub fn smspe(&mut self) -> SMSPE_W<'_, SMCRrs> {
        SMSPE_W::new(self, 24)
    }
    ///Bit 25 - SMS preload source
    #[inline(always)]
    pub fn smsps(&mut self) -> SMSPS_W<'_, SMCRrs> {
        SMSPS_W::new(self, 25)
    }
}
/**TIM4 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#TIM4:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {}
