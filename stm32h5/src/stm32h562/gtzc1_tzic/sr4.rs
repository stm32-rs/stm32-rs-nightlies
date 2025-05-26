///Register `SR4` reader
pub type R = crate::R<SR4rs>;
///Field `GPDMA1F` reader - illegal access flag for GPDMA1
pub type GPDMA1F_R = crate::BitReader;
///Field `GPDMA2F` reader - illegal access flag for GPDMA2
pub type GPDMA2F_R = crate::BitReader;
///Field `FLASH_REGF` reader - illegal access flag for FLASH registers
pub type FLASH_REGF_R = crate::BitReader;
///Field `FLASHF` reader - illegal access flag for FLASH memory
pub type FLASHF_R = crate::BitReader;
///Field `SBSF` reader - illegal access flag for SBS
pub type SBSF_R = crate::BitReader;
///Field `RTCF` reader - illegal access flag for RTC
pub type RTCF_R = crate::BitReader;
///Field `TAMPF` reader - illegal access flag for TAMP
pub type TAMPF_R = crate::BitReader;
///Field `PWRF` reader - illegal access flag for PWR
pub type PWRF_R = crate::BitReader;
///Field `RCCF` reader - illegal access flag for RCC
pub type RCCF_R = crate::BitReader;
///Field `EXTIF` reader - illegal access flag for EXTI
pub type EXTIF_R = crate::BitReader;
///Field `TZSC1F` reader - illegal access flag for GTZC1 TZSC registers
pub type TZSC1F_R = crate::BitReader;
///Field `TZIC1F` reader - illegal access flag for GTZC1 TZIC registers
pub type TZIC1F_R = crate::BitReader;
///Field `OCTOSPI1_MEMF` reader - illegal access flag for MPCWM1 (OCTOSPI1) memory bank
pub type OCTOSPI1_MEMF_R = crate::BitReader;
///Field `FMC_MEMF` reader - illegal access flag for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAMbank 2)
pub type FMC_MEMF_R = crate::BitReader;
///Field `BKPSRAMF` reader - illegal access flag for MPCWM4 (BKPSRAM) memory bank
pub type BKPSRAMF_R = crate::BitReader;
///Field `SRAM1F` reader - illegal access flag for SRAM1
pub type SRAM1F_R = crate::BitReader;
///Field `MPCBB1_REGF` reader - illegal access flag for MPCBB1 registers
pub type MPCBB1_REGF_R = crate::BitReader;
///Field `SRAM2F` reader - illegal access flag for SRAM2
pub type SRAM2F_R = crate::BitReader;
///Field `MPCBB2_REGF` reader - illegal access flag for MPCBB2 registers
pub type MPCBB2_REGF_R = crate::BitReader;
///Field `SRAM3F` reader - illegal access flag for SRAM3
pub type SRAM3F_R = crate::BitReader;
///Field `MPCBB3_REGF` reader - illegal access flag for MPCBB3 registers
pub type MPCBB3_REGF_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for GPDMA1
    #[inline(always)]
    pub fn gpdma1f(&self) -> GPDMA1F_R {
        GPDMA1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for GPDMA2
    #[inline(always)]
    pub fn gpdma2f(&self) -> GPDMA2F_R {
        GPDMA2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for FLASH registers
    #[inline(always)]
    pub fn flash_regf(&self) -> FLASH_REGF_R {
        FLASH_REGF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for FLASH memory
    #[inline(always)]
    pub fn flashf(&self) -> FLASHF_R {
        FLASHF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for SBS
    #[inline(always)]
    pub fn sbsf(&self) -> SBSF_R {
        SBSF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for RTC
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for TAMP
    #[inline(always)]
    pub fn tampf(&self) -> TAMPF_R {
        TAMPF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for PWR
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for RCC
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for EXTI
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for GTZC1 TZSC registers
    #[inline(always)]
    pub fn tzsc1f(&self) -> TZSC1F_R {
        TZSC1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for GTZC1 TZIC registers
    #[inline(always)]
    pub fn tzic1f(&self) -> TZIC1F_R {
        TZIC1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for MPCWM1 (OCTOSPI1) memory bank
    #[inline(always)]
    pub fn octospi1_memf(&self) -> OCTOSPI1_MEMF_R {
        OCTOSPI1_MEMF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAMbank 2)
    #[inline(always)]
    pub fn fmc_memf(&self) -> FMC_MEMF_R {
        FMC_MEMF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access flag for MPCWM4 (BKPSRAM) memory bank
    #[inline(always)]
    pub fn bkpsramf(&self) -> BKPSRAMF_R {
        BKPSRAMF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SRAM1
    #[inline(always)]
    pub fn sram1f(&self) -> SRAM1F_R {
        SRAM1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for MPCBB1 registers
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for SRAM2
    #[inline(always)]
    pub fn sram2f(&self) -> SRAM2F_R {
        SRAM2F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access flag for MPCBB2 registers
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access flag for SRAM3
    #[inline(always)]
    pub fn sram3f(&self) -> SRAM3F_R {
        SRAM3F_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access flag for MPCBB3 registers
    #[inline(always)]
    pub fn mpcbb3_regf(&self) -> MPCBB3_REGF_R {
        MPCBB3_REGF_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR4")
            .field("gpdma1f", &self.gpdma1f())
            .field("gpdma2f", &self.gpdma2f())
            .field("flash_regf", &self.flash_regf())
            .field("flashf", &self.flashf())
            .field("sbsf", &self.sbsf())
            .field("rtcf", &self.rtcf())
            .field("tampf", &self.tampf())
            .field("pwrf", &self.pwrf())
            .field("rccf", &self.rccf())
            .field("extif", &self.extif())
            .field("tzsc1f", &self.tzsc1f())
            .field("tzic1f", &self.tzic1f())
            .field("octospi1_memf", &self.octospi1_memf())
            .field("fmc_memf", &self.fmc_memf())
            .field("bkpsramf", &self.bkpsramf())
            .field("sram1f", &self.sram1f())
            .field("mpcbb1_regf", &self.mpcbb1_regf())
            .field("sram2f", &self.sram2f())
            .field("mpcbb2_regf", &self.mpcbb2_regf())
            .field("sram3f", &self.sram3f())
            .field("mpcbb3_regf", &self.mpcbb3_regf())
            .finish()
    }
}
/**GTZC1 TZIC status register 4

You can [`read`](crate::Reg::read) this register and get [`sr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#GTZC1_TZIC:SR4)*/
pub struct SR4rs;
impl crate::RegisterSpec for SR4rs {
    type Ux = u32;
}
///`read()` method returns [`sr4::R`](R) reader structure
impl crate::Readable for SR4rs {}
///`reset()` method sets SR4 to value 0
impl crate::Resettable for SR4rs {}
