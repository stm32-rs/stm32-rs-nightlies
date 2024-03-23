#[doc = "Register `STGENR_CIDR3` reader"]
pub type R = crate::R<STGENR_CIDR3rs>;
#[doc = "Field `PRMBL_3` reader - PRMBL_3"]
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PRMBL_3"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENR component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_CIDR3rs;
impl crate::RegisterSpec for STGENR_CIDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_cidr3::R`](R) reader structure"]
impl crate::Readable for STGENR_CIDR3rs {}
#[doc = "`reset()` method sets STGENR_CIDR3 to value 0xb1"]
impl crate::Resettable for STGENR_CIDR3rs {
    const RESET_VALUE: u32 = 0xb1;
}
