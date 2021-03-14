#[doc = "Reader of register RCR"]
pub type R = crate::R<u32, super::RCR>;
#[doc = "Writer for register RCR"]
pub type W = crate::W<u32, super::RCR>;
#[doc = "Register RCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CCM SRAM page write protection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGE0_WP_A {
    #[doc = "0: Write protection of pagex is disabled"]
    DISABLED = 0,
    #[doc = "1: Write protection of pagex is enabled"]
    ENABLED = 1,
}
impl From<PAGE0_WP_A> for bool {
    #[inline(always)]
    fn from(variant: PAGE0_WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAGE0_WP`"]
pub type PAGE0_WP_R = crate::R<bool, PAGE0_WP_A>;
impl PAGE0_WP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGE0_WP_A {
        match self.bits {
            false => PAGE0_WP_A::DISABLED,
            true => PAGE0_WP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAGE0_WP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAGE0_WP_A::ENABLED
    }
}
#[doc = "Write proxy for field `PAGE0_WP`"]
pub struct PAGE0_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE0_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE0_WP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::ENABLED)
    }
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE1_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE1_WP`"]
pub type PAGE1_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE1_WP`"]
pub struct PAGE1_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE1_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE1_WP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::ENABLED)
    }
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE2_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE2_WP`"]
pub type PAGE2_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE2_WP`"]
pub struct PAGE2_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE2_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE2_WP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::ENABLED)
    }
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE3_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE3_WP`"]
pub type PAGE3_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE3_WP`"]
pub struct PAGE3_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE3_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE3_WP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::ENABLED)
    }
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
impl R {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W {
        PAGE0_WP_W { w: self }
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W {
        PAGE1_WP_W { w: self }
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W {
        PAGE2_WP_W { w: self }
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W {
        PAGE3_WP_W { w: self }
    }
}
