///Register `TIM16_CCER` reader
pub type R = crate::R<TIM16_CCERrs>;
///Register `TIM16_CCER` writer
pub type W = crate::W<TIM16_CCERrs>;
/**Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 83 for details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E {
    ///0: Capture mode disabled / OC1 is not active (see below)
    B0x0 = 0,
    ///1: Capture mode enabled / OC1 signal is output on the corresponding output pin
    B0x1 = 1,
}
impl From<CC1E> for bool {
    #[inline(always)]
    fn from(variant: CC1E) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1E` reader - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 83 for details.
pub type CC1E_R = crate::BitReader<CC1E>;
impl CC1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1E {
        match self.bits {
            false => CC1E::B0x0,
            true => CC1E::B0x1,
        }
    }
    ///Capture mode disabled / OC1 is not active (see below)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1E::B0x0
    }
    ///Capture mode enabled / OC1 signal is output on the corresponding output pin
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1E::B0x1
    }
}
///Field `CC1E` writer - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 83 for details.
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture mode disabled / OC1 is not active (see below)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::B0x0)
    }
    ///Capture mode enabled / OC1 signal is output on the corresponding output pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E::B0x1)
    }
}
/**Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P {
    ///0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    B0x0 = 0,
    ///1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    B0x1 = 1,
}
impl From<CC1P> for bool {
    #[inline(always)]
    fn from(variant: CC1P) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1P` reader - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_R = crate::BitReader<CC1P>;
impl CC1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1P {
        match self.bits {
            false => CC1P::B0x0,
            true => CC1P::B0x1,
        }
    }
    ///OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1P::B0x0
    }
    ///OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1P::B0x1
    }
}
///Field `CC1P` writer - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG, CC1P>;
impl<'a, REG> CC1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::B0x0)
    }
    ///OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P::B0x1)
    }
}
/**Capture/Compare 1 complementary output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE {
    ///0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    B0x0 = 0,
    ///1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    B0x1 = 1,
}
impl From<CC1NE> for bool {
    #[inline(always)]
    fn from(variant: CC1NE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NE` reader - Capture/Compare 1 complementary output enable
pub type CC1NE_R = crate::BitReader<CC1NE>;
impl CC1NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NE {
        match self.bits {
            false => CC1NE::B0x0,
            true => CC1NE::B0x1,
        }
    }
    ///Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1NE::B0x0
    }
    ///On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1NE::B0x1
    }
}
///Field `CC1NE` writer - Capture/Compare 1 complementary output enable
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG, CC1NE>;
impl<'a, REG> CC1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::B0x0)
    }
    ///On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::B0x1)
    }
}
/**Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to the description of CC1P. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP {
    ///0: OC1N active high
    B0x0 = 0,
    ///1: OC1N active low
    B0x1 = 1,
}
impl From<CC1NP> for bool {
    #[inline(always)]
    fn from(variant: CC1NP) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NP` reader - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to the description of CC1P. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
pub type CC1NP_R = crate::BitReader<CC1NP>;
impl CC1NP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NP {
        match self.bits {
            false => CC1NP::B0x0,
            true => CC1NP::B0x1,
        }
    }
    ///OC1N active high
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1NP::B0x0
    }
    ///OC1N active low
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1NP::B0x1
    }
}
///Field `CC1NP` writer - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to the description of CC1P. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG, CC1NP>;
impl<'a, REG> CC1NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1N active high
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::B0x0)
    }
    ///OC1N active low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP::B0x1)
    }
}
impl R {
    ///Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 83 for details.
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to the description of CC1P. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 83 for details.
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, TIM16_CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, TIM16_CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<'_, TIM16_CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to the description of CC1P. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, TIM16_CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
/**TIM16 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim16_ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM16:TIM16_CCER)*/
pub struct TIM16_CCERrs;
impl crate::RegisterSpec for TIM16_CCERrs {
    type Ux = u16;
}
///`read()` method returns [`tim16_ccer::R`](R) reader structure
impl crate::Readable for TIM16_CCERrs {}
///`write(|w| ..)` method takes [`tim16_ccer::W`](W) writer structure
impl crate::Writable for TIM16_CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM16_CCER to value 0
impl crate::Resettable for TIM16_CCERrs {}
