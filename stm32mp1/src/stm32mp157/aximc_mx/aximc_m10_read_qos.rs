#[doc = "Reader of register AXIMC_M10_READ_QOS"]
pub type R = crate::R<u32, super::AXIMC_M10_READ_QOS>;
#[doc = "Writer for register AXIMC_M10_READ_QOS"]
pub type W = crate::W<u32, super::AXIMC_M10_READ_QOS>;
#[doc = "Register AXIMC_M10_READ_QOS `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::AXIMC_M10_READ_QOS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "Reader of field `AR_QOS`"]
pub type AR_QOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AR_QOS`"]
pub struct AR_QOS_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_QOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - AR_QOS"]
    #[inline(always)]
    pub fn ar_qos(&self) -> AR_QOS_R {
        AR_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AR_QOS"]
    #[inline(always)]
    pub fn ar_qos(&mut self) -> AR_QOS_W {
        AR_QOS_W { w: self }
    }
}
