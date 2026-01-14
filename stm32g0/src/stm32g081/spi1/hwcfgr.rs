///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `CRCCFG` reader - CRC capable at SPI mode
pub type CRCCFG_R = crate::FieldReader;
///Field `I2SCFG` reader - I2S mode implementation
pub type I2SCFG_R = crate::FieldReader;
///Field `I2SCKCFG` reader - I2S master clock generator at I2S mode
pub type I2SCKCFG_R = crate::FieldReader;
///Field `DSCFG` reader - SPI data size configuration
pub type DSCFG_R = crate::FieldReader;
///Field `NSSCFG` reader - NSS pulse feature enhancement at SPI master
pub type NSSCFG_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CRC capable at SPI mode
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - I2S mode implementation
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - I2S master clock generator at I2S mode
    #[inline(always)]
    pub fn i2sckcfg(&self) -> I2SCKCFG_R {
        I2SCKCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SPI data size configuration
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - NSS pulse feature enhancement at SPI master
    #[inline(always)]
    pub fn nsscfg(&self) -> NSSCFG_R {
        NSSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("crccfg", &self.crccfg())
            .field("i2scfg", &self.i2scfg())
            .field("i2sckcfg", &self.i2sckcfg())
            .field("dscfg", &self.dscfg())
            .field("nsscfg", &self.nsscfg())
            .finish()
    }
}
/**hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#SPI1:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0
impl crate::Resettable for HWCFGRrs {}
