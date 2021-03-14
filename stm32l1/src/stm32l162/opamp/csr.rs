#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0x0001_0101"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0101
    }
}
#[doc = "Reader of field `OPA3CALOUT`"]
pub type OPA3CALOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3CALOUT`"]
pub struct OPA3CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3CALOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `OPA2CALOUT`"]
pub type OPA2CALOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2CALOUT`"]
pub struct OPA2CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2CALOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `OPA1CALOUT`"]
pub type OPA1CALOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1CALOUT`"]
pub struct OPA1CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1CALOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `AOP_RANGE`"]
pub type AOP_RANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AOP_RANGE`"]
pub struct AOP_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> AOP_RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `S7SEL2`"]
pub type S7SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S7SEL2`"]
pub struct S7SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S7SEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ANAWSEL3`"]
pub type ANAWSEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANAWSEL3`"]
pub struct ANAWSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAWSEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ANAWSEL2`"]
pub type ANAWSEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANAWSEL2`"]
pub struct ANAWSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAWSEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ANAWSEL1`"]
pub type ANAWSEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANAWSEL1`"]
pub struct ANAWSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAWSEL1_W<'a> {
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
#[doc = "Reader of field `OPA3LPM`"]
pub type OPA3LPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3LPM`"]
pub struct OPA3LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3LPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `OPA3CAL_H`"]
pub type OPA3CAL_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3CAL_H`"]
pub struct OPA3CAL_H_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3CAL_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `OPA3CAL_L`"]
pub type OPA3CAL_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3CAL_L`"]
pub struct OPA3CAL_L_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3CAL_L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `S6SEL3`"]
pub type S6SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S6SEL3`"]
pub struct S6SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S6SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `S5SEL3`"]
pub type S5SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S5SEL3`"]
pub struct S5SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S5SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `S4SEL3`"]
pub type S4SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S4SEL3`"]
pub struct S4SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S4SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `S3SEL3`"]
pub type S3SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3SEL3`"]
pub struct S3SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OPA3PD`"]
pub type OPA3PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA3PD`"]
pub struct OPA3PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OPA2LPM`"]
pub type OPA2LPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2LPM`"]
pub struct OPA2LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2LPM_W<'a> {
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
#[doc = "Reader of field `OPA2CAL_H`"]
pub type OPA2CAL_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2CAL_H`"]
pub struct OPA2CAL_H_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2CAL_H_W<'a> {
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
#[doc = "Reader of field `OPA2CAL_L`"]
pub type OPA2CAL_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2CAL_L`"]
pub struct OPA2CAL_L_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2CAL_L_W<'a> {
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
#[doc = "Reader of field `S6SEL2`"]
pub type S6SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S6SEL2`"]
pub struct S6SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S6SEL2_W<'a> {
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
#[doc = "Reader of field `S5SEL2`"]
pub type S5SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S5SEL2`"]
pub struct S5SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S5SEL2_W<'a> {
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
#[doc = "Reader of field `S4SEL2`"]
pub type S4SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S4SEL2`"]
pub struct S4SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S4SEL2_W<'a> {
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
#[doc = "Reader of field `S3SEL2`"]
pub type S3SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3SEL2`"]
pub struct S3SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SEL2_W<'a> {
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
#[doc = "Reader of field `OPA2PD`"]
pub type OPA2PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA2PD`"]
pub struct OPA2PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2PD_W<'a> {
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
#[doc = "Reader of field `OPA1LPM`"]
pub type OPA1LPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1LPM`"]
pub struct OPA1LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1LPM_W<'a> {
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
#[doc = "Reader of field `OPA1CAL_H`"]
pub type OPA1CAL_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1CAL_H`"]
pub struct OPA1CAL_H_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1CAL_H_W<'a> {
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
#[doc = "Reader of field `OPA1CAL_L`"]
pub type OPA1CAL_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1CAL_L`"]
pub struct OPA1CAL_L_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1CAL_L_W<'a> {
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
#[doc = "Reader of field `S6SEL1`"]
pub type S6SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S6SEL1`"]
pub struct S6SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S6SEL1_W<'a> {
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
#[doc = "Reader of field `S5SEL1`"]
pub type S5SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S5SEL1`"]
pub struct S5SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S5SEL1_W<'a> {
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
#[doc = "Reader of field `S4SEL1`"]
pub type S4SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S4SEL1`"]
pub struct S4SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S4SEL1_W<'a> {
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
#[doc = "Reader of field `S3SEL1`"]
pub type S3SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S3SEL1`"]
pub struct S3SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SEL1_W<'a> {
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
#[doc = "Reader of field `OPA1PD`"]
pub type OPA1PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1PD`"]
pub struct OPA1PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1PD_W<'a> {
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
impl R {
    #[doc = "Bit 31 - OPAMP3 calibration output"]
    #[inline(always)]
    pub fn opa3calout(&self) -> OPA3CALOUT_R {
        OPA3CALOUT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPAMP2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&self) -> OPA2CALOUT_R {
        OPA2CALOUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OPAMP1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&self) -> OPA1CALOUT_R {
        OPA1CALOUT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power range selection"]
    #[inline(always)]
    pub fn aop_range(&self) -> AOP_RANGE_R {
        AOP_RANGE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Switch 7 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s7sel2(&self) -> S7SEL2_R {
        S7SEL2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Switch SanA enable for OPAMP3"]
    #[inline(always)]
    pub fn anawsel3(&self) -> ANAWSEL3_R {
        ANAWSEL3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Switch SanA enable for OPAMP2"]
    #[inline(always)]
    pub fn anawsel2(&self) -> ANAWSEL2_R {
        ANAWSEL2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Switch SanA enable for OPAMP1"]
    #[inline(always)]
    pub fn anawsel1(&self) -> ANAWSEL1_R {
        ANAWSEL1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OPAMP3 low power mode"]
    #[inline(always)]
    pub fn opa3lpm(&self) -> OPA3LPM_R {
        OPA3LPM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OPAMP3 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa3cal_h(&self) -> OPA3CAL_H_R {
        OPA3CAL_H_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OPAMP3 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa3cal_l(&self) -> OPA3CAL_L_R {
        OPA3CAL_L_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Switch 6 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s6sel3(&self) -> S6SEL3_R {
        S6SEL3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Switch 5 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s5sel3(&self) -> S5SEL3_R {
        S5SEL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Switch 4 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s4sel3(&self) -> S4SEL3_R {
        S4SEL3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Switch 3 for OPAMP3 Enable"]
    #[inline(always)]
    pub fn s3sel3(&self) -> S3SEL3_R {
        S3SEL3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OPAMP3 power down"]
    #[inline(always)]
    pub fn opa3pd(&self) -> OPA3PD_R {
        OPA3PD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - OPAMP2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&self) -> OPA2LPM_R {
        OPA2LPM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - OPAMP2 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa2cal_h(&self) -> OPA2CAL_H_R {
        OPA2CAL_H_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - OPAMP2 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa2cal_l(&self) -> OPA2CAL_L_R {
        OPA2CAL_L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Switch 6 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s6sel2(&self) -> S6SEL2_R {
        S6SEL2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Switch 5 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s5sel2(&self) -> S5SEL2_R {
        S5SEL2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Switch 4 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s4sel2(&self) -> S4SEL2_R {
        S4SEL2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Switch 3 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s3sel2(&self) -> S3SEL2_R {
        S3SEL2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAMP2 power down"]
    #[inline(always)]
    pub fn opa2pd(&self) -> OPA2PD_R {
        OPA2PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OPAMP1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&self) -> OPA1LPM_R {
        OPA1LPM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OPAMP1 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa1cal_h(&self) -> OPA1CAL_H_R {
        OPA1CAL_H_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OPAMP1 offset calibration for P differential pair"]
    #[inline(always)]
    pub fn opa1cal_l(&self) -> OPA1CAL_L_R {
        OPA1CAL_L_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Switch 6 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s6sel1(&self) -> S6SEL1_R {
        S6SEL1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Switch 5 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s5sel1(&self) -> S5SEL1_R {
        S5SEL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Switch 4 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s4sel1(&self) -> S4SEL1_R {
        S4SEL1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Switch 3 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s3sel1(&self) -> S3SEL1_R {
        S3SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OPAMP1 power down"]
    #[inline(always)]
    pub fn opa1pd(&self) -> OPA1PD_R {
        OPA1PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - OPAMP3 calibration output"]
    #[inline(always)]
    pub fn opa3calout(&mut self) -> OPA3CALOUT_W {
        OPA3CALOUT_W { w: self }
    }
    #[doc = "Bit 30 - OPAMP2 calibration output"]
    #[inline(always)]
    pub fn opa2calout(&mut self) -> OPA2CALOUT_W {
        OPA2CALOUT_W { w: self }
    }
    #[doc = "Bit 29 - OPAMP1 calibration output"]
    #[inline(always)]
    pub fn opa1calout(&mut self) -> OPA1CALOUT_W {
        OPA1CALOUT_W { w: self }
    }
    #[doc = "Bit 28 - Power range selection"]
    #[inline(always)]
    pub fn aop_range(&mut self) -> AOP_RANGE_W {
        AOP_RANGE_W { w: self }
    }
    #[doc = "Bit 27 - Switch 7 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s7sel2(&mut self) -> S7SEL2_W {
        S7SEL2_W { w: self }
    }
    #[doc = "Bit 26 - Switch SanA enable for OPAMP3"]
    #[inline(always)]
    pub fn anawsel3(&mut self) -> ANAWSEL3_W {
        ANAWSEL3_W { w: self }
    }
    #[doc = "Bit 25 - Switch SanA enable for OPAMP2"]
    #[inline(always)]
    pub fn anawsel2(&mut self) -> ANAWSEL2_W {
        ANAWSEL2_W { w: self }
    }
    #[doc = "Bit 24 - Switch SanA enable for OPAMP1"]
    #[inline(always)]
    pub fn anawsel1(&mut self) -> ANAWSEL1_W {
        ANAWSEL1_W { w: self }
    }
    #[doc = "Bit 23 - OPAMP3 low power mode"]
    #[inline(always)]
    pub fn opa3lpm(&mut self) -> OPA3LPM_W {
        OPA3LPM_W { w: self }
    }
    #[doc = "Bit 22 - OPAMP3 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa3cal_h(&mut self) -> OPA3CAL_H_W {
        OPA3CAL_H_W { w: self }
    }
    #[doc = "Bit 21 - OPAMP3 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa3cal_l(&mut self) -> OPA3CAL_L_W {
        OPA3CAL_L_W { w: self }
    }
    #[doc = "Bit 20 - Switch 6 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s6sel3(&mut self) -> S6SEL3_W {
        S6SEL3_W { w: self }
    }
    #[doc = "Bit 19 - Switch 5 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s5sel3(&mut self) -> S5SEL3_W {
        S5SEL3_W { w: self }
    }
    #[doc = "Bit 18 - Switch 4 for OPAMP3 enable"]
    #[inline(always)]
    pub fn s4sel3(&mut self) -> S4SEL3_W {
        S4SEL3_W { w: self }
    }
    #[doc = "Bit 17 - Switch 3 for OPAMP3 Enable"]
    #[inline(always)]
    pub fn s3sel3(&mut self) -> S3SEL3_W {
        S3SEL3_W { w: self }
    }
    #[doc = "Bit 16 - OPAMP3 power down"]
    #[inline(always)]
    pub fn opa3pd(&mut self) -> OPA3PD_W {
        OPA3PD_W { w: self }
    }
    #[doc = "Bit 15 - OPAMP2 low power mode"]
    #[inline(always)]
    pub fn opa2lpm(&mut self) -> OPA2LPM_W {
        OPA2LPM_W { w: self }
    }
    #[doc = "Bit 14 - OPAMP2 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa2cal_h(&mut self) -> OPA2CAL_H_W {
        OPA2CAL_H_W { w: self }
    }
    #[doc = "Bit 13 - OPAMP2 offset Calibration for P differential pair"]
    #[inline(always)]
    pub fn opa2cal_l(&mut self) -> OPA2CAL_L_W {
        OPA2CAL_L_W { w: self }
    }
    #[doc = "Bit 12 - Switch 6 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s6sel2(&mut self) -> S6SEL2_W {
        S6SEL2_W { w: self }
    }
    #[doc = "Bit 11 - Switch 5 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s5sel2(&mut self) -> S5SEL2_W {
        S5SEL2_W { w: self }
    }
    #[doc = "Bit 10 - Switch 4 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s4sel2(&mut self) -> S4SEL2_W {
        S4SEL2_W { w: self }
    }
    #[doc = "Bit 9 - Switch 3 for OPAMP2 enable"]
    #[inline(always)]
    pub fn s3sel2(&mut self) -> S3SEL2_W {
        S3SEL2_W { w: self }
    }
    #[doc = "Bit 8 - OPAMP2 power down"]
    #[inline(always)]
    pub fn opa2pd(&mut self) -> OPA2PD_W {
        OPA2PD_W { w: self }
    }
    #[doc = "Bit 7 - OPAMP1 low power mode"]
    #[inline(always)]
    pub fn opa1lpm(&mut self) -> OPA1LPM_W {
        OPA1LPM_W { w: self }
    }
    #[doc = "Bit 6 - OPAMP1 offset calibration for N differential pair"]
    #[inline(always)]
    pub fn opa1cal_h(&mut self) -> OPA1CAL_H_W {
        OPA1CAL_H_W { w: self }
    }
    #[doc = "Bit 5 - OPAMP1 offset calibration for P differential pair"]
    #[inline(always)]
    pub fn opa1cal_l(&mut self) -> OPA1CAL_L_W {
        OPA1CAL_L_W { w: self }
    }
    #[doc = "Bit 4 - Switch 6 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s6sel1(&mut self) -> S6SEL1_W {
        S6SEL1_W { w: self }
    }
    #[doc = "Bit 3 - Switch 5 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s5sel1(&mut self) -> S5SEL1_W {
        S5SEL1_W { w: self }
    }
    #[doc = "Bit 2 - Switch 4 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s4sel1(&mut self) -> S4SEL1_W {
        S4SEL1_W { w: self }
    }
    #[doc = "Bit 1 - Switch 3 for OPAMP1 enable"]
    #[inline(always)]
    pub fn s3sel1(&mut self) -> S3SEL1_W {
        S3SEL1_W { w: self }
    }
    #[doc = "Bit 0 - OPAMP1 power down"]
    #[inline(always)]
    pub fn opa1pd(&mut self) -> OPA1PD_W {
        OPA1PD_W { w: self }
    }
}
