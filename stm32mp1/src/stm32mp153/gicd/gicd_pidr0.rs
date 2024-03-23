#[doc = "Register `GICD_PIDR0` reader"]
pub type R = crate::R<GICD_PIDR0rs>;
#[doc = "Field `PIDR0` reader - PIDR0"]
pub type PIDR0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR0"]
    #[inline(always)]
    pub fn pidr0(&self) -> PIDR0_R {
        PIDR0_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR0rs;
impl crate::RegisterSpec for GICD_PIDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr0::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR0rs {}
#[doc = "`reset()` method sets GICD_PIDR0 to value 0x90"]
impl crate::Resettable for GICD_PIDR0rs {
    const RESET_VALUE: u32 = 0x90;
}
