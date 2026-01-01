///Register `AHB1LPENR` reader
pub type R = crate::R<AHB1LPENRrs>;
///Register `AHB1LPENR` writer
pub type W = crate::W<AHB1LPENRrs>;
/**GPDMA1 clock enable during sleep mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<GPDMA1LPEN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1LPEN` reader - GPDMA1 clock enable during sleep mode Set and reset by software.
pub type GPDMA1LPEN_R = crate::BitReader<GPDMA1LPEN>;
impl GPDMA1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1LPEN {
        match self.bits {
            false => GPDMA1LPEN::Disabled,
            true => GPDMA1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1LPEN::Enabled
    }
}
///Field `GPDMA1LPEN` writer - GPDMA1 clock enable during sleep mode Set and reset by software.
pub type GPDMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1LPEN>;
impl<'a, REG> GPDMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN::Enabled)
    }
}
///Field `GPDMA2LPEN` reader - GPDMA2 clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_R as GPDMA2LPEN_R;
///Field `FLITFLPEN` reader - Flash interface (FLITF) clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_R as FLITFLPEN_R;
///Field `CRCLPEN` reader - CRC clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_R as CRCLPEN_R;
///Field `CORDICLPEN` reader - CORDIC clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_R as CORDICLPEN_R;
///Field `FMACLPEN` reader - FMAC clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_R as FMACLPEN_R;
///Field `RAMCFGLPEN` reader - RAMCFG clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_R as RAMCFGLPEN_R;
///Field `TZSC1LPEN` reader - TZSC1 clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_R as TZSC1LPEN_R;
///Field `BKPRAMLPEN` reader - BKPRAM clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_R as BKPRAMLPEN_R;
///Field `ICACHELPEN` reader - ICACHE clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_R as ICACHELPEN_R;
///Field `DCACHELPEN` reader - DCACHE clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_R as DCACHELPEN_R;
///Field `SRAM1LPEN` reader - SRAM1 clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_R as SRAM1LPEN_R;
///Field `GPDMA2LPEN` writer - GPDMA2 clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_W as GPDMA2LPEN_W;
///Field `FLITFLPEN` writer - Flash interface (FLITF) clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_W as FLITFLPEN_W;
///Field `CRCLPEN` writer - CRC clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_W as CRCLPEN_W;
///Field `CORDICLPEN` writer - CORDIC clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_W as CORDICLPEN_W;
///Field `FMACLPEN` writer - FMAC clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_W as FMACLPEN_W;
///Field `RAMCFGLPEN` writer - RAMCFG clock enable during sleep mode Set and reset by software.
pub use GPDMA1LPEN_W as RAMCFGLPEN_W;
///Field `TZSC1LPEN` writer - TZSC1 clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_W as TZSC1LPEN_W;
///Field `BKPRAMLPEN` writer - BKPRAM clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_W as BKPRAMLPEN_W;
///Field `ICACHELPEN` writer - ICACHE clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_W as ICACHELPEN_W;
///Field `DCACHELPEN` writer - DCACHE clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_W as DCACHELPEN_W;
///Field `SRAM1LPEN` writer - SRAM1 clock enable during sleep mode Set and reset by software
pub use GPDMA1LPEN_W as SRAM1LPEN_W;
impl R {
    ///Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpdma1lpen(&self) -> GPDMA1LPEN_R {
        GPDMA1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpdma2lpen(&self) -> GPDMA2LPEN_R {
        GPDMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CORDIC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn cordiclpen(&self) -> CORDICLPEN_R {
        CORDICLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - FMAC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn fmaclpen(&self) -> FMACLPEN_R {
        FMACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn ramcfglpen(&self) -> RAMCFGLPEN_R {
        RAMCFGLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - TZSC1 clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn tzsc1lpen(&self) -> TZSC1LPEN_R {
        TZSC1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ICACHE clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn icachelpen(&self) -> ICACHELPEN_R {
        ICACHELPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DCACHE clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn dcachelpen(&self) -> DCACHELPEN_R {
        DCACHELPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1LPENR")
            .field("gpdma1lpen", &self.gpdma1lpen())
            .field("gpdma2lpen", &self.gpdma2lpen())
            .field("flitflpen", &self.flitflpen())
            .field("crclpen", &self.crclpen())
            .field("cordiclpen", &self.cordiclpen())
            .field("fmaclpen", &self.fmaclpen())
            .field("ramcfglpen", &self.ramcfglpen())
            .field("tzsc1lpen", &self.tzsc1lpen())
            .field("bkpramlpen", &self.bkpramlpen())
            .field("icachelpen", &self.icachelpen())
            .field("dcachelpen", &self.dcachelpen())
            .field("sram1lpen", &self.sram1lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpdma1lpen(&mut self) -> GPDMA1LPEN_W<'_, AHB1LPENRrs> {
        GPDMA1LPEN_W::new(self, 0)
    }
    ///Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn gpdma2lpen(&mut self) -> GPDMA2LPEN_W<'_, AHB1LPENRrs> {
        GPDMA2LPEN_W::new(self, 1)
    }
    ///Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<'_, AHB1LPENRrs> {
        FLITFLPEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<'_, AHB1LPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    ///Bit 14 - CORDIC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn cordiclpen(&mut self) -> CORDICLPEN_W<'_, AHB1LPENRrs> {
        CORDICLPEN_W::new(self, 14)
    }
    ///Bit 15 - FMAC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn fmaclpen(&mut self) -> FMACLPEN_W<'_, AHB1LPENRrs> {
        FMACLPEN_W::new(self, 15)
    }
    ///Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn ramcfglpen(&mut self) -> RAMCFGLPEN_W<'_, AHB1LPENRrs> {
        RAMCFGLPEN_W::new(self, 17)
    }
    ///Bit 24 - TZSC1 clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn tzsc1lpen(&mut self) -> TZSC1LPEN_W<'_, AHB1LPENRrs> {
        TZSC1LPEN_W::new(self, 24)
    }
    ///Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<'_, AHB1LPENRrs> {
        BKPRAMLPEN_W::new(self, 28)
    }
    ///Bit 29 - ICACHE clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn icachelpen(&mut self) -> ICACHELPEN_W<'_, AHB1LPENRrs> {
        ICACHELPEN_W::new(self, 29)
    }
    ///Bit 30 - DCACHE clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn dcachelpen(&mut self) -> DCACHELPEN_W<'_, AHB1LPENRrs> {
        DCACHELPEN_W::new(self, 30)
    }
    ///Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<'_, AHB1LPENRrs> {
        SRAM1LPEN_W::new(self, 31)
    }
}
/**RCC AHB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RCC:AHB1LPENR)*/
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
///`reset()` method sets AHB1LPENR to value 0xf13a_d103
impl crate::Resettable for AHB1LPENRrs {
    const RESET_VALUE: u32 = 0xf13a_d103;
}
