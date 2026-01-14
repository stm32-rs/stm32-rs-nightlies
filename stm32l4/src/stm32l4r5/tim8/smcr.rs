///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub use crate::stm32l4r5::tim1::smcr::SMS;
///Field `SMS` reader - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub use crate::stm32l4r5::tim1::smcr::SMS_R;
///Field `SMS` writer - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
pub use crate::stm32l4r5::tim1::smcr::SMS_W;
/**TS\[0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 167: TIMxTIM1 internal trigger connection on page 777 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS {
    ///0: Internal Trigger 0 (ITR0)
    Itr0 = 0,
    ///1: Internal Trigger 1 (ITR1)
    Itr1 = 1,
    ///2: Internal Trigger 2 (ITR2)
    Itr2 = 2,
    ///4: TI1 Edge Detector (TI1F_ED)
    Ti1fEd = 4,
    ///5: Filtered Timer Input 1 (TI1FP1)
    Ti1fp1 = 5,
    ///6: Filtered Timer Input 2 (TI2FP2)
    Ti2fp2 = 6,
    ///7: External Trigger input (ETRF)
    Etrf = 7,
}
impl From<TS> for u8 {
    #[inline(always)]
    fn from(variant: TS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS {
    type Ux = u8;
}
impl crate::IsEnum for TS {}
///Field `TS` reader - TS\[0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 167: TIMxTIM1 internal trigger connection on page 777 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS_R = crate::FieldReader<TS>;
impl TS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TS> {
        match self.bits {
            0 => Some(TS::Itr0),
            1 => Some(TS::Itr1),
            2 => Some(TS::Itr2),
            4 => Some(TS::Ti1fEd),
            5 => Some(TS::Ti1fp1),
            6 => Some(TS::Ti2fp2),
            7 => Some(TS::Etrf),
            _ => None,
        }
    }
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == TS::Itr0
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == TS::Itr1
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == TS::Itr2
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TS::Ti1fEd
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TS::Ti1fp1
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TS::Ti2fp2
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == TS::Etrf
    }
}
///Field `TS` writer - TS\[0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 167: TIMxTIM1 internal trigger connection on page 777 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TS>;
impl<'a, REG> TS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn itr0(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr0)
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn itr1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr1)
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn itr2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr2)
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti1fEd)
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti1fp1)
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti2fp2)
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn etrf(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Etrf)
    }
}
///External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
pub use crate::stm32l4r5::tim1::smcr::ECE;
///Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
pub use crate::stm32l4r5::tim1::smcr::ECE_R;
///Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
pub use crate::stm32l4r5::tim1::smcr::ECE_W;
///External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub use crate::stm32l4r5::tim1::smcr::ETF;
///Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub use crate::stm32l4r5::tim1::smcr::ETF_R;
///Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub use crate::stm32l4r5::tim1::smcr::ETF_W;
///External trigger polarity This bit selects whether ETR or ETR is used for trigger operations
pub use crate::stm32l4r5::tim1::smcr::ETP;
///External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of f<sub>CK_INT</sub> frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.
pub use crate::stm32l4r5::tim1::smcr::ETPS;
///Field `ETPS` reader - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of f<sub>CK_INT</sub> frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.
pub use crate::stm32l4r5::tim1::smcr::ETPS_R;
///Field `ETPS` writer - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of f<sub>CK_INT</sub> frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.
pub use crate::stm32l4r5::tim1::smcr::ETPS_W;
///Field `ETP` reader - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations
pub use crate::stm32l4r5::tim1::smcr::ETP_R;
///Field `ETP` writer - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations
pub use crate::stm32l4r5::tim1::smcr::ETP_W;
///Master/slave mode
pub use crate::stm32l4r5::tim1::smcr::MSM;
///Field `MSM` reader - Master/slave mode
pub use crate::stm32l4r5::tim1::smcr::MSM_R;
///Field `MSM` writer - Master/slave mode
pub use crate::stm32l4r5::tim1::smcr::MSM_W;
///Field `SMS_3` reader - SMS\[3\]
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - SMS\[3\]
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS2` reader - TS\[4:3\]
pub type TS2_R = crate::FieldReader;
///Field `TS2` writer - TS\[4:3\]
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TS\[0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 167: TIMxTIM1 internal trigger connection on page 777 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of f<sub>CK_INT</sub> frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms", &self.sms())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .field("sms_3", &self.sms_3())
            .field("ts2", &self.ts2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS\[0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - TS\[0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See Table 167: TIMxTIM1 internal trigger connection on page 777 for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of f<sub>CK_INT</sub> frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). Note: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<'_, SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<'_, SMCRrs> {
        TS2_W::new(self, 20)
    }
}
/**TIM8 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#TIM8:SMCR)*/
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
