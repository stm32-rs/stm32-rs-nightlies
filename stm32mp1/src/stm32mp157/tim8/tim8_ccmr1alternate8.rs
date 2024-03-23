#[doc = "Register `TIM8_CCMR1ALTERNATE8` reader"]
pub type R = crate::R<TIM8_CCMR1ALTERNATE8rs>;
#[doc = "Register `TIM8_CCMR1ALTERNATE8` writer"]
pub type W = crate::W<TIM8_CCMR1ALTERNATE8rs>;
#[doc = "Field `CC1S` reader - CC1S"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - CC1S"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - IC1PSC"]
pub type IC1PSC_R = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - IC1PSC"]
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - IC1F"]
pub type IC1F_R = crate::FieldReader;
#[doc = "Field `IC1F` writer - IC1F"]
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC2S` reader - CC2S"]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - CC2S"]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2PSC` reader - IC2PSC"]
pub type IC2PSC_R = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - IC2PSC"]
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - IC2F"]
pub type IC2F_R = crate::FieldReader;
#[doc = "Field `IC2F` writer - IC2F"]
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IC1PSC"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - IC1F"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IC2PSC"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - IC2F"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<TIM8_CCMR1ALTERNATE8rs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - IC1PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<TIM8_CCMR1ALTERNATE8rs> {
        IC1PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - IC1F"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<TIM8_CCMR1ALTERNATE8rs> {
        IC1F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<TIM8_CCMR1ALTERNATE8rs> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - IC2PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<TIM8_CCMR1ALTERNATE8rs> {
        IC2PSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - IC2F"]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<TIM8_CCMR1ALTERNATE8rs> {
        IC2F_W::new(self, 12)
    }
}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim8_ccmr1alternate8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim8_ccmr1alternate8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM8_CCMR1ALTERNATE8rs;
impl crate::RegisterSpec for TIM8_CCMR1ALTERNATE8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim8_ccmr1alternate8::R`](R) reader structure"]
impl crate::Readable for TIM8_CCMR1ALTERNATE8rs {}
#[doc = "`write(|w| ..)` method takes [`tim8_ccmr1alternate8::W`](W) writer structure"]
impl crate::Writable for TIM8_CCMR1ALTERNATE8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM8_CCMR1ALTERNATE8 to value 0"]
impl crate::Resettable for TIM8_CCMR1ALTERNATE8rs {
    const RESET_VALUE: u32 = 0;
}
