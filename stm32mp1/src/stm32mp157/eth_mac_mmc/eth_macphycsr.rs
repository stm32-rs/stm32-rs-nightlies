#[doc = "Reader of register ETH_MACPHYCSR"]
pub type R = crate::R<u32, super::ETH_MACPHYCSR>;
#[doc = "Writer for register ETH_MACPHYCSR"]
pub type W = crate::W<u32, super::ETH_MACPHYCSR>;
#[doc = "Register ETH_MACPHYCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACPHYCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LUD`"]
pub type LUD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LUD`"]
pub struct LUD_W<'a> {
    w: &'a mut W,
}
impl<'a> LUD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LNKMOD`"]
pub type LNKMOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `LNKSPEED`"]
pub type LNKSPEED_R = crate::R<u8, u8>;
#[doc = "Reader of field `LNKSTS`"]
pub type LNKSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `JABTO`"]
pub type JABTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `FALSCARDET`"]
pub type FALSCARDET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LNKMOD"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - LNKSPEED"]
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - LNKSTS"]
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - JABTO"]
    #[inline(always)]
    pub fn jabto(&self) -> JABTO_R {
        JABTO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FALSCARDET"]
    #[inline(always)]
    pub fn falscardet(&self) -> FALSCARDET_R {
        FALSCARDET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 1 - LUD"]
    #[inline(always)]
    pub fn lud(&mut self) -> LUD_W {
        LUD_W { w: self }
    }
}
