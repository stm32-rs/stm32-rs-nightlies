#[doc = "Reader of register I2C_HWCFGR"]
pub type R = crate::R<u32, super::I2C_HWCFGR>;
#[doc = "Reader of field `SMBUS`"]
pub type SMBUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `ASYN`"]
pub type ASYN_R = crate::R<u8, u8>;
#[doc = "Reader of field `WKP`"]
pub type WKP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - SMBUS"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ASYN"]
    #[inline(always)]
    pub fn asyn(&self) -> ASYN_R {
        ASYN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WKP"]
    #[inline(always)]
    pub fn wkp(&self) -> WKP_R {
        WKP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
