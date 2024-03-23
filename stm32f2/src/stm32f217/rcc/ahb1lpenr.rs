#[doc = "Register `AHB1LPENR` reader"]
pub type R = crate::R<AHB1LPENRrs>;
#[doc = "Register `AHB1LPENR` writer"]
pub type W = crate::W<AHB1LPENRrs>;
#[doc = "IO port A clock enable during sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<GPIOALPEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during sleep mode"]
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN>;
impl GPIOALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN {
        match self.bits {
            false => GPIOALPEN::DisabledInSleep,
            true => GPIOALPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN::EnabledInSleep
    }
}
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during sleep mode"]
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::EnabledInSleep)
    }
}
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOBLPEN_R;
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOCLPEN_R;
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIODLPEN_R;
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOELPEN_R;
#[doc = "Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOFLPEN_R;
#[doc = "Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOGLPEN_R;
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOHLPEN_R;
#[doc = "Field `GPIOILPEN` reader - IO port I clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOILPEN_R;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub use GPIOALPEN_R as CRCLPEN_R;
#[doc = "Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as FLITFLPEN_R;
#[doc = "Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as SRAM1LPEN_R;
#[doc = "Field `SRAM2LPEN` reader - SRAM 2 interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as SRAM2LPEN_R;
#[doc = "Field `BKPSRAMLPEN` reader - Backup SRAM interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as BKPSRAMLPEN_R;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub use GPIOALPEN_R as DMA1LPEN_R;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub use GPIOALPEN_R as DMA2LPEN_R;
#[doc = "Field `ETHMACLPEN` reader - Ethernet MAC clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACLPEN_R;
#[doc = "Field `ETHMACTXLPEN` reader - Ethernet transmission clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACTXLPEN_R;
#[doc = "Field `ETHMACRXLPEN` reader - Ethernet reception clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACRXLPEN_R;
#[doc = "Field `ETHMACPTPLPEN` reader - Ethernet PTP clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACPTPLPEN_R;
#[doc = "Field `OTGHSLPEN` reader - USB OTG HS clock enable during Sleep mode"]
pub use GPIOALPEN_R as OTGHSLPEN_R;
#[doc = "Field `OTGHSULPILPEN` reader - USB OTG HS ULPI clock enable during Sleep mode"]
pub use GPIOALPEN_R as OTGHSULPILPEN_R;
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOBLPEN_W;
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOCLPEN_W;
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIODLPEN_W;
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOELPEN_W;
#[doc = "Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOFLPEN_W;
#[doc = "Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOGLPEN_W;
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOHLPEN_W;
#[doc = "Field `GPIOILPEN` writer - IO port I clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOILPEN_W;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub use GPIOALPEN_W as CRCLPEN_W;
#[doc = "Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as FLITFLPEN_W;
#[doc = "Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as SRAM1LPEN_W;
#[doc = "Field `SRAM2LPEN` writer - SRAM 2 interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as SRAM2LPEN_W;
#[doc = "Field `BKPSRAMLPEN` writer - Backup SRAM interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as BKPSRAMLPEN_W;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub use GPIOALPEN_W as DMA1LPEN_W;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub use GPIOALPEN_W as DMA2LPEN_W;
#[doc = "Field `ETHMACLPEN` writer - Ethernet MAC clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACLPEN_W;
#[doc = "Field `ETHMACTXLPEN` writer - Ethernet transmission clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACTXLPEN_W;
#[doc = "Field `ETHMACRXLPEN` writer - Ethernet reception clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACRXLPEN_W;
#[doc = "Field `ETHMACPTPLPEN` writer - Ethernet PTP clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACPTPLPEN_W;
#[doc = "Field `OTGHSLPEN` writer - USB OTG HS clock enable during Sleep mode"]
pub use GPIOALPEN_W as OTGHSLPEN_W;
#[doc = "Field `OTGHSULPILPEN` writer - USB OTG HS ULPI clock enable during Sleep mode"]
pub use GPIOALPEN_W as OTGHSULPILPEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
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
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmactxlpen(&self) -> ETHMACTXLPEN_R {
        ETHMACTXLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacrxlpen(&self) -> ETHMACRXLPEN_R {
        ETHMACRXLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacptplpen(&self) -> ETHMACPTPLPEN_R {
        ETHMACPTPLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghslpen(&self) -> OTGHSLPEN_R {
        OTGHSLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghsulpilpen(&self) -> OTGHSULPILPEN_R {
        OTGHSULPILPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<AHB1LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<AHB1LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<AHB1LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<AHB1LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<AHB1LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHB1LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<AHB1LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<AHB1LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<AHB1LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<AHB1LPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<AHB1LPENRrs> {
        FLITFLPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<AHB1LPENRrs> {
        SRAM1LPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<AHB1LPENRrs> {
        SRAM2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<AHB1LPENRrs> {
        BKPSRAMLPEN_W::new(self, 18)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<AHB1LPENRrs> {
        DMA1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<AHB1LPENRrs> {
        DMA2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W<AHB1LPENRrs> {
        ETHMACLPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ethmactxlpen(&mut self) -> ETHMACTXLPEN_W<AHB1LPENRrs> {
        ETHMACTXLPEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrxlpen(&mut self) -> ETHMACRXLPEN_W<AHB1LPENRrs> {
        ETHMACRXLPEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacptplpen(&mut self) -> ETHMACPTPLPEN_W<AHB1LPENRrs> {
        ETHMACPTPLPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otghslpen(&mut self) -> OTGHSLPEN_W<AHB1LPENRrs> {
        OTGHSLPEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otghsulpilpen(&mut self) -> OTGHSULPILPEN_W<AHB1LPENRrs> {
        OTGHSULPILPEN_W::new(self, 30)
    }
}
#[doc = "AHB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1LPENRrs;
impl crate::RegisterSpec for AHB1LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for AHB1LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for AHB1LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0x7e67_91ff"]
impl crate::Resettable for AHB1LPENRrs {
    const RESET_VALUE: u32 = 0x7e67_91ff;
}
