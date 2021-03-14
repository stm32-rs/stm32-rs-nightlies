#[doc = "Writer for register DSI_FIR1"]
pub type W = crate::W<u32, super::DSI_FIR1>;
#[doc = "Register DSI_FIR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_FIR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FTOHSTX`"]
pub struct FTOHSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> FTOHSTX_W<'a> {
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
#[doc = "Write proxy for field `FTOLPRX`"]
pub struct FTOLPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> FTOLPRX_W<'a> {
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
#[doc = "Write proxy for field `FECCSE`"]
pub struct FECCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FECCSE_W<'a> {
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
#[doc = "Write proxy for field `FECCME`"]
pub struct FECCME_W<'a> {
    w: &'a mut W,
}
impl<'a> FECCME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `FCRCE`"]
pub struct FCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `FPSE`"]
pub struct FPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `FEOTPE`"]
pub struct FEOTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEOTPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `FLPWRE`"]
pub struct FLPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `FGCWRE`"]
pub struct FGCWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGCWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `FGPWRE`"]
pub struct FGPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `FGPTXE`"]
pub struct FGPTXE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPTXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `FGPRDE`"]
pub struct FGPRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPRDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `FGPRXE`"]
pub struct FGPRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPRXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn ftohstx(&mut self) -> FTOHSTX_W {
        FTOHSTX_W { w: self }
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn ftolprx(&mut self) -> FTOLPRX_W {
        FTOLPRX_W { w: self }
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    pub fn feccse(&mut self) -> FECCSE_W {
        FECCSE_W { w: self }
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    pub fn feccme(&mut self) -> FECCME_W {
        FECCME_W { w: self }
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    pub fn fcrce(&mut self) -> FCRCE_W {
        FCRCE_W { w: self }
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    pub fn fpse(&mut self) -> FPSE_W {
        FPSE_W { w: self }
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    pub fn feotpe(&mut self) -> FEOTPE_W {
        FEOTPE_W { w: self }
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    pub fn flpwre(&mut self) -> FLPWRE_W {
        FLPWRE_W { w: self }
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    pub fn fgcwre(&mut self) -> FGCWRE_W {
        FGCWRE_W { w: self }
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    pub fn fgpwre(&mut self) -> FGPWRE_W {
        FGPWRE_W { w: self }
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn fgptxe(&mut self) -> FGPTXE_W {
        FGPTXE_W { w: self }
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    pub fn fgprde(&mut self) -> FGPRDE_W {
        FGPRDE_W { w: self }
    }
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    pub fn fgprxe(&mut self) -> FGPRXE_W {
        FGPRXE_W { w: self }
    }
}
