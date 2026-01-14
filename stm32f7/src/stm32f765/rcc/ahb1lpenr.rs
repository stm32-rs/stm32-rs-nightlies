///Register `AHB1LPENR` reader
pub type R = crate::R<AHB1LPENRrs>;
///Register `AHB1LPENR` writer
pub type W = crate::W<AHB1LPENRrs>;
/**IO port A clock enable during sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<GPIOALPEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOALPEN` reader - IO port A clock enable during sleep mode
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN>;
impl GPIOALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN {
        match self.bits {
            false => GPIOALPEN::DisabledInSleep,
            true => GPIOALPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN::EnabledInSleep
    }
}
///Field `GPIOALPEN` writer - IO port A clock enable during sleep mode
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::EnabledInSleep)
    }
}
///Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOBLPEN_R;
///Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOCLPEN_R;
///Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode
pub use GPIOALPEN_R as GPIODLPEN_R;
///Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOELPEN_R;
///Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `GPIOILPEN` reader - IO port I clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOILPEN_R;
///Field `GPIOJLPEN` reader - IO port J clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOJLPEN_R;
///Field `GPIOKLPEN` reader - IO port K clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOKLPEN_R;
///Field `CRCLPEN` reader - CRC clock enable during Sleep mode
pub use GPIOALPEN_R as CRCLPEN_R;
///Field `AXILPEN` reader - AXI to AHB bridge clock enable during Sleep mode
pub use GPIOALPEN_R as AXILPEN_R;
///Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode
pub use GPIOALPEN_R as FLITFLPEN_R;
///Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM1LPEN_R;
///Field `SRAM2LPEN` reader - SRAM 2 interface clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM2LPEN_R;
///Field `BKPSRAMLPEN` reader - Backup SRAM interface clock enable during Sleep mode
pub use GPIOALPEN_R as BKPSRAMLPEN_R;
///Field `SRAM3LPEN` reader - SRAM 3 interface clock enable during Sleep mode
pub use GPIOALPEN_R as SRAM3LPEN_R;
///Field `DTCMLPEN` reader - DTCM RAM interface clock enable during Sleep mode
pub use GPIOALPEN_R as DTCMLPEN_R;
///Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode
pub use GPIOALPEN_R as DMA1LPEN_R;
///Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode
pub use GPIOALPEN_R as DMA2LPEN_R;
///Field `DMA2DLPEN` reader - DMA2D clock enable during Sleep mode
pub use GPIOALPEN_R as DMA2DLPEN_R;
///Field `ETHMACLPEN` reader - Ethernet MAC clock enable during Sleep mode
pub use GPIOALPEN_R as ETHMACLPEN_R;
///Field `ETHMACTXLPEN` reader - Ethernet transmission clock enable during Sleep mode
pub use GPIOALPEN_R as ETHMACTXLPEN_R;
///Field `ETHMACRXLPEN` reader - Ethernet reception clock enable during Sleep mode
pub use GPIOALPEN_R as ETHMACRXLPEN_R;
///Field `ETHMACPTPLPEN` reader - Ethernet PTP clock enable during Sleep mode
pub use GPIOALPEN_R as ETHMACPTPLPEN_R;
///Field `OTGHSLPEN` reader - USB OTG HS clock enable during Sleep mode
pub use GPIOALPEN_R as OTGHSLPEN_R;
///Field `OTGHSULPILPEN` reader - USB OTG HS ULPI clock enable during Sleep mode
pub use GPIOALPEN_R as OTGHSULPILPEN_R;
///Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `GPIOILPEN` writer - IO port I clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOILPEN_W;
///Field `GPIOJLPEN` writer - IO port J clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOJLPEN_W;
///Field `GPIOKLPEN` writer - IO port K clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOKLPEN_W;
///Field `CRCLPEN` writer - CRC clock enable during Sleep mode
pub use GPIOALPEN_W as CRCLPEN_W;
///Field `AXILPEN` writer - AXI to AHB bridge clock enable during Sleep mode
pub use GPIOALPEN_W as AXILPEN_W;
///Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode
pub use GPIOALPEN_W as FLITFLPEN_W;
///Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM1LPEN_W;
///Field `SRAM2LPEN` writer - SRAM 2 interface clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM2LPEN_W;
///Field `BKPSRAMLPEN` writer - Backup SRAM interface clock enable during Sleep mode
pub use GPIOALPEN_W as BKPSRAMLPEN_W;
///Field `SRAM3LPEN` writer - SRAM 3 interface clock enable during Sleep mode
pub use GPIOALPEN_W as SRAM3LPEN_W;
///Field `DTCMLPEN` writer - DTCM RAM interface clock enable during Sleep mode
pub use GPIOALPEN_W as DTCMLPEN_W;
///Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode
pub use GPIOALPEN_W as DMA1LPEN_W;
///Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode
pub use GPIOALPEN_W as DMA2LPEN_W;
///Field `DMA2DLPEN` writer - DMA2D clock enable during Sleep mode
pub use GPIOALPEN_W as DMA2DLPEN_W;
///Field `ETHMACLPEN` writer - Ethernet MAC clock enable during Sleep mode
pub use GPIOALPEN_W as ETHMACLPEN_W;
///Field `ETHMACTXLPEN` writer - Ethernet transmission clock enable during Sleep mode
pub use GPIOALPEN_W as ETHMACTXLPEN_W;
///Field `ETHMACRXLPEN` writer - Ethernet reception clock enable during Sleep mode
pub use GPIOALPEN_W as ETHMACRXLPEN_W;
///Field `ETHMACPTPLPEN` writer - Ethernet PTP clock enable during Sleep mode
pub use GPIOALPEN_W as ETHMACPTPLPEN_W;
///Field `OTGHSLPEN` writer - USB OTG HS clock enable during Sleep mode
pub use GPIOALPEN_W as OTGHSLPEN_W;
///Field `OTGHSULPILPEN` writer - USB OTG HS ULPI clock enable during Sleep mode
pub use GPIOALPEN_W as OTGHSULPILPEN_W;
impl R {
    ///Bit 0 - IO port A clock enable during sleep mode
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IO port J clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IO port K clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AXI to AHB bridge clock enable during Sleep mode
    #[inline(always)]
    pub fn axilpen(&self) -> AXILPEN_R {
        AXILPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Flash interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SRAM 1interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SRAM 2 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Backup SRAM interface clock enable during Sleep mode
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SRAM 3 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - DTCM RAM interface clock enable during Sleep mode
    #[inline(always)]
    pub fn dtcmlpen(&self) -> DTCMLPEN_R {
        DTCMLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - DMA2D clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Ethernet MAC clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Ethernet transmission clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmactxlpen(&self) -> ETHMACTXLPEN_R {
        ETHMACTXLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Ethernet reception clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmacrxlpen(&self) -> ETHMACRXLPEN_R {
        ETHMACRXLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Ethernet PTP clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmacptplpen(&self) -> ETHMACPTPLPEN_R {
        ETHMACPTPLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - USB OTG HS clock enable during Sleep mode
    #[inline(always)]
    pub fn otghslpen(&self) -> OTGHSLPEN_R {
        OTGHSLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - USB OTG HS ULPI clock enable during Sleep mode
    #[inline(always)]
    pub fn otghsulpilpen(&self) -> OTGHSULPILPEN_R {
        OTGHSULPILPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1LPENR")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("gpioilpen", &self.gpioilpen())
            .field("gpiojlpen", &self.gpiojlpen())
            .field("gpioklpen", &self.gpioklpen())
            .field("crclpen", &self.crclpen())
            .field("flitflpen", &self.flitflpen())
            .field("sram1lpen", &self.sram1lpen())
            .field("sram2lpen", &self.sram2lpen())
            .field("bkpsramlpen", &self.bkpsramlpen())
            .field("sram3lpen", &self.sram3lpen())
            .field("dma1lpen", &self.dma1lpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("dma2dlpen", &self.dma2dlpen())
            .field("ethmaclpen", &self.ethmaclpen())
            .field("ethmactxlpen", &self.ethmactxlpen())
            .field("ethmacrxlpen", &self.ethmacrxlpen())
            .field("ethmacptplpen", &self.ethmacptplpen())
            .field("otghslpen", &self.otghslpen())
            .field("otghsulpilpen", &self.otghsulpilpen())
            .field("axilpen", &self.axilpen())
            .field("dtcmlpen", &self.dtcmlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable during sleep mode
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, AHB1LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, AHB1LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, AHB1LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, AHB1LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, AHB1LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, AHB1LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, AHB1LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - IO port H clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, AHB1LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 8 - IO port I clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<'_, AHB1LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    ///Bit 9 - IO port J clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<'_, AHB1LPENRrs> {
        GPIOJLPEN_W::new(self, 9)
    }
    ///Bit 10 - IO port K clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<'_, AHB1LPENRrs> {
        GPIOKLPEN_W::new(self, 10)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<'_, AHB1LPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    ///Bit 13 - AXI to AHB bridge clock enable during Sleep mode
    #[inline(always)]
    pub fn axilpen(&mut self) -> AXILPEN_W<'_, AHB1LPENRrs> {
        AXILPEN_W::new(self, 13)
    }
    ///Bit 15 - Flash interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<'_, AHB1LPENRrs> {
        FLITFLPEN_W::new(self, 15)
    }
    ///Bit 16 - SRAM 1interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<'_, AHB1LPENRrs> {
        SRAM1LPEN_W::new(self, 16)
    }
    ///Bit 17 - SRAM 2 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<'_, AHB1LPENRrs> {
        SRAM2LPEN_W::new(self, 17)
    }
    ///Bit 18 - Backup SRAM interface clock enable during Sleep mode
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<'_, AHB1LPENRrs> {
        BKPSRAMLPEN_W::new(self, 18)
    }
    ///Bit 19 - SRAM 3 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<'_, AHB1LPENRrs> {
        SRAM3LPEN_W::new(self, 19)
    }
    ///Bit 20 - DTCM RAM interface clock enable during Sleep mode
    #[inline(always)]
    pub fn dtcmlpen(&mut self) -> DTCMLPEN_W<'_, AHB1LPENRrs> {
        DTCMLPEN_W::new(self, 20)
    }
    ///Bit 21 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<'_, AHB1LPENRrs> {
        DMA1LPEN_W::new(self, 21)
    }
    ///Bit 22 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<'_, AHB1LPENRrs> {
        DMA2LPEN_W::new(self, 22)
    }
    ///Bit 23 - DMA2D clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<'_, AHB1LPENRrs> {
        DMA2DLPEN_W::new(self, 23)
    }
    ///Bit 25 - Ethernet MAC clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W<'_, AHB1LPENRrs> {
        ETHMACLPEN_W::new(self, 25)
    }
    ///Bit 26 - Ethernet transmission clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmactxlpen(&mut self) -> ETHMACTXLPEN_W<'_, AHB1LPENRrs> {
        ETHMACTXLPEN_W::new(self, 26)
    }
    ///Bit 27 - Ethernet reception clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmacrxlpen(&mut self) -> ETHMACRXLPEN_W<'_, AHB1LPENRrs> {
        ETHMACRXLPEN_W::new(self, 27)
    }
    ///Bit 28 - Ethernet PTP clock enable during Sleep mode
    #[inline(always)]
    pub fn ethmacptplpen(&mut self) -> ETHMACPTPLPEN_W<'_, AHB1LPENRrs> {
        ETHMACPTPLPEN_W::new(self, 28)
    }
    ///Bit 29 - USB OTG HS clock enable during Sleep mode
    #[inline(always)]
    pub fn otghslpen(&mut self) -> OTGHSLPEN_W<'_, AHB1LPENRrs> {
        OTGHSLPEN_W::new(self, 29)
    }
    ///Bit 30 - USB OTG HS ULPI clock enable during Sleep mode
    #[inline(always)]
    pub fn otghsulpilpen(&mut self) -> OTGHSULPILPEN_W<'_, AHB1LPENRrs> {
        OTGHSULPILPEN_W::new(self, 30)
    }
}
/**AHB1 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#RCC:AHB1LPENR)*/
pub struct AHB1LPENRrs;
impl crate::RegisterSpec for AHB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1lpenr::R`](R) reader structure
impl crate::Readable for AHB1LPENRrs {}
///`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure
impl crate::Writable for AHB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1LPENR to value 0x7e67_91ff
impl crate::Resettable for AHB1LPENRrs {
    const RESET_VALUE: u32 = 0x7e67_91ff;
}
