///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
/**IO port A clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOPAEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<GPIOPAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOPAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOPAEN` reader - IO port A clock enable
pub type GPIOPAEN_R = crate::BitReader<GPIOPAEN>;
impl GPIOPAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOPAEN {
        match self.bits {
            false => GPIOPAEN::Disabled,
            true => GPIOPAEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOPAEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOPAEN::Enabled
    }
}
///Field `GPIOPAEN` writer - IO port A clock enable
pub type GPIOPAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOPAEN>;
impl<'a, REG> GPIOPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOPAEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOPAEN::Enabled)
    }
}
///Field `GPIOPBEN` reader - IO port B clock enable
pub use GPIOPAEN_R as GPIOPBEN_R;
///Field `GPIOPCEN` reader - IO port C clock enable
pub use GPIOPAEN_R as GPIOPCEN_R;
///Field `GPIOPDEN` reader - IO port D clock enable
pub use GPIOPAEN_R as GPIOPDEN_R;
///Field `GPIOPEEN` reader - IO port E clock enable
pub use GPIOPAEN_R as GPIOPEEN_R;
///Field `GPIOPHEN` reader - IO port H clock enable
pub use GPIOPAEN_R as GPIOPHEN_R;
///Field `GPIOPFEN` reader - IO port F clock enable
pub use GPIOPAEN_R as GPIOPFEN_R;
///Field `GPIOPGEN` reader - IO port G clock enable
pub use GPIOPAEN_R as GPIOPGEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use GPIOPAEN_R as CRCEN_R;
///Field `FLITFEN` reader - FLITF clock enable
pub use GPIOPAEN_R as FLITFEN_R;
///Field `DMA1EN` reader - DMA1 clock enable
pub use GPIOPAEN_R as DMA1EN_R;
///Field `DMA2EN` reader - DMA2 clock enable
pub use GPIOPAEN_R as DMA2EN_R;
///Field `FSMCEN` reader - FSMCEN
pub use GPIOPAEN_R as FSMCEN_R;
///Field `GPIOPBEN` writer - IO port B clock enable
pub use GPIOPAEN_W as GPIOPBEN_W;
///Field `GPIOPCEN` writer - IO port C clock enable
pub use GPIOPAEN_W as GPIOPCEN_W;
///Field `GPIOPDEN` writer - IO port D clock enable
pub use GPIOPAEN_W as GPIOPDEN_W;
///Field `GPIOPEEN` writer - IO port E clock enable
pub use GPIOPAEN_W as GPIOPEEN_W;
///Field `GPIOPHEN` writer - IO port H clock enable
pub use GPIOPAEN_W as GPIOPHEN_W;
///Field `GPIOPFEN` writer - IO port F clock enable
pub use GPIOPAEN_W as GPIOPFEN_W;
///Field `GPIOPGEN` writer - IO port G clock enable
pub use GPIOPAEN_W as GPIOPGEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use GPIOPAEN_W as CRCEN_W;
///Field `FLITFEN` writer - FLITF clock enable
pub use GPIOPAEN_W as FLITFEN_W;
///Field `DMA1EN` writer - DMA1 clock enable
pub use GPIOPAEN_W as DMA1EN_W;
///Field `DMA2EN` writer - DMA2 clock enable
pub use GPIOPAEN_W as DMA2EN_W;
///Field `FSMCEN` writer - FSMCEN
pub use GPIOPAEN_W as FSMCEN_W;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpiopaen(&self) -> GPIOPAEN_R {
        GPIOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpiopben(&self) -> GPIOPBEN_R {
        GPIOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiopcen(&self) -> GPIOPCEN_R {
        GPIOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpiopden(&self) -> GPIOPDEN_R {
        GPIOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpiopeen(&self) -> GPIOPEEN_R {
        GPIOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port H clock enable
    #[inline(always)]
    pub fn gpiophen(&self) -> GPIOPHEN_R {
        GPIOPHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port F clock enable
    #[inline(always)]
    pub fn gpiopfen(&self) -> GPIOPFEN_R {
        GPIOPFEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port G clock enable
    #[inline(always)]
    pub fn gpiopgen(&self) -> GPIOPGEN_R {
        GPIOPGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - FSMCEN
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("gpiopaen", &self.gpiopaen())
            .field("fsmcen", &self.fsmcen())
            .field("dma2en", &self.dma2en())
            .field("dma1en", &self.dma1en())
            .field("flitfen", &self.flitfen())
            .field("crcen", &self.crcen())
            .field("gpiopgen", &self.gpiopgen())
            .field("gpiopfen", &self.gpiopfen())
            .field("gpiophen", &self.gpiophen())
            .field("gpiopeen", &self.gpiopeen())
            .field("gpiopden", &self.gpiopden())
            .field("gpiopcen", &self.gpiopcen())
            .field("gpiopben", &self.gpiopben())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpiopaen(&mut self) -> GPIOPAEN_W<AHBENRrs> {
        GPIOPAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpiopben(&mut self) -> GPIOPBEN_W<AHBENRrs> {
        GPIOPBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiopcen(&mut self) -> GPIOPCEN_W<AHBENRrs> {
        GPIOPCEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpiopden(&mut self) -> GPIOPDEN_W<AHBENRrs> {
        GPIOPDEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpiopeen(&mut self) -> GPIOPEEN_W<AHBENRrs> {
        GPIOPEEN_W::new(self, 4)
    }
    ///Bit 5 - IO port H clock enable
    #[inline(always)]
    pub fn gpiophen(&mut self) -> GPIOPHEN_W<AHBENRrs> {
        GPIOPHEN_W::new(self, 5)
    }
    ///Bit 6 - IO port F clock enable
    #[inline(always)]
    pub fn gpiopfen(&mut self) -> GPIOPFEN_W<AHBENRrs> {
        GPIOPFEN_W::new(self, 6)
    }
    ///Bit 7 - IO port G clock enable
    #[inline(always)]
    pub fn gpiopgen(&mut self) -> GPIOPGEN_W<AHBENRrs> {
        GPIOPGEN_W::new(self, 7)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 15 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W<AHBENRrs> {
        FLITFEN_W::new(self, 15)
    }
    ///Bit 24 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBENRrs> {
        DMA1EN_W::new(self, 24)
    }
    ///Bit 25 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBENRrs> {
        DMA2EN_W::new(self, 25)
    }
    ///Bit 30 - FSMCEN
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W<AHBENRrs> {
        FSMCEN_W::new(self, 30)
    }
}
/**AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:AHBENR)*/
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
///`reset()` method sets AHBENR to value 0x8000
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x8000;
}
