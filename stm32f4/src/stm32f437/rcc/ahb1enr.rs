///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader;
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBEN` reader - IO port B clock enable
pub type GPIOBEN_R = crate::BitReader;
///Field `GPIOBEN` writer - IO port B clock enable
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCEN` reader - IO port C clock enable
pub type GPIOCEN_R = crate::BitReader;
///Field `GPIOCEN` writer - IO port C clock enable
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODEN` reader - IO port D clock enable
pub type GPIODEN_R = crate::BitReader;
///Field `GPIODEN` writer - IO port D clock enable
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEEN` reader - IO port E clock enable
pub type GPIOEEN_R = crate::BitReader;
///Field `GPIOEEN` writer - IO port E clock enable
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFEN` reader - IO port F clock enable
pub type GPIOFEN_R = crate::BitReader;
///Field `GPIOFEN` writer - IO port F clock enable
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGEN` reader - IO port G clock enable
pub type GPIOGEN_R = crate::BitReader;
///Field `GPIOGEN` writer - IO port G clock enable
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHEN` reader - IO port H clock enable
pub type GPIOHEN_R = crate::BitReader;
///Field `GPIOHEN` writer - IO port H clock enable
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOIEN` reader - IO port I clock enable
pub type GPIOIEN_R = crate::BitReader;
///Field `GPIOIEN` writer - IO port I clock enable
pub type GPIOIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC clock enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC clock enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMEN` reader - Backup SRAM interface clock enable
pub type BKPSRAMEN_R = crate::BitReader;
///Field `BKPSRAMEN` writer - Backup SRAM interface clock enable
pub type BKPSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMDATARAMEN` reader - CCM data RAM clock enable
pub type CCMDATARAMEN_R = crate::BitReader;
///Field `CCMDATARAMEN` writer - CCM data RAM clock enable
pub type CCMDATARAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader;
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2EN` reader - DMA2 clock enable
pub type DMA2EN_R = crate::BitReader;
///Field `DMA2EN` writer - DMA2 clock enable
pub type DMA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACEN` reader - Ethernet MAC clock enable
pub type ETHMACEN_R = crate::BitReader;
///Field `ETHMACEN` writer - Ethernet MAC clock enable
pub type ETHMACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACTXEN` reader - Ethernet Transmission clock enable
pub type ETHMACTXEN_R = crate::BitReader;
///Field `ETHMACTXEN` writer - Ethernet Transmission clock enable
pub type ETHMACTXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACRXEN` reader - Ethernet Reception clock enable
pub type ETHMACRXEN_R = crate::BitReader;
///Field `ETHMACRXEN` writer - Ethernet Reception clock enable
pub type ETHMACRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHMACPTPEN` reader - Ethernet PTP clock enable
pub type ETHMACPTPEN_R = crate::BitReader;
///Field `ETHMACPTPEN` writer - Ethernet PTP clock enable
pub type ETHMACPTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGHSEN` reader - USB OTG HS clock enable
pub type OTGHSEN_R = crate::BitReader;
///Field `OTGHSEN` writer - USB OTG HS clock enable
pub type OTGHSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGHSULPIEN` reader - USB OTG HSULPI clock enable
pub type OTGHSULPIEN_R = crate::BitReader;
///Field `OTGHSULPIEN` writer - USB OTG HSULPI clock enable
pub type OTGHSULPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 18 - Backup SRAM interface clock enable
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CCM data RAM clock enable
    #[inline(always)]
    pub fn ccmdataramen(&self) -> CCMDATARAMEN_R {
        CCMDATARAMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 25 - Ethernet MAC clock enable
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Ethernet Transmission clock enable
    #[inline(always)]
    pub fn ethmactxen(&self) -> ETHMACTXEN_R {
        ETHMACTXEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Ethernet Reception clock enable
    #[inline(always)]
    pub fn ethmacrxen(&self) -> ETHMACRXEN_R {
        ETHMACRXEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Ethernet PTP clock enable
    #[inline(always)]
    pub fn ethmacptpen(&self) -> ETHMACPTPEN_R {
        ETHMACPTPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - USB OTG HS clock enable
    #[inline(always)]
    pub fn otghsen(&self) -> OTGHSEN_R {
        OTGHSEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - USB OTG HSULPI clock enable
    #[inline(always)]
    pub fn otghsulpien(&self) -> OTGHSULPIEN_R {
        OTGHSULPIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("otghsulpien", &self.otghsulpien())
            .field("otghsen", &self.otghsen())
            .field("ethmacptpen", &self.ethmacptpen())
            .field("ethmacrxen", &self.ethmacrxen())
            .field("ethmactxen", &self.ethmactxen())
            .field("ethmacen", &self.ethmacen())
            .field("dma2en", &self.dma2en())
            .field("dma1en", &self.dma1en())
            .field("ccmdataramen", &self.ccmdataramen())
            .field("bkpsramen", &self.bkpsramen())
            .field("crcen", &self.crcen())
            .field("gpioien", &self.gpioien())
            .field("gpiohen", &self.gpiohen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiofen", &self.gpiofen())
            .field("gpioeen", &self.gpioeen())
            .field("gpioden", &self.gpioden())
            .field("gpiocen", &self.gpiocen())
            .field("gpioben", &self.gpioben())
            .field("gpioaen", &self.gpioaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB1ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB1ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB1ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB1ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB1ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB1ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB1ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB1ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<'_, AHB1ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 18 - Backup SRAM interface clock enable
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<'_, AHB1ENRrs> {
        BKPSRAMEN_W::new(self, 18)
    }
    ///Bit 20 - CCM data RAM clock enable
    #[inline(always)]
    pub fn ccmdataramen(&mut self) -> CCMDATARAMEN_W<'_, AHB1ENRrs> {
        CCMDATARAMEN_W::new(self, 20)
    }
    ///Bit 21 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHB1ENRrs> {
        DMA1EN_W::new(self, 21)
    }
    ///Bit 22 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHB1ENRrs> {
        DMA2EN_W::new(self, 22)
    }
    ///Bit 25 - Ethernet MAC clock enable
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W<'_, AHB1ENRrs> {
        ETHMACEN_W::new(self, 25)
    }
    ///Bit 26 - Ethernet Transmission clock enable
    #[inline(always)]
    pub fn ethmactxen(&mut self) -> ETHMACTXEN_W<'_, AHB1ENRrs> {
        ETHMACTXEN_W::new(self, 26)
    }
    ///Bit 27 - Ethernet Reception clock enable
    #[inline(always)]
    pub fn ethmacrxen(&mut self) -> ETHMACRXEN_W<'_, AHB1ENRrs> {
        ETHMACRXEN_W::new(self, 27)
    }
    ///Bit 28 - Ethernet PTP clock enable
    #[inline(always)]
    pub fn ethmacptpen(&mut self) -> ETHMACPTPEN_W<'_, AHB1ENRrs> {
        ETHMACPTPEN_W::new(self, 28)
    }
    ///Bit 29 - USB OTG HS clock enable
    #[inline(always)]
    pub fn otghsen(&mut self) -> OTGHSEN_W<'_, AHB1ENRrs> {
        OTGHSEN_W::new(self, 29)
    }
    ///Bit 30 - USB OTG HSULPI clock enable
    #[inline(always)]
    pub fn otghsulpien(&mut self) -> OTGHSULPIEN_W<'_, AHB1ENRrs> {
        OTGHSULPIEN_W::new(self, 30)
    }
}
/**AHB1 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0x0010_0000
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x0010_0000;
}
