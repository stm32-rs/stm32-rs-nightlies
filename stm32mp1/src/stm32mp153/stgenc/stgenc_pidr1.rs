#[doc = "Register `STGENC_PIDR1` reader"]
pub type R = crate::R<STGENC_PIDR1rs>;
#[doc = "Field `PART_1` reader - PART_1"]
pub type PART_1_R = crate::FieldReader;
#[doc = "Field `DES_0` reader - DES_0"]
pub type DES_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PART_1"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DES_0"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENC peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_pidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_PIDR1rs;
impl crate::RegisterSpec for STGENC_PIDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_pidr1::R`](R) reader structure"]
impl crate::Readable for STGENC_PIDR1rs {}
#[doc = "`reset()` method sets STGENC_PIDR1 to value 0xb1"]
impl crate::Resettable for STGENC_PIDR1rs {
    const RESET_VALUE: u32 = 0xb1;
}
