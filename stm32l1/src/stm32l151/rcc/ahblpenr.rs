#[doc = "Register `AHBLPENR` reader"]
pub type R = crate::R<AHBLPENRrs>;
#[doc = "Register `AHBLPENR` writer"]
pub type W = crate::W<AHBLPENRrs>;
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during Sleep mode"]
pub type GPIOALPEN_R = crate::BitReader;
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during Sleep mode"]
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub type GPIOBLPEN_R = crate::BitReader;
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub type GPIOBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub type GPIOCLPEN_R = crate::BitReader;
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub type GPIOCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub type GPIODLPEN_R = crate::BitReader;
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub type GPIODLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub type GPIOELPEN_R = crate::BitReader;
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub type GPIOELPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub type GPIOHLPEN_R = crate::BitReader;
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub type GPIOHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode"]
pub type GPIOFLPEN_R = crate::BitReader;
#[doc = "Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode"]
pub type GPIOFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode"]
pub type GPIOGLPEN_R = crate::BitReader;
#[doc = "Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode"]
pub type GPIOGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub type CRCLPEN_R = crate::BitReader;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub type CRCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITFLPEN` reader - FLITF clock enable during Sleep mode"]
pub type FLITFLPEN_R = crate::BitReader;
#[doc = "Field `FLITFLPEN` writer - FLITF clock enable during Sleep mode"]
pub type FLITFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMLPEN` reader - SRAM clock enable during Sleep mode"]
pub type SRAMLPEN_R = crate::BitReader;
#[doc = "Field `SRAMLPEN` writer - SRAM clock enable during Sleep mode"]
pub type SRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub type DMA1LPEN_R = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub type DMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub type DMA2LPEN_R = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub type DMA2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FLITF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramlpen(&self) -> SRAMLPEN_R {
        SRAMLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<AHBLPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<AHBLPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<AHBLPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<AHBLPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<AHBLPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<AHBLPENRrs> {
        GPIOHLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHBLPENRrs> {
        GPIOFLPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<AHBLPENRrs> {
        GPIOGLPEN_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<AHBLPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 15 - FLITF clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<AHBLPENRrs> {
        FLITFLPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramlpen(&mut self) -> SRAMLPEN_W<AHBLPENRrs> {
        SRAMLPEN_W::new(self, 16)
    }
    #[doc = "Bit 24 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<AHBLPENRrs> {
        DMA1LPEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<AHBLPENRrs> {
        DMA2LPEN_W::new(self, 25)
    }
}
#[doc = "AHB peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPENRrs;
impl crate::RegisterSpec for AHBLPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpenr::R`](R) reader structure"]
impl crate::Readable for AHBLPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahblpenr::W`](W) writer structure"]
impl crate::Writable for AHBLPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLPENR to value 0x0101_903f"]
impl crate::Resettable for AHBLPENRrs {
    const RESET_VALUE: u32 = 0x0101_903f;
}
