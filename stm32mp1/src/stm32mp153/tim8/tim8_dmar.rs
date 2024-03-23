#[doc = "Register `TIM8_DMAR` reader"]
pub type R = crate::R<TIM8_DMARrs>;
#[doc = "Register `TIM8_DMAR` writer"]
pub type W = crate::W<TIM8_DMARrs>;
#[doc = "Field `DMAB` reader - DMAB"]
pub type DMAB_R = crate::FieldReader<u32>;
#[doc = "Field `DMAB` writer - DMAB"]
pub type DMAB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMAB"]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMAB"]
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<TIM8_DMARrs> {
        DMAB_W::new(self, 0)
    }
}
#[doc = "TIM8 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim8_dmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim8_dmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM8_DMARrs;
impl crate::RegisterSpec for TIM8_DMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim8_dmar::R`](R) reader structure"]
impl crate::Readable for TIM8_DMARrs {}
#[doc = "`write(|w| ..)` method takes [`tim8_dmar::W`](W) writer structure"]
impl crate::Writable for TIM8_DMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM8_DMAR to value 0"]
impl crate::Resettable for TIM8_DMARrs {
    const RESET_VALUE: u32 = 0;
}
