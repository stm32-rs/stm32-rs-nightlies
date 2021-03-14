#[doc = "Reader of register LDO"]
pub type R = crate::R<u32, super::LDO>;
#[doc = "Writer for register LDO"]
pub type W = crate::W<u32, super::LDO>;
#[doc = "Register LDO `reset()`'s with value 0x01"]
impl crate::ResetValue for super::LDO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `LDO_USED`"]
pub type LDO_USED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LDO_STATUS`"]
pub type LDO_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LDO_DISABLE`"]
pub type LDO_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDO_DISABLE`"]
pub struct LDO_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DISABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Indicates the presence of the LDO in the chip"]
    #[inline(always)]
    pub fn ldo_used(&self) -> LDO_USED_R {
        LDO_USED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Monitors the status of the PHY's LDO"]
    #[inline(always)]
    pub fn ldo_status(&self) -> LDO_STATUS_R {
        LDO_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls disable of the High Speed PHY's LDO"]
    #[inline(always)]
    pub fn ldo_disable(&self) -> LDO_DISABLE_R {
        LDO_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Controls disable of the High Speed PHY's LDO"]
    #[inline(always)]
    pub fn ldo_disable(&mut self) -> LDO_DISABLE_W {
        LDO_DISABLE_W { w: self }
    }
}
