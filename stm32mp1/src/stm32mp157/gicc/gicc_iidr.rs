#[doc = "Register `GICC_IIDR` reader"]
pub type R = crate::R<GICC_IIDRrs>;
#[doc = "Field `IMPLEMENTER` reader - IMPLEMENTER"]
pub type IMPLEMENTER_R = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - REVISION"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `ARCH` reader - ARCH"]
pub type ARCH_R = crate::FieldReader;
#[doc = "Field `PRODUCTID` reader - PRODUCTID"]
pub type PRODUCTID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - IMPLEMENTER"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - REVISION"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ARCH"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - PRODUCTID"]
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "GICC interface identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_iidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_IIDRrs;
impl crate::RegisterSpec for GICC_IIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_iidr::R`](R) reader structure"]
impl crate::Readable for GICC_IIDRrs {}
#[doc = "`reset()` method sets GICC_IIDR to value 0x0102_143b"]
impl crate::Resettable for GICC_IIDRrs {
    const RESET_VALUE: u32 = 0x0102_143b;
}
