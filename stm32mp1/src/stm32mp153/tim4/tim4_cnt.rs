#[doc = "Register `TIM4_CNT` reader"]
pub type R = crate::R<TIM4_CNTrs>;
#[doc = "Register `TIM4_CNT` writer"]
pub type W = crate::W<TIM4_CNTrs>;
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - CNT"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UIFCPY` reader - UIFCPY"]
pub type UIFCPY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UIFCPY"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<TIM4_CNTrs> {
        CNT_W::new(self, 0)
    }
}
#[doc = "TIM4 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM4_CNTrs;
impl crate::RegisterSpec for TIM4_CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim4_cnt::R`](R) reader structure"]
impl crate::Readable for TIM4_CNTrs {}
#[doc = "`write(|w| ..)` method takes [`tim4_cnt::W`](W) writer structure"]
impl crate::Writable for TIM4_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM4_CNT to value 0"]
impl crate::Resettable for TIM4_CNTrs {
    const RESET_VALUE: u32 = 0;
}
