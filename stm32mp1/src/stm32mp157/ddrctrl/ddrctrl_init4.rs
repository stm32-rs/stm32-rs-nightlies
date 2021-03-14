#[doc = "Reader of register DDRCTRL_INIT4"]
pub type R = crate::R<u32, super::DDRCTRL_INIT4>;
#[doc = "Writer for register DDRCTRL_INIT4"]
pub type W = crate::W<u32, super::DDRCTRL_INIT4>;
#[doc = "Register DDRCTRL_INIT4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_INIT4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMR3`"]
pub type EMR3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EMR3`"]
pub struct EMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EMR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `EMR2`"]
pub type EMR2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EMR2`"]
pub struct EMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EMR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - EMR3"]
    #[inline(always)]
    pub fn emr3(&self) -> EMR3_R {
        EMR3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EMR2"]
    #[inline(always)]
    pub fn emr2(&self) -> EMR2_R {
        EMR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EMR3"]
    #[inline(always)]
    pub fn emr3(&mut self) -> EMR3_W {
        EMR3_W { w: self }
    }
    #[doc = "Bits 16:31 - EMR2"]
    #[inline(always)]
    pub fn emr2(&mut self) -> EMR2_W {
        EMR2_W { w: self }
    }
}
