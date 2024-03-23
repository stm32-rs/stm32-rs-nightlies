#[doc = "Register `HDP1R_PRG` reader"]
pub type R = crate::R<HDP1R_PRGrs>;
#[doc = "Field `HDP1_STRT` reader - Bank 1 HDPL barrier start set in number of 8 Kbytes sectors"]
pub type HDP1_STRT_R = crate::FieldReader;
#[doc = "Field `HDP1_END` reader - Bank 1 HDPL barrier end set in number of 8 Kbytes sectors"]
pub type HDP1_END_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bank 1 HDPL barrier start set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Bank 1 HDPL barrier end set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "FLASH HDP Bank1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp1r_prg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP1R_PRGrs;
impl crate::RegisterSpec for HDP1R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp1r_prg::R`](R) reader structure"]
impl crate::Readable for HDP1R_PRGrs {}
#[doc = "`reset()` method sets HDP1R_PRG to value 0"]
impl crate::Resettable for HDP1R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
