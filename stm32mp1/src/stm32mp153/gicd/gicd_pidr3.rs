#[doc = "Register `GICD_PIDR3` reader"]
pub type R = crate::R<GICD_PIDR3rs>;
#[doc = "Field `PIDR3` reader - PIDR3"]
pub type PIDR3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR3"]
    #[inline(always)]
    pub fn pidr3(&self) -> PIDR3_R {
        PIDR3_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR3rs;
impl crate::RegisterSpec for GICD_PIDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr3::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR3rs {}
#[doc = "`reset()` method sets GICD_PIDR3 to value 0"]
impl crate::Resettable for GICD_PIDR3rs {
    const RESET_VALUE: u32 = 0;
}
