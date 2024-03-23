#[doc = "Register `ITLINE18` reader"]
pub type R = crate::R<ITLINE18rs>;
#[doc = "Field `LPTIM2` reader - LPTIM2"]
pub type LPTIM2_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - LPTIM2"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 18 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline18::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE18rs;
impl crate::RegisterSpec for ITLINE18rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline18::R`](R) reader structure"]
impl crate::Readable for ITLINE18rs {}
#[doc = "`reset()` method sets ITLINE18 to value 0"]
impl crate::Resettable for ITLINE18rs {
    const RESET_VALUE: u32 = 0;
}
