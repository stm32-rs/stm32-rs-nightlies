///Register `IIDR` reader
pub type R = crate::R<IIDRrs>;
///Field `IMPLEMENTER` reader - IMPLEMENTER
pub type IMPLEMENTER_R = crate::FieldReader<u16>;
///Field `VARIANT` reader - VARIANT
pub type VARIANT_R = crate::FieldReader;
///Field `REVISION` reader - REVISION
pub type REVISION_R = crate::FieldReader;
///Field `PRODUCTID` reader - PRODUCTID
pub type PRODUCTID_R = crate::FieldReader;
impl R {
    ///Bits 0:11 - IMPLEMENTER
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:15 - VARIANT
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - REVISION
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:31 - PRODUCTID
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIDR")
            .field("implementer", &self.implementer())
            .field("variant", &self.variant())
            .field("revision", &self.revision())
            .field("productid", &self.productid())
            .finish()
    }
}
/**GICD implementer identification register

You can [`read`](crate::Reg::read) this register and get [`iidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:IIDR)*/
pub struct IIDRrs;
impl crate::RegisterSpec for IIDRrs {
    type Ux = u32;
}
///`read()` method returns [`iidr::R`](R) reader structure
impl crate::Readable for IIDRrs {}
///`reset()` method sets IIDR to value 0x0100_143b
impl crate::Resettable for IIDRrs {
    const RESET_VALUE: u32 = 0x0100_143b;
}
