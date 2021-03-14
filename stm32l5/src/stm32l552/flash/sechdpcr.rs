#[doc = "Reader of register SECHDPCR"]
pub type R = crate::R<u32, super::SECHDPCR>;
#[doc = "Writer for register SECHDPCR"]
pub type W = crate::W<u32, super::SECHDPCR>;
#[doc = "Register SECHDPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECHDPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HDP1_ACCDIS`"]
pub type HDP1_ACCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDP1_ACCDIS`"]
pub struct HDP1_ACCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP1_ACCDIS_W<'a> {
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
#[doc = "Reader of field `HDP2_ACCDIS`"]
pub type HDP2_ACCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDP2_ACCDIS`"]
pub struct HDP2_ACCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP2_ACCDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HDP1_ACCDIS"]
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HDP2_ACCDIS"]
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDP1_ACCDIS"]
    #[inline(always)]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W {
        HDP1_ACCDIS_W { w: self }
    }
    #[doc = "Bit 1 - HDP2_ACCDIS"]
    #[inline(always)]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W {
        HDP2_ACCDIS_W { w: self }
    }
}
