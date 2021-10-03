#[doc = "Register `RSR` reader"]
pub struct R(crate::R<RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSR` writer"]
pub struct W(crate::W<RSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub struct RMVF_R(crate::FieldReader<bool, RMVF_A>);
impl RMVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMVF_R(crate::FieldReader::new(bits))
    }
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
        **self == RMVF_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RMVF_A::CLEAR
    }
}
impl core::ops::Deref for RMVF_R {
    type Target = crate::FieldReader<bool, RMVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMVF_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
#[doc = "Field `CPURSTF` reader - CPU reset flag"]
pub struct CPURSTF_R(crate::FieldReader<bool, CPURSTF_A>);
impl CPURSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPURSTF_R(crate::FieldReader::new(bits))
    }
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
        **self == CPURSTF_A::NORESETOCCOURED
    }
    #[doc = "Checks if the value of the field is `RESETOCCOURRED`"]
    #[inline(always)]
    pub fn is_reset_occourred(&self) -> bool {
        **self == CPURSTF_A::RESETOCCOURRED
    }
}
impl core::ops::Deref for CPURSTF_R {
    type Target = crate::FieldReader<bool, CPURSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPURSTF` writer - CPU reset flag"]
pub struct CPURSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPURSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPURSTF_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "D1 domain power switch reset flag"]
pub type D1RSTF_A = CPURSTF_A;
#[doc = "Field `D1RSTF` reader - D1 domain power switch reset flag"]
pub type D1RSTF_R = CPURSTF_R;
#[doc = "Field `D1RSTF` writer - D1 domain power switch reset flag"]
pub struct D1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> D1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D1RSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(D1RSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(D1RSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "D2 domain power switch reset flag"]
pub type D2RSTF_A = CPURSTF_A;
#[doc = "Field `D2RSTF` reader - D2 domain power switch reset flag"]
pub type D2RSTF_R = CPURSTF_R;
#[doc = "Field `D2RSTF` writer - D2 domain power switch reset flag"]
pub struct D2RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> D2RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D2RSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(D2RSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(D2RSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "BOR reset flag"]
pub type BORRSTF_A = CPURSTF_A;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BORRSTF_R = CPURSTF_R;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub struct BORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BORRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BORRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(BORRSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(BORRSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Pin reset flag (NRST)"]
pub type PINRSTF_A = CPURSTF_A;
#[doc = "Field `PINRSTF` reader - Pin reset flag (NRST)"]
pub type PINRSTF_R = CPURSTF_R;
#[doc = "Field `PINRSTF` writer - Pin reset flag (NRST)"]
pub struct PINRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PINRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(PINRSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(PINRSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "POR/PDR reset flag"]
pub type PORRSTF_A = CPURSTF_A;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = CPURSTF_R;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub struct PORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(PORRSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(PORRSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "System reset from CPU reset flag"]
pub type SFTRSTF_A = CPURSTF_A;
#[doc = "Field `SFTRSTF` reader - System reset from CPU reset flag"]
pub type SFTRSTF_R = CPURSTF_R;
#[doc = "Field `SFTRSTF` writer - System reset from CPU reset flag"]
pub struct SFTRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFTRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(SFTRSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(SFTRSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Independent Watchdog reset flag"]
pub type IWDG1RSTF_A = CPURSTF_A;
#[doc = "Field `IWDG1RSTF` reader - Independent Watchdog reset flag"]
pub type IWDG1RSTF_R = CPURSTF_R;
#[doc = "Field `IWDG1RSTF` writer - Independent Watchdog reset flag"]
pub struct IWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG1RSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(IWDG1RSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(IWDG1RSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Window Watchdog reset flag"]
pub type WWDG1RSTF_A = CPURSTF_A;
#[doc = "Field `WWDG1RSTF` reader - Window Watchdog reset flag"]
pub type WWDG1RSTF_R = CPURSTF_R;
#[doc = "Field `WWDG1RSTF` writer - Window Watchdog reset flag"]
pub struct WWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDG1RSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(WWDG1RSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(WWDG1RSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LPWRRSTF_A = CPURSTF_A;
#[doc = "Field `LPWRRSTF` reader - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LPWRRSTF_R = CPURSTF_R;
#[doc = "Field `LPWRRSTF` writer - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub struct LPWRRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPWRRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESETOCCOURED)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESETOCCOURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](index.html) module"]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr::R](R) reader structure"]
impl crate::Readable for RSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsr::W](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
