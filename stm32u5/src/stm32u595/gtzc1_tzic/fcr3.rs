///Register `FCR3` writer
pub type W = crate::W<FCR3rs>;
///Field `CMDF1F` writer - clear the illegal access flag for MDF1
pub type CMDF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCORDICF` writer - clear the illegal access flag for CORDIC
pub type CCORDICF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFMACF` writer - clear the illegal access flag for FMAC
pub type CFMACF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRCF` writer - clear the illegal access flag for CRC
pub type CCRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - clear the illegal access flag for TSC
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMA2DF` writer - clear the illegal access flag for register of DMA2D
pub type CDMA2DF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CICACHE_REGF` writer - clear the illegal access flag for ICACHE registers
pub type CICACHE_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDCACHE1_REGF` writer - clear the illegal access flag for DCACHE1 registers
pub type CDCACHE1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CADC12F` writer - clear the illegal access flag for ADC1 and ADC2
pub type CADC12F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDCMIF` writer - clear the illegal access flag for DCMI
pub type CDCMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COTGF` writer - clear the illegal access flag for OTG_FS
pub type COTGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHASHF` writer - clear the illegal access flag for HASH
pub type CHASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRNGF` writer - clear the illegal access flag for RNG
pub type CRNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPIMF` writer - clear the illegal access flag for OCTOSPIM
pub type COCTOSPIMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSDMMC1F` writer - clear the illegal access flag for SDMMC2
pub type CSDMMC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSDMMC2F` writer - clear the illegal access flag for SDMMC1
pub type CSDMMC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFSMC_REGF` writer - clear the illegal access flag for FSMC registers
pub type CFSMC_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI1_REGF` writer - clear the illegal access flag for OCTOSPI1 registers
pub type COCTOSPI1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI2_REGF` writer - clear the illegal access flag for OCTOSPI2 registers
pub type COCTOSPI2_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRAMCFGF` writer - clear the illegal access flag for RAMCFG
pub type CRAMCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for MDF1
    #[inline(always)]
    pub fn cmdf1f(&mut self) -> CMDF1F_W<FCR3rs> {
        CMDF1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for CORDIC
    #[inline(always)]
    pub fn ccordicf(&mut self) -> CCORDICF_W<FCR3rs> {
        CCORDICF_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for FMAC
    #[inline(always)]
    pub fn cfmacf(&mut self) -> CFMACF_W<FCR3rs> {
        CFMACF_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for CRC
    #[inline(always)]
    pub fn ccrcf(&mut self) -> CCRCF_W<FCR3rs> {
        CCRCF_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for TSC
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<FCR3rs> {
        CTSCF_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for register of DMA2D
    #[inline(always)]
    pub fn cdma2df(&mut self) -> CDMA2DF_W<FCR3rs> {
        CDMA2DF_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for ICACHE registers
    #[inline(always)]
    pub fn cicache_regf(&mut self) -> CICACHE_REGF_W<FCR3rs> {
        CICACHE_REGF_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for DCACHE1 registers
    #[inline(always)]
    pub fn cdcache1_regf(&mut self) -> CDCACHE1_REGF_W<FCR3rs> {
        CDCACHE1_REGF_W::new(self, 7)
    }
    ///Bit 8 - clear the illegal access flag for ADC1 and ADC2
    #[inline(always)]
    pub fn cadc12f(&mut self) -> CADC12F_W<FCR3rs> {
        CADC12F_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for DCMI
    #[inline(always)]
    pub fn cdcmif(&mut self) -> CDCMIF_W<FCR3rs> {
        CDCMIF_W::new(self, 9)
    }
    ///Bit 10 - clear the illegal access flag for OTG_FS
    #[inline(always)]
    pub fn cotgf(&mut self) -> COTGF_W<FCR3rs> {
        COTGF_W::new(self, 10)
    }
    ///Bit 12 - clear the illegal access flag for HASH
    #[inline(always)]
    pub fn chashf(&mut self) -> CHASHF_W<FCR3rs> {
        CHASHF_W::new(self, 12)
    }
    ///Bit 13 - clear the illegal access flag for RNG
    #[inline(always)]
    pub fn crngf(&mut self) -> CRNGF_W<FCR3rs> {
        CRNGF_W::new(self, 13)
    }
    ///Bit 16 - clear the illegal access flag for OCTOSPIM
    #[inline(always)]
    pub fn coctospimf(&mut self) -> COCTOSPIMF_W<FCR3rs> {
        COCTOSPIMF_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for SDMMC2
    #[inline(always)]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<FCR3rs> {
        CSDMMC1F_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for SDMMC1
    #[inline(always)]
    pub fn csdmmc2f(&mut self) -> CSDMMC2F_W<FCR3rs> {
        CSDMMC2F_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for FSMC registers
    #[inline(always)]
    pub fn cfsmc_regf(&mut self) -> CFSMC_REGF_W<FCR3rs> {
        CFSMC_REGF_W::new(self, 19)
    }
    ///Bit 20 - clear the illegal access flag for OCTOSPI1 registers
    #[inline(always)]
    pub fn coctospi1_regf(&mut self) -> COCTOSPI1_REGF_W<FCR3rs> {
        COCTOSPI1_REGF_W::new(self, 20)
    }
    ///Bit 21 - clear the illegal access flag for OCTOSPI2 registers
    #[inline(always)]
    pub fn coctospi2_regf(&mut self) -> COCTOSPI2_REGF_W<FCR3rs> {
        COCTOSPI2_REGF_W::new(self, 21)
    }
    ///Bit 22 - clear the illegal access flag for RAMCFG
    #[inline(always)]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<FCR3rs> {
        CRAMCFGF_W::new(self, 22)
    }
}
/**TZIC flag clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GTZC1_TZIC:FCR3)*/
pub struct FCR3rs;
impl crate::RegisterSpec for FCR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr3::W`](W) writer structure
impl crate::Writable for FCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FCR3 to value 0
impl crate::Resettable for FCR3rs {
    const RESET_VALUE: u32 = 0;
}