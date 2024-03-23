#[doc = "Register `STGENR_CIDR1` reader"]
pub type R = crate::R<STGENR_CIDR1rs>;
#[doc = "Field `PRMBL_1` reader - PRMBL_1"]
pub type PRMBL_1_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - CLASS"]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PRMBL_1"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLASS"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENR component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_CIDR1rs;
impl crate::RegisterSpec for STGENR_CIDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_cidr1::R`](R) reader structure"]
impl crate::Readable for STGENR_CIDR1rs {}
#[doc = "`reset()` method sets STGENR_CIDR1 to value 0xf0"]
impl crate::Resettable for STGENR_CIDR1rs {
    const RESET_VALUE: u32 = 0xf0;
}
