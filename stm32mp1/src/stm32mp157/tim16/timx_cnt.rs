#[doc = "Register `TIMx_CNT` reader"]
pub type R = crate::R<TIMX_CNTrs>;
#[doc = "Register `TIMx_CNT` writer"]
pub type W = crate::W<TIMX_CNTrs>;
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
    pub fn cnt(&mut self) -> CNT_W<TIMX_CNTrs> {
        CNT_W::new(self, 0)
    }
}
#[doc = "TIM16/TIM17 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMX_CNTrs;
impl crate::RegisterSpec for TIMX_CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timx_cnt::R`](R) reader structure"]
impl crate::Readable for TIMX_CNTrs {}
#[doc = "`write(|w| ..)` method takes [`timx_cnt::W`](W) writer structure"]
impl crate::Writable for TIMX_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMx_CNT to value 0"]
impl crate::Resettable for TIMX_CNTrs {
    const RESET_VALUE: u32 = 0;
}
