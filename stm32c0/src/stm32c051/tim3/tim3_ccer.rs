///Register `TIM3_CCER` reader
pub type R = crate::R<TIM3_CCERrs>;
///Register `TIM3_CCER` writer
pub type W = crate::W<TIM3_CCERrs>;
/**Capture/Compare 1 output enable.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E {
    ///0: Capture mode disabled / OC1 is not active
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
///Field `CC1E` reader - Capture/Compare 1 output enable.
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
    ///Capture mode disabled / OC1 is not active
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
///Field `CC1E` writer - Capture/Compare 1 output enable.
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture mode disabled / OC1 is not active
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
/**Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used.

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
///Field `CC1P` reader - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used.
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
///Field `CC1P` writer - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used.
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
///Field `CC1NP` reader - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2E` reader - Capture/Compare 2 output enable. Refer to CC1E description
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - Capture/Compare 2 output enable. Refer to CC1E description
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - Capture/Compare 2 output Polarity. refer to CC1P description
pub type CC2P_R = crate::BitReader;
///Field `CC2P` writer - Capture/Compare 2 output Polarity. refer to CC1P description
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NP` reader - Capture/Compare 2 output Polarity. Refer to CC1NP description
pub type CC2NP_R = crate::BitReader;
///Field `CC2NP` writer - Capture/Compare 2 output Polarity. Refer to CC1NP description
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - Capture/Compare 3 output enable. Refer to CC1E description
pub type CC3E_R = crate::BitReader;
///Field `CC3E` writer - Capture/Compare 3 output enable. Refer to CC1E description
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - Capture/Compare 3 output Polarity. Refer to CC1P description
pub type CC3P_R = crate::BitReader;
///Field `CC3P` writer - Capture/Compare 3 output Polarity. Refer to CC1P description
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NP` reader - Capture/Compare 3 output Polarity. Refer to CC1NP description
pub type CC3NP_R = crate::BitReader;
///Field `CC3NP` writer - Capture/Compare 3 output Polarity. Refer to CC1NP description
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - Capture/Compare 4 output enable. refer to CC1E description
pub type CC4E_R = crate::BitReader;
///Field `CC4E` writer - Capture/Compare 4 output enable. refer to CC1E description
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - Capture/Compare 4 output Polarity. Refer to CC1P description
pub type CC4P_R = crate::BitReader;
///Field `CC4P` writer - Capture/Compare 4 output Polarity. Refer to CC1P description
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NP` reader - Capture/Compare 4 output Polarity. Refer to CC1NP description
pub type CC4NP_R = crate::BitReader;
///Field `CC4NP` writer - Capture/Compare 4 output Polarity. Refer to CC1NP description
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable.
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 output enable. refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1np", &self.cc1np())
            .field("cc2e", &self.cc2e())
            .field("cc2p", &self.cc2p())
            .field("cc2np", &self.cc2np())
            .field("cc3e", &self.cc3e())
            .field("cc3p", &self.cc3p())
            .field("cc3np", &self.cc3np())
            .field("cc4e", &self.cc4e())
            .field("cc4p", &self.cc4p())
            .field("cc4np", &self.cc4np())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable.
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, TIM3_CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: This configuration is reserved, it must not be used.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, TIM3_CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description.
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, TIM3_CCERrs> {
        CC1NP_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<'_, TIM3_CCERrs> {
        CC2E_W::new(self, 4)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<'_, TIM3_CCERrs> {
        CC2P_W::new(self, 5)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<'_, TIM3_CCERrs> {
        CC2NP_W::new(self, 7)
    }
    ///Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<'_, TIM3_CCERrs> {
        CC3E_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<'_, TIM3_CCERrs> {
        CC3P_W::new(self, 9)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<'_, TIM3_CCERrs> {
        CC3NP_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 output enable. refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<'_, TIM3_CCERrs> {
        CC4E_W::new(self, 12)
    }
    ///Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<'_, TIM3_CCERrs> {
        CC4P_W::new(self, 13)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W<'_, TIM3_CCERrs> {
        CC4NP_W::new(self, 15)
    }
}
/**TIM3 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim3_ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM3:TIM3_CCER)*/
pub struct TIM3_CCERrs;
impl crate::RegisterSpec for TIM3_CCERrs {
    type Ux = u16;
}
///`read()` method returns [`tim3_ccer::R`](R) reader structure
impl crate::Readable for TIM3_CCERrs {}
///`write(|w| ..)` method takes [`tim3_ccer::W`](W) writer structure
impl crate::Writable for TIM3_CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_CCER to value 0
impl crate::Resettable for TIM3_CCERrs {}
