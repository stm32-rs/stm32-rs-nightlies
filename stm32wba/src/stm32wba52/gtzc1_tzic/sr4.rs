///Register `SR4` reader
pub type R = crate::R<SR4rs>;
///Field `GPDMA1F` reader - illegal access flag for GPDMA1
pub type GPDMA1F_R = crate::BitReader;
///Field `FLASHF` reader - illegal access flag for FLASH memory
pub type FLASHF_R = crate::BitReader;
///Field `FLASH_REGF` reader - illegal access flag for FLASH interface
pub type FLASH_REGF_R = crate::BitReader;
///Field `SYSCFGF` reader - illegal access flag for SYSCFG
pub type SYSCFGF_R = crate::BitReader;
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
///Field `TZSCF` reader - illegal access flag for GTZC1 TZSC
pub type TZSCF_R = crate::BitReader;
///Field `TZICF` reader - illegal access flag for GTZC1 TZIC
pub type TZICF_R = crate::BitReader;
///Field `SRAM1F` reader - illegal access flag for SRAM1 memory
pub type SRAM1F_R = crate::BitReader;
///Field `MPCBB1F` reader - illegal access flag for MPCBB1
pub type MPCBB1F_R = crate::BitReader;
///Field `SRAM2F` reader - illegal access flag for SRAM2 memory
pub type SRAM2F_R = crate::BitReader;
///Field `MPCBB2F` reader - illegal access flag for MPCBB2
pub type MPCBB2F_R = crate::BitReader;
///Field `SRAM6F` reader - illegal access flag for 2.4 GHZ RADIO RXTXRAM memory
pub type SRAM6F_R = crate::BitReader;
///Field `MPCBB6F` reader - illegal access flag for MPCBB6
pub type MPCBB6F_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for GPDMA1
    #[inline(always)]
    pub fn gpdma1f(&self) -> GPDMA1F_R {
        GPDMA1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for FLASH memory
    #[inline(always)]
    pub fn flashf(&self) -> FLASHF_R {
        FLASHF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for FLASH interface
    #[inline(always)]
    pub fn flash_regf(&self) -> FLASH_REGF_R {
        FLASH_REGF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for SYSCFG
    #[inline(always)]
    pub fn syscfgf(&self) -> SYSCFGF_R {
        SYSCFGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for RTC
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for TAMP
    #[inline(always)]
    pub fn tampf(&self) -> TAMPF_R {
        TAMPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for PWR
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for RCC
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for EXTI
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for GTZC1 TZSC
    #[inline(always)]
    pub fn tzscf(&self) -> TZSCF_R {
        TZSCF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for GTZC1 TZIC
    #[inline(always)]
    pub fn tzicf(&self) -> TZICF_R {
        TZICF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for SRAM1 memory
    #[inline(always)]
    pub fn sram1f(&self) -> SRAM1F_R {
        SRAM1F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access flag for MPCBB1
    #[inline(always)]
    pub fn mpcbb1f(&self) -> MPCBB1F_R {
        MPCBB1F_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SRAM2 memory
    #[inline(always)]
    pub fn sram2f(&self) -> SRAM2F_R {
        SRAM2F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for MPCBB2
    #[inline(always)]
    pub fn mpcbb2f(&self) -> MPCBB2F_R {
        MPCBB2F_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - illegal access flag for 2.4 GHZ RADIO RXTXRAM memory
    #[inline(always)]
    pub fn sram6f(&self) -> SRAM6F_R {
        SRAM6F_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access flag for MPCBB6
    #[inline(always)]
    pub fn mpcbb6f(&self) -> MPCBB6F_R {
        MPCBB6F_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR4")
            .field("gpdma1f", &self.gpdma1f())
            .field("flashf", &self.flashf())
            .field("flash_regf", &self.flash_regf())
            .field("syscfgf", &self.syscfgf())
            .field("rtcf", &self.rtcf())
            .field("tampf", &self.tampf())
            .field("pwrf", &self.pwrf())
            .field("rccf", &self.rccf())
            .field("extif", &self.extif())
            .field("tzscf", &self.tzscf())
            .field("tzicf", &self.tzicf())
            .field("sram1f", &self.sram1f())
            .field("mpcbb1f", &self.mpcbb1f())
            .field("sram2f", &self.sram2f())
            .field("mpcbb2f", &self.mpcbb2f())
            .field("sram6f", &self.sram6f())
            .field("mpcbb6f", &self.mpcbb6f())
            .finish()
    }
}
/**GTZC1 TZIC status register 4

You can [`read`](crate::Reg::read) this register and get [`sr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:SR4)*/
pub struct SR4rs;
impl crate::RegisterSpec for SR4rs {
    type Ux = u32;
}
///`read()` method returns [`sr4::R`](R) reader structure
impl crate::Readable for SR4rs {}
///`reset()` method sets SR4 to value 0
impl crate::Resettable for SR4rs {}
