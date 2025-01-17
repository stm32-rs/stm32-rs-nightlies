///Register `TIM1_CCMR2_INPUT` reader
pub type R = crate::R<TIM1_CCMR2_INPUTrs>;
///Register `TIM1_CCMR2_INPUT` writer
pub type W = crate::W<TIM1_CCMR2_INPUTrs>;
///Field `CC3S` reader - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
pub type CC3S_R = crate::FieldReader;
///Field `CC3S` writer - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `IC3PSC` reader - Input capture 3 prescaler Refer to IC1PSC\[1:0\]
description.*/
pub type IC3PSC_R = crate::FieldReader;
/**Field `IC3PSC` writer - Input capture 3 prescaler Refer to IC1PSC\[1:0\]
description.*/
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `IC3F` reader - Input capture 3 filter Refer to IC1F\[3:0\]
description.*/
pub type IC3F_R = crate::FieldReader;
/**Field `IC3F` writer - Input capture 3 filter Refer to IC1F\[3:0\]
description.*/
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
pub type CC4S_R = crate::FieldReader;
///Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `IC4PSC` reader - Input capture 4 prescaler Refer to IC1PSC\[1:0\]
description.*/
pub type IC4PSC_R = crate::FieldReader;
/**Field `IC4PSC` writer - Input capture 4 prescaler Refer to IC1PSC\[1:0\]
description.*/
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `IC4F` reader - Input capture 4 filter Refer to IC1F\[3:0\]
description.*/
pub type IC4F_R = crate::FieldReader;
/**Field `IC4F` writer - Input capture 4 filter Refer to IC1F\[3:0\]
description.*/
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    /**Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\[1:0\]
    description.*/
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    /**Bits 4:7 - Input capture 3 filter Refer to IC1F\[3:0\]
    description.*/
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    /**Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\[1:0\]
    description.*/
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    /**Bits 12:15 - Input capture 4 filter Refer to IC1F\[3:0\]
    description.*/
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCMR2_INPUT")
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
    ///Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<TIM1_CCMR2_INPUTrs> {
        CC3S_W::new(self, 0)
    }
    /**Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\[1:0\]
    description.*/
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<TIM1_CCMR2_INPUTrs> {
        IC3PSC_W::new(self, 2)
    }
    /**Bits 4:7 - Input capture 3 filter Refer to IC1F\[3:0\]
    description.*/
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<TIM1_CCMR2_INPUTrs> {
        IC3F_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<TIM1_CCMR2_INPUTrs> {
        CC4S_W::new(self, 8)
    }
    /**Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\[1:0\]
    description.*/
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<TIM1_CCMR2_INPUTrs> {
        IC4PSC_W::new(self, 10)
    }
    /**Bits 12:15 - Input capture 4 filter Refer to IC1F\[3:0\]
    description.*/
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<TIM1_CCMR2_INPUTrs> {
        IC4F_W::new(self, 12)
    }
}
/**TIM1 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM1:TIM1_CCMR2_INPUT)*/
pub struct TIM1_CCMR2_INPUTrs;
impl crate::RegisterSpec for TIM1_CCMR2_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`tim1_ccmr2_input::R`](R) reader structure
impl crate::Readable for TIM1_CCMR2_INPUTrs {}
///`write(|w| ..)` method takes [`tim1_ccmr2_input::W`](W) writer structure
impl crate::Writable for TIM1_CCMR2_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM1_CCMR2_INPUT to value 0
impl crate::Resettable for TIM1_CCMR2_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
