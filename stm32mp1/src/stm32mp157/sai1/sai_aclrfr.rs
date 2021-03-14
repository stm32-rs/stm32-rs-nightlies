#[doc = "Writer for register SAI_ACLRFR"]
pub type W = crate::W<u32, super::SAI_ACLRFR>;
#[doc = "Register SAI_ACLRFR `reset()`'s with value 0"]
impl crate::ResetValue for super::SAI_ACLRFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `COVRUDR`"]
pub struct COVRUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> COVRUDR_W<'a> {
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
#[doc = "Write proxy for field `CMUTEDET`"]
pub struct CMUTEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUTEDET_W<'a> {
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
#[doc = "Write proxy for field `CWCKCFG`"]
pub struct CWCKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CWCKCFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CCNRDY`"]
pub struct CCNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCNRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `CAFSDET`"]
pub struct CAFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAFSDET_W<'a> {
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
#[doc = "Write proxy for field `CLFSDET`"]
pub struct CLFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLFSDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - COVRUDR"]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W {
        COVRUDR_W { w: self }
    }
    #[doc = "Bit 1 - CMUTEDET"]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W {
        CMUTEDET_W { w: self }
    }
    #[doc = "Bit 2 - CWCKCFG"]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W {
        CWCKCFG_W { w: self }
    }
    #[doc = "Bit 4 - CCNRDY"]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W {
        CCNRDY_W { w: self }
    }
    #[doc = "Bit 5 - CAFSDET"]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W {
        CAFSDET_W { w: self }
    }
    #[doc = "Bit 6 - CLFSDET"]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W {
        CLFSDET_W { w: self }
    }
}
