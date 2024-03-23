#[doc = "Register `ITLINE4` reader"]
pub type R = crate::R<ITLINE4rs>;
#[doc = "Field `RCC` reader - RCC"]
pub type RCC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RCC"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE4rs;
impl crate::RegisterSpec for ITLINE4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline4::R`](R) reader structure"]
impl crate::Readable for ITLINE4rs {}
#[doc = "`reset()` method sets ITLINE4 to value 0"]
impl crate::Resettable for ITLINE4rs {
    const RESET_VALUE: u32 = 0;
}
