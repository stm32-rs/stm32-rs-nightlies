#[doc = "Register `GICD_CIDR3` reader"]
pub type R = crate::R<GICD_CIDR3rs>;
#[doc = "Field `CIDR3` reader - CIDR3"]
pub type CIDR3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR3"]
    #[inline(always)]
    pub fn cidr3(&self) -> CIDR3_R {
        CIDR3_R::new(self.bits)
    }
}
#[doc = "GICD component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR3rs;
impl crate::RegisterSpec for GICD_CIDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr3::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR3rs {}
#[doc = "`reset()` method sets GICD_CIDR3 to value 0xb1"]
impl crate::Resettable for GICD_CIDR3rs {
    const RESET_VALUE: u32 = 0xb1;
}
