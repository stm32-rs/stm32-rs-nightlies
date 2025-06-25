///Register `TIM_CCMR1_ALTERNATE1` reader
pub type R = crate::R<TIM_CCMR1_ALTERNATE1rs>;
///Register `TIM_CCMR1_ALTERNATE1` writer
pub type W = crate::W<TIM_CCMR1_ALTERNATE1rs>;
///Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OC1FE` reader - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
pub type OC1FE_R = crate::BitReader;
///Field `OC1FE` writer - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1PE` reader - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output).
pub type OC1PE_R = crate::BitReader;
///Field `OC1PE` writer - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output).
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M` reader - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
pub type OC1M_R = crate::FieldReader;
///Field `OC1M` writer - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC1CE` reader - Output Compare 1 clear enable
pub type OC1CE_R = crate::BitReader;
///Field `OC1CE` writer - Output Compare 1 clear enable
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M_1` reader - OC1M\[3\]
pub type OC1M_1_R = crate::BitReader;
///Field `OC1M_1` writer - OC1M\[3\]
pub type OC1M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output).
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output Compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&self) -> OC1M_1_R {
        OC1M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM_CCMR1_ALTERNATE1")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .field("oc1ce", &self.oc1ce())
            .field("oc1m_1", &self.oc1m_1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<TIM_CCMR1_ALTERNATE1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W<TIM_CCMR1_ALTERNATE1rs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output).
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W<TIM_CCMR1_ALTERNATE1rs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active level depends on CC1P and CC1NP bits. Others: Reserved Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). Note: In PWM mode 1 or 2, the tim_oc1ref level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W<TIM_CCMR1_ALTERNATE1rs> {
        OC1M_W::new(self, 4)
    }
    ///Bit 7 - Output Compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W<TIM_CCMR1_ALTERNATE1rs> {
        OC1CE_W::new(self, 7)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&mut self) -> OC1M_1_W<TIM_CCMR1_ALTERNATE1rs> {
        OC1M_1_W::new(self, 16)
    }
}
/**TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr1_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr1_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM16:TIM_CCMR1_ALTERNATE1)*/
pub struct TIM_CCMR1_ALTERNATE1rs;
impl crate::RegisterSpec for TIM_CCMR1_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`tim_ccmr1_alternate1::R`](R) reader structure
impl crate::Readable for TIM_CCMR1_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`tim_ccmr1_alternate1::W`](W) writer structure
impl crate::Writable for TIM_CCMR1_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM_CCMR1_ALTERNATE1 to value 0
impl crate::Resettable for TIM_CCMR1_ALTERNATE1rs {}
