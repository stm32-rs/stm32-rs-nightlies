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
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DMA1EN_R as SRAMEN_R;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub use DMA1EN_R as FLITFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub use DMA1EN_R as IOPAEN_R;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub use DMA1EN_R as IOPBEN_R;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub use DMA1EN_R as IOPCEN_R;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub use DMA1EN_R as IOPDEN_R;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub use DMA1EN_R as IOPFEN_R;
#[doc = "Field `TSCEN` reader - Touch sensing controller clock enable"]
pub use DMA1EN_R as TSCEN_R;
#[doc = "Field `ADC12EN` reader - ADC1 and ADC2 clock enable"]
pub use DMA1EN_R as ADC12EN_R;
#[doc = "Field `ADC34EN` reader - ADC3 and ADC4 clock enable"]
pub use DMA1EN_R as ADC34EN_R;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DMA1EN_W as SRAMEN_W;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub use DMA1EN_W as FLITFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub use DMA1EN_W as IOPAEN_W;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub use DMA1EN_W as IOPBEN_W;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub use DMA1EN_W as IOPCEN_W;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub use DMA1EN_W as IOPDEN_W;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub use DMA1EN_W as IOPFEN_W;
#[doc = "Field `TSCEN` writer - Touch sensing controller clock enable"]
pub use DMA1EN_W as TSCEN_W;
#[doc = "Field `ADC12EN` writer - ADC1 and ADC2 clock enable"]
pub use DMA1EN_W as ADC12EN_W;
#[doc = "Field `ADC34EN` writer - ADC3 and ADC4 clock enable"]
pub use DMA1EN_W as ADC34EN_W;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 clock enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 clock enable"]
    #[inline(always)]
    pub fn adc34en(&self) -> ADC34EN_R {
        ADC34EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBENRrs> {
        DMA1EN_W::new(self, 0)
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
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<AHBENRrs> {
        IOPAEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<AHBENRrs> {
        IOPBEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<AHBENRrs> {
        IOPCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<AHBENRrs> {
        IOPDEN_W::new(self, 20)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IOPFEN_W<AHBENRrs> {
        IOPFEN_W::new(self, 22)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<AHBENRrs> {
        TSCEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<AHBENRrs> {
        ADC12EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc34en(&mut self) -> ADC34EN_W<AHBENRrs> {
        ADC34EN_W::new(self, 29)
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
