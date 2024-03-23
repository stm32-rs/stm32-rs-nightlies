#[doc = "Register `ITLINE3` reader"]
pub type R = crate::R<ITLINE3rs>;
#[doc = "Field `FLASH_ITF` reader - FLASH_ITF"]
pub type FLASH_ITF_R = crate::BitReader;
#[doc = "Field `FLASH_ECC` reader - FLASH_ECC"]
pub type FLASH_ECC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FLASH_ITF"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH_ECC"]
    #[inline(always)]
    pub fn flash_ecc(&self) -> FLASH_ECC_R {
        FLASH_ECC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE3rs;
impl crate::RegisterSpec for ITLINE3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline3::R`](R) reader structure"]
impl crate::Readable for ITLINE3rs {}
#[doc = "`reset()` method sets ITLINE3 to value 0"]
impl crate::Resettable for ITLINE3rs {
    const RESET_VALUE: u32 = 0;
}
