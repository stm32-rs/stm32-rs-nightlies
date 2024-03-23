#[doc = "Register `TIM13_CCMR1` reader"]
pub type R = crate::R<TIM13_CCMR1rs>;
#[doc = "Register `TIM13_CCMR1` writer"]
pub type W = crate::W<TIM13_CCMR1rs>;
#[doc = "Field `CC1S` reader - CC1S"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - CC1S"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - OC1FE"]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - OC1FE"]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - OC1PE"]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - OC1PE"]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - OC1M"]
pub type OC1M_R = crate::FieldReader;
#[doc = "Field `OC1M` writer - OC1M"]
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1M3` reader - OC1M3"]
pub type OC1M3_R = crate::BitReader;
#[doc = "Field `OC1M3` writer - OC1M3"]
pub type OC1M3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC1FE"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC1PE"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - OC1M3"]
    #[inline(always)]
    pub fn oc1m3(&self) -> OC1M3_R {
        OC1M3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<TIM13_CCMR1rs> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - OC1FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc1fe(&mut self) -> OC1FE_W<TIM13_CCMR1rs> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OC1PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc1pe(&mut self) -> OC1PE_W<TIM13_CCMR1rs> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m(&mut self) -> OC1M_W<TIM13_CCMR1rs> {
        OC1M_W::new(self, 4)
    }
    #[doc = "Bit 16 - OC1M3"]
    #[inline(always)]
    #[must_use]
    pub fn oc1m3(&mut self) -> OC1M3_W<TIM13_CCMR1rs> {
        OC1M3_W::new(self, 16)
    }
}
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim13_ccmr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim13_ccmr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM13_CCMR1rs;
impl crate::RegisterSpec for TIM13_CCMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim13_ccmr1::R`](R) reader structure"]
impl crate::Readable for TIM13_CCMR1rs {}
#[doc = "`write(|w| ..)` method takes [`tim13_ccmr1::W`](W) writer structure"]
impl crate::Writable for TIM13_CCMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM13_CCMR1 to value 0"]
impl crate::Resettable for TIM13_CCMR1rs {
    const RESET_VALUE: u32 = 0;
}
