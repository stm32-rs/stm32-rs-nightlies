#[doc = "Reader of register SDMMC_IDMABASER"]
pub type R = crate::R<u32, super::SDMMC_IDMABASER>;
#[doc = "Writer for register SDMMC_IDMABASER"]
pub type W = crate::W<u32, super::SDMMC_IDMABASER>;
#[doc = "Register SDMMC_IDMABASER `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_IDMABASER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABASE`"]
pub type IDMABASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDMABASE`"]
pub struct IDMABASE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IDMABASE"]
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDMABASE"]
    #[inline(always)]
    pub fn idmabase(&mut self) -> IDMABASE_W {
        IDMABASE_W { w: self }
    }
}
