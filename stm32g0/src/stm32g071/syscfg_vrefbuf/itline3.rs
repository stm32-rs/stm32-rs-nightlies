#[doc = "Reader of register ITLINE3"]
pub type R = crate::R<u32, super::ITLINE3>;
#[doc = "Reader of field `FLASH_ITF`"]
pub type FLASH_ITF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASH_ECC`"]
pub type FLASH_ECC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FLASH_ITF"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLASH_ECC"]
    #[inline(always)]
    pub fn flash_ecc(&self) -> FLASH_ECC_R {
        FLASH_ECC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
