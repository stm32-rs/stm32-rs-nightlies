#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENRrs>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENRrs>;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOIEN` reader - IO port I clock enable"]
pub use GPIOAEN_R as GPIOIEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use GPIOAEN_R as CRCEN_R;
#[doc = "Field `BKPSRAMEN` reader - Backup SRAM interface clock enable"]
pub use GPIOAEN_R as BKPSRAMEN_R;
#[doc = "Field `DTCMRAMEN` reader - CCM data RAM clock enable"]
pub use GPIOAEN_R as DTCMRAMEN_R;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub use GPIOAEN_R as DMA1EN_R;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use GPIOAEN_R as DMA2EN_R;
#[doc = "Field `OTGHSEN` reader - USB OTG HS clock enable"]
pub use GPIOAEN_R as OTGHSEN_R;
#[doc = "Field `OTGHSULPIEN` reader - USB OTG HSULPI clock enable"]
pub use GPIOAEN_R as OTGHSULPIEN_R;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOIEN` writer - IO port I clock enable"]
pub use GPIOAEN_W as GPIOIEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use GPIOAEN_W as CRCEN_W;
#[doc = "Field `BKPSRAMEN` writer - Backup SRAM interface clock enable"]
pub use GPIOAEN_W as BKPSRAMEN_W;
#[doc = "Field `DTCMRAMEN` writer - CCM data RAM clock enable"]
pub use GPIOAEN_W as DTCMRAMEN_W;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub use GPIOAEN_W as DMA1EN_W;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use GPIOAEN_W as DMA2EN_W;
#[doc = "Field `OTGHSEN` writer - USB OTG HS clock enable"]
pub use GPIOAEN_W as OTGHSEN_W;
#[doc = "Field `OTGHSULPIEN` writer - USB OTG HSULPI clock enable"]
pub use GPIOAEN_W as OTGHSULPIEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CCM data RAM clock enable"]
    #[inline(always)]
    pub fn dtcmramen(&self) -> DTCMRAMEN_R {
        DTCMRAMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline(always)]
    pub fn otghsen(&self) -> OTGHSEN_R {
        OTGHSEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline(always)]
    pub fn otghsulpien(&self) -> OTGHSULPIEN_R {
        OTGHSULPIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB1ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB1ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB1ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB1ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB1ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB1ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB1ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB1ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<AHB1ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<AHB1ENRrs> {
        BKPSRAMEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - CCM data RAM clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmramen(&mut self) -> DTCMRAMEN_W<AHB1ENRrs> {
        DTCMRAMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHB1ENRrs> {
        DMA1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHB1ENRrs> {
        DMA2EN_W::new(self, 22)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otghsen(&mut self) -> OTGHSEN_W<AHB1ENRrs> {
        OTGHSEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otghsulpien(&mut self) -> OTGHSULPIEN_W<AHB1ENRrs> {
        OTGHSULPIEN_W::new(self, 30)
    }
}
#[doc = "AHB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0010_0000"]
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x0010_0000;
}
