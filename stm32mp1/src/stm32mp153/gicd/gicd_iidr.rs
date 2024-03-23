#[doc = "Register `GICD_IIDR` reader"]
pub type R = crate::R<GICD_IIDRrs>;
#[doc = "Field `IMPLEMENTER` reader - IMPLEMENTER"]
pub type IMPLEMENTER_R = crate::FieldReader<u16>;
#[doc = "Field `VARIANT` reader - VARIANT"]
pub type VARIANT_R = crate::FieldReader;
#[doc = "Field `REVISION` reader - REVISION"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `PRODUCTID` reader - PRODUCTID"]
pub type PRODUCTID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - IMPLEMENTER"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - VARIANT"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - REVISION"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - PRODUCTID"]
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "GICD implementer identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_iidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IIDRrs;
impl crate::RegisterSpec for GICD_IIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_iidr::R`](R) reader structure"]
impl crate::Readable for GICD_IIDRrs {}
#[doc = "`reset()` method sets GICD_IIDR to value 0x0100_143b"]
impl crate::Resettable for GICD_IIDRrs {
    const RESET_VALUE: u32 = 0x0100_143b;
}
