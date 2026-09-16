///Register `ID` reader
pub type R = crate::R<IDrs>;
///Field `REVISION` reader - Incremented for metal fix version
pub type REVISION_R = crate::FieldReader;
///Field `VERSION` reader - Cut Number
pub type VERSION_R = crate::FieldReader;
///Field `PRODUCT` reader - incremented on major features add-on like new Bluetooth LE SIG version support
pub type PRODUCT_R = crate::FieldReader;
impl R {
    ///Bits 4:7 - Incremented for metal fix version
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Cut Number
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - incremented on major features add-on like new Bluetooth LE SIG version support
    #[inline(always)]
    pub fn product(&self) -> PRODUCT_R {
        PRODUCT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("revision", &self.revision())
            .field("version", &self.version())
            .field("product", &self.product())
            .finish()
    }
}
/**RADIO_CONTROL_ID register

You can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO_CONTROL:ID)*/
pub struct IDrs;
impl crate::RegisterSpec for IDrs {
    type Ux = u32;
}
///`read()` method returns [`id::R`](R) reader structure
impl crate::Readable for IDrs {}
///`reset()` method sets ID to value 0x3000
impl crate::Resettable for IDrs {
    const RESET_VALUE: u32 = 0x3000;
}
