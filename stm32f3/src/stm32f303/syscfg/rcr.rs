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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE4_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE4_WP`"]
pub type PAGE4_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE4_WP`"]
pub struct PAGE4_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE4_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE4_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE5_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE5_WP`"]
pub type PAGE5_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE5_WP`"]
pub struct PAGE5_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE5_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE5_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE6_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE6_WP`"]
pub type PAGE6_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE6_WP`"]
pub struct PAGE6_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE6_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE6_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE7_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE7_WP`"]
pub type PAGE7_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE7_WP`"]
pub struct PAGE7_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE7_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE7_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE8_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE8_WP`"]
pub type PAGE8_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE8_WP`"]
pub struct PAGE8_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE8_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE8_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE9_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE9_WP`"]
pub type PAGE9_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE9_WP`"]
pub struct PAGE9_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE9_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE9_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE10_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE10_WP`"]
pub type PAGE10_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE10_WP`"]
pub struct PAGE10_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE10_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE10_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE11_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE11_WP`"]
pub type PAGE11_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE11_WP`"]
pub struct PAGE11_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE11_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE11_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE12_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE12_WP`"]
pub type PAGE12_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE12_WP`"]
pub struct PAGE12_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE12_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE12_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE13_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE13_WP`"]
pub type PAGE13_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE13_WP`"]
pub struct PAGE13_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE13_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE13_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE14_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE14_WP`"]
pub type PAGE14_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE14_WP`"]
pub struct PAGE14_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE14_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE14_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE15_WP_A = PAGE0_WP_A;
#[doc = "Reader of field `PAGE15_WP`"]
pub type PAGE15_WP_R = crate::R<bool, PAGE0_WP_A>;
#[doc = "Write proxy for field `PAGE15_WP`"]
pub struct PAGE15_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE15_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE15_WP_A) -> &'a mut W {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page8_wp(&self) -> PAGE8_WP_R {
        PAGE8_WP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page9_wp(&self) -> PAGE9_WP_R {
        PAGE9_WP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page10_wp(&self) -> PAGE10_WP_R {
        PAGE10_WP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page11_wp(&self) -> PAGE11_WP_R {
        PAGE11_WP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page12_wp(&self) -> PAGE12_WP_R {
        PAGE12_WP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page13_wp(&self) -> PAGE13_WP_R {
        PAGE13_WP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page14_wp(&self) -> PAGE14_WP_R {
        PAGE14_WP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page15_wp(&self) -> PAGE15_WP_R {
        PAGE15_WP_R::new(((self.bits >> 15) & 0x01) != 0)
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
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W {
        PAGE4_WP_W { w: self }
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W {
        PAGE5_WP_W { w: self }
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W {
        PAGE6_WP_W { w: self }
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W {
        PAGE7_WP_W { w: self }
    }
    #[doc = "Bit 8 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page8_wp(&mut self) -> PAGE8_WP_W {
        PAGE8_WP_W { w: self }
    }
    #[doc = "Bit 9 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page9_wp(&mut self) -> PAGE9_WP_W {
        PAGE9_WP_W { w: self }
    }
    #[doc = "Bit 10 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page10_wp(&mut self) -> PAGE10_WP_W {
        PAGE10_WP_W { w: self }
    }
    #[doc = "Bit 11 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page11_wp(&mut self) -> PAGE11_WP_W {
        PAGE11_WP_W { w: self }
    }
    #[doc = "Bit 12 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page12_wp(&mut self) -> PAGE12_WP_W {
        PAGE12_WP_W { w: self }
    }
    #[doc = "Bit 13 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page13_wp(&mut self) -> PAGE13_WP_W {
        PAGE13_WP_W { w: self }
    }
    #[doc = "Bit 14 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page14_wp(&mut self) -> PAGE14_WP_W {
        PAGE14_WP_W { w: self }
    }
    #[doc = "Bit 15 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page15_wp(&mut self) -> PAGE15_WP_W {
        PAGE15_WP_W { w: self }
    }
}
