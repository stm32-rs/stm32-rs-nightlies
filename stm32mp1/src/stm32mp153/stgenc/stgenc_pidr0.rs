#[doc = "Register `STGENC_PIDR0` reader"]
pub type R = crate::R<STGENC_PIDR0rs>;
#[doc = "Field `PART_0` reader - PART_0"]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PART_0"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENC peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_pidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_PIDR0rs;
impl crate::RegisterSpec for STGENC_PIDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_pidr0::R`](R) reader structure"]
impl crate::Readable for STGENC_PIDR0rs {}
#[doc = "`reset()` method sets STGENC_PIDR0 to value 0x01"]
impl crate::Resettable for STGENC_PIDR0rs {
    const RESET_VALUE: u32 = 0x01;
}
