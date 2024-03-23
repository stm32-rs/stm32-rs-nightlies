#[doc = "Register `SPI2S_IER` reader"]
pub type R = crate::R<SPI2S_IERrs>;
#[doc = "Register `SPI2S_IER` writer"]
pub type W = crate::W<SPI2S_IERrs>;
#[doc = "Field `RXPIE` reader - RXPIE"]
pub type RXPIE_R = crate::BitReader;
#[doc = "Field `RXPIE` writer - RXPIE"]
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPIE` reader - TXPIE"]
pub type TXPIE_R = crate::BitReader;
#[doc = "Field `TXPIE` writer - TXPIE"]
pub type TXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DXPIE` reader - DXPIE"]
pub type DXPIE_R = crate::BitReader;
#[doc = "Field `DXPIE` writer - DXPIE"]
pub type DXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTIE` reader - EOTIE"]
pub type EOTIE_R = crate::BitReader;
#[doc = "Field `EOTIE` writer - EOTIE"]
pub type EOTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFIE` reader - TXTFIE"]
pub type TXTFIE_R = crate::BitReader;
#[doc = "Field `TXTFIE` writer - TXTFIE"]
pub type TXTFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE` reader - UDRIE"]
pub type UDRIE_R = crate::BitReader;
#[doc = "Field `UDRIE` writer - UDRIE"]
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEIE` reader - CRCEIE"]
pub type CRCEIE_R = crate::BitReader;
#[doc = "Field `CRCEIE` writer - CRCEIE"]
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREIE` reader - TIFREIE"]
pub type TIFREIE_R = crate::BitReader;
#[doc = "Field `TIFREIE` writer - TIFREIE"]
pub type TIFREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFIE` reader - MODFIE"]
pub type MODFIE_R = crate::BitReader;
#[doc = "Field `MODFIE` writer - MODFIE"]
pub type MODFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSERFIE` reader - TSERFIE"]
pub type TSERFIE_R = crate::BitReader;
#[doc = "Field `TSERFIE` writer - TSERFIE"]
pub type TSERFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<SPI2S_IERrs> {
        RXPIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<SPI2S_IERrs> {
        TXPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<SPI2S_IERrs> {
        DXPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<SPI2S_IERrs> {
        EOTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<SPI2S_IERrs> {
        TXTFIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<SPI2S_IERrs> {
        UDRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<SPI2S_IERrs> {
        OVRIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<SPI2S_IERrs> {
        CRCEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<SPI2S_IERrs> {
        TIFREIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<SPI2S_IERrs> {
        MODFIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    #[must_use]
    pub fn tserfie(&mut self) -> TSERFIE_W<SPI2S_IERrs> {
        TSERFIE_W::new(self, 10)
    }
}
#[doc = "SPI/I2S interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI2S_IERrs;
impl crate::RegisterSpec for SPI2S_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi2s_ier::R`](R) reader structure"]
impl crate::Readable for SPI2S_IERrs {}
#[doc = "`write(|w| ..)` method takes [`spi2s_ier::W`](W) writer structure"]
impl crate::Writable for SPI2S_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI2S_IER to value 0"]
impl crate::Resettable for SPI2S_IERrs {
    const RESET_VALUE: u32 = 0;
}
