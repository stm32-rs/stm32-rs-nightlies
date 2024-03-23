#[doc = "Register `RCC_MC_APB2ENSETR` reader"]
pub type R = crate::R<RCC_MC_APB2ENSETRrs>;
#[doc = "Register `RCC_MC_APB2ENSETR` writer"]
pub type W = crate::W<RCC_MC_APB2ENSETRrs>;
#[doc = "Field `TIM1EN` reader - TIM1EN"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1EN"]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - TIM8EN"]
pub type TIM8EN_R = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8EN"]
pub type TIM8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15EN` reader - TIM15EN"]
pub type TIM15EN_R = crate::BitReader;
#[doc = "Field `TIM15EN` writer - TIM15EN"]
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16EN"]
pub type TIM16EN_R = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16EN"]
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - TIM17EN"]
pub type TIM17EN_R = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM17EN"]
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4EN` reader - SPI4EN"]
pub type SPI4EN_R = crate::BitReader;
#[doc = "Field `SPI4EN` writer - SPI4EN"]
pub type SPI4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5EN` reader - SPI5EN"]
pub type SPI5EN_R = crate::BitReader;
#[doc = "Field `SPI5EN` writer - SPI5EN"]
pub type SPI5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6EN` reader - USART6EN"]
pub type USART6EN_R = crate::BitReader;
#[doc = "Field `USART6EN` writer - USART6EN"]
pub type USART6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1EN` reader - SAI1EN"]
pub type SAI1EN_R = crate::BitReader;
#[doc = "Field `SAI1EN` writer - SAI1EN"]
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2EN` reader - SAI2EN"]
pub type SAI2EN_R = crate::BitReader;
#[doc = "Field `SAI2EN` writer - SAI2EN"]
pub type SAI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI3EN` reader - SAI3EN"]
pub type SAI3EN_R = crate::BitReader;
#[doc = "Field `SAI3EN` writer - SAI3EN"]
pub type SAI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub type DFSDMEN_R = crate::BitReader;
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADFSDMEN` reader - ADFSDMEN"]
pub type ADFSDMEN_R = crate::BitReader;
#[doc = "Field `ADFSDMEN` writer - ADFSDMEN"]
pub type ADFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANEN` reader - FDCANEN"]
pub type FDCANEN_R = crate::BitReader;
#[doc = "Field `FDCANEN` writer - FDCANEN"]
pub type FDCANEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    pub fn adfsdmen(&self) -> ADFSDMEN_R {
        ADFSDMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<RCC_MC_APB2ENSETRrs> {
        TIM1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<RCC_MC_APB2ENSETRrs> {
        TIM8EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<RCC_MC_APB2ENSETRrs> {
        TIM15EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<RCC_MC_APB2ENSETRrs> {
        TIM16EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<RCC_MC_APB2ENSETRrs> {
        TIM17EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<RCC_MC_APB2ENSETRrs> {
        SPI1EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<RCC_MC_APB2ENSETRrs> {
        SPI4EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<RCC_MC_APB2ENSETRrs> {
        SPI5EN_W::new(self, 10)
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<RCC_MC_APB2ENSETRrs> {
        USART6EN_W::new(self, 13)
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<RCC_MC_APB2ENSETRrs> {
        SAI1EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<RCC_MC_APB2ENSETRrs> {
        SAI2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    #[must_use]
    pub fn sai3en(&mut self) -> SAI3EN_W<RCC_MC_APB2ENSETRrs> {
        SAI3EN_W::new(self, 18)
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<RCC_MC_APB2ENSETRrs> {
        DFSDMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmen(&mut self) -> ADFSDMEN_W<RCC_MC_APB2ENSETRrs> {
        ADFSDMEN_W::new(self, 21)
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<RCC_MC_APB2ENSETRrs> {
        FDCANEN_W::new(self, 24)
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb2ensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb2ensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_APB2ENSETRrs;
impl crate::RegisterSpec for RCC_MC_APB2ENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_apb2ensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_APB2ENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_apb2ensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_APB2ENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB2ENSETR to value 0"]
impl crate::Resettable for RCC_MC_APB2ENSETRrs {
    const RESET_VALUE: u32 = 0;
}
