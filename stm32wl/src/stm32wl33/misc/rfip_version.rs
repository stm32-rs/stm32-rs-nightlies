///Register `RFIP_VERSION` reader
pub type R = crate::R<RFIP_VERSIONrs>;
///Field `REVISION` reader - Revision of the MR_SubG (to be used for metal fixes)
pub type REVISION_R = crate::FieldReader;
///Field `VERSION` reader - Version of the MR_SubG (to be used for cut upgrades)
pub type VERSION_R = crate::FieldReader;
///Field `PRODUCT` reader - Used for major upgrades (new protocols support / new features)
pub type PRODUCT_R = crate::FieldReader;
impl R {
    ///Bits 4:7 - Revision of the MR_SubG (to be used for metal fixes)
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Version of the MR_SubG (to be used for cut upgrades)
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Used for major upgrades (new protocols support / new features)
    #[inline(always)]
    pub fn product(&self) -> PRODUCT_R {
        PRODUCT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFIP_VERSION")
            .field("revision", &self.revision())
            .field("version", &self.version())
            .field("product", &self.product())
            .finish()
    }
}
/**RFIP_VERSION register

You can [`read`](crate::Reg::read) this register and get [`rfip_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:RFIP_VERSION)*/
pub struct RFIP_VERSIONrs;
impl crate::RegisterSpec for RFIP_VERSIONrs {
    type Ux = u32;
}
///`read()` method returns [`rfip_version::R`](R) reader structure
impl crate::Readable for RFIP_VERSIONrs {}
///`reset()` method sets RFIP_VERSION to value 0x1200
impl crate::Resettable for RFIP_VERSIONrs {
    const RESET_VALUE: u32 = 0x1200;
}
