#[doc = "Register `STGENC_CIDR2` reader"]
pub type R = crate::R<STGENC_CIDR2rs>;
#[doc = "Field `PRMBL_2` reader - PRMBL_2"]
pub type PRMBL_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PRMBL_2"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENC component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_cidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_CIDR2rs;
impl crate::RegisterSpec for STGENC_CIDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_cidr2::R`](R) reader structure"]
impl crate::Readable for STGENC_CIDR2rs {}
#[doc = "`reset()` method sets STGENC_CIDR2 to value 0x50"]
impl crate::Resettable for STGENC_CIDR2rs {
    const RESET_VALUE: u32 = 0x50;
}
