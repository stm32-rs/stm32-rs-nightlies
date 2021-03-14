#[doc = "Reader of register INI3_WRITE_QOS"]
pub type R = crate::R<u32, super::INI3_WRITE_QOS>;
#[doc = "Writer for register INI3_WRITE_QOS"]
pub type W = crate::W<u32, super::INI3_WRITE_QOS>;
#[doc = "Register INI3_WRITE_QOS `reset()`'s with value 0x04"]
impl crate::ResetValue for super::INI3_WRITE_QOS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `AW_QOS`"]
pub type AW_QOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AW_QOS`"]
pub struct AW_QOS_W<'a> {
    w: &'a mut W,
}
impl<'a> AW_QOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Write channel QoS setting"]
    #[inline(always)]
    pub fn aw_qos(&self) -> AW_QOS_R {
        AW_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write channel QoS setting"]
    #[inline(always)]
    pub fn aw_qos(&mut self) -> AW_QOS_W {
        AW_QOS_W { w: self }
    }
}
