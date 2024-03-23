#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AHBENRrs>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AHBENRrs>;
#[doc = "DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DMA1EN_R as SRAMEN_R;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub use DMA1EN_R as FLITFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `OTGFSEN` reader - USB OTG FS clock enable"]
pub use DMA1EN_R as OTGFSEN_R;
#[doc = "Field `ETHMACEN` reader - Ethernet MAC clock enable"]
pub use DMA1EN_R as ETHMACEN_R;
#[doc = "Field `ETHMACTXEN` reader - Ethernet MAC TX clock enable"]
pub use DMA1EN_R as ETHMACTXEN_R;
#[doc = "Field `ETHMACRXEN` reader - Ethernet MAC RX clock enable"]
pub use DMA1EN_R as ETHMACRXEN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DMA1EN_W as SRAMEN_W;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub use DMA1EN_W as FLITFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
#[doc = "Field `OTGFSEN` writer - USB OTG FS clock enable"]
pub use DMA1EN_W as OTGFSEN_W;
#[doc = "Field `ETHMACEN` writer - Ethernet MAC clock enable"]
pub use DMA1EN_W as ETHMACEN_W;
#[doc = "Field `ETHMACTXEN` writer - Ethernet MAC TX clock enable"]
pub use DMA1EN_W as ETHMACTXEN_W;
#[doc = "Field `ETHMACRXEN` writer - Ethernet MAC RX clock enable"]
pub use DMA1EN_W as ETHMACRXEN_W;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet MAC clock enable"]
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC TX clock enable"]
    #[inline(always)]
    pub fn ethmactxen(&self) -> ETHMACTXEN_R {
        ETHMACTXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet MAC RX clock enable"]
    #[inline(always)]
    pub fn ethmacrxen(&self) -> ETHMACRXEN_R {
        ETHMACRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<AHBENRrs> {
        SRAMEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<AHBENRrs> {
        FLITFEN_W::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBENRrs> {
        CRCEN_W::new(self, 6)
    }
    #[doc = "Bit 12 - USB OTG FS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<AHBENRrs> {
        OTGFSEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - Ethernet MAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<AHBENRrs> {
        ETHMACEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Ethernet MAC TX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmactxen(&mut self) -> ETHMACTXEN_W<AHBENRrs> {
        ETHMACTXEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet MAC RX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrxen(&mut self) -> ETHMACRXEN_W<AHBENRrs> {
        ETHMACRXEN_W::new(self, 16)
    }
}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AHBENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x14;
}
