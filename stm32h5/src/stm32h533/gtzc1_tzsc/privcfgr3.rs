///Register `PRIVCFGR3` reader
pub type R = crate::R<PRIVCFGR3rs>;
///Register `PRIVCFGR3` writer
pub type W = crate::W<PRIVCFGR3rs>;
///Field `LPTIM6PRIV` reader - privileged access mode for LPTIM6
pub type LPTIM6PRIV_R = crate::BitReader;
///Field `LPTIM6PRIV` writer - privileged access mode for LPTIM6
pub type LPTIM6PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFPRIV` reader - privileged access mode for VREFBUF
pub type VREFBUFPRIV_R = crate::BitReader;
///Field `VREFBUFPRIV` writer - privileged access mode for VREFBUF
pub type VREFBUFPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2PRIV` reader - privileged access mode for I3C2
pub type I3C2PRIV_R = crate::BitReader;
///Field `I3C2PRIV` writer - privileged access mode for I3C2
pub type I3C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCPRIV` reader - privileged access mode for CRC
pub type CRCPRIV_R = crate::BitReader;
///Field `CRCPRIV` writer - privileged access mode for CRC
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORDICPRIV` reader - privileged access mode for CORDIC
pub type CORDICPRIV_R = crate::BitReader;
///Field `CORDICPRIV` writer - privileged access mode for CORDIC
pub type CORDICPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMACPRIV` reader - privileged access mode for FMAC
pub type FMACPRIV_R = crate::BitReader;
///Field `FMACPRIV` writer - privileged access mode for FMAC
pub type FMACPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETHPRIV` reader - privileged access mode for register of ETH
pub type ETHPRIV_R = crate::BitReader;
///Field `ETHPRIV` writer - privileged access mode for register of ETH
pub type ETHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHEPRIV` reader - privileged access mode for ICACHE
pub type ICACHEPRIV_R = crate::BitReader;
///Field `ICACHEPRIV` writer - privileged access mode for ICACHE
pub type ICACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHEPRIV` reader - privileged access mode for DCACHE
pub type DCACHEPRIV_R = crate::BitReader;
///Field `DCACHEPRIV` writer - privileged access mode for DCACHE
pub type DCACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12PRIV` reader - privileged access mode for ADC1 and ADC2
pub type ADC12PRIV_R = crate::BitReader;
///Field `ADC12PRIV` writer - privileged access mode for ADC1 and ADC2
pub type ADC12PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPRIV` reader - privileged access mode for DCMI
pub type DCMIPRIV_R = crate::BitReader;
///Field `DCMIPRIV` writer - privileged access mode for DCMI
pub type DCMIPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESPRIV` reader - privileged access mode for AES
pub type AESPRIV_R = crate::BitReader;
///Field `AESPRIV` writer - privileged access mode for AES
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHPRIV` reader - privileged access mode for HASH
pub type HASHPRIV_R = crate::BitReader;
///Field `HASHPRIV` writer - privileged access mode for HASH
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGPRIV` reader - privileged access mode for RNG
pub type RNGPRIV_R = crate::BitReader;
///Field `RNGPRIV` writer - privileged access mode for RNG
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESPRIV` reader - privileged access mode for SAES
pub type SAESPRIV_R = crate::BitReader;
///Field `SAESPRIV` writer - privileged access mode for SAES
pub type SAESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAPRIV` reader - privileged access mode for PKA
pub type PKAPRIV_R = crate::BitReader;
///Field `PKAPRIV` writer - privileged access mode for PKA
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1PRIV` reader - privileged access mode for SDMMC1
pub type SDMMC1PRIV_R = crate::BitReader;
///Field `SDMMC1PRIV` writer - privileged access mode for SDMMC1
pub type SDMMC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCPRIV` reader - privileged access mode for FMC
pub type FMCPRIV_R = crate::BitReader;
///Field `FMCPRIV` writer - privileged access mode for FMC
pub type FMCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1PRIV` reader - privileged access mode for OCTOSPI1
pub type OCTOSPI1PRIV_R = crate::BitReader;
///Field `OCTOSPI1PRIV` writer - privileged access mode for OCTOSPI1
pub type OCTOSPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGPRIV` reader - privileged access mode for RAMSCFG
pub type RAMCFGPRIV_R = crate::BitReader;
///Field `RAMCFGPRIV` writer - privileged access mode for RAMSCFG
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for LPTIM6
    #[inline(always)]
    pub fn lptim6priv(&self) -> LPTIM6PRIV_R {
        LPTIM6PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for I3C2
    #[inline(always)]
    pub fn i3c2priv(&self) -> I3C2PRIV_R {
        I3C2PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for CORDIC
    #[inline(always)]
    pub fn cordicpriv(&self) -> CORDICPRIV_R {
        CORDICPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - privileged access mode for FMAC
    #[inline(always)]
    pub fn fmacpriv(&self) -> FMACPRIV_R {
        FMACPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for register of ETH
    #[inline(always)]
    pub fn ethpriv(&self) -> ETHPRIV_R {
        ETHPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for ICACHE
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for DCACHE
    #[inline(always)]
    pub fn dcachepriv(&self) -> DCACHEPRIV_R {
        DCACHEPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - privileged access mode for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12priv(&self) -> ADC12PRIV_R {
        ADC12PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - privileged access mode for DCMI
    #[inline(always)]
    pub fn dcmipriv(&self) -> DCMIPRIV_R {
        DCMIPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&self) -> SAESPRIV_R {
        SAESPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - privileged access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - privileged access mode for FMC
    #[inline(always)]
    pub fn fmcpriv(&self) -> FMCPRIV_R {
        FMCPRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - privileged access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1priv(&self) -> OCTOSPI1PRIV_R {
        OCTOSPI1PRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - privileged access mode for RAMSCFG
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR3")
            .field("lptim6priv", &self.lptim6priv())
            .field("vrefbufpriv", &self.vrefbufpriv())
            .field("i3c2priv", &self.i3c2priv())
            .field("crcpriv", &self.crcpriv())
            .field("cordicpriv", &self.cordicpriv())
            .field("fmacpriv", &self.fmacpriv())
            .field("ethpriv", &self.ethpriv())
            .field("icachepriv", &self.icachepriv())
            .field("dcachepriv", &self.dcachepriv())
            .field("adc12priv", &self.adc12priv())
            .field("dcmipriv", &self.dcmipriv())
            .field("aespriv", &self.aespriv())
            .field("hashpriv", &self.hashpriv())
            .field("rngpriv", &self.rngpriv())
            .field("saespriv", &self.saespriv())
            .field("pkapriv", &self.pkapriv())
            .field("sdmmc1priv", &self.sdmmc1priv())
            .field("fmcpriv", &self.fmcpriv())
            .field("octospi1priv", &self.octospi1priv())
            .field("ramcfgpriv", &self.ramcfgpriv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for LPTIM6
    #[inline(always)]
    pub fn lptim6priv(&mut self) -> LPTIM6PRIV_W<'_, PRIVCFGR3rs> {
        LPTIM6PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<'_, PRIVCFGR3rs> {
        VREFBUFPRIV_W::new(self, 1)
    }
    ///Bit 2 - privileged access mode for I3C2
    #[inline(always)]
    pub fn i3c2priv(&mut self) -> I3C2PRIV_W<'_, PRIVCFGR3rs> {
        I3C2PRIV_W::new(self, 2)
    }
    ///Bit 8 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<'_, PRIVCFGR3rs> {
        CRCPRIV_W::new(self, 8)
    }
    ///Bit 9 - privileged access mode for CORDIC
    #[inline(always)]
    pub fn cordicpriv(&mut self) -> CORDICPRIV_W<'_, PRIVCFGR3rs> {
        CORDICPRIV_W::new(self, 9)
    }
    ///Bit 10 - privileged access mode for FMAC
    #[inline(always)]
    pub fn fmacpriv(&mut self) -> FMACPRIV_W<'_, PRIVCFGR3rs> {
        FMACPRIV_W::new(self, 10)
    }
    ///Bit 11 - privileged access mode for register of ETH
    #[inline(always)]
    pub fn ethpriv(&mut self) -> ETHPRIV_W<'_, PRIVCFGR3rs> {
        ETHPRIV_W::new(self, 11)
    }
    ///Bit 12 - privileged access mode for ICACHE
    #[inline(always)]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<'_, PRIVCFGR3rs> {
        ICACHEPRIV_W::new(self, 12)
    }
    ///Bit 13 - privileged access mode for DCACHE
    #[inline(always)]
    pub fn dcachepriv(&mut self) -> DCACHEPRIV_W<'_, PRIVCFGR3rs> {
        DCACHEPRIV_W::new(self, 13)
    }
    ///Bit 14 - privileged access mode for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12priv(&mut self) -> ADC12PRIV_W<'_, PRIVCFGR3rs> {
        ADC12PRIV_W::new(self, 14)
    }
    ///Bit 15 - privileged access mode for DCMI
    #[inline(always)]
    pub fn dcmipriv(&mut self) -> DCMIPRIV_W<'_, PRIVCFGR3rs> {
        DCMIPRIV_W::new(self, 15)
    }
    ///Bit 16 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W<'_, PRIVCFGR3rs> {
        AESPRIV_W::new(self, 16)
    }
    ///Bit 17 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<'_, PRIVCFGR3rs> {
        HASHPRIV_W::new(self, 17)
    }
    ///Bit 18 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<'_, PRIVCFGR3rs> {
        RNGPRIV_W::new(self, 18)
    }
    ///Bit 19 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&mut self) -> SAESPRIV_W<'_, PRIVCFGR3rs> {
        SAESPRIV_W::new(self, 19)
    }
    ///Bit 20 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<'_, PRIVCFGR3rs> {
        PKAPRIV_W::new(self, 20)
    }
    ///Bit 21 - privileged access mode for SDMMC1
    #[inline(always)]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<'_, PRIVCFGR3rs> {
        SDMMC1PRIV_W::new(self, 21)
    }
    ///Bit 23 - privileged access mode for FMC
    #[inline(always)]
    pub fn fmcpriv(&mut self) -> FMCPRIV_W<'_, PRIVCFGR3rs> {
        FMCPRIV_W::new(self, 23)
    }
    ///Bit 24 - privileged access mode for OCTOSPI1
    #[inline(always)]
    pub fn octospi1priv(&mut self) -> OCTOSPI1PRIV_W<'_, PRIVCFGR3rs> {
        OCTOSPI1PRIV_W::new(self, 24)
    }
    ///Bit 26 - privileged access mode for RAMSCFG
    #[inline(always)]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<'_, PRIVCFGR3rs> {
        RAMCFGPRIV_W::new(self, 26)
    }
}
/**GTZC1 TZSC privilege configuration register 3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GTZC1_TZSC:PRIVCFGR3)*/
pub struct PRIVCFGR3rs;
impl crate::RegisterSpec for PRIVCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr3::R`](R) reader structure
impl crate::Readable for PRIVCFGR3rs {}
///`write(|w| ..)` method takes [`privcfgr3::W`](W) writer structure
impl crate::Writable for PRIVCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR3 to value 0
impl crate::Resettable for PRIVCFGR3rs {}
