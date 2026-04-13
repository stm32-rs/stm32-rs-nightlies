///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
/**DMA1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
///Field `DMA2EN` reader - DMA2 clock enable
pub use DMA1EN_R as DMA2EN_R;
///Field `SRAMEN` reader - SRAM interface clock enable
pub use DMA1EN_R as SRAMEN_R;
///Field `FLITFEN` reader - FLITF clock enable
pub use DMA1EN_R as FLITFEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use DMA1EN_R as CRCEN_R;
///Field `OTGFSEN` reader - USB OTG FS clock enable
pub use DMA1EN_R as OTGFSEN_R;
///Field `ETHMACEN` reader - Ethernet MAC clock enable
pub use DMA1EN_R as ETHMACEN_R;
///Field `ETHMACTXEN` reader - Ethernet MAC TX clock enable
pub use DMA1EN_R as ETHMACTXEN_R;
///Field `ETHMACRXEN` reader - Ethernet MAC RX clock enable
pub use DMA1EN_R as ETHMACRXEN_R;
///Field `DMA2EN` writer - DMA2 clock enable
pub use DMA1EN_W as DMA2EN_W;
///Field `SRAMEN` writer - SRAM interface clock enable
pub use DMA1EN_W as SRAMEN_W;
///Field `FLITFEN` writer - FLITF clock enable
pub use DMA1EN_W as FLITFEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use DMA1EN_W as CRCEN_W;
///Field `OTGFSEN` writer - USB OTG FS clock enable
pub use DMA1EN_W as OTGFSEN_W;
///Field `ETHMACEN` writer - Ethernet MAC clock enable
pub use DMA1EN_W as ETHMACEN_W;
///Field `ETHMACTXEN` writer - Ethernet MAC TX clock enable
pub use DMA1EN_W as ETHMACTXEN_W;
///Field `ETHMACRXEN` writer - Ethernet MAC RX clock enable
pub use DMA1EN_W as ETHMACRXEN_W;
impl R {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM interface clock enable
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Ethernet MAC clock enable
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ethernet MAC TX clock enable
    #[inline(always)]
    pub fn ethmactxen(&self) -> ETHMACTXEN_R {
        ETHMACTXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Ethernet MAC RX clock enable
    #[inline(always)]
    pub fn ethmacrxen(&self) -> ETHMACRXEN_R {
        ETHMACRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("sramen", &self.sramen())
            .field("flitfen", &self.flitfen())
            .field("crcen", &self.crcen())
            .field("otgfsen", &self.otgfsen())
            .field("ethmacen", &self.ethmacen())
            .field("ethmactxen", &self.ethmactxen())
            .field("ethmacrxen", &self.ethmacrxen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHBENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 2 - SRAM interface clock enable
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W<'_, AHBENRrs> {
        SRAMEN_W::new(self, 2)
    }
    ///Bit 4 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W<'_, AHBENRrs> {
        FLITFEN_W::new(self, 4)
    }
    ///Bit 6 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHBENRrs> {
        CRCEN_W::new(self, 6)
    }
    ///Bit 12 - USB OTG FS clock enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<'_, AHBENRrs> {
        OTGFSEN_W::new(self, 12)
    }
    ///Bit 14 - Ethernet MAC clock enable
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<'_, AHBENRrs> {
        ETHMACEN_W::new(self, 14)
    }
    ///Bit 15 - Ethernet MAC TX clock enable
    #[inline(always)]
    pub fn ethmactxen(&mut self) -> ETHMACTXEN_W<'_, AHBENRrs> {
        ETHMACTXEN_W::new(self, 15)
    }
    ///Bit 16 - Ethernet MAC RX clock enable
    #[inline(always)]
    pub fn ethmacrxen(&mut self) -> ETHMACRXEN_W<'_, AHBENRrs> {
        ETHMACRXEN_W::new(self, 16)
    }
}
/**AHB Peripheral Clock enable register (RCC_AHBENR)

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#RCC:AHBENR)*/
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbenr::R`](R) reader structure
impl crate::Readable for AHBENRrs {}
///`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBENR to value 0x14
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x14;
}
