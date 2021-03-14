#[doc = "Reader of register COMP1_OR"]
pub type R = crate::R<u32, super::COMP1_OR>;
#[doc = "Writer for register COMP1_OR"]
pub type W = crate::W<u32, super::COMP1_OR>;
#[doc = "Register COMP1_OR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP1_OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AFOP`"]
pub type AFOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AFOP`"]
pub struct AFOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AFOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `OR`"]
pub type OR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OR`"]
pub struct OR_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&self) -> AFOP_R {
        AFOP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&mut self) -> AFOP_W {
        AFOP_W { w: self }
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&mut self) -> OR_W {
        OR_W { w: self }
    }
}
