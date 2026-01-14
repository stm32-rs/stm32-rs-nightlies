///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
/**DMA1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
///Field `DMA2EN` reader - DMA2 clock enable
pub use DMA1EN_R as DMA2EN_R;
///Field `SRAMEN` reader - SRAM interface clock enable
pub use DMA1EN_R as SRAMEN_R;
///Field `FLITFEN` reader - FLITF clock enable
pub use DMA1EN_R as FLITFEN_R;
///Field `FMCEN` reader - FMC clock enable
pub use DMA1EN_R as FMCEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use DMA1EN_R as CRCEN_R;
///Field `IOPHEN` reader - IO port H clock enable
pub use DMA1EN_R as IOPHEN_R;
///Field `IOPAEN` reader - I/O port A clock enable
pub use DMA1EN_R as IOPAEN_R;
///Field `IOPBEN` reader - I/O port B clock enable
pub use DMA1EN_R as IOPBEN_R;
///Field `IOPCEN` reader - I/O port C clock enable
pub use DMA1EN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable
pub use DMA1EN_R as IOPDEN_R;
///Field `IOPEEN` reader - I/O port E clock enable
pub use DMA1EN_R as IOPEEN_R;
///Field `IOPFEN` reader - I/O port F clock enable
pub use DMA1EN_R as IOPFEN_R;
///Field `IOPGEN` reader - IO port G clock enable
pub use DMA1EN_R as IOPGEN_R;
///Field `TSCEN` reader - Touch sensing controller clock enable
pub use DMA1EN_R as TSCEN_R;
///Field `ADC12EN` reader - ADC1 and ADC2 clock enable
pub use DMA1EN_R as ADC12EN_R;
///Field `DMA2EN` writer - DMA2 clock enable
pub use DMA1EN_W as DMA2EN_W;
///Field `SRAMEN` writer - SRAM interface clock enable
pub use DMA1EN_W as SRAMEN_W;
///Field `FLITFEN` writer - FLITF clock enable
pub use DMA1EN_W as FLITFEN_W;
///Field `FMCEN` writer - FMC clock enable
pub use DMA1EN_W as FMCEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use DMA1EN_W as CRCEN_W;
///Field `IOPHEN` writer - IO port H clock enable
pub use DMA1EN_W as IOPHEN_W;
///Field `IOPAEN` writer - I/O port A clock enable
pub use DMA1EN_W as IOPAEN_W;
///Field `IOPBEN` writer - I/O port B clock enable
pub use DMA1EN_W as IOPBEN_W;
///Field `IOPCEN` writer - I/O port C clock enable
pub use DMA1EN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable
pub use DMA1EN_W as IOPDEN_W;
///Field `IOPEEN` writer - I/O port E clock enable
pub use DMA1EN_W as IOPEEN_W;
///Field `IOPFEN` writer - I/O port F clock enable
pub use DMA1EN_W as IOPFEN_W;
///Field `IOPGEN` writer - IO port G clock enable
pub use DMA1EN_W as IOPGEN_W;
///Field `TSCEN` writer - Touch sensing controller clock enable
pub use DMA1EN_W as TSCEN_W;
///Field `ADC12EN` writer - ADC1 and ADC2 clock enable
pub use DMA1EN_W as ADC12EN_W;
impl R {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM interface clock enable
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FMC clock enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - IO port H clock enable
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I/O port E clock enable
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - IO port G clock enable
    #[inline(always)]
    pub fn iopgen(&self) -> IOPGEN_R {
        IOPGEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller clock enable
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ADC1 and ADC2 clock enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("sramen", &self.sramen())
            .field("flitfen", &self.flitfen())
            .field("crcen", &self.crcen())
            .field("iopaen", &self.iopaen())
            .field("iopben", &self.iopben())
            .field("iopcen", &self.iopcen())
            .field("iopden", &self.iopden())
            .field("iopeen", &self.iopeen())
            .field("iopfen", &self.iopfen())
            .field("tscen", &self.tscen())
            .field("adc12en", &self.adc12en())
            .field("fmcen", &self.fmcen())
            .field("iophen", &self.iophen())
            .field("iopgen", &self.iopgen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHBENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 2 - SRAM interface clock enable
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W<'_, AHBENRrs> {
        SRAMEN_W::new(self, 2)
    }
    ///Bit 4 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W<'_, AHBENRrs> {
        FLITFEN_W::new(self, 4)
    }
    ///Bit 5 - FMC clock enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHBENRrs> {
        FMCEN_W::new(self, 5)
    }
    ///Bit 6 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHBENRrs> {
        CRCEN_W::new(self, 6)
    }
    ///Bit 16 - IO port H clock enable
    #[inline(always)]
    pub fn iophen(&mut self) -> IOPHEN_W<'_, AHBENRrs> {
        IOPHEN_W::new(self, 16)
    }
    ///Bit 17 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<'_, AHBENRrs> {
        IOPAEN_W::new(self, 17)
    }
    ///Bit 18 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<'_, AHBENRrs> {
        IOPBEN_W::new(self, 18)
    }
    ///Bit 19 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<'_, AHBENRrs> {
        IOPCEN_W::new(self, 19)
    }
    ///Bit 20 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<'_, AHBENRrs> {
        IOPDEN_W::new(self, 20)
    }
    ///Bit 21 - I/O port E clock enable
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W<'_, AHBENRrs> {
        IOPEEN_W::new(self, 21)
    }
    ///Bit 22 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&mut self) -> IOPFEN_W<'_, AHBENRrs> {
        IOPFEN_W::new(self, 22)
    }
    ///Bit 23 - IO port G clock enable
    #[inline(always)]
    pub fn iopgen(&mut self) -> IOPGEN_W<'_, AHBENRrs> {
        IOPGEN_W::new(self, 23)
    }
    ///Bit 24 - Touch sensing controller clock enable
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<'_, AHBENRrs> {
        TSCEN_W::new(self, 24)
    }
    ///Bit 28 - ADC1 and ADC2 clock enable
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<'_, AHBENRrs> {
        ADC12EN_W::new(self, 28)
    }
}
/**AHB Peripheral Clock enable register (RCC_AHBENR)

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#RCC:AHBENR)*/
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
///`reset()` method sets AHBENR to value 0x14
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x14;
}
