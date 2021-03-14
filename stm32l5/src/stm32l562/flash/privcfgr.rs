#[doc = "Reader of register PRIVCFGR"]
pub type R = crate::R<u32, super::PRIVCFGR>;
#[doc = "Writer for register PRIVCFGR"]
pub type W = crate::W<u32, super::PRIVCFGR>;
#[doc = "Register PRIVCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIV`"]
pub type PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV`"]
pub struct PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_W<'a> {
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
    #[doc = "Bit 0 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W {
        PRIV_W { w: self }
    }
}
