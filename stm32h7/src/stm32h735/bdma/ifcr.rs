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
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF7_AW = CTEIF0_AW;
#[doc = "Field `CTEIF7` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF7_AW = CHTIF0_AW;
#[doc = "Field `CHTIF7` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF7_AW = CTCIF0_AW;
#[doc = "Field `CTCIF7` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF7_AW = CGIF0_AW;
#[doc = "Field `CGIF7` writer - Global interrupt flag clear for channel x"]
pub struct CGIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF6_AW = CTEIF0_AW;
#[doc = "Field `CTEIF6` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF6_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF6_AW = CHTIF0_AW;
#[doc = "Field `CHTIF6` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF6_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF6_AW = CTCIF0_AW;
#[doc = "Field `CTCIF6` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF6_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF6_AW = CGIF0_AW;
#[doc = "Field `CGIF6` writer - Global interrupt flag clear for channel x"]
pub struct CGIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF6_AW::CLEAR)
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
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF5_AW = CTEIF0_AW;
#[doc = "Field `CTEIF5` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF5_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF5_AW = CHTIF0_AW;
#[doc = "Field `CHTIF5` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF5_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF5_AW = CTCIF0_AW;
#[doc = "Field `CTCIF5` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF5_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF5_AW = CGIF0_AW;
#[doc = "Field `CGIF5` writer - Global interrupt flag clear for channel x"]
pub struct CGIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF5_AW::CLEAR)
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
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF4_AW = CTEIF0_AW;
#[doc = "Field `CTEIF4` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF4_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF4_AW = CHTIF0_AW;
#[doc = "Field `CHTIF4` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF4_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF4_AW = CTCIF0_AW;
#[doc = "Field `CTCIF4` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF4_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF4_AW = CGIF0_AW;
#[doc = "Field `CGIF4` writer - Global interrupt flag clear for channel x"]
pub struct CGIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF4_AW::CLEAR)
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
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF3_AW = CTEIF0_AW;
#[doc = "Field `CTEIF3` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF3_AW = CHTIF0_AW;
#[doc = "Field `CHTIF3` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF3_AW = CTCIF0_AW;
#[doc = "Field `CTCIF3` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF3_AW = CGIF0_AW;
#[doc = "Field `CGIF3` writer - Global interrupt flag clear for channel x"]
pub struct CGIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF3_AW::CLEAR)
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
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF2_AW = CTEIF0_AW;
#[doc = "Field `CTEIF2` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF2_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF2_AW = CHTIF0_AW;
#[doc = "Field `CHTIF2` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF2_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF2_AW = CTCIF0_AW;
#[doc = "Field `CTCIF2` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF2_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF2_AW = CGIF0_AW;
#[doc = "Field `CGIF2` writer - Global interrupt flag clear for channel x"]
pub struct CGIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF2_AW::CLEAR)
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
#[doc = "Transfer error (TE) flag clear for channel x"]
pub type CTEIF1_AW = CTEIF0_AW;
#[doc = "Field `CTEIF1` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x"]
pub type CHTIF1_AW = CHTIF0_AW;
#[doc = "Field `CHTIF1` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x"]
pub type CTCIF1_AW = CTCIF0_AW;
#[doc = "Field `CTCIF1` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x"]
pub type CGIF1_AW = CGIF0_AW;
#[doc = "Field `CGIF1` writer - Global interrupt flag clear for channel x"]
pub struct CGIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1_AW::CLEAR)
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
#[doc = "Transfer error (TE) flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF0_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    CLEAR = 1,
}
impl From<CTEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF0` writer - Transfer error (TE) flag clear for channel x"]
pub struct CTEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF0_AW::CLEAR)
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
#[doc = "Half transfer (HT) flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF0_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    CLEAR = 1,
}
impl From<CHTIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF0` writer - Half transfer (HT) flag clear for channel x"]
pub struct CHTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF0_AW::CLEAR)
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
#[doc = "Transfer complete (TC) flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF0_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    CLEAR = 1,
}
impl From<CTCIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF0` writer - Transfer complete (TC) flag clear for channel x"]
pub struct CTCIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF0_AW::CLEAR)
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
#[doc = "Global interrupt flag clear for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGIF0_AW {
    #[doc = "1: Clear the corresponding CGIFx flag"]
    CLEAR = 1,
}
impl From<CGIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CGIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGIF0` writer - Global interrupt flag clear for channel x"]
pub struct CGIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF0_AW::CLEAR)
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
    #[doc = "Bit 31 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W {
        CTEIF7_W { w: self }
    }
    #[doc = "Bit 30 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W {
        CHTIF7_W { w: self }
    }
    #[doc = "Bit 29 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W {
        CTCIF7_W { w: self }
    }
    #[doc = "Bit 28 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF7_W {
        CGIF7_W { w: self }
    }
    #[doc = "Bit 27 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W {
        CTEIF6_W { w: self }
    }
    #[doc = "Bit 26 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W {
        CHTIF6_W { w: self }
    }
    #[doc = "Bit 25 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W {
        CTCIF6_W { w: self }
    }
    #[doc = "Bit 24 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF6_W {
        CGIF6_W { w: self }
    }
    #[doc = "Bit 23 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W {
        CTEIF5_W { w: self }
    }
    #[doc = "Bit 22 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W {
        CHTIF5_W { w: self }
    }
    #[doc = "Bit 21 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W {
        CTCIF5_W { w: self }
    }
    #[doc = "Bit 20 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W {
        CGIF5_W { w: self }
    }
    #[doc = "Bit 19 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W { w: self }
    }
    #[doc = "Bit 18 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W {
        CHTIF4_W { w: self }
    }
    #[doc = "Bit 17 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W {
        CTCIF4_W { w: self }
    }
    #[doc = "Bit 16 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W {
        CGIF4_W { w: self }
    }
    #[doc = "Bit 15 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W { w: self }
    }
    #[doc = "Bit 14 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W {
        CHTIF3_W { w: self }
    }
    #[doc = "Bit 13 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W {
        CTCIF3_W { w: self }
    }
    #[doc = "Bit 12 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W {
        CGIF3_W { w: self }
    }
    #[doc = "Bit 11 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W { w: self }
    }
    #[doc = "Bit 10 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W {
        CHTIF2_W { w: self }
    }
    #[doc = "Bit 9 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W {
        CTCIF2_W { w: self }
    }
    #[doc = "Bit 8 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W {
        CGIF2_W { w: self }
    }
    #[doc = "Bit 7 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W { w: self }
    }
    #[doc = "Bit 6 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W {
        CHTIF1_W { w: self }
    }
    #[doc = "Bit 5 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W {
        CTCIF1_W { w: self }
    }
    #[doc = "Bit 4 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W {
        CGIF1_W { w: self }
    }
    #[doc = "Bit 3 - Transfer error (TE) flag clear for channel x"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W {
        CTEIF0_W { w: self }
    }
    #[doc = "Bit 2 - Half transfer (HT) flag clear for channel x"]
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W {
        CHTIF0_W { w: self }
    }
    #[doc = "Bit 1 - Transfer complete (TC) flag clear for channel x"]
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W {
        CTCIF0_W { w: self }
    }
    #[doc = "Bit 0 - Global interrupt flag clear for channel x"]
    #[inline(always)]
    pub fn cgif0(&mut self) -> CGIF0_W {
        CGIF0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
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
