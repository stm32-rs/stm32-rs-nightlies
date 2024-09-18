///Register `I2C_HWCFGR` reader
pub type R = crate::R<I2C_HWCFGRrs>;
///Field `SMBUS` reader - SMBUS
pub type SMBUS_R = crate::FieldReader;
///Field `ASYN` reader - ASYN
pub type ASYN_R = crate::FieldReader;
///Field `WKP` reader - WKP
pub type WKP_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - SMBUS
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ASYN
    #[inline(always)]
    pub fn asyn(&self) -> ASYN_R {
        ASYN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - WKP
    #[inline(always)]
    pub fn wkp(&self) -> WKP_R {
        WKP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_HWCFGR")
            .field("smbus", &self.smbus())
            .field("asyn", &self.asyn())
            .field("wkp", &self.wkp())
            .finish()
    }
}
/**I2C hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`i2c_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_HWCFGR)*/
pub struct I2C_HWCFGRrs;
impl crate::RegisterSpec for I2C_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_hwcfgr::R`](R) reader structure
impl crate::Readable for I2C_HWCFGRrs {}
///`reset()` method sets I2C_HWCFGR to value 0x0111
impl crate::Resettable for I2C_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0111;
}
