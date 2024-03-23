#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "Field `TIM1EN` reader - CPU1 TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - CPU1 TIM1 timer clock enable"]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - CPU1 SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - CPU1 SPI1 clock enable"]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - CPU1 USART1clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - CPU1 USART1clock enable"]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - CPU1 TIM16 timer clock enable"]
pub type TIM16EN_R = crate::BitReader;
#[doc = "Field `TIM16EN` writer - CPU1 TIM16 timer clock enable"]
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - CPU1 TIM17 timer clock enable"]
pub type TIM17EN_R = crate::BitReader;
#[doc = "Field `TIM17EN` writer - CPU1 TIM17 timer clock enable"]
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1EN` reader - CPU1 SAI1 clock enable"]
pub type SAI1EN_R = crate::BitReader;
#[doc = "Field `SAI1EN` writer - CPU1 SAI1 clock enable"]
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - CPU1 TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU1 SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU1 TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU1 TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU1 SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - CPU1 TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU1 SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - CPU1 USART1clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 17 - CPU1 TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU1 TIM17 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    #[doc = "Bit 21 - CPU1 SAI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
}
#[doc = "APB2ENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
