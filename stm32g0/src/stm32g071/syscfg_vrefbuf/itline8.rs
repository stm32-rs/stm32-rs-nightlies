#[doc = "Register `ITLINE8` reader"]
pub type R = crate::R<ITLINE8rs>;
#[doc = "Field `UCPD1` reader - UCPD1"]
pub type UCPD1_R = crate::BitReader;
#[doc = "Field `UCPD2` reader - UCPD2"]
pub type UCPD2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UCPD1"]
    #[inline(always)]
    pub fn ucpd1(&self) -> UCPD1_R {
        UCPD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCPD2"]
    #[inline(always)]
    pub fn ucpd2(&self) -> UCPD2_R {
        UCPD2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 8 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE8rs;
impl crate::RegisterSpec for ITLINE8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline8::R`](R) reader structure"]
impl crate::Readable for ITLINE8rs {}
#[doc = "`reset()` method sets ITLINE8 to value 0"]
impl crate::Resettable for ITLINE8rs {
    const RESET_VALUE: u32 = 0;
}
