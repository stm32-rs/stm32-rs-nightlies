///Register `SECCFGR3` reader
pub type R = crate::R<SECCFGR3rs>;
///Register `SECCFGR3` writer
pub type W = crate::W<SECCFGR3rs>;
///Field `LPTIM6SEC` reader - secure access mode for LPTIM6
pub type LPTIM6SEC_R = crate::BitReader;
///Field `LPTIM6SEC` writer - secure access mode for LPTIM6
pub type LPTIM6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFSEC` reader - secure access mode for VREFBUF
pub type VREFBUFSEC_R = crate::BitReader;
///Field `VREFBUFSEC` writer - secure access mode for VREFBUF
pub type VREFBUFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSEC` reader - secure access mode for CRC
pub type CRCSEC_R = crate::BitReader;
///Field `CRCSEC` writer - secure access mode for CRC
pub type CRCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICSEC` reader - secure access mode for CORDIC
pub type CORDICSEC_R = crate::BitReader;
///Field `CORDICSEC` writer - secure access mode for CORDIC
pub type CORDICSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACSEC` reader - secure access mode for FMAC
pub type FMACSEC_R = crate::BitReader;
///Field `FMACSEC` writer - secure access mode for FMAC
pub type FMACSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHSEC` reader - secure access mode for register of ETH
pub type ETHSEC_R = crate::BitReader;
///Field `ETHSEC` writer - secure access mode for register of ETH
pub type ETHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHESEC` reader - secure access mode for ICACHE
pub type ICACHESEC_R = crate::BitReader;
///Field `ICACHESEC` writer - secure access mode for ICACHE
pub type ICACHESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHESEC` reader - secure access mode for DCACHE
pub type DCACHESEC_R = crate::BitReader;
///Field `DCACHESEC` writer - secure access mode for DCACHE
pub type DCACHESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12SEC` reader - secure access mode for ADC1 and ADC2
pub type ADC12SEC_R = crate::BitReader;
///Field `ADC12SEC` writer - secure access mode for ADC1 and ADC2
pub type ADC12SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMISEC` reader - secure access mode for DCMI
pub type DCMISEC_R = crate::BitReader;
///Field `DCMISEC` writer - secure access mode for DCMI
pub type DCMISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSEC` reader - secure access mode for AES
pub type AESSEC_R = crate::BitReader;
///Field `AESSEC` writer - secure access mode for AES
pub type AESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHSEC` reader - secure access mode for HASH
pub type HASHSEC_R = crate::BitReader;
///Field `HASHSEC` writer - secure access mode for HASH
pub type HASHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEC` reader - secure access mode for RNG
pub type RNGSEC_R = crate::BitReader;
///Field `RNGSEC` writer - secure access mode for RNG
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESSEC` reader - secure access mode for SAES
pub type SAESSEC_R = crate::BitReader;
///Field `SAESSEC` writer - secure access mode for SAES
pub type SAESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKASEC` reader - secure access mode for PKA
pub type PKASEC_R = crate::BitReader;
///Field `PKASEC` writer - secure access mode for PKA
pub type PKASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1SEC` reader - secure access mode for SDMMC2
pub type SDMMC1SEC_R = crate::BitReader;
///Field `SDMMC1SEC` writer - secure access mode for SDMMC2
pub type SDMMC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2SEC` reader - secure access mode for SDMMC1
pub type SDMMC2SEC_R = crate::BitReader;
///Field `SDMMC2SEC` writer - secure access mode for SDMMC1
pub type SDMMC2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCSEC` reader - secure access mode for FMC
pub type FMCSEC_R = crate::BitReader;
///Field `FMCSEC` writer - secure access mode for FMC
pub type FMCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1SEC` reader - secure access mode for OCTOSPI1
pub type OCTOSPI1SEC_R = crate::BitReader;
///Field `OCTOSPI1SEC` writer - secure access mode for OCTOSPI1
pub type OCTOSPI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGSEC` reader - secure access mode for RAMSCFG
pub type RAMCFGSEC_R = crate::BitReader;
///Field `RAMCFGSEC` writer - secure access mode for RAMSCFG
pub type RAMCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for LPTIM6
    #[inline(always)]
    pub fn lptim6sec(&self) -> LPTIM6SEC_R {
        LPTIM6SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure access mode for CORDIC
    #[inline(always)]
    pub fn cordicsec(&self) -> CORDICSEC_R {
        CORDICSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - secure access mode for FMAC
    #[inline(always)]
    pub fn fmacsec(&self) -> FMACSEC_R {
        FMACSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - secure access mode for register of ETH
    #[inline(always)]
    pub fn ethsec(&self) -> ETHSEC_R {
        ETHSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for ICACHE
    #[inline(always)]
    pub fn icachesec(&self) -> ICACHESEC_R {
        ICACHESEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure access mode for DCACHE
    #[inline(always)]
    pub fn dcachesec(&self) -> DCACHESEC_R {
        DCACHESEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - secure access mode for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12sec(&self) -> ADC12SEC_R {
        ADC12SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - secure access mode for DCMI
    #[inline(always)]
    pub fn dcmisec(&self) -> DCMISEC_R {
        DCMISEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - secure access mode for AES
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - secure access mode for SAES
    #[inline(always)]
    pub fn saessec(&self) -> SAESSEC_R {
        SAESSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - secure access mode for PKA
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - secure access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - secure access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2sec(&self) -> SDMMC2SEC_R {
        SDMMC2SEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - secure access mode for FMC
    #[inline(always)]
    pub fn fmcsec(&self) -> FMCSEC_R {
        FMCSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - secure access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1sec(&self) -> OCTOSPI1SEC_R {
        OCTOSPI1SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - secure access mode for RAMSCFG
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR3")
            .field("lptim6sec", &self.lptim6sec())
            .field("vrefbufsec", &self.vrefbufsec())
            .field("crcsec", &self.crcsec())
            .field("cordicsec", &self.cordicsec())
            .field("fmacsec", &self.fmacsec())
            .field("ethsec", &self.ethsec())
            .field("icachesec", &self.icachesec())
            .field("dcachesec", &self.dcachesec())
            .field("adc12sec", &self.adc12sec())
            .field("dcmisec", &self.dcmisec())
            .field("aessec", &self.aessec())
            .field("hashsec", &self.hashsec())
            .field("rngsec", &self.rngsec())
            .field("saessec", &self.saessec())
            .field("pkasec", &self.pkasec())
            .field("sdmmc1sec", &self.sdmmc1sec())
            .field("sdmmc2sec", &self.sdmmc2sec())
            .field("fmcsec", &self.fmcsec())
            .field("octospi1sec", &self.octospi1sec())
            .field("ramcfgsec", &self.ramcfgsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for LPTIM6
    #[inline(always)]
    pub fn lptim6sec(&mut self) -> LPTIM6SEC_W<'_, SECCFGR3rs> {
        LPTIM6SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<'_, SECCFGR3rs> {
        VREFBUFSEC_W::new(self, 1)
    }
    ///Bit 8 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&mut self) -> CRCSEC_W<'_, SECCFGR3rs> {
        CRCSEC_W::new(self, 8)
    }
    ///Bit 9 - secure access mode for CORDIC
    #[inline(always)]
    pub fn cordicsec(&mut self) -> CORDICSEC_W<'_, SECCFGR3rs> {
        CORDICSEC_W::new(self, 9)
    }
    ///Bit 10 - secure access mode for FMAC
    #[inline(always)]
    pub fn fmacsec(&mut self) -> FMACSEC_W<'_, SECCFGR3rs> {
        FMACSEC_W::new(self, 10)
    }
    ///Bit 11 - secure access mode for register of ETH
    #[inline(always)]
    pub fn ethsec(&mut self) -> ETHSEC_W<'_, SECCFGR3rs> {
        ETHSEC_W::new(self, 11)
    }
    ///Bit 12 - secure access mode for ICACHE
    #[inline(always)]
    pub fn icachesec(&mut self) -> ICACHESEC_W<'_, SECCFGR3rs> {
        ICACHESEC_W::new(self, 12)
    }
    ///Bit 13 - secure access mode for DCACHE
    #[inline(always)]
    pub fn dcachesec(&mut self) -> DCACHESEC_W<'_, SECCFGR3rs> {
        DCACHESEC_W::new(self, 13)
    }
    ///Bit 14 - secure access mode for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12sec(&mut self) -> ADC12SEC_W<'_, SECCFGR3rs> {
        ADC12SEC_W::new(self, 14)
    }
    ///Bit 15 - secure access mode for DCMI
    #[inline(always)]
    pub fn dcmisec(&mut self) -> DCMISEC_W<'_, SECCFGR3rs> {
        DCMISEC_W::new(self, 15)
    }
    ///Bit 16 - secure access mode for AES
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W<'_, SECCFGR3rs> {
        AESSEC_W::new(self, 16)
    }
    ///Bit 17 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&mut self) -> HASHSEC_W<'_, SECCFGR3rs> {
        HASHSEC_W::new(self, 17)
    }
    ///Bit 18 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W<'_, SECCFGR3rs> {
        RNGSEC_W::new(self, 18)
    }
    ///Bit 19 - secure access mode for SAES
    #[inline(always)]
    pub fn saessec(&mut self) -> SAESSEC_W<'_, SECCFGR3rs> {
        SAESSEC_W::new(self, 19)
    }
    ///Bit 20 - secure access mode for PKA
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W<'_, SECCFGR3rs> {
        PKASEC_W::new(self, 20)
    }
    ///Bit 21 - secure access mode for SDMMC2
    #[inline(always)]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<'_, SECCFGR3rs> {
        SDMMC1SEC_W::new(self, 21)
    }
    ///Bit 22 - secure access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc2sec(&mut self) -> SDMMC2SEC_W<'_, SECCFGR3rs> {
        SDMMC2SEC_W::new(self, 22)
    }
    ///Bit 23 - secure access mode for FMC
    #[inline(always)]
    pub fn fmcsec(&mut self) -> FMCSEC_W<'_, SECCFGR3rs> {
        FMCSEC_W::new(self, 23)
    }
    ///Bit 24 - secure access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1sec(&mut self) -> OCTOSPI1SEC_W<'_, SECCFGR3rs> {
        OCTOSPI1SEC_W::new(self, 24)
    }
    ///Bit 26 - secure access mode for RAMSCFG
    #[inline(always)]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<'_, SECCFGR3rs> {
        RAMCFGSEC_W::new(self, 26)
    }
}
/**GTZC1 TZSC secure configuration register 3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GTZC1_TZSC:SECCFGR3)*/
pub struct SECCFGR3rs;
impl crate::RegisterSpec for SECCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr3::R`](R) reader structure
impl crate::Readable for SECCFGR3rs {}
///`write(|w| ..)` method takes [`seccfgr3::W`](W) writer structure
impl crate::Writable for SECCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR3 to value 0
impl crate::Resettable for SECCFGR3rs {}
