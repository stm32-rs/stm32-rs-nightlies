#[doc = "Register `RCC_TIMG2PRER` reader"]
pub type R = crate::R<RCC_TIMG2PRERrs>;
#[doc = "Register `RCC_TIMG2PRER` writer"]
pub type W = crate::W<RCC_TIMG2PRERrs>;
#[doc = "Field `TIMG2PRE` reader - TIMG2PRE"]
pub type TIMG2PRE_R = crate::BitReader;
#[doc = "Field `TIMG2PRE` writer - TIMG2PRE"]
pub type TIMG2PRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMG2PRERDY` reader - TIMG2PRERDY"]
pub type TIMG2PRERDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&self) -> TIMG2PRE_R {
        TIMG2PRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - TIMG2PRERDY"]
    #[inline(always)]
    pub fn timg2prerdy(&self) -> TIMG2PRERDY_R {
        TIMG2PRERDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    #[must_use]
    pub fn timg2pre(&mut self) -> TIMG2PRE_W<RCC_TIMG2PRERrs> {
        TIMG2PRE_W::new(self, 0)
    }
}
#[doc = "This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_timg2prer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_timg2prer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_TIMG2PRERrs;
impl crate::RegisterSpec for RCC_TIMG2PRERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_timg2prer::R`](R) reader structure"]
impl crate::Readable for RCC_TIMG2PRERrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_timg2prer::W`](W) writer structure"]
impl crate::Writable for RCC_TIMG2PRERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_TIMG2PRER to value 0x8000_0000"]
impl crate::Resettable for RCC_TIMG2PRERrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
