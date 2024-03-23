#[doc = "Register `GICD_CIDR0` reader"]
pub type R = crate::R<GICD_CIDR0rs>;
#[doc = "Field `CIDR0` reader - CIDR0"]
pub type CIDR0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR0"]
    #[inline(always)]
    pub fn cidr0(&self) -> CIDR0_R {
        CIDR0_R::new(self.bits)
    }
}
#[doc = "GICD component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR0rs;
impl crate::RegisterSpec for GICD_CIDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr0::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR0rs {}
#[doc = "`reset()` method sets GICD_CIDR0 to value 0x0d"]
impl crate::Resettable for GICD_CIDR0rs {
    const RESET_VALUE: u32 = 0x0d;
}
