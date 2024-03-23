#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "Field `SYSCFGRST` reader - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMCRST` reader - SDMMC reset"]
pub type SDMMCRST_R = crate::BitReader;
#[doc = "Field `SDMMCRST` writer - SDMMC reset"]
pub type SDMMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub type TIM15RST_R = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type TIM16RST_R = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_R = crate::BitReader;
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - SDMMC reset"]
    #[inline(always)]
    pub fn sdmmcrst(&self) -> SDMMCRST_R {
        SDMMCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 10 - SDMMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcrst(&mut self) -> SDMMCRST_W<APB2RSTRrs> {
        SDMMCRST_W::new(self, 10)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
}
#[doc = "APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
