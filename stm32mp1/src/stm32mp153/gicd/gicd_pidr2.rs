#[doc = "Register `GICD_PIDR2` reader"]
pub type R = crate::R<GICD_PIDR2rs>;
#[doc = "Field `PIDR2` reader - PIDR2"]
pub type PIDR2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR2"]
    #[inline(always)]
    pub fn pidr2(&self) -> PIDR2_R {
        PIDR2_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR2rs;
impl crate::RegisterSpec for GICD_PIDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr2::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR2rs {}
#[doc = "`reset()` method sets GICD_PIDR2 to value 0x2b"]
impl crate::Resettable for GICD_PIDR2rs {
    const RESET_VALUE: u32 = 0x2b;
}
