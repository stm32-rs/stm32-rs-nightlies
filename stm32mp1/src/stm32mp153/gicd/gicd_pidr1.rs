#[doc = "Register `GICD_PIDR1` reader"]
pub type R = crate::R<GICD_PIDR1rs>;
#[doc = "Field `PIDR1` reader - PIDR1"]
pub type PIDR1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR1"]
    #[inline(always)]
    pub fn pidr1(&self) -> PIDR1_R {
        PIDR1_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR1rs;
impl crate::RegisterSpec for GICD_PIDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr1::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR1rs {}
#[doc = "`reset()` method sets GICD_PIDR1 to value 0xb4"]
impl crate::Resettable for GICD_PIDR1rs {
    const RESET_VALUE: u32 = 0xb4;
}
