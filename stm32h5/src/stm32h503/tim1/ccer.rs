///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
///Field `CC1E` reader - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE {
    ///0: Complementary output disabled
    Disabled = 0,
    ///1: Complementary output enabled
    Enabled = 1,
}
impl From<CC1NE> for bool {
    #[inline(always)]
    fn from(variant: CC1NE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NE` reader - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1NE_R = crate::BitReader<CC1NE>;
impl CC1NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1NE {
        match self.bits {
            false => CC1NE::Disabled,
            true => CC1NE::Enabled,
        }
    }
    ///Complementary output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1NE::Disabled
    }
    ///Complementary output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1NE::Enabled
    }
}
///Field `CC1NE` writer - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG, CC1NE>;
impl<'a, REG> CC1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Complementary output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::Disabled)
    }
    ///Complementary output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE::Enabled)
    }
}
///Field `CC1NP` reader - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2E` reader - Capture/compare 2 output enable Refer to CC1E description
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - Capture/compare 2 output enable Refer to CC1E description
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - Capture/compare 2 output polarity Refer to CC1P description
pub type CC2P_R = crate::BitReader;
///Field `CC2P` writer - Capture/compare 2 output polarity Refer to CC1P description
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NE` reader - Capture/compare 2 complementary output enable Refer to CC1NE description
pub use CC1NE_R as CC2NE_R;
///Field `CC2NE` writer - Capture/compare 2 complementary output enable Refer to CC1NE description
pub use CC1NE_W as CC2NE_W;
///Field `CC2NP` reader - Capture/compare 2 complementary output polarity Refer to CC1NP description
pub type CC2NP_R = crate::BitReader;
///Field `CC2NP` writer - Capture/compare 2 complementary output polarity Refer to CC1NP description
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - Capture/compare 3 output enable Refer to CC1E description
pub type CC3E_R = crate::BitReader;
///Field `CC3E` writer - Capture/compare 3 output enable Refer to CC1E description
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - Capture/compare 3 output polarity Refer to CC1P description
pub type CC3P_R = crate::BitReader;
///Field `CC3P` writer - Capture/compare 3 output polarity Refer to CC1P description
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NE` reader - Capture/compare 3 complementary output enable Refer to CC1NE description
pub use CC1NE_R as CC3NE_R;
///Field `CC3NE` writer - Capture/compare 3 complementary output enable Refer to CC1NE description
pub use CC1NE_W as CC3NE_W;
///Field `CC3NP` reader - Capture/compare 3 complementary output polarity Refer to CC1NP description
pub type CC3NP_R = crate::BitReader;
///Field `CC3NP` writer - Capture/compare 3 complementary output polarity Refer to CC1NP description
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - Capture/compare 4 output enable Refer to CC1E description
pub type CC4E_R = crate::BitReader;
///Field `CC4E` writer - Capture/compare 4 output enable Refer to CC1E description
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - Capture/compare 4 output polarity Refer to CC1P description
pub type CC4P_R = crate::BitReader;
///Field `CC4P` writer - Capture/compare 4 output polarity Refer to CC1P description
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NE` reader - Capture/compare 4 complementary output enable Refer to CC1NE description
pub use CC1NE_R as CC4NE_R;
///Field `CC4NE` writer - Capture/compare 4 complementary output enable Refer to CC1NE description
pub use CC1NE_W as CC4NE_W;
///Field `CC4NP` reader - Capture/compare 4 complementary output polarity Refer to CC1NP description
pub type CC4NP_R = crate::BitReader;
///Field `CC4NP` writer - Capture/compare 4 complementary output polarity Refer to CC1NP description
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5E` reader - Capture/compare 5 output enable Refer to CC1E description
pub type CC5E_R = crate::BitReader;
///Field `CC5E` writer - Capture/compare 5 output enable Refer to CC1E description
pub type CC5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5P` reader - Capture/compare 5 output polarity Refer to CC1P description
pub type CC5P_R = crate::BitReader;
///Field `CC5P` writer - Capture/compare 5 output polarity Refer to CC1P description
pub type CC5P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6E` reader - Capture/compare 6 output enable Refer to CC1E description
pub type CC6E_R = crate::BitReader;
///Field `CC6E` writer - Capture/compare 6 output enable Refer to CC1E description
pub type CC6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6P` reader - Capture/compare 6 output polarity Refer to CC1P description
pub type CC6P_R = crate::BitReader;
///Field `CC6P` writer - Capture/compare 6 output polarity Refer to CC1P description
pub type CC6P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/compare 2 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/compare 2 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Capture/compare 2 complementary output enable Refer to CC1NE description
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Capture/compare 2 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture/compare 3 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/compare 3 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 3 complementary output enable Refer to CC1NE description
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/compare 3 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/compare 4 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/compare 4 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Capture/compare 4 complementary output enable Refer to CC1NE description
    #[inline(always)]
    pub fn cc4ne(&self) -> CC4NE_R {
        CC4NE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Capture/compare 4 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Capture/compare 5 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Capture/compare 5 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Capture/compare 6 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Capture/compare 6 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .field("cc2e", &self.cc2e())
            .field("cc2p", &self.cc2p())
            .field("cc2ne", &self.cc2ne())
            .field("cc2np", &self.cc2np())
            .field("cc3e", &self.cc3e())
            .field("cc3p", &self.cc3p())
            .field("cc3ne", &self.cc3ne())
            .field("cc3np", &self.cc3np())
            .field("cc4e", &self.cc4e())
            .field("cc4p", &self.cc4p())
            .field("cc4ne", &self.cc4ne())
            .field("cc4np", &self.cc4np())
            .field("cc5e", &self.cc5e())
            .field("cc5p", &self.cc5p())
            .field("cc6e", &self.cc6e())
            .field("cc6p", &self.cc6p())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<CCERrs> {
        CC1NP_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 2 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<CCERrs> {
        CC2E_W::new(self, 4)
    }
    ///Bit 5 - Capture/compare 2 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<CCERrs> {
        CC2P_W::new(self, 5)
    }
    ///Bit 6 - Capture/compare 2 complementary output enable Refer to CC1NE description
    #[inline(always)]
    pub fn cc2ne(&mut self) -> CC2NE_W<CCERrs> {
        CC2NE_W::new(self, 6)
    }
    ///Bit 7 - Capture/compare 2 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<CCERrs> {
        CC2NP_W::new(self, 7)
    }
    ///Bit 8 - Capture/compare 3 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<CCERrs> {
        CC3E_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 3 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<CCERrs> {
        CC3P_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 3 complementary output enable Refer to CC1NE description
    #[inline(always)]
    pub fn cc3ne(&mut self) -> CC3NE_W<CCERrs> {
        CC3NE_W::new(self, 10)
    }
    ///Bit 11 - Capture/compare 3 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<CCERrs> {
        CC3NP_W::new(self, 11)
    }
    ///Bit 12 - Capture/compare 4 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<CCERrs> {
        CC4E_W::new(self, 12)
    }
    ///Bit 13 - Capture/compare 4 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<CCERrs> {
        CC4P_W::new(self, 13)
    }
    ///Bit 14 - Capture/compare 4 complementary output enable Refer to CC1NE description
    #[inline(always)]
    pub fn cc4ne(&mut self) -> CC4NE_W<CCERrs> {
        CC4NE_W::new(self, 14)
    }
    ///Bit 15 - Capture/compare 4 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W<CCERrs> {
        CC4NP_W::new(self, 15)
    }
    ///Bit 16 - Capture/compare 5 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc5e(&mut self) -> CC5E_W<CCERrs> {
        CC5E_W::new(self, 16)
    }
    ///Bit 17 - Capture/compare 5 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc5p(&mut self) -> CC5P_W<CCERrs> {
        CC5P_W::new(self, 17)
    }
    ///Bit 20 - Capture/compare 6 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc6e(&mut self) -> CC6E_W<CCERrs> {
        CC6E_W::new(self, 20)
    }
    ///Bit 21 - Capture/compare 6 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc6p(&mut self) -> CC6P_W<CCERrs> {
        CC6P_W::new(self, 21)
    }
}
/**TIM1 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#TIM1:CCER)*/
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u32;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {
    const RESET_VALUE: u32 = 0;
}
