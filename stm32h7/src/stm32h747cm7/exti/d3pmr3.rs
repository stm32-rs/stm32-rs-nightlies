#[doc = "Reader of register D3PMR3"]
pub type R = crate::R<u32, super::D3PMR3>;
#[doc = "Writer for register D3PMR3"]
pub type W = crate::W<u32, super::D3PMR3>;
#[doc = "Register D3PMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "D3 Pending Mask on Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR88_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<MR88_A> for bool {
    #[inline(always)]
    fn from(variant: MR88_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR88`"]
pub type MR88_R = crate::R<bool, MR88_A>;
impl MR88_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR88_A {
        match self.bits {
            false => MR88_A::MASKED,
            true => MR88_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR88_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR88_A::UNMASKED
    }
}
#[doc = "Write proxy for field `MR88`"]
pub struct MR88_W<'a> {
    w: &'a mut W,
}
impl<'a> MR88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR88_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR88_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR88_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&mut self) -> MR88_W {
        MR88_W { w: self }
    }
}
