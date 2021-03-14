#[doc = "Reader of register DDRCTRL_DBG0"]
pub type R = crate::R<u32, super::DDRCTRL_DBG0>;
#[doc = "Writer for register DDRCTRL_DBG0"]
pub type W = crate::W<u32, super::DDRCTRL_DBG0>;
#[doc = "Register DDRCTRL_DBG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_DBG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIS_WC`"]
pub type DIS_WC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_WC`"]
pub struct DIS_WC_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_WC_W<'a> {
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
#[doc = "Reader of field `DIS_COLLISION_PAGE_OPT`"]
pub type DIS_COLLISION_PAGE_OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_COLLISION_PAGE_OPT`"]
pub struct DIS_COLLISION_PAGE_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_COLLISION_PAGE_OPT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DIS_WC"]
    #[inline(always)]
    pub fn dis_wc(&self) -> DIS_WC_R {
        DIS_WC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIS_COLLISION_PAGE_OPT"]
    #[inline(always)]
    pub fn dis_collision_page_opt(&self) -> DIS_COLLISION_PAGE_OPT_R {
        DIS_COLLISION_PAGE_OPT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_WC"]
    #[inline(always)]
    pub fn dis_wc(&mut self) -> DIS_WC_W {
        DIS_WC_W { w: self }
    }
    #[doc = "Bit 4 - DIS_COLLISION_PAGE_OPT"]
    #[inline(always)]
    pub fn dis_collision_page_opt(&mut self) -> DIS_COLLISION_PAGE_OPT_W {
        DIS_COLLISION_PAGE_OPT_W { w: self }
    }
}
