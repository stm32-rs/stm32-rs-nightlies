#[doc = "Register `GICD_PIDR4` reader"]
pub type R = crate::R<GICD_PIDR4rs>;
#[doc = "Field `PIDR4` reader - PIDR4"]
pub type PIDR4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR4"]
    #[inline(always)]
    pub fn pidr4(&self) -> PIDR4_R {
        PIDR4_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR4rs;
impl crate::RegisterSpec for GICD_PIDR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr4::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR4rs {}
#[doc = "`reset()` method sets GICD_PIDR4 to value 0x04"]
impl crate::Resettable for GICD_PIDR4rs {
    const RESET_VALUE: u32 = 0x04;
}
