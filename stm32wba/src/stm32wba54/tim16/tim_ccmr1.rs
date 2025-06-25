///Register `TIM_CCMR1` reader
pub type R = crate::R<TIM_CCMR1rs>;
///Register `TIM_CCMR1` writer
pub type W = crate::W<TIM_CCMR1rs>;
///Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
pub type CC1S_R = crate::FieldReader;
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register).
pub type IC1PSC_R = crate::FieldReader;
///Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register).
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_R = crate::FieldReader;
///Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM_CCMR1")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0' in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<TIM_CCMR1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E='0' (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<TIM_CCMR1rs> {
        IC1PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<TIM_CCMR1rs> {
        IC1F_W::new(self, 4)
    }
}
/**TIM capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`tim_ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TIM16:TIM_CCMR1)*/
pub struct TIM_CCMR1rs;
impl crate::RegisterSpec for TIM_CCMR1rs {
    type Ux = u32;
}
///`read()` method returns [`tim_ccmr1::R`](R) reader structure
impl crate::Readable for TIM_CCMR1rs {}
///`write(|w| ..)` method takes [`tim_ccmr1::W`](W) writer structure
impl crate::Writable for TIM_CCMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM_CCMR1 to value 0
impl crate::Resettable for TIM_CCMR1rs {}
