#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Boot mode selected by the boot pins status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOT_MODE_A {
    #[doc = "0: Main Flash memory boot mode"]
    MAINFLASH = 0,
    #[doc = "1: System Flash memory boot mode"]
    SYSTEMFLASH = 1,
    #[doc = "3: Embedded SRAM boot mode"]
    SRAM = 3,
}
impl From<BOOT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BOOT_MODE` reader - Boot mode selected by the boot pins status bits"]
pub struct BOOT_MODE_R(crate::FieldReader<u8, BOOT_MODE_A>);
impl BOOT_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOT_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_MODE_A> {
        match self.bits {
            0 => Some(BOOT_MODE_A::MAINFLASH),
            1 => Some(BOOT_MODE_A::SYSTEMFLASH),
            3 => Some(BOOT_MODE_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAINFLASH`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        **self == BOOT_MODE_A::MAINFLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMFLASH`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        **self == BOOT_MODE_A::SYSTEMFLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == BOOT_MODE_A::SRAM
    }
}
impl core::ops::Deref for BOOT_MODE_R {
    type Target = crate::FieldReader<u8, BOOT_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MAINFLASH = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SYSTEMFLASH = 1,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    SRAM = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub struct MEM_MODE_R(crate::FieldReader<u8, MEM_MODE_A>);
impl MEM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEM_MODE_A> {
        match self.bits {
            0 => Some(MEM_MODE_A::MAINFLASH),
            1 => Some(MEM_MODE_A::SYSTEMFLASH),
            3 => Some(MEM_MODE_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAINFLASH`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        **self == MEM_MODE_A::MAINFLASH
    }
    #[doc = "Checks if the value of the field is `SYSTEMFLASH`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        **self == MEM_MODE_A::SYSTEMFLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == MEM_MODE_A::SRAM
    }
}
impl core::ops::Deref for MEM_MODE_R {
    type Target = crate::FieldReader<u8, MEM_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MAINFLASH)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SYSTEMFLASH)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SRAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "User bank swapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFB_A {
    #[doc = "0: Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)"]
    BANK1 = 0,
    #[doc = "1: Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)"]
    BANK2 = 1,
}
impl From<UFB_A> for bool {
    #[inline(always)]
    fn from(variant: UFB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UFB` reader - User bank swapping"]
pub struct UFB_R(crate::FieldReader<bool, UFB_A>);
impl UFB_R {
    pub(crate) fn new(bits: bool) -> Self {
        UFB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFB_A {
        match self.bits {
            false => UFB_A::BANK1,
            true => UFB_A::BANK2,
        }
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        **self == UFB_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        **self == UFB_A::BANK2
    }
}
impl core::ops::Deref for UFB_R {
    type Target = crate::FieldReader<bool, UFB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFB` writer - User bank swapping"]
pub struct UFB_W<'a> {
    w: &'a mut W,
}
impl<'a> UFB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UFB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(UFB_A::BANK1)
    }
    #[doc = "Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(UFB_A::BANK2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Boot mode selected by the boot pins status bits"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - User bank swapping"]
    #[inline(always)]
    pub fn ufb(&self) -> UFB_R {
        UFB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 3 - User bank swapping"]
    #[inline(always)]
    pub fn ufb(&mut self) -> UFB_W {
        UFB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
