///Register `FCR3` writer
pub type W = crate::W<FCR3rs>;
///Field `CLPTIM6F` writer - clear illegal access flag for LPTIM6
pub type CLPTIM6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CVREFBUFF` writer - clear illegal access flag for VREFBUF
pub type CVREFBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI3C2F` writer - clear illegal access flag for I3C2
pub type CI3C2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRCF` writer - clear illegal access flag for CRC
pub type CCRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCORDICF` writer - clear illegal access flag for CORDIC
pub type CCORDICF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFMACF` writer - clear illegal access flag for FMAC
pub type CFMACF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CETHF` writer - clear illegal access flag for register of ETH
pub type CETHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CICACHEF` writer - clear illegal access flag for ICACHE
pub type CICACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDCACHEF` writer - clear illegal access flag for DCACHE
pub type CDCACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CADC12F` writer - clear illegal access flag for ADC1 and ADC2
pub type CADC12F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDCMIF` writer - clear illegal access flag for DCMI
pub type CDCMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAESF` writer - clear illegal access flag for AES
pub type CAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHASHF` writer - clear illegal access flag for HASH
pub type CHASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRNGF` writer - clear illegal access flag for RNG
pub type CRNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSAESF` writer - clear illegal access flag for SAES
pub type CSAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPKAF` writer - clear illegal access flag for PKA
pub type CPKAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSDMMC1F` writer - clear illegal access flag for SDMMC1
pub type CSDMMC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSDMMC2F` writer - clear illegal access flag for SDMMC2
pub type CSDMMC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFMCF` writer - clear illegal access flag for FMC
pub type CFMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCTOSPI1F` writer - clear illegal access flag for OCTOSPI1
pub type COCTOSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRAMCFGF` writer - clear illegal access flag for RAMSCFG
pub type CRAMCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear illegal access flag for LPTIM6
    #[inline(always)]
    pub fn clptim6f(&mut self) -> CLPTIM6F_W<'_, FCR3rs> {
        CLPTIM6F_W::new(self, 0)
    }
    ///Bit 1 - clear illegal access flag for VREFBUF
    #[inline(always)]
    pub fn cvrefbuff(&mut self) -> CVREFBUFF_W<'_, FCR3rs> {
        CVREFBUFF_W::new(self, 1)
    }
    ///Bit 2 - clear illegal access flag for I3C2
    #[inline(always)]
    pub fn ci3c2f(&mut self) -> CI3C2F_W<'_, FCR3rs> {
        CI3C2F_W::new(self, 2)
    }
    ///Bit 8 - clear illegal access flag for CRC
    #[inline(always)]
    pub fn ccrcf(&mut self) -> CCRCF_W<'_, FCR3rs> {
        CCRCF_W::new(self, 8)
    }
    ///Bit 9 - clear illegal access flag for CORDIC
    #[inline(always)]
    pub fn ccordicf(&mut self) -> CCORDICF_W<'_, FCR3rs> {
        CCORDICF_W::new(self, 9)
    }
    ///Bit 10 - clear illegal access flag for FMAC
    #[inline(always)]
    pub fn cfmacf(&mut self) -> CFMACF_W<'_, FCR3rs> {
        CFMACF_W::new(self, 10)
    }
    ///Bit 11 - clear illegal access flag for register of ETH
    #[inline(always)]
    pub fn cethf(&mut self) -> CETHF_W<'_, FCR3rs> {
        CETHF_W::new(self, 11)
    }
    ///Bit 12 - clear illegal access flag for ICACHE
    #[inline(always)]
    pub fn cicachef(&mut self) -> CICACHEF_W<'_, FCR3rs> {
        CICACHEF_W::new(self, 12)
    }
    ///Bit 13 - clear illegal access flag for DCACHE
    #[inline(always)]
    pub fn cdcachef(&mut self) -> CDCACHEF_W<'_, FCR3rs> {
        CDCACHEF_W::new(self, 13)
    }
    ///Bit 14 - clear illegal access flag for ADC1 and ADC2
    #[inline(always)]
    pub fn cadc12f(&mut self) -> CADC12F_W<'_, FCR3rs> {
        CADC12F_W::new(self, 14)
    }
    ///Bit 15 - clear illegal access flag for DCMI
    #[inline(always)]
    pub fn cdcmif(&mut self) -> CDCMIF_W<'_, FCR3rs> {
        CDCMIF_W::new(self, 15)
    }
    ///Bit 16 - clear illegal access flag for AES
    #[inline(always)]
    pub fn caesf(&mut self) -> CAESF_W<'_, FCR3rs> {
        CAESF_W::new(self, 16)
    }
    ///Bit 17 - clear illegal access flag for HASH
    #[inline(always)]
    pub fn chashf(&mut self) -> CHASHF_W<'_, FCR3rs> {
        CHASHF_W::new(self, 17)
    }
    ///Bit 18 - clear illegal access flag for RNG
    #[inline(always)]
    pub fn crngf(&mut self) -> CRNGF_W<'_, FCR3rs> {
        CRNGF_W::new(self, 18)
    }
    ///Bit 19 - clear illegal access flag for SAES
    #[inline(always)]
    pub fn csaesf(&mut self) -> CSAESF_W<'_, FCR3rs> {
        CSAESF_W::new(self, 19)
    }
    ///Bit 20 - clear illegal access flag for PKA
    #[inline(always)]
    pub fn cpkaf(&mut self) -> CPKAF_W<'_, FCR3rs> {
        CPKAF_W::new(self, 20)
    }
    ///Bit 21 - clear illegal access flag for SDMMC1
    #[inline(always)]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<'_, FCR3rs> {
        CSDMMC1F_W::new(self, 21)
    }
    ///Bit 22 - clear illegal access flag for SDMMC2
    #[inline(always)]
    pub fn csdmmc2f(&mut self) -> CSDMMC2F_W<'_, FCR3rs> {
        CSDMMC2F_W::new(self, 22)
    }
    ///Bit 23 - clear illegal access flag for FMC
    #[inline(always)]
    pub fn cfmcf(&mut self) -> CFMCF_W<'_, FCR3rs> {
        CFMCF_W::new(self, 23)
    }
    ///Bit 24 - clear illegal access flag for OCTOSPI1
    #[inline(always)]
    pub fn coctospi1f(&mut self) -> COCTOSPI1F_W<'_, FCR3rs> {
        COCTOSPI1F_W::new(self, 24)
    }
    ///Bit 26 - clear illegal access flag for RAMSCFG
    #[inline(always)]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<'_, FCR3rs> {
        CRAMCFGF_W::new(self, 26)
    }
}
/**GTZC1 TZIC flag clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZIC:FCR3)*/
pub struct FCR3rs;
impl crate::RegisterSpec for FCR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr3::W`](W) writer structure
impl crate::Writable for FCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR3 to value 0
impl crate::Resettable for FCR3rs {}
