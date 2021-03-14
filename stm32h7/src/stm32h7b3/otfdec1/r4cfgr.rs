#[doc = "Reader of register R4CFGR"]
pub type R = crate::R<u32, super::R4CFGR>;
#[doc = "Writer for register R4CFGR"]
pub type W = crate::W<u32, super::R4CFGR>;
#[doc = "Register R4CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::R4CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG_EN`"]
pub type REG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG_EN`"]
pub struct REG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_W<'a> {
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
#[doc = "Reader of field `CONFIGLOCK`"]
pub type CONFIGLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONFIGLOCK`"]
pub struct CONFIGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIGLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `KEYLOCK`"]
pub type KEYLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYLOCK`"]
pub struct KEYLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `KEYCRC`"]
pub type KEYCRC_R = crate::R<u8, u8>;
#[doc = "Reader of field `REGx_VERSION`"]
pub type REGX_VERSION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REGx_VERSION`"]
pub struct REGX_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - region on-the-fly decryption enable"]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - region config lock"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - region key lock"]
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - region key 8-bit CRC"]
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - region firmware version"]
    #[inline(always)]
    pub fn regx_version(&self) -> REGX_VERSION_R {
        REGX_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - region on-the-fly decryption enable"]
    #[inline(always)]
    pub fn reg_en(&mut self) -> REG_EN_W {
        REG_EN_W { w: self }
    }
    #[doc = "Bit 1 - region config lock"]
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W {
        CONFIGLOCK_W { w: self }
    }
    #[doc = "Bit 2 - region key lock"]
    #[inline(always)]
    pub fn keylock(&mut self) -> KEYLOCK_W {
        KEYLOCK_W { w: self }
    }
    #[doc = "Bits 4:5 - operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:31 - region firmware version"]
    #[inline(always)]
    pub fn regx_version(&mut self) -> REGX_VERSION_W {
        REGX_VERSION_W { w: self }
    }
}
