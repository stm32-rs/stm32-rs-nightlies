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
#[doc = "Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SITP_A {
    #[doc = "0: SPI with rising edge to strobe data"]
    B_0X0 = 0,
    #[doc = "1: SPI with falling edge to strobe data"]
    B_0X1 = 1,
    #[doc = "2: Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    B_0X2 = 2,
    #[doc = "3: Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    B_0X3 = 3,
}
impl From<SITP_A> for u8 {
    #[inline(always)]
    fn from(variant: SITP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SITP` reader - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct SITP_R(crate::FieldReader<u8, SITP_A>);
impl SITP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SITP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SITP_A {
        match self.bits {
            0 => SITP_A::B_0X0,
            1 => SITP_A::B_0X1,
            2 => SITP_A::B_0X2,
            3 => SITP_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SITP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SITP_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == SITP_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == SITP_A::B_0X3
    }
}
impl core::ops::Deref for SITP_R {
    type Target = crate::FieldReader<u8, SITP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SITP` writer - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct SITP_W<'a> {
    w: &'a mut W,
}
impl<'a> SITP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SITP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SPI with rising edge to strobe data"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SITP_A::B_0X0)
    }
    #[doc = "SPI with falling edge to strobe data"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SITP_A::B_0X1)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SITP_A::B_0X2)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SITP_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPICKSEL_A {
    #[doc = "0: clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    B_0X0 = 0,
    #[doc = "1: clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    B_0X1 = 1,
}
impl From<SPICKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPICKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPICKSEL` reader - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct SPICKSEL_R(crate::FieldReader<u8, SPICKSEL_A>);
impl SPICKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPICKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPICKSEL_A> {
        match self.bits {
            0 => Some(SPICKSEL_A::B_0X0),
            1 => Some(SPICKSEL_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SPICKSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SPICKSEL_A::B_0X1
    }
}
impl core::ops::Deref for SPICKSEL_R {
    type Target = crate::FieldReader<u8, SPICKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPICKSEL` writer - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct SPICKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPICKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPICKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPICKSEL_A::B_0X0)
    }
    #[doc = "clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPICKSEL_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Short-circuit detector enable on channel y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCDEN_A {
    #[doc = "0: Input channel y will not be guarded by the short-circuit detector"]
    B_0X0 = 0,
    #[doc = "1: Input channel y will be continuously guarded by the short-circuit detector "]
    B_0X1 = 1,
}
impl From<SCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCDEN` reader - Short-circuit detector enable on channel y"]
pub struct SCDEN_R(crate::FieldReader<bool, SCDEN_A>);
impl SCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDEN_A {
        match self.bits {
            false => SCDEN_A::B_0X0,
            true => SCDEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SCDEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SCDEN_A::B_0X1
    }
}
impl core::ops::Deref for SCDEN_R {
    type Target = crate::FieldReader<bool, SCDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDEN` writer - Short-circuit detector enable on channel y"]
pub struct SCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel y will not be guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SCDEN_A::B_0X0)
    }
    #[doc = "Input channel y will be continuously guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SCDEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Clock absence detector enable on channel y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKABEN_A {
    #[doc = "0: Clock absence detector disabled on channel y"]
    B_0X0 = 0,
    #[doc = "1: Clock absence detector enabled on channel y"]
    B_0X1 = 1,
}
impl From<CKABEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKABEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKABEN` reader - Clock absence detector enable on channel y"]
pub struct CKABEN_R(crate::FieldReader<bool, CKABEN_A>);
impl CKABEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKABEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKABEN_A {
        match self.bits {
            false => CKABEN_A::B_0X0,
            true => CKABEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKABEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKABEN_A::B_0X1
    }
}
impl core::ops::Deref for CKABEN_R {
    type Target = crate::FieldReader<bool, CKABEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKABEN` writer - Clock absence detector enable on channel y"]
