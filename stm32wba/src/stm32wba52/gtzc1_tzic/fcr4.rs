///Register `FCR4` writer
pub type W = crate::W<FCR4rs>;
///Field `CGPDMA1F` writer - clear the illegal access flag for GPDMA1
pub type CGPDMA1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFLASHF` writer - clear the illegal access flag for FLASH memory
pub type CFLASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFLASH_REGF` writer - clear the illegal access flag for FLASH interface
pub type CFLASH_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSYSCFGF` writer - clear the illegal access flag for SYSCFG
pub type CSYSCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRTCF` writer - clear the illegal access flag for RTC
pub type CRTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMPF` writer - clear the illegal access flag for TAMP
pub type CTAMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPWRF` writer - clear the illegal access flag for PWR
pub type CPWRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCCF` writer - clear the illegal access flag for RCC
pub type CRCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEXTIF` writer - clear the illegal access flag for EXTI
pub type CEXTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTZSCF` writer - clear the illegal access flag for GTZC1 TZSC
pub type CTZSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTZICF` writer - clear the illegal access flag for GTZC1 TZIC
pub type CTZICF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM1F` writer - clear the illegal access flag for SRAM1 memory
pub type CSRAM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB1F` writer - clear the illegal access flag for MPCBB1
pub type CMPCBB1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM2F` writer - clear the illegal access flag for SRAM2 memory
pub type CSRAM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB2F` writer - clear the illegal access flag for MPCBB2
pub type CMPCBB2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRAM6F` writer - clear the illegal access flag for 2.4 GHz RADIO RXTXRAM memory
pub type CSRAM6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPCBB6F` writer - clear the illegal access flag for MPCBB6
pub type CMPCBB6F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for GPDMA1
    #[inline(always)]
    pub fn cgpdma1f(&mut self) -> CGPDMA1F_W<'_, FCR4rs> {
        CGPDMA1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for FLASH memory
    #[inline(always)]
    pub fn cflashf(&mut self) -> CFLASHF_W<'_, FCR4rs> {
        CFLASHF_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for FLASH interface
    #[inline(always)]
    pub fn cflash_regf(&mut self) -> CFLASH_REGF_W<'_, FCR4rs> {
        CFLASH_REGF_W::new(self, 2)
    }
    ///Bit 7 - clear the illegal access flag for SYSCFG
    #[inline(always)]
    pub fn csyscfgf(&mut self) -> CSYSCFGF_W<'_, FCR4rs> {
        CSYSCFGF_W::new(self, 7)
    }
    ///Bit 8 - clear the illegal access flag for RTC
    #[inline(always)]
    pub fn crtcf(&mut self) -> CRTCF_W<'_, FCR4rs> {
        CRTCF_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for TAMP
    #[inline(always)]
    pub fn ctampf(&mut self) -> CTAMPF_W<'_, FCR4rs> {
        CTAMPF_W::new(self, 9)
    }
    ///Bit 10 - clear the illegal access flag for PWR
    #[inline(always)]
    pub fn cpwrf(&mut self) -> CPWRF_W<'_, FCR4rs> {
        CPWRF_W::new(self, 10)
    }
    ///Bit 11 - clear the illegal access flag for RCC
    #[inline(always)]
    pub fn crccf(&mut self) -> CRCCF_W<'_, FCR4rs> {
        CRCCF_W::new(self, 11)
    }
    ///Bit 13 - clear the illegal access flag for EXTI
    #[inline(always)]
    pub fn cextif(&mut self) -> CEXTIF_W<'_, FCR4rs> {
        CEXTIF_W::new(self, 13)
    }
    ///Bit 14 - clear the illegal access flag for GTZC1 TZSC
    #[inline(always)]
    pub fn ctzscf(&mut self) -> CTZSCF_W<'_, FCR4rs> {
        CTZSCF_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for GTZC1 TZIC
    #[inline(always)]
    pub fn ctzicf(&mut self) -> CTZICF_W<'_, FCR4rs> {
        CTZICF_W::new(self, 15)
    }
    ///Bit 22 - clear the illegal access flag for SRAM1 memory
    #[inline(always)]
    pub fn csram1f(&mut self) -> CSRAM1F_W<'_, FCR4rs> {
        CSRAM1F_W::new(self, 22)
    }
    ///Bit 23 - clear the illegal access flag for MPCBB1
    #[inline(always)]
    pub fn cmpcbb1f(&mut self) -> CMPCBB1F_W<'_, FCR4rs> {
        CMPCBB1F_W::new(self, 23)
    }
    ///Bit 24 - clear the illegal access flag for SRAM2 memory
    #[inline(always)]
    pub fn csram2f(&mut self) -> CSRAM2F_W<'_, FCR4rs> {
        CSRAM2F_W::new(self, 24)
    }
    ///Bit 25 - clear the illegal access flag for MPCBB2
    #[inline(always)]
    pub fn cmpcbb2f(&mut self) -> CMPCBB2F_W<'_, FCR4rs> {
        CMPCBB2F_W::new(self, 25)
    }
    ///Bit 30 - clear the illegal access flag for 2.4 GHz RADIO RXTXRAM memory
    #[inline(always)]
    pub fn csram6f(&mut self) -> CSRAM6F_W<'_, FCR4rs> {
        CSRAM6F_W::new(self, 30)
    }
    ///Bit 31 - clear the illegal access flag for MPCBB6
    #[inline(always)]
    pub fn cmpcbb6f(&mut self) -> CMPCBB6F_W<'_, FCR4rs> {
        CMPCBB6F_W::new(self, 31)
    }
}
/**GTZC1 TZIC flag clear register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:FCR4)*/
pub struct FCR4rs;
impl crate::RegisterSpec for FCR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr4::W`](W) writer structure
impl crate::Writable for FCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR4 to value 0
impl crate::Resettable for FCR4rs {}
