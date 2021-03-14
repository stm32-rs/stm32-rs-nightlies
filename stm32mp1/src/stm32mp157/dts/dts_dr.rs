#[doc = "Reader of register DTS_DR"]
pub type R = crate::R<u32, super::DTS_DR>;
#[doc = "Writer for register DTS_DR"]
pub type W = crate::W<u32, super::DTS_DR>;
#[doc = "Register DTS_DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTS_DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS1_MFREQ`"]
pub type TS1_MFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TS1_MFREQ`"]
pub struct TS1_MFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_MFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TS1_MFREQ"]
    #[inline(always)]
    pub fn ts1_mfreq(&self) -> TS1_MFREQ_R {
        TS1_MFREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TS1_MFREQ"]
    #[inline(always)]
    pub fn ts1_mfreq(&mut self) -> TS1_MFREQ_W {
        TS1_MFREQ_W { w: self }
    }
}
