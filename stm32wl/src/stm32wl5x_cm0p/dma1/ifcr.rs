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
#[doc = "transfer error flag clear for channel 7"]
pub type TEIF7_AW = TEIF1_AW;
#[doc = "Field `TEIF7` writer - transfer error flag clear for channel 7"]
pub struct TEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 7"]
pub type HTIF7_AW = HTIF1_AW;
#[doc = "Field `HTIF7` writer - half transfer flag clear for channel 7"]
pub struct HTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 7"]
pub type TCIF7_AW = TCIF1_AW;
#[doc = "Field `TCIF7` writer - transfer complete flag clear for channel 7"]
pub struct TCIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 7"]
pub type GIF7_AW = GIF1_AW;
#[doc = "Field `GIF7` writer - global interrupt flag clear for channel 7"]
pub struct GIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "transfer error flag clear for channel 6"]
pub type TEIF6_AW = TEIF1_AW;
#[doc = "Field `TEIF6` writer - transfer error flag clear for channel 6"]
pub struct TEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF6_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 6"]
pub type HTIF6_AW = HTIF1_AW;
#[doc = "Field `HTIF6` writer - half transfer flag clear for channel 6"]
pub struct HTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF6_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 6"]
pub type TCIF6_AW = TCIF1_AW;
#[doc = "Field `TCIF6` writer - transfer complete flag clear for channel 6"]
pub struct TCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF6_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 6"]
pub type GIF6_AW = GIF1_AW;
#[doc = "Field `GIF6` writer - global interrupt flag clear for channel 6"]
pub struct GIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF6_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "transfer error flag clear for channel 5"]
pub type TEIF5_AW = TEIF1_AW;
#[doc = "Field `TEIF5` writer - transfer error flag clear for channel 5"]
pub struct TEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF5_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 5"]
pub type HTIF5_AW = HTIF1_AW;
#[doc = "Field `HTIF5` writer - half transfer flag clear for channel 5"]
pub struct HTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF5_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 5"]
pub type TCIF5_AW = TCIF1_AW;
#[doc = "Field `TCIF5` writer - transfer complete flag clear for channel 5"]
pub struct TCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF5_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 5"]
pub type GIF5_AW = GIF1_AW;
#[doc = "Field `GIF5` writer - global interrupt flag clear for channel 5"]
pub struct GIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF5_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "transfer error flag clear for channel 4"]
pub type TEIF4_AW = TEIF1_AW;
#[doc = "Field `TEIF4` writer - transfer error flag clear for channel 4"]
pub struct TEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF4_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 4"]
pub type HTIF4_AW = HTIF1_AW;
#[doc = "Field `HTIF4` writer - half transfer flag clear for channel 4"]
pub struct HTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF4_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 4"]
pub type TCIF4_AW = TCIF1_AW;
#[doc = "Field `TCIF4` writer - transfer complete flag clear for channel 4"]
pub struct TCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF4_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 4"]
pub type GIF4_AW = GIF1_AW;
#[doc = "Field `GIF4` writer - global interrupt flag clear for channel 4"]
pub struct GIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF4_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "transfer error flag clear for channel 3"]
pub type TEIF3_AW = TEIF1_AW;
#[doc = "Field `TEIF3` writer - transfer error flag clear for channel 3"]
pub struct TEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 3"]
pub type HTIF3_AW = HTIF1_AW;
#[doc = "Field `HTIF3` writer - half transfer flag clear for channel 3"]
pub struct HTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 3"]
pub type TCIF3_AW = TCIF1_AW;
#[doc = "Field `TCIF3` writer - transfer complete flag clear for channel 3"]
pub struct TCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 3"]
pub type GIF3_AW = GIF1_AW;
#[doc = "Field `GIF3` writer - global interrupt flag clear for channel 3"]
pub struct GIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "transfer error flag clear for channel 2"]
pub type TEIF2_AW = TEIF1_AW;
#[doc = "Field `TEIF2` writer - transfer error flag clear for channel 2"]
pub struct TEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 2"]
pub type HTIF2_AW = HTIF1_AW;
#[doc = "Field `HTIF2` writer - half transfer flag clear for channel 2"]
pub struct HTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 2"]
pub type TCIF2_AW = TCIF1_AW;
#[doc = "Field `TCIF2` writer - transfer complete flag clear for channel 2"]
pub struct TCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 2"]
pub type GIF2_AW = GIF1_AW;
#[doc = "Field `GIF2` writer - global interrupt flag clear for channel 2"]
pub struct GIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "transfer error flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF1_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    CLEAR = 1,
}
impl From<TEIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF1` writer - transfer error flag clear for channel 1"]
pub struct TEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "half transfer flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF1_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    CLEAR = 1,
}
impl From<HTIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: HTIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF1` writer - half transfer flag clear for channel 1"]
pub struct HTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "transfer complete flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF1_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    CLEAR = 1,
}
impl From<TCIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF1` writer - transfer complete flag clear for channel 1"]
pub struct TCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "global interrupt flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF1_AW {
    #[doc = "1: Clear the corresponding CGIFx flag"]
    CLEAR = 1,
}
impl From<GIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: GIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF1` writer - global interrupt flag clear for channel 1"]
pub struct GIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> GIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 27 - transfer error flag clear for channel 7"]
    #[inline(always)]
    pub fn teif7(&mut self) -> TEIF7_W {
        TEIF7_W { w: self }
    }
    #[doc = "Bit 26 - half transfer flag clear for channel 7"]
    #[inline(always)]
    pub fn htif7(&mut self) -> HTIF7_W {
        HTIF7_W { w: self }
    }
    #[doc = "Bit 25 - transfer complete flag clear for channel 7"]
    #[inline(always)]
    pub fn tcif7(&mut self) -> TCIF7_W {
        TCIF7_W { w: self }
    }
    #[doc = "Bit 24 - global interrupt flag clear for channel 7"]
    #[inline(always)]
    pub fn gif7(&mut self) -> GIF7_W {
        GIF7_W { w: self }
    }
    #[doc = "Bit 23 - transfer error flag clear for channel 6"]
    #[inline(always)]
    pub fn teif6(&mut self) -> TEIF6_W {
        TEIF6_W { w: self }
    }
    #[doc = "Bit 22 - half transfer flag clear for channel 6"]
    #[inline(always)]
    pub fn htif6(&mut self) -> HTIF6_W {
        HTIF6_W { w: self }
    }
    #[doc = "Bit 21 - transfer complete flag clear for channel 6"]
    #[inline(always)]
    pub fn tcif6(&mut self) -> TCIF6_W {
        TCIF6_W { w: self }
    }
    #[doc = "Bit 20 - global interrupt flag clear for channel 6"]
    #[inline(always)]
    pub fn gif6(&mut self) -> GIF6_W {
        GIF6_W { w: self }
    }
    #[doc = "Bit 19 - transfer error flag clear for channel 5"]
    #[inline(always)]
    pub fn teif5(&mut self) -> TEIF5_W {
        TEIF5_W { w: self }
    }
    #[doc = "Bit 18 - half transfer flag clear for channel 5"]
    #[inline(always)]
    pub fn htif5(&mut self) -> HTIF5_W {
        HTIF5_W { w: self }
    }
    #[doc = "Bit 17 - transfer complete flag clear for channel 5"]
    #[inline(always)]
    pub fn tcif5(&mut self) -> TCIF5_W {
        TCIF5_W { w: self }
    }
    #[doc = "Bit 16 - global interrupt flag clear for channel 5"]
    #[inline(always)]
    pub fn gif5(&mut self) -> GIF5_W {
        GIF5_W { w: self }
    }
    #[doc = "Bit 15 - transfer error flag clear for channel 4"]
    #[inline(always)]
    pub fn teif4(&mut self) -> TEIF4_W {
        TEIF4_W { w: self }
    }
    #[doc = "Bit 14 - half transfer flag clear for channel 4"]
    #[inline(always)]
    pub fn htif4(&mut self) -> HTIF4_W {
        HTIF4_W { w: self }
    }
    #[doc = "Bit 13 - transfer complete flag clear for channel 4"]
    #[inline(always)]
    pub fn tcif4(&mut self) -> TCIF4_W {
        TCIF4_W { w: self }
    }
    #[doc = "Bit 12 - global interrupt flag clear for channel 4"]
    #[inline(always)]
    pub fn gif4(&mut self) -> GIF4_W {
        GIF4_W { w: self }
    }
    #[doc = "Bit 11 - transfer error flag clear for channel 3"]
    #[inline(always)]
    pub fn teif3(&mut self) -> TEIF3_W {
        TEIF3_W { w: self }
    }
    #[doc = "Bit 10 - half transfer flag clear for channel 3"]
    #[inline(always)]
    pub fn htif3(&mut self) -> HTIF3_W {
        HTIF3_W { w: self }
    }
    #[doc = "Bit 9 - transfer complete flag clear for channel 3"]
    #[inline(always)]
    pub fn tcif3(&mut self) -> TCIF3_W {
        TCIF3_W { w: self }
    }
    #[doc = "Bit 8 - global interrupt flag clear for channel 3"]
    #[inline(always)]
    pub fn gif3(&mut self) -> GIF3_W {
        GIF3_W { w: self }
    }
    #[doc = "Bit 7 - transfer error flag clear for channel 2"]
    #[inline(always)]
    pub fn teif2(&mut self) -> TEIF2_W {
        TEIF2_W { w: self }
    }
    #[doc = "Bit 6 - half transfer flag clear for channel 2"]
    #[inline(always)]
    pub fn htif2(&mut self) -> HTIF2_W {
        HTIF2_W { w: self }
    }
    #[doc = "Bit 5 - transfer complete flag clear for channel 2"]
    #[inline(always)]
    pub fn tcif2(&mut self) -> TCIF2_W {
        TCIF2_W { w: self }
    }
    #[doc = "Bit 4 - global interrupt flag clear for channel 2"]
    #[inline(always)]
    pub fn gif2(&mut self) -> GIF2_W {
        GIF2_W { w: self }
    }
    #[doc = "Bit 3 - transfer error flag clear for channel 1"]
    #[inline(always)]
    pub fn teif1(&mut self) -> TEIF1_W {
        TEIF1_W { w: self }
    }
    #[doc = "Bit 2 - half transfer flag clear for channel 1"]
    #[inline(always)]
    pub fn htif1(&mut self) -> HTIF1_W {
        HTIF1_W { w: self }
    }
    #[doc = "Bit 1 - transfer complete flag clear for channel 1"]
    #[inline(always)]
    pub fn tcif1(&mut self) -> TCIF1_W {
        TCIF1_W { w: self }
    }
    #[doc = "Bit 0 - global interrupt flag clear for channel 1"]
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
#[doc = "interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
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
