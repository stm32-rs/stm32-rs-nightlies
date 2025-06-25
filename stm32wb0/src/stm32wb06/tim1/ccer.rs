///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
///Field `CC1E` reader - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NE` reader -
pub type CC1NE_R = crate::BitReader;
///Field `CC1NE` writer -
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output).
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output).
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2E` reader - CC2E: Capture/Compare 2 output enable refer to CC1E description
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - CC2E: Capture/Compare 2 output enable refer to CC1E description
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - CC2P: Capture/Compare 2 output polarity refer to CC1P description
pub type CC2P_R = crate::BitReader;
///Field `CC2P` writer - CC2P: Capture/Compare 2 output polarity refer to CC1P description
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NE` reader - Capture/compare 2 complementary output enable. Refer to CC1NE description.
pub type CC2NE_R = crate::BitReader;
///Field `CC2NE` writer - Capture/compare 2 complementary output enable. Refer to CC1NE description.
pub type CC2NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NP` reader - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
pub type CC2NP_R = crate::BitReader;
///Field `CC2NP` writer - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - CC3E: Capture/Compare 3 output enable refer to CC1E description
pub type CC3E_R = crate::BitReader;
///Field `CC3E` writer - CC3E: Capture/Compare 3 output enable refer to CC1E description
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - CC3P: Capture/Compare 3 output polarity refer to CC1P description
pub type CC3P_R = crate::BitReader;
///Field `CC3P` writer - CC3P: Capture/Compare 3 output polarity refer to CC1P description
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NE` reader - Capture/compare 3 complementary output enable. Refer to CC1NE description.
pub type CC3NE_R = crate::BitReader;
///Field `CC3NE` writer - Capture/compare 3 complementary output enable. Refer to CC1NE description.
pub type CC3NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NP` reader - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
pub type CC3NP_R = crate::BitReader;
///Field `CC3NP` writer - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - CC4E: Capture/Compare 4 output enable refer to CC1E description
pub type CC4E_R = crate::BitReader;
///Field `CC4E` writer - CC4E: Capture/Compare 4 output enable refer to CC1E description
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - CC4P: Capture/Compare 4 output polarity refer to CC1P description
pub type CC4P_R = crate::BitReader;
///Field `CC4P` writer - CC4P: Capture/Compare 4 output polarity refer to CC1P description
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NE` reader - Capture/compare 4 complementary output enable. Refer to CC1NE description.
pub type CC4NE_R = crate::BitReader;
///Field `CC4NE` writer - Capture/compare 4 complementary output enable. Refer to CC1NE description.
pub type CC4NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NP` reader - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
pub type CC4NP_R = crate::BitReader;
///Field `CC4NP` writer - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5E` reader - Capture/compare 5 output enable. Refer to CC1E description.
pub type CC5E_R = crate::BitReader;
///Field `CC5E` writer - Capture/compare 5 output enable. Refer to CC1E description.
pub type CC5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5P` reader - Capture/compare 5 output polarity. Refer to CC1P description.
pub type CC5P_R = crate::BitReader;
///Field `CC5P` writer - Capture/compare 5 output polarity. Refer to CC1P description.
pub type CC5P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6E` reader - Capture/compare 6 output enable. Refer to CC1E description.
pub type CC6E_R = crate::BitReader;
///Field `CC6E` writer - Capture/compare 6 output enable. Refer to CC1E description.
pub type CC6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6P` reader - Capture/compare 6 output polarity. Refer to CC1P description.
pub type CC6P_R = crate::BitReader;
///Field `CC6P` writer - Capture/compare 6 output polarity. Refer to CC1P description.
pub type CC6P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output).
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CC2E: Capture/Compare 2 output enable refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CC2P: Capture/Compare 2 output polarity refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Capture/compare 2 complementary output enable. Refer to CC1NE description.
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CC3E: Capture/Compare 3 output enable refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CC3P: Capture/Compare 3 output polarity refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 3 complementary output enable. Refer to CC1NE description.
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CC4E: Capture/Compare 4 output enable refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CC4P: Capture/Compare 4 output polarity refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Capture/compare 4 complementary output enable. Refer to CC1NE description.
    #[inline(always)]
    pub fn cc4ne(&self) -> CC4NE_R {
        CC4NE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Capture/compare 5 output enable. Refer to CC1E description.
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Capture/compare 5 output polarity. Refer to CC1P description.
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Capture/compare 6 output enable. Refer to CC1E description.
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Capture/compare 6 output polarity. Refer to CC1P description.
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
    ///Bit 0 - CC1E: Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (TIMx_CCR1) or not. 0: Capture disabled 1: Capture enabled
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - CC1P: Capture/Compare 1 output polarity CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: The CC1NP/CC1P bits select the polarity of TI1FP1 for trigger or capture operations.. 00: Non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). 01: Inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode. 10: Reserved, do not use this configuration. 11: Non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode). Note: 1. This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). 2. On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - CC1NP: Capture/Compare 1 Complementary output Polarity. This field is not used in Blue51. Not available in IUM Note: This bit is no longer writeable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in GPT_BDTR register) and CC1S='00' (the channel is configured in output).
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<CCERrs> {
        CC1NP_W::new(self, 3)
    }
    ///Bit 4 - CC2E: Capture/Compare 2 output enable refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<CCERrs> {
        CC2E_W::new(self, 4)
    }
    ///Bit 5 - CC2P: Capture/Compare 2 output polarity refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<CCERrs> {
        CC2P_W::new(self, 5)
    }
    ///Bit 6 - Capture/compare 2 complementary output enable. Refer to CC1NE description.
    #[inline(always)]
    pub fn cc2ne(&mut self) -> CC2NE_W<CCERrs> {
        CC2NE_W::new(self, 6)
    }
    ///Bit 7 - CC2NP: Capture/Compare 2 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<CCERrs> {
        CC2NP_W::new(self, 7)
    }
    ///Bit 8 - CC3E: Capture/Compare 3 output enable refer to CC1E description
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<CCERrs> {
        CC3E_W::new(self, 8)
    }
    ///Bit 9 - CC3P: Capture/Compare 3 output polarity refer to CC1P description
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<CCERrs> {
        CC3P_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 3 complementary output enable. Refer to CC1NE description.
    #[inline(always)]
    pub fn cc3ne(&mut self) -> CC3NE_W<CCERrs> {
        CC3NE_W::new(self, 10)
    }
    ///Bit 11 - CC3NP: Capture/Compare 3 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<CCERrs> {
        CC3NP_W::new(self, 11)
    }
    ///Bit 12 - CC4E: Capture/Compare 4 output enable refer to CC1E description
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<CCERrs> {
        CC4E_W::new(self, 12)
    }
    ///Bit 13 - CC4P: Capture/Compare 4 output polarity refer to CC1P description
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<CCERrs> {
        CC4P_W::new(self, 13)
    }
    ///Bit 14 - Capture/compare 4 complementary output enable. Refer to CC1NE description.
    #[inline(always)]
    pub fn cc4ne(&mut self) -> CC4NE_W<CCERrs> {
        CC4NE_W::new(self, 14)
    }
    ///Bit 15 - CC4NP: Capture/Compare 4 Complementary output Polarity. This field is not used in Blue51. Not available in IUM refer to CC1NP description
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W<CCERrs> {
        CC4NP_W::new(self, 15)
    }
    ///Bit 16 - Capture/compare 5 output enable. Refer to CC1E description.
    #[inline(always)]
    pub fn cc5e(&mut self) -> CC5E_W<CCERrs> {
        CC5E_W::new(self, 16)
    }
    ///Bit 17 - Capture/compare 5 output polarity. Refer to CC1P description.
    #[inline(always)]
    pub fn cc5p(&mut self) -> CC5P_W<CCERrs> {
        CC5P_W::new(self, 17)
    }
    ///Bit 20 - Capture/compare 6 output enable. Refer to CC1E description.
    #[inline(always)]
    pub fn cc6e(&mut self) -> CC6E_W<CCERrs> {
        CC6E_W::new(self, 20)
    }
    ///Bit 21 - Capture/compare 6 output polarity. Refer to CC1P description.
    #[inline(always)]
    pub fn cc6p(&mut self) -> CC6P_W<CCERrs> {
        CC6P_W::new(self, 21)
    }
}
/**CCER register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#TIM1:CCER)*/
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
