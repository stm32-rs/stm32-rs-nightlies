#[doc = "Register `HDP2R_CUR` reader"]
pub type R = crate::R<HDP2R_CURrs>;
#[doc = "Field `HDP2_STRT` reader - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
pub type HDP2_STRT_R = crate::FieldReader;
#[doc = "Field `HDP2_END` reader - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
pub type HDP2_END_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn hdp2_strt(&self) -> HDP2_STRT_R {
        HDP2_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn hdp2_end(&self) -> HDP2_END_R {
        HDP2_END_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "FLASH HDP Bank2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp2r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP2R_CURrs;
impl crate::RegisterSpec for HDP2R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp2r_cur::R`](R) reader structure"]
impl crate::Readable for HDP2R_CURrs {}
#[doc = "`reset()` method sets HDP2R_CUR to value 0"]
impl crate::Resettable for HDP2R_CURrs {
    const RESET_VALUE: u32 = 0;
}
