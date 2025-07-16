///Register `SR3` reader
pub type R = crate::R<SR3rs>;
///Field `CRCF` reader - illegal access flag for CRC
pub type CRCF_R = crate::BitReader;
///Field `TSCF` reader - illegal access flag for TSC
pub type TSCF_R = crate::BitReader;
///Field `ICACHEF` reader - illegal access flag for ICACHE registers
pub type ICACHEF_R = crate::BitReader;
///Field `AESF` reader - illegal access flag for AES
pub type AESF_R = crate::BitReader;
///Field `HASHF` reader - illegal access flag for HASH
pub type HASHF_R = crate::BitReader;
///Field `RNGF` reader - illegal access flag for RNG
pub type RNGF_R = crate::BitReader;
///Field `SAESF` reader - illegal access flag for SAES
pub type SAESF_R = crate::BitReader;
///Field `HSEMF` reader - illegal access flag for HSEM
pub type HSEMF_R = crate::BitReader;
///Field `PKAF` reader - illegal access flag for PKA
pub type PKAF_R = crate::BitReader;
///Field `RAMCFGF` reader - illegal access flag for RAMCFG
pub type RAMCFGF_R = crate::BitReader;
///Field `RADIOF` reader - illegal access flag for 2.4 GHz RADIO
pub type RADIOF_R = crate::BitReader;
impl R {
    ///Bit 3 - illegal access flag for CRC
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TSC
    #[inline(always)]
    pub fn tscf(&self) -> TSCF_R {
        TSCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for ICACHE registers
    #[inline(always)]
    pub fn icachef(&self) -> ICACHEF_R {
        ICACHEF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for AES
    #[inline(always)]
    pub fn aesf(&self) -> AESF_R {
        AESF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for HASH
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for RNG
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for SAES
    #[inline(always)]
    pub fn saesf(&self) -> SAESF_R {
        SAESF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for HSEM
    #[inline(always)]
    pub fn hsemf(&self) -> HSEMF_R {
        HSEMF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for PKA
    #[inline(always)]
    pub fn pkaf(&self) -> PKAF_R {
        PKAF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - illegal access flag for RAMCFG
    #[inline(always)]
    pub fn ramcfgf(&self) -> RAMCFGF_R {
        RAMCFGF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access flag for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radiof(&self) -> RADIOF_R {
        RADIOF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR3")
            .field("crcf", &self.crcf())
            .field("tscf", &self.tscf())
            .field("icachef", &self.icachef())
            .field("aesf", &self.aesf())
            .field("hashf", &self.hashf())
            .field("rngf", &self.rngf())
            .field("saesf", &self.saesf())
            .field("hsemf", &self.hsemf())
            .field("pkaf", &self.pkaf())
            .field("ramcfgf", &self.ramcfgf())
            .field("radiof", &self.radiof())
            .finish()
    }
}
/**TZIC status register 3

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZIC:SR3)*/
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
///`read()` method returns [`sr3::R`](R) reader structure
impl crate::Readable for SR3rs {}
///`reset()` method sets SR3 to value 0
impl crate::Resettable for SR3rs {}
