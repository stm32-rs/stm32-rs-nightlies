#[doc = "Reader of register C1_RSR"]
pub type R = crate::R<u32, super::C1_RSR>;
#[doc = "Writer for register C1_RSR"]
pub type W = crate::W<u32, super::C1_RSR>;
#[doc = "Register C1_RSR `reset()`'s with value 0"]
impl crate::ResetValue for super::C1_RSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVF_A {
    #[doc = "0: Not clearing the the reset flags"]
    NOTACTIVE = 0,
    #[doc = "1: Clear the reset flags"]
    CLEAR = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RMVF`"]
pub type RMVF_R = crate::R<bool, RMVF_A>;
impl RMVF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMVF_A {
        match self.bits {
            false => RMVF_A::NOTACTIVE,
            true => RMVF_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == RMVF_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF_A::CLEAR
    }
}
#[doc = "Write proxy for field `RMVF`"]
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMVF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not clearing the the reset flags"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(RMVF_A::NOTACTIVE)
    }
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::CLEAR)
    }
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
#[doc = "CPU reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURSTF_A {
    #[doc = "0: No reset occoured for block"]
    NORESETOCCOURED = 0,
    #[doc = "1: Reset occoured for block"]
    RESETOCCOURRED = 1,
}
impl From<CPURSTF_A> for bool {
    #[inline(always)]
    fn from(variant: CPURSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPURSTF`"]
pub type CPURSTF_R = crate::R<bool, CPURSTF_A>;
impl CPURSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPURSTF_A {
        match self.bits {
            false => CPURSTF_A::NORESETOCCOURED,
            true => CPURSTF_A::RESETOCCOURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NORESETOCCOURED`"]
    #[inline(always)]
    pub fn is_no_reset_occoured(&self) -> bool {
        *self == CPURSTF_A::NORESETOCCOURED
    }
    #[doc = "Checks if the value of the field is `RESETOCCOURRED`"]
    #[inline(always)]
    pub fn is_reset_occourred(&self) -> bool {
        *self == CPURSTF_A::RESETOCCOURRED
    }
}
#[doc = "Write proxy for field `CPURSTF`"]
pub struct CPURSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPURSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPURSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
#[doc = "D1 domain power switch reset flag"]
pub type D1RSTF_A = CPURSTF_A;
#[doc = "Reader of field `D1RSTF`"]
pub type D1RSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `D1RSTF`"]
pub struct D1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> D1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D1RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "D2 domain power switch reset flag"]
pub type D2RSTF_A = CPURSTF_A;
#[doc = "Reader of field `D2RSTF`"]
pub type D2RSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `D2RSTF`"]
pub struct D2RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> D2RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D2RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "BOR reset flag"]
pub type BORRSTF_A = CPURSTF_A;
#[doc = "Reader of field `BORRSTF`"]
pub type BORRSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `BORRSTF`"]
pub struct BORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BORRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BORRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Pin reset flag (NRST)"]
pub type PINRSTF_A = CPURSTF_A;
#[doc = "Reader of field `PINRSTF`"]
pub type PINRSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `PINRSTF`"]
pub struct PINRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PINRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "POR/PDR reset flag"]
pub type PORRSTF_A = CPURSTF_A;
#[doc = "Reader of field `PORRSTF`"]
pub type PORRSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `PORRSTF`"]
pub struct PORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "System reset from CPU reset flag"]
pub type SFTRSTF_A = CPURSTF_A;
#[doc = "Reader of field `SFTRSTF`"]
pub type SFTRSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `SFTRSTF`"]
pub struct SFTRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFTRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Independent Watchdog reset flag"]
pub type IWDG1RSTF_A = CPURSTF_A;
#[doc = "Reader of field `IWDG1RSTF`"]
pub type IWDG1RSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `IWDG1RSTF`"]
pub struct IWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG1RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Window Watchdog reset flag"]
pub type WWDG1RSTF_A = CPURSTF_A;
#[doc = "Reader of field `WWDG1RSTF`"]
pub type WWDG1RSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `WWDG1RSTF`"]
pub struct WWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDG1RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
#[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LPWRRSTF_A = CPURSTF_A;
#[doc = "Reader of field `LPWRRSTF`"]
pub type LPWRRSTF_R = crate::R<bool, CPURSTF_A>;
#[doc = "Write proxy for field `LPWRRSTF`"]
pub struct LPWRRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPWRRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTF_A::RESETOCCOURRED)
    }
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
impl R {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&self) -> CPURSTF_R {
        CPURSTF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&self) -> D1RSTF_R {
        D1RSTF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&self) -> D2RSTF_R {
        D2RSTF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&mut self) -> CPURSTF_W {
        CPURSTF_W { w: self }
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&mut self) -> D1RSTF_W {
        D1RSTF_W { w: self }
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&mut self) -> D2RSTF_W {
        D2RSTF_W { w: self }
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W {
        BORRSTF_W { w: self }
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W {
        PINRSTF_W { w: self }
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W { w: self }
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W {
        SFTRSTF_W { w: self }
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W {
        IWDG1RSTF_W { w: self }
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W {
        WWDG1RSTF_W { w: self }
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W {
        LPWRRSTF_W { w: self }
    }
}
