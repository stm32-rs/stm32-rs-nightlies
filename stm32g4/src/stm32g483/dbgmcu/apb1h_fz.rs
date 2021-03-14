#[doc = "Reader of register APB1H_FZ"]
pub type R = crate::R<u32, super::APB1H_FZ>;
#[doc = "Writer for register APB1H_FZ"]
pub type W = crate::W<u32, super::APB1H_FZ>;
#[doc = "Register APB1H_FZ `reset()`'s with value 0"]
impl crate::ResetValue for super::APB1H_FZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBG_I2C4_STOP`"]
pub type DBG_I2C4_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_I2C4_STOP`"]
pub struct DBG_I2C4_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C4_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W {
        DBG_I2C4_STOP_W { w: self }
    }
}
