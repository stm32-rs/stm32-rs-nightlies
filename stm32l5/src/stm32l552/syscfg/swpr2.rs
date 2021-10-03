#[doc = "Register `SWPR2` writer"]
pub struct W(crate::W<SWPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SWPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P32WP` writer - P32WP"]
pub struct P32WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P32WP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `P33WP` writer - P33WP"]
pub struct P33WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P33WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `P34WP` writer - P34WP"]
pub struct P34WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P34WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `P35WP` writer - P35WP"]
pub struct P35WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P35WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `P36WP` writer - P36WP"]
pub struct P36WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P36WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `P37WP` writer - P37WP"]
pub struct P37WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P37WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `P38WP` writer - P38WP"]
pub struct P38WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P38WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `P39WP` writer - P39WP"]
pub struct P39WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P39WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `P40WP` writer - P40WP"]
pub struct P40WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P40WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `P41WP` writer - P41WP"]
pub struct P41WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P41WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `P42WP` writer - P42WP"]
pub struct P42WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P42WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `P43WP` writer - P43WP"]
pub struct P43WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P43WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `P44WP` writer - P44WP"]
pub struct P44WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P44WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `P45WP` writer - P45WP"]
pub struct P45WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P45WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `P46WP` writer - P46WP"]
pub struct P46WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P46WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `P47WP` writer - P47WP"]
pub struct P47WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P47WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `P48WP` writer - P48WP"]
pub struct P48WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P48WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `P49WP` writer - P49WP"]
pub struct P49WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P49WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `P50WP` writer - P50WP"]
pub struct P50WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P50WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `P51WP` writer - P51WP"]
pub struct P51WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P51WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `P52WP` writer - P52WP"]
pub struct P52WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P52WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `P53WP` writer - P53WP"]
pub struct P53WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P53WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `P54WP` writer - P54WP"]
pub struct P54WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P54WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `P55WP` writer - P55WP"]
pub struct P55WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P55WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `P56WP` writer - P56WP"]
pub struct P56WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P56WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `P57WP` writer - P57WP"]
pub struct P57WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P57WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `P58WP` writer - P58WP"]
pub struct P58WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P58WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `P59WP` writer - P59WP"]
pub struct P59WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P59WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `P60WP` writer - P60WP"]
pub struct P60WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P60WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `P61WP` writer - P61WP"]
pub struct P61WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P61WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `P62WP` writer - P62WP"]
pub struct P62WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P62WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `P63WP` writer - P63WP"]
pub struct P63WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P63WP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - P32WP"]
    #[inline(always)]
    pub fn p32wp(&mut self) -> P32WP_W {
        P32WP_W { w: self }
    }
    #[doc = "Bit 1 - P33WP"]
    #[inline(always)]
    pub fn p33wp(&mut self) -> P33WP_W {
        P33WP_W { w: self }
    }
    #[doc = "Bit 2 - P34WP"]
    #[inline(always)]
    pub fn p34wp(&mut self) -> P34WP_W {
        P34WP_W { w: self }
    }
    #[doc = "Bit 3 - P35WP"]
    #[inline(always)]
    pub fn p35wp(&mut self) -> P35WP_W {
        P35WP_W { w: self }
    }
    #[doc = "Bit 4 - P36WP"]
    #[inline(always)]
    pub fn p36wp(&mut self) -> P36WP_W {
        P36WP_W { w: self }
    }
    #[doc = "Bit 5 - P37WP"]
    #[inline(always)]
    pub fn p37wp(&mut self) -> P37WP_W {
        P37WP_W { w: self }
    }
    #[doc = "Bit 6 - P38WP"]
    #[inline(always)]
    pub fn p38wp(&mut self) -> P38WP_W {
        P38WP_W { w: self }
    }
    #[doc = "Bit 7 - P39WP"]
    #[inline(always)]
    pub fn p39wp(&mut self) -> P39WP_W {
        P39WP_W { w: self }
    }
    #[doc = "Bit 8 - P40WP"]
    #[inline(always)]
    pub fn p40wp(&mut self) -> P40WP_W {
        P40WP_W { w: self }
    }
    #[doc = "Bit 9 - P41WP"]
    #[inline(always)]
    pub fn p41wp(&mut self) -> P41WP_W {
        P41WP_W { w: self }
    }
    #[doc = "Bit 10 - P42WP"]
    #[inline(always)]
    pub fn p42wp(&mut self) -> P42WP_W {
        P42WP_W { w: self }
    }
    #[doc = "Bit 11 - P43WP"]
    #[inline(always)]
    pub fn p43wp(&mut self) -> P43WP_W {
        P43WP_W { w: self }
    }
    #[doc = "Bit 12 - P44WP"]
    #[inline(always)]
    pub fn p44wp(&mut self) -> P44WP_W {
        P44WP_W { w: self }
    }
    #[doc = "Bit 13 - P45WP"]
    #[inline(always)]
    pub fn p45wp(&mut self) -> P45WP_W {
        P45WP_W { w: self }
    }
    #[doc = "Bit 14 - P46WP"]
    #[inline(always)]
    pub fn p46wp(&mut self) -> P46WP_W {
        P46WP_W { w: self }
    }
    #[doc = "Bit 15 - P47WP"]
    #[inline(always)]
    pub fn p47wp(&mut self) -> P47WP_W {
        P47WP_W { w: self }
    }
    #[doc = "Bit 16 - P48WP"]
    #[inline(always)]
    pub fn p48wp(&mut self) -> P48WP_W {
        P48WP_W { w: self }
    }
    #[doc = "Bit 17 - P49WP"]
    #[inline(always)]
    pub fn p49wp(&mut self) -> P49WP_W {
        P49WP_W { w: self }
    }
    #[doc = "Bit 18 - P50WP"]
    #[inline(always)]
    pub fn p50wp(&mut self) -> P50WP_W {
        P50WP_W { w: self }
    }
    #[doc = "Bit 19 - P51WP"]
    #[inline(always)]
    pub fn p51wp(&mut self) -> P51WP_W {
        P51WP_W { w: self }
    }
    #[doc = "Bit 20 - P52WP"]
    #[inline(always)]
    pub fn p52wp(&mut self) -> P52WP_W {
        P52WP_W { w: self }
    }
    #[doc = "Bit 21 - P53WP"]
    #[inline(always)]
    pub fn p53wp(&mut self) -> P53WP_W {
        P53WP_W { w: self }
    }
    #[doc = "Bit 22 - P54WP"]
    #[inline(always)]
    pub fn p54wp(&mut self) -> P54WP_W {
        P54WP_W { w: self }
    }
    #[doc = "Bit 23 - P55WP"]
    #[inline(always)]
    pub fn p55wp(&mut self) -> P55WP_W {
        P55WP_W { w: self }
    }
    #[doc = "Bit 24 - P56WP"]
    #[inline(always)]
    pub fn p56wp(&mut self) -> P56WP_W {
        P56WP_W { w: self }
    }
    #[doc = "Bit 25 - P57WP"]
    #[inline(always)]
    pub fn p57wp(&mut self) -> P57WP_W {
        P57WP_W { w: self }
    }
    #[doc = "Bit 26 - P58WP"]
    #[inline(always)]
    pub fn p58wp(&mut self) -> P58WP_W {
        P58WP_W { w: self }
    }
    #[doc = "Bit 27 - P59WP"]
    #[inline(always)]
    pub fn p59wp(&mut self) -> P59WP_W {
        P59WP_W { w: self }
    }
    #[doc = "Bit 28 - P60WP"]
    #[inline(always)]
    pub fn p60wp(&mut self) -> P60WP_W {
        P60WP_W { w: self }
    }
    #[doc = "Bit 29 - P61WP"]
    #[inline(always)]
    pub fn p61wp(&mut self) -> P61WP_W {
        P61WP_W { w: self }
    }
    #[doc = "Bit 30 - P62WP"]
    #[inline(always)]
    pub fn p62wp(&mut self) -> P62WP_W {
        P62WP_W { w: self }
    }
    #[doc = "Bit 31 - P63WP"]
    #[inline(always)]
    pub fn p63wp(&mut self) -> P63WP_W {
        P63WP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SWPR2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr2](index.html) module"]
pub struct SWPR2_SPEC;
impl crate::RegisterSpec for SWPR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swpr2::W](W) writer structure"]
impl crate::Writable for SWPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWPR2 to value 0"]
impl crate::Resettable for SWPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
