#[doc = "Register `SECWM2R_CUR` reader"]
pub type R = crate::R<SECWM2R_CURrs>;
#[doc = "Field `SECWM_STRT2` reader - Bank2 security WM area start sector"]
pub type SECWM_STRT2_R = crate::FieldReader;
#[doc = "Field `SECWM_END2` reader - Bank2 security WM end sector"]
pub type SECWM_END2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Bank2 security WM area start sector"]
    #[inline(always)]
    pub fn secwm_strt2(&self) -> SECWM_STRT2_R {
        SECWM_STRT2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank2 security WM end sector"]
    #[inline(always)]
    pub fn secwm_end2(&self) -> SECWM_END2_R {
        SECWM_END2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "FLASH security watermark for Bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm2r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM2R_CURrs;
impl crate::RegisterSpec for SECWM2R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm2r_cur::R`](R) reader structure"]
impl crate::Readable for SECWM2R_CURrs {}
#[doc = "`reset()` method sets SECWM2R_CUR to value 0"]
impl crate::Resettable for SECWM2R_CURrs {
    const RESET_VALUE: u32 = 0;
}
