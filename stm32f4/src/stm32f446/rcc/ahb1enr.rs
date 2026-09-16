///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
/**IO port A clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - IO port D clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - IO port E clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - IO port F clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - IO port G clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - IO port H clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use GPIOAEN_R as CRCEN_R;
///Field `BKPSRAMEN` reader - Backup SRAM interface clock enable
pub use GPIOAEN_R as BKPSRAMEN_R;
///Field `DMA1EN` reader - DMA1 clock enable
pub use GPIOAEN_R as DMA1EN_R;
///Field `DMA2EN` reader - DMA2 clock enable
pub use GPIOAEN_R as DMA2EN_R;
///Field `OTGHSEN` reader - USB OTG HS clock enable
pub use GPIOAEN_R as OTGHSEN_R;
///Field `OTGHSULPIEN` reader - USB OTG HSULPI clock enable
pub use GPIOAEN_R as OTGHSULPIEN_R;
///Field `GPIOBEN` writer - IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - IO port D clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - IO port E clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - IO port F clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - IO port G clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - IO port H clock enable
pub use GPIOAEN_W as GPIOHEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use GPIOAEN_W as CRCEN_W;
///Field `BKPSRAMEN` writer - Backup SRAM interface clock enable
pub use GPIOAEN_W as BKPSRAMEN_W;
///Field `DMA1EN` writer - DMA1 clock enable
pub use GPIOAEN_W as DMA1EN_W;
///Field `DMA2EN` writer - DMA2 clock enable
pub use GPIOAEN_W as DMA2EN_W;
///Field `OTGHSEN` writer - USB OTG HS clock enable
pub use GPIOAEN_W as OTGHSEN_W;
///Field `OTGHSULPIEN` writer - USB OTG HSULPI clock enable
pub use GPIOAEN_W as OTGHSULPIEN_W;
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
            .field("gpioaen", &self.gpioaen())
            .field("otghsulpien", &self.otghsulpien())
            .field("otghsen", &self.otghsen())
            .field("dma2en", &self.dma2en())
            .field("dma1en", &self.dma1en())
            .field("bkpsramen", &self.bkpsramen())
            .field("crcen", &self.crcen())
            .field("gpiohen", &self.gpiohen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiofen", &self.gpiofen())
            .field("gpioeen", &self.gpioeen())
            .field("gpioden", &self.gpioden())
            .field("gpiocen", &self.gpiocen())
            .field("gpioben", &self.gpioben())
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#RCC:AHB1ENR)*/
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
