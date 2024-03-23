#[doc = "Register `HDP1R_CUR` reader"]
pub type R = crate::R<HDP1R_CURrs>;
#[doc = "Field `HDP1_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors"]
pub type HDP1_STRT_R = crate::FieldReader;
#[doc = "Field `HDP1_END` reader - HDPL barrier end set in number of 8-Kbyte sectors"]
pub type HDP1_END_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors"]
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors"]
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "FLASH HDP Bank 1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp1r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP1R_CURrs;
impl crate::RegisterSpec for HDP1R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp1r_cur::R`](R) reader structure"]
impl crate::Readable for HDP1R_CURrs {}
#[doc = "`reset()` method sets HDP1R_CUR to value 0"]
impl crate::Resettable for HDP1R_CURrs {
    const RESET_VALUE: u32 = 0;
}
