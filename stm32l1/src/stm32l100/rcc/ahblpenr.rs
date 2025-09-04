///Register `AHBLPENR` reader
pub type R = crate::R<AHBLPENRrs>;
///Register `AHBLPENR` writer
pub type W = crate::W<AHBLPENRrs>;
/**IO port A clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<GPIOALPEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOALPEN` reader - IO port A clock enable during Sleep mode
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN>;
impl GPIOALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN {
        match self.bits {
            false => GPIOALPEN::Disabled,
            true => GPIOALPEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOALPEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOALPEN::Enabled
    }
}
///Field `GPIOALPEN` writer - IO port A clock enable during Sleep mode
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::Enabled)
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
///Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `CRCLPEN` reader - CRC clock enable during Sleep mode
pub use GPIOALPEN_R as CRCLPEN_R;
///Field `FLITFLPEN` reader - FLITF clock enable during Sleep mode
pub use GPIOALPEN_R as FLITFLPEN_R;
///Field `SRAMLPEN` reader - SRAM clock enable during Sleep mode
pub use GPIOALPEN_R as SRAMLPEN_R;
///Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode
pub use GPIOALPEN_R as DMA1LPEN_R;
///Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode
pub use GPIOALPEN_R as DMA2LPEN_R;
///Field `AESLPEN` reader - AES clock enable during Sleep mode
pub use GPIOALPEN_R as AESLPEN_R;
///Field `FSMCLPEN` reader - FSMC clock enable during Sleep mode
pub use GPIOALPEN_R as FSMCLPEN_R;
///Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `CRCLPEN` writer - CRC clock enable during Sleep mode
pub use GPIOALPEN_W as CRCLPEN_W;
///Field `FLITFLPEN` writer - FLITF clock enable during Sleep mode
pub use GPIOALPEN_W as FLITFLPEN_W;
///Field `SRAMLPEN` writer - SRAM clock enable during Sleep mode
pub use GPIOALPEN_W as SRAMLPEN_W;
///Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode
pub use GPIOALPEN_W as DMA1LPEN_W;
///Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode
pub use GPIOALPEN_W as DMA2LPEN_W;
///Field `AESLPEN` writer - AES clock enable during Sleep mode
pub use GPIOALPEN_W as AESLPEN_W;
///Field `FSMCLPEN` writer - FSMC clock enable during Sleep mode
pub use GPIOALPEN_W as FSMCLPEN_W;
impl R {
    ///Bit 0 - IO port A clock enable during Sleep mode
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
    ///Bit 5 - IO port H clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port G clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - FLITF clock enable during Sleep mode
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramlpen(&self) -> SRAMLPEN_R {
        SRAMLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - AES clock enable during Sleep mode
    #[inline(always)]
    pub fn aeslpen(&self) -> AESLPEN_R {
        AESLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - FSMC clock enable during Sleep mode
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPENR")
            .field("gpioalpen", &self.gpioalpen())
            .field("dma2lpen", &self.dma2lpen())
            .field("dma1lpen", &self.dma1lpen())
            .field("sramlpen", &self.sramlpen())
            .field("flitflpen", &self.flitflpen())
            .field("crclpen", &self.crclpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("fsmclpen", &self.fsmclpen())
            .field("aeslpen", &self.aeslpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<AHBLPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<AHBLPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<AHBLPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<AHBLPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<AHBLPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - IO port H clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<AHBLPENRrs> {
        GPIOHLPEN_W::new(self, 5)
    }
    ///Bit 6 - IO port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHBLPENRrs> {
        GPIOFLPEN_W::new(self, 6)
    }
    ///Bit 7 - IO port G clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<AHBLPENRrs> {
        GPIOGLPEN_W::new(self, 7)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<AHBLPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    ///Bit 15 - FLITF clock enable during Sleep mode
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<AHBLPENRrs> {
        FLITFLPEN_W::new(self, 15)
    }
    ///Bit 16 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramlpen(&mut self) -> SRAMLPEN_W<AHBLPENRrs> {
        SRAMLPEN_W::new(self, 16)
    }
    ///Bit 24 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<AHBLPENRrs> {
        DMA1LPEN_W::new(self, 24)
    }
    ///Bit 25 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<AHBLPENRrs> {
        DMA2LPEN_W::new(self, 25)
    }
    ///Bit 27 - AES clock enable during Sleep mode
    #[inline(always)]
    pub fn aeslpen(&mut self) -> AESLPEN_W<AHBLPENRrs> {
        AESLPEN_W::new(self, 27)
    }
    ///Bit 30 - FSMC clock enable during Sleep mode
    #[inline(always)]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W<AHBLPENRrs> {
        FSMCLPEN_W::new(self, 30)
    }
}
/**AHB peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`ahblpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:AHBLPENR)*/
pub struct AHBLPENRrs;
impl crate::RegisterSpec for AHBLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahblpenr::R`](R) reader structure
impl crate::Readable for AHBLPENRrs {}
///`write(|w| ..)` method takes [`ahblpenr::W`](W) writer structure
impl crate::Writable for AHBLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBLPENR to value 0x0101_903f
impl crate::Resettable for AHBLPENRrs {
    const RESET_VALUE: u32 = 0x0101_903f;
}
