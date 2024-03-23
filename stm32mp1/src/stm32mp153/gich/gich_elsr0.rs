#[doc = "Register `GICH_ELSR0` reader"]
pub type R = crate::R<GICH_ELSR0rs>;
#[doc = "Field `ELSR0` reader - ELSR0"]
pub type ELSR0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ELSR0"]
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new(self.bits)
    }
}
#[doc = "GICH empty list status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_elsr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_ELSR0rs;
impl crate::RegisterSpec for GICH_ELSR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_elsr0::R`](R) reader structure"]
impl crate::Readable for GICH_ELSR0rs {}
#[doc = "`reset()` method sets GICH_ELSR0 to value 0x0f"]
impl crate::Resettable for GICH_ELSR0rs {
    const RESET_VALUE: u32 = 0x0f;
}
