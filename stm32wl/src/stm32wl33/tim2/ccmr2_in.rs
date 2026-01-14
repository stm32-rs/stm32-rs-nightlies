///Register `CCMR2_in` reader
pub type R = crate::R<CCMR2_INrs>;
///Register `CCMR2_in` writer
pub type W = crate::W<CCMR2_INrs>;
///Field `CC3S` reader - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3PSC` reader - IC3PSC: Input capture 3 prescaler
pub type IC3PSC_R = crate::FieldReader;
///Field `IC3PSC` writer - IC3PSC: Input capture 3 prescaler
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3F` reader - IC3F: Input capture 3 filter
pub type IC3F_R = crate::FieldReader;
///Field `IC3F` writer - IC3F: Input capture 3 filter
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CC4S` reader - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4PSC` reader - IC4PSC: Input capture 4 prescaler
pub type IC4PSC_R = crate::FieldReader;
///Field `IC4PSC` writer - IC4PSC: Input capture 4 prescaler
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4F` reader - IC4F: Input capture 4 filter
pub type IC4F_R = crate::FieldReader;
///Field `IC4F` writer - IC4F: Input capture 4 filter
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - IC3PSC: Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - IC3F: Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - IC4PSC: Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - IC4F: Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_in")
            .field("cc3s", &self.cc3s())
            .field("ic3psc", &self.ic3psc())
            .field("ic3f", &self.ic3f())
            .field("cc4s", &self.cc4s())
            .field("ic4psc", &self.ic4psc())
            .field("ic4f", &self.ic4f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CC3S: Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC3S bits are writable only when the channel is OFF (CC3E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, CCMR2_INrs> {
        CC3S_W::new(self, 0)
    }
    ///Bits 2:3 - IC3PSC: Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<'_, CCMR2_INrs> {
        IC3PSC_W::new(self, 2)
    }
    ///Bits 4:7 - IC3F: Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<'_, CCMR2_INrs> {
        IC3F_W::new(self, 4)
    }
    ///Bits 8:9 - CC4S: Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC4 channel is configured as output 01: CC4 channel is configured as input, IC4 is mapped on TI4 10: CC4 channel is configured as input, IC4 is mapped on TI3 11: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register) Note: CC4S bits are writable only when the channel is OFF (CC4E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, CCMR2_INrs> {
        CC4S_W::new(self, 8)
    }
    ///Bits 10:11 - IC4PSC: Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<'_, CCMR2_INrs> {
        IC4PSC_W::new(self, 10)
    }
    ///Bits 12:15 - IC4F: Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<'_, CCMR2_INrs> {
        IC4F_W::new(self, 12)
    }
}
/**CCMR2_in register

You can [`read`](crate::Reg::read) this register and get [`ccmr2_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:CCMR2_in)*/
pub struct CCMR2_INrs;
impl crate::RegisterSpec for CCMR2_INrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_in::R`](R) reader structure
impl crate::Readable for CCMR2_INrs {}
///`write(|w| ..)` method takes [`ccmr2_in::W`](W) writer structure
impl crate::Writable for CCMR2_INrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2_in to value 0
impl crate::Resettable for CCMR2_INrs {}
