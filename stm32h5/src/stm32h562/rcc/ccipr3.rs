#[doc = "Register `CCIPR3` reader"]
pub type R = crate::R<CCIPR3rs>;
#[doc = "Register `CCIPR3` writer"]
pub type W = crate::W<CCIPR3rs>;
#[doc = "Field `SPI1SEL` reader - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI1SEL_R = crate::FieldReader;
#[doc = "Field `SPI1SEL` writer - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI2SEL` reader - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI2SEL_R = crate::FieldReader;
#[doc = "Field `SPI2SEL` writer - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI3SEL` reader - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI3SEL_R = crate::FieldReader;
#[doc = "Field `SPI3SEL` writer - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI4SEL` reader - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI4SEL_R = crate::FieldReader;
#[doc = "Field `SPI4SEL` writer - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI5SEL` reader - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI5SEL_R = crate::FieldReader;
#[doc = "Field `SPI5SEL` writer - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI6SEL` reader - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI6SEL_R = crate::FieldReader;
#[doc = "Field `SPI6SEL` writer - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type SPI6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_R = crate::FieldReader;
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi4sel(&self) -> SPI4SEL_R {
        SPI4SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi5sel(&self) -> SPI5SEL_R {
        SPI5SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<CCIPR3rs> {
        SPI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<CCIPR3rs> {
        SPI2SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<CCIPR3rs> {
        SPI3SEL_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - SPI4 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi4sel(&mut self) -> SPI4SEL_W<CCIPR3rs> {
        SPI4SEL_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - SPI5 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi5sel(&mut self) -> SPI5SEL_W<CCIPR3rs> {
        SPI5SEL_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - SPI6 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<CCIPR3rs> {
        SPI6SEL_W::new(self, 15)
    }
    #[doc = "Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPR3rs> {
        LPUART1SEL_W::new(self, 24)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR3rs;
impl crate::RegisterSpec for CCIPR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr3::R`](R) reader structure"]
impl crate::Readable for CCIPR3rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure"]
impl crate::Writable for CCIPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR3 to value 0"]
impl crate::Resettable for CCIPR3rs {
    const RESET_VALUE: u32 = 0;
}
