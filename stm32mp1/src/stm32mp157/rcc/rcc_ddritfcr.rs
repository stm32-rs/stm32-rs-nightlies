#[doc = "Reader of register RCC_DDRITFCR"]
pub type R = crate::R<u32, super::RCC_DDRITFCR>;
#[doc = "Writer for register RCC_DDRITFCR"]
pub type W = crate::W<u32, super::RCC_DDRITFCR>;
#[doc = "Register RCC_DDRITFCR `reset()`'s with value 0x000f_d02a"]
impl crate::ResetValue for super::RCC_DDRITFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_d02a
    }
}
#[doc = "Reader of field `DDRC1EN`"]
pub type DDRC1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRC1EN`"]
pub struct DDRC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC1EN_W<'a> {
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
#[doc = "Reader of field `DDRC1LPEN`"]
pub type DDRC1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRC1LPEN`"]
pub struct DDRC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC1LPEN_W<'a> {
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
#[doc = "Reader of field `DDRC2EN`"]
pub type DDRC2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRC2EN`"]
pub struct DDRC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC2EN_W<'a> {
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
#[doc = "Reader of field `DDRC2LPEN`"]
pub type DDRC2LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRC2LPEN`"]
pub struct DDRC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC2LPEN_W<'a> {
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
#[doc = "Reader of field `DDRPHYCEN`"]
pub type DDRPHYCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRPHYCEN`"]
pub struct DDRPHYCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCEN_W<'a> {
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
#[doc = "Reader of field `DDRPHYCLPEN`"]
pub type DDRPHYCLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRPHYCLPEN`"]
pub struct DDRPHYCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCLPEN_W<'a> {
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
#[doc = "Reader of field `DDRCAPBEN`"]
pub type DDRCAPBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRCAPBEN`"]
pub struct DDRCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBEN_W<'a> {
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
#[doc = "Reader of field `DDRCAPBLPEN`"]
pub type DDRCAPBLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRCAPBLPEN`"]
pub struct DDRCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBLPEN_W<'a> {
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
#[doc = "Reader of field `AXIDCGEN`"]
pub type AXIDCGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIDCGEN`"]
pub struct AXIDCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIDCGEN_W<'a> {
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
#[doc = "Reader of field `DDRPHYCAPBEN`"]
pub type DDRPHYCAPBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRPHYCAPBEN`"]
pub struct DDRPHYCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCAPBEN_W<'a> {
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
#[doc = "Reader of field `DDRPHYCAPBLPEN`"]
pub type DDRPHYCAPBLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRPHYCAPBLPEN`"]
pub struct DDRPHYCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCAPBLPEN_W<'a> {
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
#[doc = "Reader of field `KERDCG_DLY`"]
pub type KERDCG_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KERDCG_DLY`"]
pub struct KERDCG_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> KERDCG_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `DDRCAPBRST`"]
pub type DDRCAPBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRCAPBRST`"]
pub struct DDRCAPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBRST_W<'a> {
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
#[doc = "Reader of field `DDRCAXIRST`"]
pub type DDRCAXIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRCAXIRST`"]
pub struct DDRCAXIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAXIRST_W<'a> {
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
#[doc = "Reader of field `DDRCORERST`"]
pub type DDRCORERST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRCORERST`"]
pub struct DDRCORERST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCORERST_W<'a> {
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
#[doc = "Reader of field `DPHYAPBRST`"]
pub type DPHYAPBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPHYAPBRST`"]
pub struct DPHYAPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYAPBRST_W<'a> {
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
#[doc = "Reader of field `DPHYRST`"]
pub type DPHYRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPHYRST`"]
pub struct DPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYRST_W<'a> {
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
#[doc = "Reader of field `DPHYCTLRST`"]
pub type DPHYCTLRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPHYCTLRST`"]
pub struct DPHYCTLRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYCTLRST_W<'a> {
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
#[doc = "Reader of field `DDRCKMOD`"]
pub type DDRCKMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDRCKMOD`"]
pub struct DDRCKMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCKMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `GSKPMOD`"]
pub type GSKPMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSKPMOD`"]
pub struct GSKPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKPMOD_W<'a> {
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
#[doc = "Reader of field `GSKPCTRL`"]
pub type GSKPCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSKPCTRL`"]
pub struct GSKPCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKPCTRL_W<'a> {
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
#[doc = "Reader of field `DFILP_WIDTH`"]
pub type DFILP_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFILP_WIDTH`"]
pub struct DFILP_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DFILP_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `GSKP_DUR`"]
pub type GSKP_DUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GSKP_DUR`"]
pub struct GSKP_DUR_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKP_DUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&self) -> DDRC1EN_R {
        DDRC1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&self) -> DDRC1LPEN_R {
        DDRC1LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&self) -> DDRC2EN_R {
        DDRC2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&self) -> DDRC2LPEN_R {
        DDRC2LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&self) -> DDRPHYCEN_R {
        DDRPHYCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&self) -> DDRPHYCLPEN_R {
        DDRPHYCLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&self) -> DDRCAPBEN_R {
        DDRCAPBEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&self) -> DDRCAPBLPEN_R {
        DDRCAPBLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&self) -> AXIDCGEN_R {
        AXIDCGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&self) -> DDRPHYCAPBEN_R {
        DDRPHYCAPBEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&self) -> DDRPHYCAPBLPEN_R {
        DDRPHYCAPBLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&self) -> KERDCG_DLY_R {
        KERDCG_DLY_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&self) -> DDRCAPBRST_R {
        DDRCAPBRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&self) -> DDRCAXIRST_R {
        DDRCAXIRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&self) -> DDRCORERST_R {
        DDRCORERST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&self) -> DPHYAPBRST_R {
        DPHYAPBRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&self) -> DPHYRST_R {
        DPHYRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&self) -> DPHYCTLRST_R {
        DPHYCTLRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&self) -> DDRCKMOD_R {
        DDRCKMOD_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&self) -> GSKPMOD_R {
        GSKPMOD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&self) -> GSKPCTRL_R {
        GSKPCTRL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&self) -> DFILP_WIDTH_R {
        DFILP_WIDTH_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&self) -> GSKP_DUR_R {
        GSKP_DUR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&mut self) -> DDRC1EN_W {
        DDRC1EN_W { w: self }
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&mut self) -> DDRC1LPEN_W {
        DDRC1LPEN_W { w: self }
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&mut self) -> DDRC2EN_W {
        DDRC2EN_W { w: self }
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&mut self) -> DDRC2LPEN_W {
        DDRC2LPEN_W { w: self }
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&mut self) -> DDRPHYCEN_W {
        DDRPHYCEN_W { w: self }
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&mut self) -> DDRPHYCLPEN_W {
        DDRPHYCLPEN_W { w: self }
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&mut self) -> DDRCAPBEN_W {
        DDRCAPBEN_W { w: self }
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&mut self) -> DDRCAPBLPEN_W {
        DDRCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&mut self) -> AXIDCGEN_W {
        AXIDCGEN_W { w: self }
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&mut self) -> DDRPHYCAPBEN_W {
        DDRPHYCAPBEN_W { w: self }
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&mut self) -> DDRPHYCAPBLPEN_W {
        DDRPHYCAPBLPEN_W { w: self }
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&mut self) -> KERDCG_DLY_W {
        KERDCG_DLY_W { w: self }
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&mut self) -> DDRCAPBRST_W {
        DDRCAPBRST_W { w: self }
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&mut self) -> DDRCAXIRST_W {
        DDRCAXIRST_W { w: self }
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&mut self) -> DDRCORERST_W {
        DDRCORERST_W { w: self }
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&mut self) -> DPHYAPBRST_W {
        DPHYAPBRST_W { w: self }
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&mut self) -> DPHYRST_W {
        DPHYRST_W { w: self }
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&mut self) -> DPHYCTLRST_W {
        DPHYCTLRST_W { w: self }
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&mut self) -> DDRCKMOD_W {
        DDRCKMOD_W { w: self }
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&mut self) -> GSKPMOD_W {
        GSKPMOD_W { w: self }
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&mut self) -> GSKPCTRL_W {
        GSKPCTRL_W { w: self }
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&mut self) -> DFILP_WIDTH_W {
        DFILP_WIDTH_W { w: self }
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&mut self) -> GSKP_DUR_W {
        GSKP_DUR_W { w: self }
    }
}
