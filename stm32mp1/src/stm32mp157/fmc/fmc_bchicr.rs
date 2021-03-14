#[doc = "Writer for register FMC_BCHICR"]
pub type W = crate::W<u32, super::FMC_BCHICR>;
#[doc = "Register FMC_BCHICR `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_BCHICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CDUEF`"]
pub struct CDUEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDUEF_W<'a> {
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
#[doc = "Write proxy for field `CDERF`"]
pub struct CDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDERF_W<'a> {
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
#[doc = "Write proxy for field `CDEFF`"]
pub struct CDEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEFF_W<'a> {
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
#[doc = "Write proxy for field `CDSRF`"]
pub struct CDSRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSRF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CEPBRF`"]
pub struct CEPBRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPBRF_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CDUEF"]
    #[inline(always)]
    pub fn cduef(&mut self) -> CDUEF_W {
        CDUEF_W { w: self }
    }
    #[doc = "Bit 1 - CDERF"]
    #[inline(always)]
    pub fn cderf(&mut self) -> CDERF_W {
        CDERF_W { w: self }
    }
    #[doc = "Bit 2 - CDEFF"]
    #[inline(always)]
    pub fn cdeff(&mut self) -> CDEFF_W {
        CDEFF_W { w: self }
    }
    #[doc = "Bit 3 - CDSRF"]
    #[inline(always)]
    pub fn cdsrf(&mut self) -> CDSRF_W {
        CDSRF_W { w: self }
    }
    #[doc = "Bit 4 - CEPBRF"]
    #[inline(always)]
    pub fn cepbrf(&mut self) -> CEPBRF_W {
        CEPBRF_W { w: self }
    }
}
