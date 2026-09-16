///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSIEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DCMI_PSSIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMI_PSSIEN` reader - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSIEN_R = crate::BitReader<DCMI_PSSIEN>;
impl DCMI_PSSIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMI_PSSIEN {
        match self.bits {
            false => DCMI_PSSIEN::Disabled,
            true => DCMI_PSSIEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMI_PSSIEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMI_PSSIEN::Enabled
    }
}
///Field `DCMI_PSSIEN` writer - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMI_PSSIEN>;
impl<'a, REG> DCMI_PSSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSIEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSIEN::Enabled)
    }
}
///Field `HSEMEN` reader - HSEM peripheral clock enable Set and reset by software.
pub use DCMI_PSSIEN_R as HSEMEN_R;
///Field `CRYPTEN` reader - CRYPT peripheral clock enable Set and reset by software.
pub use DCMI_PSSIEN_R as CRYPTEN_R;
///Field `HASHEN` reader - HASH peripheral clock enable Set and reset by software.
pub use DCMI_PSSIEN_R as HASHEN_R;
///Field `RNGEN` reader - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
pub use DCMI_PSSIEN_R as RNGEN_R;
///Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable Set and reset by software.
pub use DCMI_PSSIEN_R as SDMMC2EN_R;
///Field `BDMA1EN` reader - DMA clock enable (DFSDM dedicated DMA) Set and reset by software.
pub use DCMI_PSSIEN_R as BDMA1EN_R;
///Field `AHBSRAM1EN` reader - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
pub use DCMI_PSSIEN_R as AHBSRAM1EN_R;
///Field `AHBSRAM2EN` reader - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
pub use DCMI_PSSIEN_R as AHBSRAM2EN_R;
///Field `HSEMEN` writer - HSEM peripheral clock enable Set and reset by software.
pub use DCMI_PSSIEN_W as HSEMEN_W;
///Field `CRYPTEN` writer - CRYPT peripheral clock enable Set and reset by software.
pub use DCMI_PSSIEN_W as CRYPTEN_W;
///Field `HASHEN` writer - HASH peripheral clock enable Set and reset by software.
pub use DCMI_PSSIEN_W as HASHEN_W;
///Field `RNGEN` writer - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
pub use DCMI_PSSIEN_W as RNGEN_W;
///Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable Set and reset by software.
pub use DCMI_PSSIEN_W as SDMMC2EN_W;
///Field `BDMA1EN` writer - DMA clock enable (DFSDM dedicated DMA) Set and reset by software.
pub use DCMI_PSSIEN_W as BDMA1EN_W;
///Field `AHBSRAM1EN` writer - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
pub use DCMI_PSSIEN_W as AHBSRAM1EN_W;
///Field `AHBSRAM2EN` writer - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
pub use DCMI_PSSIEN_W as AHBSRAM2EN_W;
impl R {
    ///Bit 0 - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - HSEM peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CRYPT peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - DMA clock enable (DFSDM dedicated DMA) Set and reset by software.
    #[inline(always)]
    pub fn bdma1en(&self) -> BDMA1EN_R {
        BDMA1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 29 - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
    #[inline(always)]
    pub fn ahbsram1en(&self) -> AHBSRAM1EN_R {
        AHBSRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
    #[inline(always)]
    pub fn ahbsram2en(&self) -> AHBSRAM2EN_R {
        AHBSRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("dcmi_pssien", &self.dcmi_pssien())
            .field("hsemen", &self.hsemen())
            .field("crypten", &self.crypten())
            .field("hashen", &self.hashen())
            .field("rngen", &self.rngen())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("bdma1en", &self.bdma1en())
            .field("ahbsram1en", &self.ahbsram1en())
            .field("ahbsram2en", &self.ahbsram2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<'_, AHB2ENRrs> {
        DCMI_PSSIEN_W::new(self, 0)
    }
    ///Bit 2 - HSEM peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W<'_, AHB2ENRrs> {
        HSEMEN_W::new(self, 2)
    }
    ///Bit 4 - CRYPT peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn crypten(&mut self) -> CRYPTEN_W<'_, AHB2ENRrs> {
        CRYPTEN_W::new(self, 4)
    }
    ///Bit 5 - HASH peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB2ENRrs> {
        HASHEN_W::new(self, 5)
    }
    ///Bit 6 - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 6)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<'_, AHB2ENRrs> {
        SDMMC2EN_W::new(self, 9)
    }
    ///Bit 11 - DMA clock enable (DFSDM dedicated DMA) Set and reset by software.
    #[inline(always)]
    pub fn bdma1en(&mut self) -> BDMA1EN_W<'_, AHB2ENRrs> {
        BDMA1EN_W::new(self, 11)
    }
    ///Bit 29 - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
    #[inline(always)]
    pub fn ahbsram1en(&mut self) -> AHBSRAM1EN_W<'_, AHB2ENRrs> {
        AHBSRAM1EN_W::new(self, 29)
    }
    ///Bit 30 - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun.
    #[inline(always)]
    pub fn ahbsram2en(&mut self) -> AHBSRAM2EN_W<'_, AHB2ENRrs> {
        AHBSRAM2EN_W::new(self, 30)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
