#[doc = "Register `GICD_CIDR2` reader"]
pub type R = crate::R<GICD_CIDR2rs>;
#[doc = "Field `CIDR2` reader - CIDR2"]
pub type CIDR2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR2"]
    #[inline(always)]
    pub fn cidr2(&self) -> CIDR2_R {
        CIDR2_R::new(self.bits)
    }
}
#[doc = "GICD component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR2rs;
impl crate::RegisterSpec for GICD_CIDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr2::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR2rs {}
#[doc = "`reset()` method sets GICD_CIDR2 to value 0x05"]
impl crate::Resettable for GICD_CIDR2rs {
    const RESET_VALUE: u32 = 0x05;
}
