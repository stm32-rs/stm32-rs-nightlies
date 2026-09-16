///Register `I2S_HWCFGR` reader
pub type R = crate::R<I2S_HWCFGRrs>;
///Field `TXFCFG` reader - TXFCFG
pub type TXFCFG_R = crate::FieldReader;
///Field `RXFCFG` reader - RXFCFG
pub type RXFCFG_R = crate::FieldReader;
///Field `CRCCFG` reader - CRCCFG
pub type CRCCFG_R = crate::FieldReader;
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader;
///Field `DSCFG` reader - DSCFG
pub type DSCFG_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - TXFCFG
    #[inline(always)]
    pub fn txfcfg(&self) -> TXFCFG_R {
        TXFCFG_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - RXFCFG
    #[inline(always)]
    pub fn rxfcfg(&self) -> RXFCFG_R {
        RXFCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - CRCCFG
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DSCFG
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S_HWCFGR")
            .field("txfcfg", &self.txfcfg())
            .field("rxfcfg", &self.rxfcfg())
            .field("crccfg", &self.crccfg())
            .field("i2scfg", &self.i2scfg())
            .field("dscfg", &self.dscfg())
            .finish()
    }
}
/**SPI/I2S hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`i2s_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPI1:I2S_HWCFGR)*/
pub struct I2S_HWCFGRrs;
impl crate::RegisterSpec for I2S_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`i2s_hwcfgr::R`](R) reader structure
impl crate::Readable for I2S_HWCFGRrs {}
///`reset()` method sets I2S_HWCFGR to value 0
impl crate::Resettable for I2S_HWCFGRrs {}
