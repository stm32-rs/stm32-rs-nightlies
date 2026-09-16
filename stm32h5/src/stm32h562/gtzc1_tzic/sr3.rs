///Register `SR3` reader
pub type R = crate::R<SR3rs>;
///Field `LPTIM6F` reader - illegal access flag for LPTIM6
pub type LPTIM6F_R = crate::BitReader;
///Field `VREFBUFF` reader - illegal access flag for VREFBUF
pub type VREFBUFF_R = crate::BitReader;
///Field `CRCF` reader - illegal access flag for CRC
pub type CRCF_R = crate::BitReader;
///Field `CORDICF` reader - illegal access flag for CORDIC
pub type CORDICF_R = crate::BitReader;
///Field `FMACF` reader - illegal access flag for FMAC
pub type FMACF_R = crate::BitReader;
///Field `ICACHEF` reader - illegal access flag for ICACHE
pub type ICACHEF_R = crate::BitReader;
///Field `DCACHEF` reader - illegal access flag for DCACHE
pub type DCACHEF_R = crate::BitReader;
///Field `ADC12F` reader - illegal access flag for ADC1 and ADC2
pub type ADC12F_R = crate::BitReader;
///Field `DCMIF` reader - illegal access flag for DCMI
pub type DCMIF_R = crate::BitReader;
///Field `HASHF` reader - illegal access flag for HASH
pub type HASHF_R = crate::BitReader;
///Field `RNGF` reader - illegal access flag for RNG
pub type RNGF_R = crate::BitReader;
///Field `SDMMC1F` reader - illegal access flag for SDMMC1
pub type SDMMC1F_R = crate::BitReader;
///Field `FMCF` reader - illegal access flag for FMC
pub type FMCF_R = crate::BitReader;
///Field `OCTOSPI1F` reader - illegal access flag for OCTOSPI1
pub type OCTOSPI1F_R = crate::BitReader;
///Field `RAMCFGF` reader - illegal access flag for RAMSCFG
pub type RAMCFGF_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for LPTIM6
    #[inline(always)]
    pub fn lptim6f(&self) -> LPTIM6F_R {
        LPTIM6F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for VREFBUF
    #[inline(always)]
    pub fn vrefbuff(&self) -> VREFBUFF_R {
        VREFBUFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for CRC
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for CORDIC
    #[inline(always)]
    pub fn cordicf(&self) -> CORDICF_R {
        CORDICF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for FMAC
    #[inline(always)]
    pub fn fmacf(&self) -> FMACF_R {
        FMACF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for ICACHE
    #[inline(always)]
    pub fn icachef(&self) -> ICACHEF_R {
        ICACHEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for DCACHE
    #[inline(always)]
    pub fn dcachef(&self) -> DCACHEF_R {
        DCACHEF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for ADC1 and ADC2
    #[inline(always)]
    pub fn adc12f(&self) -> ADC12F_R {
        ADC12F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for DCMI
    #[inline(always)]
    pub fn dcmif(&self) -> DCMIF_R {
        DCMIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for HASH
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for RNG
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for SDMMC1
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access flag for FMC
    #[inline(always)]
    pub fn fmcf(&self) -> FMCF_R {
        FMCF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for OCTOSPI1
    #[inline(always)]
    pub fn octospi1f(&self) -> OCTOSPI1F_R {
        OCTOSPI1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for RAMSCFG
    #[inline(always)]
    pub fn ramcfgf(&self) -> RAMCFGF_R {
        RAMCFGF_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR3")
            .field("lptim6f", &self.lptim6f())
            .field("vrefbuff", &self.vrefbuff())
            .field("crcf", &self.crcf())
            .field("cordicf", &self.cordicf())
            .field("fmacf", &self.fmacf())
            .field("icachef", &self.icachef())
            .field("dcachef", &self.dcachef())
            .field("adc12f", &self.adc12f())
            .field("dcmif", &self.dcmif())
            .field("hashf", &self.hashf())
            .field("rngf", &self.rngf())
            .field("sdmmc1f", &self.sdmmc1f())
            .field("fmcf", &self.fmcf())
            .field("octospi1f", &self.octospi1f())
            .field("ramcfgf", &self.ramcfgf())
            .finish()
    }
}
/**TZIC status register 3

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#GTZC1_TZIC:SR3)*/
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
///`read()` method returns [`sr3::R`](R) reader structure
impl crate::Readable for SR3rs {}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3rs {}
