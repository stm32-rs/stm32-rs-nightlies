#[doc = "Register `GICD_CIDR1` reader"]
pub type R = crate::R<GICD_CIDR1rs>;
#[doc = "Field `CIDR1` reader - CIDR1"]
pub type CIDR1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR1"]
    #[inline(always)]
    pub fn cidr1(&self) -> CIDR1_R {
        CIDR1_R::new(self.bits)
    }
}
#[doc = "GICD component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR1rs;
impl crate::RegisterSpec for GICD_CIDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr1::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR1rs {}
#[doc = "`reset()` method sets GICD_CIDR1 to value 0xf0"]
impl crate::Resettable for GICD_CIDR1rs {
    const RESET_VALUE: u32 = 0xf0;
}
