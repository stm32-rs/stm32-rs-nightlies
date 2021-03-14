#[doc = "Reader of register INI3_FN_MOD"]
pub type R = crate::R<u32, super::INI3_FN_MOD>;
#[doc = "Writer for register INI3_FN_MOD"]
pub type W = crate::W<u32, super::INI3_FN_MOD>;
#[doc = "Register INI3_FN_MOD `reset()`'s with value 0x04"]
impl crate::ResetValue for super::INI3_FN_MOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Override ASIB read issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_ISS_OVERRIDE_A {
    #[doc = "0: Normal ASIB read issuing capability"]
    NORMAL = 0,
    #[doc = "1: Force ASIB read issuing capability to 1"]
    FORCE1 = 1,
}
impl From<READ_ISS_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_ISS_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READ_ISS_OVERRIDE`"]
pub type READ_ISS_OVERRIDE_R = crate::R<bool, READ_ISS_OVERRIDE_A>;
impl READ_ISS_OVERRIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_ISS_OVERRIDE_A {
        match self.bits {
            false => READ_ISS_OVERRIDE_A::NORMAL,
            true => READ_ISS_OVERRIDE_A::FORCE1,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == READ_ISS_OVERRIDE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == READ_ISS_OVERRIDE_A::FORCE1
    }
}
#[doc = "Write proxy for field `READ_ISS_OVERRIDE`"]
pub struct READ_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ISS_OVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_ISS_OVERRIDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal ASIB read issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(READ_ISS_OVERRIDE_A::NORMAL)
    }
    #[doc = "Force ASIB read issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(READ_ISS_OVERRIDE_A::FORCE1)
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
#[doc = "Override ASIB write issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_ISS_OVERRIDE_A {
    #[doc = "0: Normal ASIB write issuing capability"]
    NORMAL = 0,
    #[doc = "1: Force ASIB write issuing capability to 1"]
    FORCE1 = 1,
}
impl From<WRITE_ISS_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_ISS_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRITE_ISS_OVERRIDE`"]
pub type WRITE_ISS_OVERRIDE_R = crate::R<bool, WRITE_ISS_OVERRIDE_A>;
impl WRITE_ISS_OVERRIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_ISS_OVERRIDE_A {
        match self.bits {
            false => WRITE_ISS_OVERRIDE_A::NORMAL,
            true => WRITE_ISS_OVERRIDE_A::FORCE1,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE_A::FORCE1
    }
}
#[doc = "Write proxy for field `WRITE_ISS_OVERRIDE`"]
pub struct WRITE_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ISS_OVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_ISS_OVERRIDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal ASIB write issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WRITE_ISS_OVERRIDE_A::NORMAL)
    }
    #[doc = "Force ASIB write issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(WRITE_ISS_OVERRIDE_A::FORCE1)
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
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W {
        READ_ISS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W {
        WRITE_ISS_OVERRIDE_W { w: self }
    }
}
