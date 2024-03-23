#[doc = "Register `ITLINE31` reader"]
pub type R = crate::R<ITLINE31rs>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::BitReader;
#[doc = "Field `AES` reader - AES"]
pub type AES_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 31 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline31::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE31rs;
impl crate::RegisterSpec for ITLINE31rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline31::R`](R) reader structure"]
impl crate::Readable for ITLINE31rs {}
#[doc = "`reset()` method sets ITLINE31 to value 0"]
impl crate::Resettable for ITLINE31rs {
    const RESET_VALUE: u32 = 0;
}
