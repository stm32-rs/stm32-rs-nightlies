#[doc = "Reader of register SUBGHZSPICR"]
pub type R = crate::R<u32, super::SUBGHZSPICR>;
#[doc = "Writer for register SUBGHZSPICR"]
pub type W = crate::W<u32, super::SUBGHZSPICR>;
#[doc = "Register SUBGHZSPICR `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::SUBGHZSPICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `NSS`"]
pub type NSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSS`"]
pub struct NSS_W<'a> {
    w: &'a mut W,
}
impl<'a> NSS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    pub fn nss(&mut self) -> NSS_W {
        NSS_W { w: self }
    }
}
