///Register `AHB1LPENR` reader
pub type R = crate::R<AHB1LPENRrs>;
///Register `AHB1LPENR` writer
pub type W = crate::W<AHB1LPENRrs>;
///Field `GPIOALPEN` reader - IO port A clock enable during sleep mode
pub type GPIOALPEN_R = crate::BitReader;
///Field `GPIOALPEN` writer - IO port A clock enable during sleep mode
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode
pub type GPIOBLPEN_R = crate::BitReader;
///Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode
pub type GPIOBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode
pub type GPIOCLPEN_R = crate::BitReader;
///Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode
pub type GPIOCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode
pub type GPIODLPEN_R = crate::BitReader;
///Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode
pub type GPIODLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode
pub type GPIOELPEN_R = crate::BitReader;
///Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode
pub type GPIOELPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode
pub type GPIOFLPEN_R = crate::BitReader;
///Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode
pub type GPIOFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode
pub type GPIOGLPEN_R = crate::BitReader;
///Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode
pub type GPIOGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode
pub type GPIOHLPEN_R = crate::BitReader;
///Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode
pub type GPIOHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOILPEN` reader - IO port I clock enable during Sleep mode
pub type GPIOILPEN_R = crate::BitReader;
///Field `GPIOILPEN` writer - IO port I clock enable during Sleep mode
pub type GPIOILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCLPEN` reader - CRC clock enable during Sleep mode
pub type CRCLPEN_R = crate::BitReader;
///Field `CRCLPEN` writer - CRC clock enable during Sleep mode
pub type CRCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode
pub type FLITFLPEN_R = crate::BitReader;
///Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode
pub type FLITFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode
pub type SRAM1LPEN_R = crate::BitReader;
///Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode
pub type SRAM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2LPEN` reader - SRAM 2 interface clock enable during Sleep mode
pub type SRAM2LPEN_R = crate::BitReader;
///Field `SRAM2LPEN` writer - SRAM 2 interface clock enable during Sleep mode
pub type SRAM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMLPEN` reader - Backup SRAM interface clock enable during Sleep mode
pub type BKPSRAMLPEN_R = crate::BitReader;
///Field `BKPSRAMLPEN` writer - Backup SRAM interface clock enable during Sleep mode
pub type BKPSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM3LPEN` reader - SRAM 3 interface clock enable during Sleep mode
pub type SRAM3LPEN_R = crate::BitReader;
///Field `SRAM3LPEN` writer - SRAM 3 interface clock enable during Sleep mode
pub type SRAM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode
pub type DMA1LPEN_R = crate::BitReader;
///Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode
pub type DMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode
pub type DMA2LPEN_R = crate::BitReader;
///Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode
pub type DMA2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACLPEN` reader - Ethernet MAC clock enable during Sleep mode
pub type ETHMACLPEN_R = crate::BitReader;
///Field `ETHMACLPEN` writer - Ethernet MAC clock enable during Sleep mode
pub type ETHMACLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACTXLPEN` reader - Ethernet transmission clock enable during Sleep mode
pub type ETHMACTXLPEN_R = crate::BitReader;
///Field `ETHMACTXLPEN` writer - Ethernet transmission clock enable during Sleep mode
pub type ETHMACTXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACRXLPEN` reader - Ethernet reception clock enable during Sleep mode
pub type ETHMACRXLPEN_R = crate::BitReader;
///Field `ETHMACRXLPEN` writer - Ethernet reception clock enable during Sleep mode
pub type ETHMACRXLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACPTPLPEN` reader - Ethernet PTP clock enable during Sleep mode
pub type ETHMACPTPLPEN_R = crate::BitReader;
///Field `ETHMACPTPLPEN` writer - Ethernet PTP clock enable during Sleep mode
pub type ETHMACPTPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGHSLPEN` reader - USB OTG HS clock enable during Sleep mode
pub type OTGHSLPEN_R = crate::BitReader;
///Field `OTGHSLPEN` writer - USB OTG HS clock enable during Sleep mode
pub type OTGHSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGHSULPILPEN` reader - USB OTG HS ULPI clock enable during Sleep mode
pub type OTGHSULPILPEN_R = crate::BitReader;
///Field `OTGHSULPILPEN` writer - USB OTG HS ULPI clock enable during Sleep mode
pub type OTGHSULPILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
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
            .field("crclpen", &self.crclpen())
            .field("flitflpen", &self.flitflpen())
            .field("sram1lpen", &self.sram1lpen())
            .field("sram2lpen", &self.sram2lpen())
            .field("bkpsramlpen", &self.bkpsramlpen())
            .field("sram3lpen", &self.sram3lpen())
            .field("dma1lpen", &self.dma1lpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("ethmaclpen", &self.ethmaclpen())
            .field("ethmactxlpen", &self.ethmactxlpen())
            .field("ethmacrxlpen", &self.ethmacrxlpen())
            .field("ethmacptplpen", &self.ethmacptplpen())
            .field("otghslpen", &self.otghslpen())
            .field("otghsulpilpen", &self.otghsulpilpen())
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
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<'_, AHB1LPENRrs> {
        CRCLPEN_W::new(self, 12)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RCC:AHB1LPENR)*/
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
