///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
///Field `CC1E` reader - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NE` reader - CC1NE: Capture/Compare 1 complementary output enable 0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. 1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
pub type CC1NE_R = crate::BitReader;
///Field `CC1NE` writer - CC1NE: Capture/Compare 1 complementary output enable 0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. 1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - CC1NP: Capture/Compare 1 complementary output polarity CC1 channel configured as output: 0: OC1N active high 1: OC1N active low CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1. Refer to the description of CC1P. Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - CC1NP: Capture/Compare 1 complementary output polarity CC1 channel configured as output: 0: OC1N active high 1: OC1N active low CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1. Refer to the description of CC1P. Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC1NE: Capture/Compare 1 complementary output enable 0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. 1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC1NP: Capture/Compare 1 complementary output polarity CC1 channel configured as output: 0: OC1N active high 1: OC1N active low CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1. Refer to the description of CC1P. Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
impl W {
    ///Bit 0 - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - CC1NE: Capture/Compare 1 complementary output enable 0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. 1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<'_, CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - CC1NP: Capture/Compare 1 complementary output polarity CC1 channel configured as output: 0: OC1N active high 1: OC1N active low CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1. Refer to the description of CC1P. Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (the channel is configured in output). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
/**CCER register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM16:CCER)*/
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u32;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {}
