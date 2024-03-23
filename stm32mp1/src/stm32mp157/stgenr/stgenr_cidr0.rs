#[doc = "Register `STGENR_CIDR0` reader"]
pub type R = crate::R<STGENR_CIDR0rs>;
#[doc = "Field `PRMBL_0` reader - PRMBL_0"]
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PRMBL_0"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENR component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_CIDR0rs;
impl crate::RegisterSpec for STGENR_CIDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_cidr0::R`](R) reader structure"]
impl crate::Readable for STGENR_CIDR0rs {}
#[doc = "`reset()` method sets STGENR_CIDR0 to value 0x0d"]
impl crate::Resettable for STGENR_CIDR0rs {
    const RESET_VALUE: u32 = 0x0d;
}
