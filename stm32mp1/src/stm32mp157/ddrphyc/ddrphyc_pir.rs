#[doc = "Writer for register DDRPHYC_PIR"]
pub type W = crate::W<u32, super::DDRPHYC_PIR>;
#[doc = "Register DDRPHYC_PIR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_PIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
#[doc = "Write proxy for field `DLLSRST`"]
pub struct DLLSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSRST_W<'a> {
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
#[doc = "Write proxy for field `DLLLOCK`"]
pub struct DLLLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLLOCK_W<'a> {
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
#[doc = "Write proxy for field `ZCAL`"]
pub struct ZCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZCAL_W<'a> {
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
#[doc = "Write proxy for field `ITMSRST`"]
pub struct ITMSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMSRST_W<'a> {
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
#[doc = "Write proxy for field `DRAMRST`"]
pub struct DRAMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAMRST_W<'a> {
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
#[doc = "Write proxy for field `DRAMINIT`"]
pub struct DRAMINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAMINIT_W<'a> {
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
#[doc = "Write proxy for field `QSTRN`"]
pub struct QSTRN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSTRN_W<'a> {
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
#[doc = "Write proxy for field `RVTRN`"]
pub struct RVTRN_W<'a> {
    w: &'a mut W,
}
impl<'a> RVTRN_W<'a> {
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
#[doc = "Write proxy for field `ICPC`"]
pub struct ICPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ICPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `DLLBYP`"]
pub struct DLLBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `CTLDINIT`"]
pub struct CTLDINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLDINIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `CLRSR`"]
pub struct CLRSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `LOCKBYP`"]
pub struct LOCKBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `ZCALBYP`"]
pub struct ZCALBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> ZCALBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `INITBYP`"]
pub struct INITBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> INITBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W {
        DLLSRST_W { w: self }
    }
    #[doc = "Bit 2 - DLLLOCK"]
    #[inline(always)]
    pub fn dlllock(&mut self) -> DLLLOCK_W {
        DLLLOCK_W { w: self }
    }
    #[doc = "Bit 3 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&mut self) -> ZCAL_W {
        ZCAL_W { w: self }
    }
    #[doc = "Bit 4 - ITMSRST"]
    #[inline(always)]
    pub fn itmsrst(&mut self) -> ITMSRST_W {
        ITMSRST_W { w: self }
    }
    #[doc = "Bit 5 - DRAMRST"]
    #[inline(always)]
    pub fn dramrst(&mut self) -> DRAMRST_W {
        DRAMRST_W { w: self }
    }
    #[doc = "Bit 6 - DRAMINIT"]
    #[inline(always)]
    pub fn draminit(&mut self) -> DRAMINIT_W {
        DRAMINIT_W { w: self }
    }
    #[doc = "Bit 7 - QSTRN"]
    #[inline(always)]
    pub fn qstrn(&mut self) -> QSTRN_W {
        QSTRN_W { w: self }
    }
    #[doc = "Bit 8 - RVTRN"]
    #[inline(always)]
    pub fn rvtrn(&mut self) -> RVTRN_W {
        RVTRN_W { w: self }
    }
    #[doc = "Bit 16 - ICPC"]
    #[inline(always)]
    pub fn icpc(&mut self) -> ICPC_W {
        ICPC_W { w: self }
    }
    #[doc = "Bit 17 - DLLBYP"]
    #[inline(always)]
    pub fn dllbyp(&mut self) -> DLLBYP_W {
        DLLBYP_W { w: self }
    }
    #[doc = "Bit 18 - CTLDINIT"]
    #[inline(always)]
    pub fn ctldinit(&mut self) -> CTLDINIT_W {
        CTLDINIT_W { w: self }
    }
    #[doc = "Bit 28 - CLRSR"]
    #[inline(always)]
    pub fn clrsr(&mut self) -> CLRSR_W {
        CLRSR_W { w: self }
    }
    #[doc = "Bit 29 - LOCKBYP"]
    #[inline(always)]
    pub fn lockbyp(&mut self) -> LOCKBYP_W {
        LOCKBYP_W { w: self }
    }
    #[doc = "Bit 30 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W {
        ZCALBYP_W { w: self }
    }
    #[doc = "Bit 31 - INITBYP"]
    #[inline(always)]
    pub fn initbyp(&mut self) -> INITBYP_W {
        INITBYP_W { w: self }
    }
}
