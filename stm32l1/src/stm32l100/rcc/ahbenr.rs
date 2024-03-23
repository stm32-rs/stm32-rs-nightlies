#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AHBENRrs>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AHBENRrs>;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOPAEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<GPIOPAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOPAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOPAEN` reader - IO port A clock enable"]
pub type GPIOPAEN_R = crate::BitReader<GPIOPAEN>;
impl GPIOPAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOPAEN {
        match self.bits {
            false => GPIOPAEN::Disabled,
            true => GPIOPAEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOPAEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOPAEN::Enabled
    }
}
#[doc = "Field `GPIOPAEN` writer - IO port A clock enable"]
pub type GPIOPAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOPAEN>;
impl<'a, REG> GPIOPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOPAEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOPAEN::Enabled)
    }
}
#[doc = "Field `GPIOPBEN` reader - IO port B clock enable"]
pub use GPIOPAEN_R as GPIOPBEN_R;
#[doc = "Field `GPIOPCEN` reader - IO port C clock enable"]
pub use GPIOPAEN_R as GPIOPCEN_R;
#[doc = "Field `GPIOPDEN` reader - IO port D clock enable"]
pub use GPIOPAEN_R as GPIOPDEN_R;
#[doc = "Field `GPIOPEEN` reader - IO port E clock enable"]
pub use GPIOPAEN_R as GPIOPEEN_R;
#[doc = "Field `GPIOPHEN` reader - IO port H clock enable"]
pub use GPIOPAEN_R as GPIOPHEN_R;
#[doc = "Field `GPIOPFEN` reader - IO port F clock enable"]
pub use GPIOPAEN_R as GPIOPFEN_R;
#[doc = "Field `GPIOPGEN` reader - IO port G clock enable"]
pub use GPIOPAEN_R as GPIOPGEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use GPIOPAEN_R as CRCEN_R;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub use GPIOPAEN_R as FLITFEN_R;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub use GPIOPAEN_R as DMA1EN_R;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use GPIOPAEN_R as DMA2EN_R;
#[doc = "Field `FSMCEN` reader - FSMCEN"]
pub use GPIOPAEN_R as FSMCEN_R;
#[doc = "Field `GPIOPBEN` writer - IO port B clock enable"]
pub use GPIOPAEN_W as GPIOPBEN_W;
#[doc = "Field `GPIOPCEN` writer - IO port C clock enable"]
pub use GPIOPAEN_W as GPIOPCEN_W;
#[doc = "Field `GPIOPDEN` writer - IO port D clock enable"]
pub use GPIOPAEN_W as GPIOPDEN_W;
#[doc = "Field `GPIOPEEN` writer - IO port E clock enable"]
pub use GPIOPAEN_W as GPIOPEEN_W;
#[doc = "Field `GPIOPHEN` writer - IO port H clock enable"]
pub use GPIOPAEN_W as GPIOPHEN_W;
#[doc = "Field `GPIOPFEN` writer - IO port F clock enable"]
pub use GPIOPAEN_W as GPIOPFEN_W;
#[doc = "Field `GPIOPGEN` writer - IO port G clock enable"]
pub use GPIOPAEN_W as GPIOPGEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use GPIOPAEN_W as CRCEN_W;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub use GPIOPAEN_W as FLITFEN_W;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub use GPIOPAEN_W as DMA1EN_W;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use GPIOPAEN_W as DMA2EN_W;
#[doc = "Field `FSMCEN` writer - FSMCEN"]
pub use GPIOPAEN_W as FSMCEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpiopaen(&self) -> GPIOPAEN_R {
        GPIOPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpiopben(&self) -> GPIOPBEN_R {
        GPIOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiopcen(&self) -> GPIOPCEN_R {
        GPIOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpiopden(&self) -> GPIOPDEN_R {
        GPIOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpiopeen(&self) -> GPIOPEEN_R {
        GPIOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiophen(&self) -> GPIOPHEN_R {
        GPIOPHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiopfen(&self) -> GPIOPFEN_R {
        GPIOPFEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiopgen(&self) -> GPIOPGEN_R {
        GPIOPGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - FSMCEN"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopaen(&mut self) -> GPIOPAEN_W<AHBENRrs> {
        GPIOPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopben(&mut self) -> GPIOPBEN_W<AHBENRrs> {
        GPIOPBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopcen(&mut self) -> GPIOPCEN_W<AHBENRrs> {
        GPIOPCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopden(&mut self) -> GPIOPDEN_W<AHBENRrs> {
        GPIOPDEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopeen(&mut self) -> GPIOPEEN_W<AHBENRrs> {
        GPIOPEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiophen(&mut self) -> GPIOPHEN_W<AHBENRrs> {
        GPIOPHEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopfen(&mut self) -> GPIOPFEN_W<AHBENRrs> {
        GPIOPFEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiopgen(&mut self) -> GPIOPGEN_W<AHBENRrs> {
        GPIOPGEN_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 15 - FLITF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<AHBENRrs> {
        FLITFEN_W::new(self, 15)
    }
    #[doc = "Bit 24 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBENRrs> {
        DMA1EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBENRrs> {
        DMA2EN_W::new(self, 25)
    }
    #[doc = "Bit 30 - FSMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<AHBENRrs> {
        FSMCEN_W::new(self, 30)
    }
}
#[doc = "AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets AHBENR to value 0x8000"]
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x8000;
}
