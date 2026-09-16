///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS_2_0` reader - SMS\[3:0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description.
pub type SMS_2_0_R = crate::FieldReader;
///Field `SMS_2_0` writer - SMS\[3:0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description.
pub type SMS_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TS_2_0` reader - TS\[4:0\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer.
pub type TS_2_0_R = crate::FieldReader;
///Field `TS_2_0` writer - TS\[4:0\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer.
pub type TS_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSM` reader - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
pub type MSM_R = crate::BitReader;
///Field `MSM` writer - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMS_3` reader - SMS\[3:0\]: Slave mode selection. See SMS_LSB description
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - SMS\[3:0\]: Slave mode selection. See SMS_LSB description
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_4_3` reader - TS\[4:0\]: Trigger selection. See TS_LSB description
pub type TS_4_3_R = crate::FieldReader;
///Field `TS_4_3` writer - TS\[4:0\]: Trigger selection. See TS_LSB description
pub type TS_4_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - SMS\[3:0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description.
    #[inline(always)]
    pub fn sms_2_0(&self) -> SMS_2_0_R {
        SMS_2_0_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TS\[4:0\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer.
    #[inline(always)]
    pub fn ts_2_0(&self) -> TS_2_0_R {
        TS_2_0_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - SMS\[3:0\]: Slave mode selection. See SMS_LSB description
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - TS\[4:0\]: Trigger selection. See TS_LSB description
    #[inline(always)]
    pub fn ts_4_3(&self) -> TS_4_3_R {
        TS_4_3_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms_2_0", &self.sms_2_0())
            .field("ts_2_0", &self.ts_2_0())
            .field("msm", &self.msm())
            .field("sms_3", &self.sms_3())
            .field("ts_4_3", &self.ts_4_3())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS\[3:0\]: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description.
    #[inline(always)]
    pub fn sms_2_0(&mut self) -> SMS_2_0_W<'_, SMCRrs> {
        SMS_2_0_W::new(self, 0)
    }
    ///Bits 4:6 - TS\[4:0\]: Trigger selection This bitfield selects the trigger input to be used to synchronize the counter. 00000: Internal Trigger 0 (ITR0) 00001: Internal Trigger 1 (ITR1) 00010: Internal Trigger 2 (ITR2) 00011: Internal Trigger 3 (ITR3) 00100: TI1 Edge Detector (TI1F_ED) 00101: Filtered Timer Input 1 (TI1FP1) Other codes: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition. See Table 79 in IUM: TIM16 register map and reset values on page 469 for more details on ITRx meaning for each Timer.
    #[inline(always)]
    pub fn ts_2_0(&mut self) -> TS_2_0_W<'_, SMCRrs> {
        TS_2_0_W::new(self, 4)
    }
    ///Bit 7 - MSM: Master/slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bit 16 - SMS\[3:0\]: Slave mode selection. See SMS_LSB description
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<'_, SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - TS\[4:0\]: Trigger selection. See TS_LSB description
    #[inline(always)]
    pub fn ts_4_3(&mut self) -> TS_4_3_W<'_, SMCRrs> {
        TS_4_3_W::new(self, 20)
    }
}
/**SMCR register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM16:SMCR)*/
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
