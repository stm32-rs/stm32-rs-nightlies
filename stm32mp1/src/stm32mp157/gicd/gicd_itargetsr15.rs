#[doc = "Reader of register GICD_ITARGETSR15"]
pub type R = crate::R<u32, super::GICD_ITARGETSR15>;
#[doc = "Writer for register GICD_ITARGETSR15"]
pub type W = crate::W<u32, super::GICD_ITARGETSR15>;
#[doc = "Register GICD_ITARGETSR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ITARGETSR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPU_TARGETS0`"]
pub type CPU_TARGETS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_TARGETS0`"]
pub struct CPU_TARGETS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CPU_TARGETS1`"]
pub type CPU_TARGETS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_TARGETS1`"]
pub struct CPU_TARGETS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CPU_TARGETS2`"]
pub type CPU_TARGETS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_TARGETS2`"]
pub struct CPU_TARGETS2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CPU_TARGETS3`"]
pub type CPU_TARGETS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_TARGETS3`"]
pub struct CPU_TARGETS3_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    pub fn cpu_targets0(&mut self) -> CPU_TARGETS0_W {
        CPU_TARGETS0_W { w: self }
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    pub fn cpu_targets1(&mut self) -> CPU_TARGETS1_W {
        CPU_TARGETS1_W { w: self }
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    pub fn cpu_targets2(&mut self) -> CPU_TARGETS2_W {
        CPU_TARGETS2_W { w: self }
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    pub fn cpu_targets3(&mut self) -> CPU_TARGETS3_W {
        CPU_TARGETS3_W { w: self }
    }
}
