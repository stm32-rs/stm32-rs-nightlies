#[doc = "Reader of register M0AR"]
pub type R = crate::R<u32, super::M0AR>;
#[doc = "Writer for register M0AR"]
pub type W = crate::W<u32, super::M0AR>;
#[doc = "Register M0AR `reset()`'s with value 0"]
impl crate::ResetValue for super::M0AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MA`"]
pub type MA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MA`"]
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
}
