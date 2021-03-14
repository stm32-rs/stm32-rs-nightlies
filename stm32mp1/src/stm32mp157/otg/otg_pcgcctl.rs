#[doc = "Reader of register OTG_PCGCCTL"]
pub type R = crate::R<u32, super::OTG_PCGCCTL>;
#[doc = "Writer for register OTG_PCGCCTL"]
pub type W = crate::W<u32, super::OTG_PCGCCTL>;
#[doc = "Register OTG_PCGCCTL `reset()`'s with value 0x200b_8000"]
impl crate::ResetValue for super::OTG_PCGCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x200b_8000
    }
}
#[doc = "Reader of field `STPPCLK`"]
pub type STPPCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPPCLK`"]
pub struct STPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPPCLK_W<'a> {
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
#[doc = "Reader of field `GATEHCLK`"]
pub type GATEHCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GATEHCLK`"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
#[doc = "Reader of field `PHYSUSP`"]
pub type PHYSUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENL1GTG`"]
pub type ENL1GTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENL1GTG`"]
pub struct ENL1GTG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENL1GTG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PHYSLEEP`"]
pub type PHYSLEEP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHYSUSP"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PHYSLEEP"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W {
        STPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W {
        ENL1GTG_W { w: self }
    }
}
