#[doc = "Register `KLR` writer"]
pub struct W(crate::W<KLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KLR_SPEC>;
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
impl From<crate::W<KLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b224` writer - b224"]
pub struct B224_W<'a> {
    w: &'a mut W,
}
impl<'a> B224_W<'a> {
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
#[doc = "Field `b225` writer - b225"]
pub struct B225_W<'a> {
    w: &'a mut W,
}
impl<'a> B225_W<'a> {
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
#[doc = "Field `b226` writer - b226"]
pub struct B226_W<'a> {
    w: &'a mut W,
}
impl<'a> B226_W<'a> {
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
#[doc = "Field `b227` writer - b227"]
pub struct B227_W<'a> {
    w: &'a mut W,
}
impl<'a> B227_W<'a> {
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
#[doc = "Field `b228` writer - b228"]
pub struct B228_W<'a> {
    w: &'a mut W,
}
impl<'a> B228_W<'a> {
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
#[doc = "Field `b229` writer - b229"]
pub struct B229_W<'a> {
    w: &'a mut W,
}
impl<'a> B229_W<'a> {
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
#[doc = "Field `b230` writer - b230"]
pub struct B230_W<'a> {
    w: &'a mut W,
}
impl<'a> B230_W<'a> {
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
#[doc = "Field `b231` writer - b231"]
pub struct B231_W<'a> {
    w: &'a mut W,
}
impl<'a> B231_W<'a> {
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
#[doc = "Field `b232` writer - b232"]
pub struct B232_W<'a> {
    w: &'a mut W,
}
impl<'a> B232_W<'a> {
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
#[doc = "Field `b233` writer - b233"]
pub struct B233_W<'a> {
    w: &'a mut W,
}
impl<'a> B233_W<'a> {
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
#[doc = "Field `b234` writer - b234"]
pub struct B234_W<'a> {
    w: &'a mut W,
}
impl<'a> B234_W<'a> {
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
#[doc = "Field `b235` writer - b235"]
pub struct B235_W<'a> {
    w: &'a mut W,
}
impl<'a> B235_W<'a> {
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
#[doc = "Field `b236` writer - b236"]
pub struct B236_W<'a> {
    w: &'a mut W,
}
impl<'a> B236_W<'a> {
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
#[doc = "Field `b237` writer - b237"]
pub struct B237_W<'a> {
    w: &'a mut W,
}
impl<'a> B237_W<'a> {
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
#[doc = "Field `b238` writer - b238"]
pub struct B238_W<'a> {
    w: &'a mut W,
}
impl<'a> B238_W<'a> {
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
#[doc = "Field `b239` writer - b239"]
pub struct B239_W<'a> {
    w: &'a mut W,
}
impl<'a> B239_W<'a> {
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
#[doc = "Field `b240` writer - b240"]
pub struct B240_W<'a> {
    w: &'a mut W,
}
impl<'a> B240_W<'a> {
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
#[doc = "Field `b241` writer - b241"]
pub struct B241_W<'a> {
    w: &'a mut W,
}
impl<'a> B241_W<'a> {
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
#[doc = "Field `b242` writer - b242"]
pub struct B242_W<'a> {
    w: &'a mut W,
}
impl<'a> B242_W<'a> {
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
#[doc = "Field `b243` writer - b243"]
pub struct B243_W<'a> {
    w: &'a mut W,
}
impl<'a> B243_W<'a> {
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
#[doc = "Field `b244` writer - b244"]
pub struct B244_W<'a> {
    w: &'a mut W,
}
impl<'a> B244_W<'a> {
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
#[doc = "Field `b245` writer - b245"]
pub struct B245_W<'a> {
    w: &'a mut W,
}
impl<'a> B245_W<'a> {
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
#[doc = "Field `b246` writer - b246"]
pub struct B246_W<'a> {
    w: &'a mut W,
}
impl<'a> B246_W<'a> {
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
#[doc = "Field `b247` writer - b247"]
pub struct B247_W<'a> {
    w: &'a mut W,
}
impl<'a> B247_W<'a> {
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
#[doc = "Field `b248` writer - b248"]
pub struct B248_W<'a> {
    w: &'a mut W,
}
impl<'a> B248_W<'a> {
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
#[doc = "Field `b249` writer - b249"]
pub struct B249_W<'a> {
    w: &'a mut W,
}
impl<'a> B249_W<'a> {
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
#[doc = "Field `b250` writer - b250"]
pub struct B250_W<'a> {
    w: &'a mut W,
}
impl<'a> B250_W<'a> {
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
#[doc = "Field `b251` writer - b251"]
pub struct B251_W<'a> {
    w: &'a mut W,
}
impl<'a> B251_W<'a> {
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
#[doc = "Field `b252` writer - b252"]
pub struct B252_W<'a> {
    w: &'a mut W,
}
impl<'a> B252_W<'a> {
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
#[doc = "Field `b253` writer - b253"]
pub struct B253_W<'a> {
    w: &'a mut W,
}
impl<'a> B253_W<'a> {
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
#[doc = "Field `b254` writer - b254"]
pub struct B254_W<'a> {
    w: &'a mut W,
}
impl<'a> B254_W<'a> {
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
#[doc = "Field `b255` writer - b255"]
pub struct B255_W<'a> {
    w: &'a mut W,
}
impl<'a> B255_W<'a> {
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
    #[doc = "Bit 0 - b224"]
    #[inline(always)]
    pub fn b224(&mut self) -> B224_W {
        B224_W { w: self }
    }
    #[doc = "Bit 1 - b225"]
    #[inline(always)]
    pub fn b225(&mut self) -> B225_W {
        B225_W { w: self }
    }
    #[doc = "Bit 2 - b226"]
    #[inline(always)]
    pub fn b226(&mut self) -> B226_W {
        B226_W { w: self }
    }
    #[doc = "Bit 3 - b227"]
    #[inline(always)]
    pub fn b227(&mut self) -> B227_W {
        B227_W { w: self }
    }
    #[doc = "Bit 4 - b228"]
    #[inline(always)]
    pub fn b228(&mut self) -> B228_W {
        B228_W { w: self }
    }
    #[doc = "Bit 5 - b229"]
    #[inline(always)]
    pub fn b229(&mut self) -> B229_W {
        B229_W { w: self }
    }
    #[doc = "Bit 6 - b230"]
    #[inline(always)]
    pub fn b230(&mut self) -> B230_W {
        B230_W { w: self }
    }
    #[doc = "Bit 7 - b231"]
    #[inline(always)]
    pub fn b231(&mut self) -> B231_W {
        B231_W { w: self }
    }
    #[doc = "Bit 8 - b232"]
    #[inline(always)]
    pub fn b232(&mut self) -> B232_W {
        B232_W { w: self }
    }
    #[doc = "Bit 9 - b233"]
    #[inline(always)]
    pub fn b233(&mut self) -> B233_W {
        B233_W { w: self }
    }
    #[doc = "Bit 10 - b234"]
    #[inline(always)]
    pub fn b234(&mut self) -> B234_W {
        B234_W { w: self }
    }
    #[doc = "Bit 11 - b235"]
    #[inline(always)]
    pub fn b235(&mut self) -> B235_W {
        B235_W { w: self }
    }
    #[doc = "Bit 12 - b236"]
    #[inline(always)]
    pub fn b236(&mut self) -> B236_W {
        B236_W { w: self }
    }
    #[doc = "Bit 13 - b237"]
    #[inline(always)]
    pub fn b237(&mut self) -> B237_W {
        B237_W { w: self }
    }
    #[doc = "Bit 14 - b238"]
    #[inline(always)]
    pub fn b238(&mut self) -> B238_W {
        B238_W { w: self }
    }
    #[doc = "Bit 15 - b239"]
    #[inline(always)]
    pub fn b239(&mut self) -> B239_W {
        B239_W { w: self }
    }
    #[doc = "Bit 16 - b240"]
    #[inline(always)]
    pub fn b240(&mut self) -> B240_W {
        B240_W { w: self }
    }
    #[doc = "Bit 17 - b241"]
    #[inline(always)]
    pub fn b241(&mut self) -> B241_W {
        B241_W { w: self }
    }
    #[doc = "Bit 18 - b242"]
    #[inline(always)]
    pub fn b242(&mut self) -> B242_W {
        B242_W { w: self }
    }
    #[doc = "Bit 19 - b243"]
    #[inline(always)]
    pub fn b243(&mut self) -> B243_W {
        B243_W { w: self }
    }
    #[doc = "Bit 20 - b244"]
    #[inline(always)]
    pub fn b244(&mut self) -> B244_W {
        B244_W { w: self }
    }
    #[doc = "Bit 21 - b245"]
    #[inline(always)]
    pub fn b245(&mut self) -> B245_W {
        B245_W { w: self }
    }
    #[doc = "Bit 22 - b246"]
    #[inline(always)]
    pub fn b246(&mut self) -> B246_W {
        B246_W { w: self }
    }
    #[doc = "Bit 23 - b247"]
    #[inline(always)]
    pub fn b247(&mut self) -> B247_W {
        B247_W { w: self }
    }
    #[doc = "Bit 24 - b248"]
    #[inline(always)]
    pub fn b248(&mut self) -> B248_W {
        B248_W { w: self }
    }
    #[doc = "Bit 25 - b249"]
    #[inline(always)]
    pub fn b249(&mut self) -> B249_W {
        B249_W { w: self }
    }
    #[doc = "Bit 26 - b250"]
    #[inline(always)]
    pub fn b250(&mut self) -> B250_W {
        B250_W { w: self }
    }
    #[doc = "Bit 27 - b251"]
    #[inline(always)]
    pub fn b251(&mut self) -> B251_W {
        B251_W { w: self }
    }
    #[doc = "Bit 28 - b252"]
    #[inline(always)]
    pub fn b252(&mut self) -> B252_W {
        B252_W { w: self }
    }
    #[doc = "Bit 29 - b253"]
    #[inline(always)]
    pub fn b253(&mut self) -> B253_W {
        B253_W { w: self }
    }
    #[doc = "Bit 30 - b254"]
    #[inline(always)]
    pub fn b254(&mut self) -> B254_W {
        B254_W { w: self }
    }
    #[doc = "Bit 31 - b255"]
    #[inline(always)]
    pub fn b255(&mut self) -> B255_W {
        B255_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [klr](index.html) module"]
pub struct KLR_SPEC;
impl crate::RegisterSpec for KLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [klr::W](W) writer structure"]
impl crate::Writable for KLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KLR to value 0"]
impl crate::Resettable for KLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
