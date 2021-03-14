#[doc = "Reader of register DDRCTRL_INIT3"]
pub type R = crate::R<u32, super::DDRCTRL_INIT3>;
#[doc = "Writer for register DDRCTRL_INIT3"]
pub type W = crate::W<u32, super::DDRCTRL_INIT3>;
#[doc = "Register DDRCTRL_INIT3 `reset()`'s with value 0x0510"]
impl crate::ResetValue for super::DDRCTRL_INIT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0510
    }
}
#[doc = "Reader of field `EMR`"]
pub type EMR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EMR`"]
pub struct EMR_W<'a> {
    w: &'a mut W,
}
impl<'a> EMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MR`"]
pub type MR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MR`"]
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - EMR"]
    #[inline(always)]
    pub fn emr(&self) -> EMR_R {
        EMR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MR"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EMR"]
    #[inline(always)]
    pub fn emr(&mut self) -> EMR_W {
        EMR_W { w: self }
    }
    #[doc = "Bits 16:31 - MR"]
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W {
        MR_W { w: self }
    }
}
