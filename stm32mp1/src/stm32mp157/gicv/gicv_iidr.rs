#[doc = "Register `GICV_IIDR` reader"]
pub type R = crate::R<GICV_IIDRrs>;
#[doc = "Field `IIDR` reader - IIDR"]
pub type IIDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IIDR"]
    #[inline(always)]
    pub fn iidr(&self) -> IIDR_R {
        IIDR_R::new(self.bits)
    }
}
#[doc = "The GICV_IIDR is an alias of GICC_IIDR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_iidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_IIDRrs;
impl crate::RegisterSpec for GICV_IIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_iidr::R`](R) reader structure"]
impl crate::Readable for GICV_IIDRrs {}
#[doc = "`reset()` method sets GICV_IIDR to value 0x0102_143b"]
impl crate::Resettable for GICV_IIDRrs {
    const RESET_VALUE: u32 = 0x0102_143b;
}
