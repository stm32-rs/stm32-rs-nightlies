///Register `CCMR1` reader
pub type R = crate::R<CCMR1rs>;
///Register `CCMR1` writer
pub type W = crate::W<CCMR1rs>;
///Field `CC1S` reader - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC1FE` reader - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode.
pub type OC1FE_R = crate::BitReader;
///Field `OC1FE` writer - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode.
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1PE` reader - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
pub type OC1PE_R = crate::BitReader;
///Field `OC1PE` writer - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M_2_0` reader - OC1M\[2:0\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits.
pub type OC1M_2_0_R = crate::FieldReader;
///Field `OC1M_2_0` writer - OC1M\[2:0\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits.
pub type OC1M_2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC1CE` reader - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal.
pub type OC1CE_R = crate::BitReader;
///Field `OC1CE` writer - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal.
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M_3` reader - OC1M\[3\]: Output Compare 1 mode (bit 3)
pub type OC1M_3_R = crate::BitReader;
///Field `OC1M_3` writer - OC1M\[3\]: Output Compare 1 mode (bit 3)
pub type OC1M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode.
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits.
    #[inline(always)]
    pub fn oc1m_2_0(&self) -> OC1M_2_0_R {
        OC1M_2_0_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal.
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - OC1M\[3\]: Output Compare 1 mode (bit 3)
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m_2_0", &self.oc1m_2_0())
            .field("oc1ce", &self.oc1ce())
            .field("oc1m_3", &self.oc1m_3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC1S: Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 1x: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<'_, CCMR1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - OC1FE: Output Compare 1 fast enable This bit is used to accelerate the effect of an event on the trigger in input on the CC output. 0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles. 1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently of the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OC1FE acts only if the channel is configured in PWM1 or PWM2 mode.
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W<'_, CCMR1rs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - OC1PE: Output Compare 1 preload enable 0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.. 1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event. Note: 1: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: 2: The PWM mode can be used without validating the preload register only in one pulse mode (OPM bit set in TIMx_CR1 register). Else the behavior is not guaranteed.
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W<'_, CCMR1rs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode (bits 2 to 0) These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits.
    #[inline(always)]
    pub fn oc1m_2_0(&mut self) -> OC1M_2_0_W<'_, CCMR1rs> {
        OC1M_2_0_W::new(self, 4)
    }
    ///Bit 7 - OC1CE: Output Compare 1 Clear Enable. Not used in Blue51. Not available in IUM 0: OC1REF is not affected by the ocref_clr_int signal. 1: OC1REF is cleared as soon as a high level is detected on the ocref_clr_int signal.
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W<'_, CCMR1rs> {
        OC1CE_W::new(self, 7)
    }
    ///Bit 16 - OC1M\[3\]: Output Compare 1 mode (bit 3)
    #[inline(always)]
    pub fn oc1m_3(&mut self) -> OC1M_3_W<'_, CCMR1rs> {
        OC1M_3_W::new(self, 16)
    }
}
/**CCMR1 register

You can [`read`](crate::Reg::read) this register and get [`ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM16:CCMR1)*/
pub struct CCMR1rs;
impl crate::RegisterSpec for CCMR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1::R`](R) reader structure
impl crate::Readable for CCMR1rs {}
///`write(|w| ..)` method takes [`ccmr1::W`](W) writer structure
impl crate::Writable for CCMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR1 to value 0
impl crate::Resettable for CCMR1rs {}
