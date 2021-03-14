#[doc = "Reader of register FAR"]
pub type R = crate::R<u32, super::FAR>;
#[doc = "Writer for register FAR"]
pub type W = crate::W<u32, super::FAR>;
#[doc = "Register FAR `reset()`'s with value 0"]
impl crate::ResetValue for super::FAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAIL_ECC_ADDR`"]
pub type FAIL_ECC_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FAIL_ECC_ADDR`"]
pub struct FAIL_ECC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FAIL_ECC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&mut self) -> FAIL_ECC_ADDR_W {
        FAIL_ECC_ADDR_W { w: self }
    }
}
