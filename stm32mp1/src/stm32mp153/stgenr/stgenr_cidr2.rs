#[doc = "Register `STGENR_CIDR2` reader"]
pub type R = crate::R<STGENR_CIDR2rs>;
#[doc = "Field `PRMBL_2` reader - PRMBL_2"]
pub type PRMBL_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PRMBL_2"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENR component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_CIDR2rs;
impl crate::RegisterSpec for STGENR_CIDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_cidr2::R`](R) reader structure"]
impl crate::Readable for STGENR_CIDR2rs {}
#[doc = "`reset()` method sets STGENR_CIDR2 to value 0x50"]
impl crate::Resettable for STGENR_CIDR2rs {
    const RESET_VALUE: u32 = 0x50;
}