pub struct CKABEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKABEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock absence detector disabled on channel y"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKABEN_A::B_0X0)
    }
    #[doc = "Clock absence detector enabled on channel y"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKABEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN_A {
    #[doc = "0: Channel y disabled"]
    B_0X0 = 0,
    #[doc = "1: Channel y enabled"]
    B_0X1 = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` reader - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
pub struct CHEN_R(crate::FieldReader<bool, CHEN_A>);
impl CHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::B_0X0,
            true => CHEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CHEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CHEN_A::B_0X1
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<bool, CHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel y disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CHEN_A::B_0X0)
    }
    #[doc = "Channel y enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CHEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHINSEL_A {
    #[doc = "0: Channel inputs are taken from pins of the same channel y."]
    B_0X0 = 0,
    #[doc = "1: Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)."]
    B_0X1 = 1,
}
impl From<CHINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHINSEL` reader - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct CHINSEL_R(crate::FieldReader<bool, CHINSEL_A>);
impl CHINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHINSEL_A {
        match self.bits {
            false => CHINSEL_A::B_0X0,
            true => CHINSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CHINSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CHINSEL_A::B_0X1
    }
}
impl core::ops::Deref for CHINSEL_R {
    type Target = crate::FieldReader<bool, CHINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINSEL` writer - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct CHINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHINSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel inputs are taken from pins of the same channel y."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CHINSEL_A::B_0X0)
    }
    #[doc = "Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CHINSEL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATMPX_A {
    #[doc = "0: Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected."]
    B_0X0 = 0,
    #[doc = "1: Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register."]
    B_0X1 = 1,
}
impl From<DATMPX_A> for u8 {
    #[inline(always)]
    fn from(variant: DATMPX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATMPX` reader - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct DATMPX_R(crate::FieldReader<u8, DATMPX_A>);
impl DATMPX_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATMPX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATMPX_A> {
        match self.bits {
            0 => Some(DATMPX_A::B_0X0),
            1 => Some(DATMPX_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DATMPX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DATMPX_A::B_0X1
    }
}
impl core::ops::Deref for DATMPX_R {
    type Target = crate::FieldReader<u8, DATMPX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMPX` writer - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct DATMPX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMPX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMPX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DATMPX_A::B_0X0)
    }
    #[doc = "Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DATMPX_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATPACK_A {
    #[doc = "0: Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y."]
    B_0X0 = 0,
    #[doc = "1: Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples:"]
    B_0X1 = 1,
}
impl From<DATPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DATPACK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATPACK` reader - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct DATPACK_R(crate::FieldReader<u8, DATPACK_A>);
impl DATPACK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATPACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATPACK_A> {
        match self.bits {
            0 => Some(DATPACK_A::B_0X0),
            1 => Some(DATPACK_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DATPACK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DATPACK_A::B_0X1
    }
}
impl core::ops::Deref for DATPACK_R {
    type Target = crate::FieldReader<u8, DATPACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATPACK` writer - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
pub struct DATPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DATPACK_A::B_0X0)
    }
    #[doc = "Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DATPACK_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKOUTDIV_A {
    #[doc = "0: Output clock generation is disabled (CKOUT signal is set to low state)"]
    B_0X0 = 0,
}
impl From<CKOUTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOUTDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKOUTDIV` reader - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
pub struct CKOUTDIV_R(crate::FieldReader<u8, CKOUTDIV_A>);
impl CKOUTDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKOUTDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKOUTDIV_A> {
        match self.bits {
            0 => Some(CKOUTDIV_A::B_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKOUTDIV_A::B_0X0
    }
}
impl core::ops::Deref for CKOUTDIV_R {
    type Target = crate::FieldReader<u8, CKOUTDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKOUTDIV` writer - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
pub struct CKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKOUTDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output clock generation is disabled (CKOUT signal is set to low state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOUTSRC_A {
    #[doc = "0: Source for output clock is from system clock"]
    B_0X0 = 0,
    #[doc = "1: Source for output clock is from audio clock"]
    B_0X1 = 1,
}
impl From<CKOUTSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CKOUTSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKOUTSRC` reader - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub struct CKOUTSRC_R(crate::FieldReader<bool, CKOUTSRC_A>);
impl CKOUTSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKOUTSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOUTSRC_A {
        match self.bits {
            false => CKOUTSRC_A::B_0X0,
            true => CKOUTSRC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKOUTSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKOUTSRC_A::B_0X1
    }
}
impl core::ops::Deref for CKOUTSRC_R {
    type Target = crate::FieldReader<bool, CKOUTSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKOUTSRC` writer - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub struct CKOUTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKOUTSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Source for output clock is from system clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKOUTSRC_A::B_0X0)
    }
    #[doc = "Source for output clock is from audio clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKOUTSRC_A::B_0X1)
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
#[doc = "Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDMEN_A {
    #[doc = "0: DFSDM interface disabled"]
    B_0X0 = 0,
    #[doc = "1: DFSDM interface enabled"]
    B_0X1 = 1,
}
impl From<DFSDMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDMEN` reader - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub struct DFSDMEN_R(crate::FieldReader<bool, DFSDMEN_A>);
impl DFSDMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMEN_A {
        match self.bits {
            false => DFSDMEN_A::B_0X0,
            true => DFSDMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DFSDMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DFSDMEN_A::B_0X1
    }
}
impl core::ops::Deref for DFSDMEN_R {
    type Target = crate::FieldReader<bool, DFSDMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMEN` writer - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DFSDM interface disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFSDMEN_A::B_0X0)
    }
    #[doc = "DFSDM interface enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFSDMEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel y"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel y"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W {
        SITP_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â\u{88}\u{86} modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W {
        SPICKSEL_W { w: self }
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel y"]
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W {
        SCDEN_W { w: self }
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel y"]
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W {
        CKABEN_W { w: self }
    }
    #[doc = "Bit 7 - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 8 - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W {
        CHINSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W {
        DATMPX_W { w: self }
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W {
        DATPACK_W { w: self }
    }
    #[doc = "Bits 16:23 - Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W { w: self }
    }
    #[doc = "Bit 30 - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W {
        CKOUTSRC_W { w: self }
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel 0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
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
