#[doc = "Register `APB1ENR2` reader"]
pub type R = crate::R<APB1ENR2rs>;
#[doc = "Register `APB1ENR2` writer"]
pub type W = crate::W<APB1ENR2rs>;
#[doc = "Field `LPUART1EN` reader - CPU1 Low power UART 1 clock enable"]
pub type LPUART1EN_R = crate::BitReader;
#[doc = "Field `LPUART1EN` writer - CPU1 Low power UART 1 clock enable"]
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2EN` reader - CPU1 LPTIM2EN"]
pub type LPTIM2EN_R = crate::BitReader;
#[doc = "Field `LPTIM2EN` writer - CPU1 LPTIM2EN"]
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU1 Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - CPU1 LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 Low power UART 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB1ENR2rs> {
        LPUART1EN_W::new(self, 0)
    }
    #[doc = "Bit 5 - CPU1 LPTIM2EN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APB1ENR2rs> {
        LPTIM2EN_W::new(self, 5)
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1ENR2rs;
impl crate::RegisterSpec for APB1ENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr2::R`](R) reader structure"]
impl crate::Readable for APB1ENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure"]
impl crate::Writable for APB1ENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2rs {
    const RESET_VALUE: u32 = 0;
}
