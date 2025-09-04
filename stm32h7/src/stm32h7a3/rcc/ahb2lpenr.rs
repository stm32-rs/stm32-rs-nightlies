///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSILPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<DCMI_PSSILPEN> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSILPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMI_PSSILPEN` reader - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSILPEN_R = crate::BitReader<DCMI_PSSILPEN>;
impl DCMI_PSSILPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMI_PSSILPEN {
        match self.bits {
            false => DCMI_PSSILPEN::Disabled,
            true => DCMI_PSSILPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMI_PSSILPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMI_PSSILPEN::Enabled
    }
}
///Field `DCMI_PSSILPEN` writer - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSILPEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMI_PSSILPEN>;
impl<'a, REG> DCMI_PSSILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSILPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSILPEN::Enabled)
    }
}
///Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
pub use DCMI_PSSILPEN_R as RNGLPEN_R;
///Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as SDMMC2LPEN_R;
///Field `DFSDMDMALPEN` reader - DFSDMDMA clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as DFSDMDMALPEN_R;
///Field `AHBSRAM1LPEN` reader - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as AHBSRAM1LPEN_R;
///Field `AHBSRAM2LPEN` reader - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as AHBSRAM2LPEN_R;
///Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
pub use DCMI_PSSILPEN_W as RNGLPEN_W;
///Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as SDMMC2LPEN_W;
///Field `DFSDMDMALPEN` writer - DFSDMDMA clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as DFSDMDMALPEN_W;
///Field `AHBSRAM1LPEN` writer - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as AHBSRAM1LPEN_W;
///Field `AHBSRAM2LPEN` writer - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as AHBSRAM2LPEN_W;
impl R {
    ///Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssilpen(&self) -> DCMI_PSSILPEN_R {
        DCMI_PSSILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dfsdmdmalpen(&self) -> DFSDMDMALPEN_R {
        DFSDMDMALPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn ahbsram1lpen(&self) -> AHBSRAM1LPEN_R {
        AHBSRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn ahbsram2lpen(&self) -> AHBSRAM2LPEN_R {
        AHBSRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2LPENR")
            .field("dcmi_pssilpen", &self.dcmi_pssilpen())
            .field("rnglpen", &self.rnglpen())
            .field("sdmmc2lpen", &self.sdmmc2lpen())
            .field("dfsdmdmalpen", &self.dfsdmdmalpen())
            .field("ahbsram1lpen", &self.ahbsram1lpen())
            .field("ahbsram2lpen", &self.ahbsram2lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssilpen(&mut self) -> DCMI_PSSILPEN_W<AHB2LPENRrs> {
        DCMI_PSSILPEN_W::new(self, 0)
    }
    ///Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<AHB2LPENRrs> {
        RNGLPEN_W::new(self, 6)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<AHB2LPENRrs> {
        SDMMC2LPEN_W::new(self, 9)
    }
    ///Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dfsdmdmalpen(&mut self) -> DFSDMDMALPEN_W<AHB2LPENRrs> {
        DFSDMDMALPEN_W::new(self, 11)
    }
    ///Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn ahbsram1lpen(&mut self) -> AHBSRAM1LPEN_W<AHB2LPENRrs> {
        AHBSRAM1LPEN_W::new(self, 29)
    }
    ///Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn ahbsram2lpen(&mut self) -> AHBSRAM2LPEN_W<AHB2LPENRrs> {
        AHBSRAM2LPEN_W::new(self, 30)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#RCC:AHB2LPENR)*/
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2lpenr::R`](R) reader structure
impl crate::Readable for AHB2LPENRrs {}
///`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENR to value 0x6000_0a71
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0x6000_0a71;
}
