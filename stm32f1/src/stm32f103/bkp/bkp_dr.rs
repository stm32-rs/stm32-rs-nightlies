#[doc = "Reader of register BKP_DR%s"]
pub type R = crate::R<u32, super::BKP_DR>;
#[doc = "Writer for register BKP_DR%s"]
pub type W = crate::W<u32, super::BKP_DR>;
#[doc = "Register BKP_DR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::BKP_DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D`"]
pub type D_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D`"]
pub struct D_W<'a> {
    w: &'a mut W,
}
impl<'a> D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d(&mut self) -> D_W {
        D_W { w: self }
    }
}
