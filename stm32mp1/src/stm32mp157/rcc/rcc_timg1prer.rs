#[doc = "Register `RCC_TIMG1PRER` reader"]
pub type R = crate::R<RCC_TIMG1PRERrs>;
#[doc = "Register `RCC_TIMG1PRER` writer"]
pub type W = crate::W<RCC_TIMG1PRERrs>;
#[doc = "Field `TIMG1PRE` reader - TIMG1PRE"]
pub type TIMG1PRE_R = crate::BitReader;
#[doc = "Field `TIMG1PRE` writer - TIMG1PRE"]
pub type TIMG1PRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMG1PRERDY` reader - TIMG1PRERDY"]
pub type TIMG1PRERDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIMG1PRE"]
    #[inline(always)]
    pub fn timg1pre(&self) -> TIMG1PRE_R {
        TIMG1PRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - TIMG1PRERDY"]
    #[inline(always)]
    pub fn timg1prerdy(&self) -> TIMG1PRERDY_R {
        TIMG1PRERDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG1PRE"]
    #[inline(always)]
    #[must_use]
    pub fn timg1pre(&mut self) -> TIMG1PRE_W<RCC_TIMG1PRERrs> {
        TIMG1PRE_W::new(self, 0)
    }
}
#[doc = "This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_timg1prer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_timg1prer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_TIMG1PRERrs;
impl crate::RegisterSpec for RCC_TIMG1PRERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_timg1prer::R`](R) reader structure"]
impl crate::Readable for RCC_TIMG1PRERrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_timg1prer::W`](W) writer structure"]
impl crate::Writable for RCC_TIMG1PRERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_TIMG1PRER to value 0x8000_0000"]
impl crate::Resettable for RCC_TIMG1PRERrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
