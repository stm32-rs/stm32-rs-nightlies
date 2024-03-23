#[doc = "Register `SECWM1R_CUR` reader"]
pub type R = crate::R<SECWM1R_CURrs>;
#[doc = "Field `SECWM1_STRT` reader - Bank1 security WM area 1 start sector"]
pub type SECWM1_STRT_R = crate::FieldReader;
#[doc = "Field `SECWM1_END` reader - Bank1 security WM area 1 end sector"]
pub type SECWM1_END_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Bank1 security WM area 1 start sector"]
    #[inline(always)]
    pub fn secwm1_strt(&self) -> SECWM1_STRT_R {
        SECWM1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank1 security WM area 1 end sector"]
    #[inline(always)]
    pub fn secwm1_end(&self) -> SECWM1_END_R {
        SECWM1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "FLASH security watermark for Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secwm1r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECWM1R_CURrs;
impl crate::RegisterSpec for SECWM1R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secwm1r_cur::R`](R) reader structure"]
impl crate::Readable for SECWM1R_CURrs {}
#[doc = "`reset()` method sets SECWM1R_CUR to value 0"]
impl crate::Resettable for SECWM1R_CURrs {
    const RESET_VALUE: u32 = 0;
}
