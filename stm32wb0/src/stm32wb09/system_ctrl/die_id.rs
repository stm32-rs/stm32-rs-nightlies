///Register `DIE_ID` reader
pub type R = crate::R<DIE_IDrs>;
///Field `REVISION` reader - Cut revision (metal fix)
pub type REVISION_R = crate::FieldReader;
///Field `VERSION` reader - Cut version
pub type VERSION_R = crate::FieldReader;
///Field `PRODUCT` reader - Product version. May be used to discriminate several version of a same digital BLE LPH device embedding different analog versions
pub type PRODUCT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Cut revision (metal fix)
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Cut version
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Product version. May be used to discriminate several version of a same digital BLE LPH device embedding different analog versions
    #[inline(always)]
    pub fn product(&self) -> PRODUCT_R {
        PRODUCT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIE_ID")
            .field("revision", &self.revision())
            .field("version", &self.version())
            .field("product", &self.product())
            .finish()
    }
}
/**DIE_ID register

You can [`read`](crate::Reg::read) this register and get [`die_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#SYSTEM_CTRL:DIE_ID)*/
pub struct DIE_IDrs;
impl crate::RegisterSpec for DIE_IDrs {
    type Ux = u32;
}
///`read()` method returns [`die_id::R`](R) reader structure
impl crate::Readable for DIE_IDrs {}
///`reset()` method sets DIE_ID to value 0x0120
impl crate::Resettable for DIE_IDrs {
    const RESET_VALUE: u32 = 0x0120;
}
