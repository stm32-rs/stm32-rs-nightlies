#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEIF8` writer - TEIF8"]
pub struct TEIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF8_W<'a> {
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
#[doc = "Field `HTIF8` writer - HTIF8"]
pub struct HTIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF8_W<'a> {
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
#[doc = "Field `TCIF8` writer - TCIF8"]
pub struct TCIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF8_W<'a> {
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
#[doc = "Field `GIF8` writer - GIF8"]
pub struct GIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF8_W<'a> {
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
#[doc = "Field `TEIF7` writer - TEIF7"]
pub struct TEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF7_W<'a> {
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
#[doc = "Field `HTIF7` writer - HTIF7"]
pub struct HTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF7_W<'a> {
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
#[doc = "Field `TCIF7` writer - TCIF7"]
pub struct TCIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF7_W<'a> {
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
#[doc = "Field `GIF7` writer - GIF7"]
pub struct GIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF7_W<'a> {
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
#[doc = "Field `TEIF6` writer - TEIF6"]
pub struct TEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF6_W<'a> {
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
#[doc = "Field `HTIF6` writer - HTIF6"]
pub struct HTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF6_W<'a> {
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
#[doc = "Field `TCIF6` writer - TCIF6"]
pub struct TCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF6_W<'a> {
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
#[doc = "Field `GIF6` writer - GIF6"]
pub struct GIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF6_W<'a> {
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
#[doc = "Field `TEIF5` writer - TEIF5"]
pub struct TEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF5_W<'a> {
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
#[doc = "Field `HTIF5` writer - HTIF5"]
pub struct HTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF5_W<'a> {
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
#[doc = "Field `TCIF5` writer - TCIF5"]
pub struct TCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF5_W<'a> {
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
#[doc = "Field `GIF5` writer - GIF5"]
pub struct GIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF5_W<'a> {
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
#[doc = "Field `TEIF4` writer - TEIF4"]
pub struct TEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF4_W<'a> {
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
#[doc = "Field `HTIF4` writer - HTIF4"]
pub struct HTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF4_W<'a> {
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
#[doc = "Field `TCIF4` writer - TCIF4"]
pub struct TCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF4_W<'a> {
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
#[doc = "Field `GIF4` writer - GIF4"]
pub struct GIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF4_W<'a> {
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
#[doc = "Field `TEIF3` writer - TEIF3"]
pub struct TEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF3_W<'a> {
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
#[doc = "Field `HTIF3` writer - HTIF3"]
pub struct HTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF3_W<'a> {
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
#[doc = "Field `TCIF3` writer - TCIF3"]
pub struct TCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF3_W<'a> {
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
#[doc = "Field `GIF3` writer - GIF3"]
pub struct GIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF3_W<'a> {
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
#[doc = "Field `TEIF2` writer - TEIF2"]
pub struct TEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF2_W<'a> {
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
#[doc = "Field `HTIF2` writer - HTIF2"]
pub struct HTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF2_W<'a> {
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
#[doc = "Field `TCIF2` writer - TCIF2"]
pub struct TCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF2_W<'a> {
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
#[doc = "Field `GIF2` writer - GIF2"]
pub struct GIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF2_W<'a> {
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
#[doc = "Field `TEIF1` writer - TEIF1"]
pub struct TEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF1_W<'a> {
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
#[doc = "Field `HTIF1` writer - HTIF1"]
pub struct HTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF1_W<'a> {
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
#[doc = "Field `TCIF1` writer - TCIF1"]
pub struct TCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF1_W<'a> {
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
#[doc = "Field `GIF1` writer - GIF1"]
pub struct GIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF1_W<'a> {
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
impl W {
    #[doc = "Bit 31 - TEIF8"]
    #[inline(always)]
    pub fn teif8(&mut self) -> TEIF8_W {
        TEIF8_W { w: self }
    }
    #[doc = "Bit 30 - HTIF8"]
    #[inline(always)]
    pub fn htif8(&mut self) -> HTIF8_W {
        HTIF8_W { w: self }
    }
    #[doc = "Bit 29 - TCIF8"]
    #[inline(always)]
    pub fn tcif8(&mut self) -> TCIF8_W {
        TCIF8_W { w: self }
    }
    #[doc = "Bit 28 - GIF8"]
    #[inline(always)]
    pub fn gif8(&mut self) -> GIF8_W {
        GIF8_W { w: self }
    }
    #[doc = "Bit 27 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&mut self) -> TEIF7_W {
        TEIF7_W { w: self }
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    pub fn htif7(&mut self) -> HTIF7_W {
        HTIF7_W { w: self }
    }
    #[doc = "Bit 25 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&mut self) -> TCIF7_W {
        TCIF7_W { w: self }
    }
    #[doc = "Bit 24 - GIF7"]
    #[inline(always)]
    pub fn gif7(&mut self) -> GIF7_W {
        GIF7_W { w: self }
    }
    #[doc = "Bit 23 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&mut self) -> TEIF6_W {
        TEIF6_W { w: self }
    }
    #[doc = "Bit 22 - HTIF6"]
    #[inline(always)]
    pub fn htif6(&mut self) -> HTIF6_W {
        HTIF6_W { w: self }
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&mut self) -> TCIF6_W {
        TCIF6_W { w: self }
    }
    #[doc = "Bit 20 - GIF6"]
    #[inline(always)]
    pub fn gif6(&mut self) -> GIF6_W {
        GIF6_W { w: self }
    }
    #[doc = "Bit 19 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&mut self) -> TEIF5_W {
        TEIF5_W { w: self }
    }
    #[doc = "Bit 18 - HTIF5"]
    #[inline(always)]
    pub fn htif5(&mut self) -> HTIF5_W {
        HTIF5_W { w: self }
    }
    #[doc = "Bit 17 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&mut self) -> TCIF5_W {
        TCIF5_W { w: self }
    }
    #[doc = "Bit 16 - GIF5"]
    #[inline(always)]
    pub fn gif5(&mut self) -> GIF5_W {
        GIF5_W { w: self }
    }
    #[doc = "Bit 15 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&mut self) -> TEIF4_W {
        TEIF4_W { w: self }
    }
    #[doc = "Bit 14 - HTIF4"]
    #[inline(always)]
    pub fn htif4(&mut self) -> HTIF4_W {
        HTIF4_W { w: self }
    }
    #[doc = "Bit 13 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&mut self) -> TCIF4_W {
        TCIF4_W { w: self }
    }
    #[doc = "Bit 12 - GIF4"]
    #[inline(always)]
    pub fn gif4(&mut self) -> GIF4_W {
        GIF4_W { w: self }
    }
    #[doc = "Bit 11 - TEIF3"]
    #[inline(always)]
    pub fn teif3(&mut self) -> TEIF3_W {
        TEIF3_W { w: self }
    }
    #[doc = "Bit 10 - HTIF3"]
    #[inline(always)]
    pub fn htif3(&mut self) -> HTIF3_W {
        HTIF3_W { w: self }
    }
    #[doc = "Bit 9 - TCIF3"]
    #[inline(always)]
    pub fn tcif3(&mut self) -> TCIF3_W {
        TCIF3_W { w: self }
    }
    #[doc = "Bit 8 - GIF3"]
    #[inline(always)]
    pub fn gif3(&mut self) -> GIF3_W {
        GIF3_W { w: self }
    }
    #[doc = "Bit 7 - TEIF2"]
    #[inline(always)]
    pub fn teif2(&mut self) -> TEIF2_W {
        TEIF2_W { w: self }
    }
    #[doc = "Bit 6 - HTIF2"]
    #[inline(always)]
    pub fn htif2(&mut self) -> HTIF2_W {
        HTIF2_W { w: self }
    }
    #[doc = "Bit 5 - TCIF2"]
    #[inline(always)]
    pub fn tcif2(&mut self) -> TCIF2_W {
        TCIF2_W { w: self }
    }
    #[doc = "Bit 4 - GIF2"]
    #[inline(always)]
    pub fn gif2(&mut self) -> GIF2_W {
        GIF2_W { w: self }
    }
    #[doc = "Bit 3 - TEIF1"]
    #[inline(always)]
    pub fn teif1(&mut self) -> TEIF1_W {
        TEIF1_W { w: self }
    }
    #[doc = "Bit 2 - HTIF1"]
    #[inline(always)]
    pub fn htif1(&mut self) -> HTIF1_W {
        HTIF1_W { w: self }
    }
    #[doc = "Bit 1 - TCIF1"]
    #[inline(always)]
    pub fn tcif1(&mut self) -> TCIF1_W {
        TCIF1_W { w: self }
    }
    #[doc = "Bit 0 - GIF1"]
    #[inline(always)]
    pub fn gif1(&mut self) -> GIF1_W {
        GIF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
