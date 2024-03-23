#[doc = "Register `CCIPR1` reader"]
pub type R = crate::R<CCIPR1rs>;
#[doc = "Register `CCIPR1` writer"]
pub type W = crate::W<CCIPR1rs>;
#[doc = "Field `USART1SEL` reader - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_R = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USART2SEL` reader - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_R = crate::FieldReader;
#[doc = "Field `USART2SEL` writer - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USART3SEL` reader - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_R = crate::FieldReader;
#[doc = "Field `USART3SEL` writer - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART4SEL` reader - UART4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART4SEL_R = crate::FieldReader;
#[doc = "Field `UART4SEL` writer - UART4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART5SEL` reader - UART5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART5SEL_R = crate::FieldReader;
#[doc = "Field `UART5SEL` writer - UART5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USART6SEL` reader - USART6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type USART6SEL_R = crate::FieldReader;
#[doc = "Field `USART6SEL` writer - USART6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type USART6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART7SEL` reader - UART7 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART7SEL_R = crate::FieldReader;
#[doc = "Field `UART7SEL` writer - UART7 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART7SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART8SEL` reader - UART8 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART8SEL_R = crate::FieldReader;
#[doc = "Field `UART8SEL` writer - UART8 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART8SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART9SEL` reader - UART9 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART9SEL_R = crate::FieldReader;
#[doc = "Field `UART9SEL` writer - UART9 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type UART9SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USART10SEL` reader - USART10 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type USART10SEL_R = crate::FieldReader;
#[doc = "Field `USART10SEL` writer - USART10 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type USART10SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TIMICSEL` reader - TIM12, TIM15 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_R = crate::BitReader;
#[doc = "Field `TIMICSEL` writer - TIM12, TIM15 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - UART4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - UART5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - USART6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - UART7 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - UART8 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - UART9 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn uart9sel(&self) -> UART9SEL_R {
        UART9SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - USART10 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn usart10sel(&self) -> USART10SEL_R {
        USART10SEL_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - TIM12, TIM15 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPR1rs> {
        USART2SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<CCIPR1rs> {
        USART3SEL_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - UART4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<CCIPR1rs> {
        UART4SEL_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - UART5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<CCIPR1rs> {
        UART5SEL_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - USART6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart6sel(&mut self) -> USART6SEL_W<CCIPR1rs> {
        USART6SEL_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - UART7 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn uart7sel(&mut self) -> UART7SEL_W<CCIPR1rs> {
        UART7SEL_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - UART8 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn uart8sel(&mut self) -> UART8SEL_W<CCIPR1rs> {
        UART8SEL_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - UART9 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn uart9sel(&mut self) -> UART9SEL_W<CCIPR1rs> {
        UART9SEL_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - USART10 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn usart10sel(&mut self) -> USART10SEL_W<CCIPR1rs> {
        USART10SEL_W::new(self, 27)
    }
    #[doc = "Bit 31 - TIM12, TIM15 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn timicsel(&mut self) -> TIMICSEL_W<CCIPR1rs> {
        TIMICSEL_W::new(self, 31)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr1::R`](R) reader structure"]
impl crate::Readable for CCIPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure"]
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR1 to value 0"]
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0;
}
