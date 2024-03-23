#[doc = "Register `AHB3SECSR` reader"]
pub type R = crate::R<AHB3SECSRrs>;
#[doc = "Field `FSMCSECF` reader - FSMCSECF"]
pub type FSMCSECF_R = crate::BitReader;
#[doc = "Field `OSPI1SECF` reader - OSPI1SECF"]
pub type OSPI1SECF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FSMCSECF"]
    #[inline(always)]
    pub fn fsmcsecf(&self) -> FSMCSECF_R {
        FSMCSECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OSPI1SECF"]
    #[inline(always)]
    pub fn ospi1secf(&self) -> OSPI1SECF_R {
        OSPI1SECF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RCC AHB3 security status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3secsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3SECSRrs;
impl crate::RegisterSpec for AHB3SECSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3secsr::R`](R) reader structure"]
impl crate::Readable for AHB3SECSRrs {}
#[doc = "`reset()` method sets AHB3SECSR to value 0"]
impl crate::Resettable for AHB3SECSRrs {
    const RESET_VALUE: u32 = 0;
}
