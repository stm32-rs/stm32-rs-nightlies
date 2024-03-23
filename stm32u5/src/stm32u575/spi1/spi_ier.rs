#[doc = "Register `SPI_IER` reader"]
pub type R = crate::R<SPI_IERrs>;
#[doc = "Register `SPI_IER` writer"]
pub type W = crate::W<SPI_IERrs>;
#[doc = "Field `RXPIE` reader - RXP interrupt enable"]
pub type RXPIE_R = crate::BitReader;
#[doc = "Field `RXPIE` writer - RXP interrupt enable"]
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPIE` reader - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
pub type TXPIE_R = crate::BitReader;
#[doc = "Field `TXPIE` writer - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
pub type TXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DXPIE` reader - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
pub type DXPIE_R = crate::BitReader;
#[doc = "Field `DXPIE` writer - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
pub type DXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_R = crate::BitReader;
#[doc = "Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFIE` reader - TXTFIE interrupt enable"]
pub type TXTFIE_R = crate::BitReader;
#[doc = "Field `TXTFIE` writer - TXTFIE interrupt enable"]
pub type TXTFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE` reader - UDR interrupt enable"]
pub type UDRIE_R = crate::BitReader;
#[doc = "Field `UDRIE` writer - UDR interrupt enable"]
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - OVR interrupt enable"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - OVR interrupt enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEIE` reader - CRC error interrupt enable"]
pub type CRCEIE_R = crate::BitReader;
#[doc = "Field `CRCEIE` writer - CRC error interrupt enable"]
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREIE` reader - TIFRE interrupt enable"]
pub type TIFREIE_R = crate::BitReader;
#[doc = "Field `TIFREIE` writer - TIFRE interrupt enable"]
pub type TIFREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFIE` reader - mode Fault interrupt enable"]
pub type MODFIE_R = crate::BitReader;
#[doc = "Field `MODFIE` writer - mode Fault interrupt enable"]
pub type MODFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXP interrupt enable"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC error interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXP interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<SPI_IERrs> {
        RXPIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<SPI_IERrs> {
        TXPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<SPI_IERrs> {
        DXPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<SPI_IERrs> {
        EOTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<SPI_IERrs> {
        TXTFIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<SPI_IERrs> {
        UDRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<SPI_IERrs> {
        OVRIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<SPI_IERrs> {
        CRCEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<SPI_IERrs> {
        TIFREIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - mode Fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<SPI_IERrs> {
        MODFIE_W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_IERrs;
impl crate::RegisterSpec for SPI_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ier::R`](R) reader structure"]
impl crate::Readable for SPI_IERrs {}
#[doc = "`write(|w| ..)` method takes [`spi_ier::W`](W) writer structure"]
impl crate::Writable for SPI_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_IER to value 0"]
impl crate::Resettable for SPI_IERrs {
    const RESET_VALUE: u32 = 0;
}
