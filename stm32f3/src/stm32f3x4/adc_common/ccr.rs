#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Dual ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DUAL_A {
    #[doc = "0: Independent mode"]
    INDEPENDENT = 0,
    #[doc = "1: Dual, combined regular simultaneous + injected simultaneous mode"]
    DUALRJ = 1,
    #[doc = "2: Dual, combined regular simultaneous + alternate trigger mode"]
    DUALRA = 2,
    #[doc = "3: Dual, combined interleaved mode + injected simultaneous mode"]
    DUALIJ = 3,
    #[doc = "5: Dual, injected simultaneous mode only"]
    DUALJ = 5,
    #[doc = "6: Dual, regular simultaneous mode only"]
    DUALR = 6,
    #[doc = "7: Dual, interleaved mode only"]
    DUALI = 7,
    #[doc = "9: Dual, alternate trigger mode only"]
    DUALA = 9,
}
impl From<DUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub struct DUAL_R(crate::FieldReader<u8, DUAL_A>);
impl DUAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DUAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUAL_A> {
        match self.bits {
            0 => Some(DUAL_A::INDEPENDENT),
            1 => Some(DUAL_A::DUALRJ),
            2 => Some(DUAL_A::DUALRA),
            3 => Some(DUAL_A::DUALIJ),
            5 => Some(DUAL_A::DUALJ),
            6 => Some(DUAL_A::DUALR),
            7 => Some(DUAL_A::DUALI),
            9 => Some(DUAL_A::DUALA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == DUAL_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `DUALRJ`"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        **self == DUAL_A::DUALRJ
    }
    #[doc = "Checks if the value of the field is `DUALRA`"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        **self == DUAL_A::DUALRA
    }
    #[doc = "Checks if the value of the field is `DUALIJ`"]
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        **self == DUAL_A::DUALIJ
    }
    #[doc = "Checks if the value of the field is `DUALJ`"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        **self == DUAL_A::DUALJ
    }
    #[doc = "Checks if the value of the field is `DUALR`"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        **self == DUAL_A::DUALR
    }
    #[doc = "Checks if the value of the field is `DUALI`"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        **self == DUAL_A::DUALI
    }
    #[doc = "Checks if the value of the field is `DUALA`"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        **self == DUAL_A::DUALA
    }
}
impl core::ops::Deref for DUAL_R {
    type Target = crate::FieldReader<u8, DUAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub struct DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUAL_A::INDEPENDENT)
    }
    #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(DUAL_A::DUALRJ)
    }
    #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(DUAL_A::DUALRA)
    }
    #[doc = "Dual, combined interleaved mode + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut W {
        self.variant(DUAL_A::DUALIJ)
    }
    #[doc = "Dual, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(DUAL_A::DUALJ)
    }
    #[doc = "Dual, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(DUAL_A::DUALR)
    }
    #[doc = "Dual, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(DUAL_A::DUALI)
    }
    #[doc = "Dual, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(DUAL_A::DUALA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub struct DELAY_R(crate::FieldReader<u8, u8>);
impl DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "DMA configuration (for dual ADC mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMACFG_A {
    #[doc = "0: DMA one shot mode selected"]
    ONESHOT = 0,
    #[doc = "1: DMA circular mode selected"]
    CIRCULATOR = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACFG` reader - DMA configuration (for dual ADC mode)"]
pub struct DMACFG_R(crate::FieldReader<bool, DMACFG_A>);
impl DMACFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMACFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::ONESHOT,
            true => DMACFG_A::CIRCULATOR,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        **self == DMACFG_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `CIRCULATOR`"]
    #[inline(always)]
    pub fn is_circulator(&self) -> bool {
        **self == DMACFG_A::CIRCULATOR
    }
}
impl core::ops::Deref for DMACFG_R {
    type Target = crate::FieldReader<bool, DMACFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMACFG` writer - DMA configuration (for dual ADC mode)"]
pub struct DMACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMACFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::ONESHOT)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn circulator(self) -> &'a mut W {
        self.variant(DMACFG_A::CIRCULATOR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Direct memory access mode for dual ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MDMA_A {
    #[doc = "0: MDMA mode disabled"]
    DISABLED = 0,
    #[doc = "2: MDMA mode enabled for 12 and 10-bit resolution"]
    BITS12_10 = 2,
    #[doc = "3: MDMA mode enabled for 8 and 6-bit resolution"]
    BITS8_6 = 3,
}
impl From<MDMA_A> for u8 {
    #[inline(always)]
    fn from(variant: MDMA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MDMA` reader - Direct memory access mode for dual ADC mode"]
pub struct MDMA_R(crate::FieldReader<u8, MDMA_A>);
impl MDMA_R {
    pub(crate) fn new(bits: u8) -> Self {
        MDMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MDMA_A> {
        match self.bits {
            0 => Some(MDMA_A::DISABLED),
            2 => Some(MDMA_A::BITS12_10),
            3 => Some(MDMA_A::BITS8_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MDMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `BITS12_10`"]
    #[inline(always)]
    pub fn is_bits12_10(&self) -> bool {
        **self == MDMA_A::BITS12_10
    }
    #[doc = "Checks if the value of the field is `BITS8_6`"]
    #[inline(always)]
    pub fn is_bits8_6(&self) -> bool {
        **self == MDMA_A::BITS8_6
    }
}
impl core::ops::Deref for MDMA_R {
    type Target = crate::FieldReader<u8, MDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMA` writer - Direct memory access mode for dual ADC mode"]
pub struct MDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MDMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMA_A::DISABLED)
    }
    #[doc = "MDMA mode enabled for 12 and 10-bit resolution"]
    #[inline(always)]
    pub fn bits12_10(self) -> &'a mut W {
        self.variant(MDMA_A::BITS12_10)
    }
    #[doc = "MDMA mode enabled for 8 and 6-bit resolution"]
    #[inline(always)]
    pub fn bits8_6(self) -> &'a mut W {
        self.variant(MDMA_A::BITS8_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    ASYNCHRONOUS = 0,
    #[doc = "1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    SYNCDIV1 = 1,
    #[doc = "2: Use AHB clock rcc_hclk3 divided by 2"]
    SYNCDIV2 = 2,
    #[doc = "3: Use AHB clock rcc_hclk3 divided by 4"]
    SYNCDIV4 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub struct CKMODE_R(crate::FieldReader<u8, CKMODE_A>);
impl CKMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::ASYNCHRONOUS,
            1 => CKMODE_A::SYNCDIV1,
            2 => CKMODE_A::SYNCDIV2,
            3 => CKMODE_A::SYNCDIV4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == CKMODE_A::ASYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `SYNCDIV1`"]
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        **self == CKMODE_A::SYNCDIV1
    }
    #[doc = "Checks if the value of the field is `SYNCDIV2`"]
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        **self == CKMODE_A::SYNCDIV2
    }
    #[doc = "Checks if the value of the field is `SYNCDIV4`"]
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        **self == CKMODE_A::SYNCDIV4
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<u8, CKMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CKMODE_A::ASYNCHRONOUS)
    }
    #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNCDIV1)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 2"]
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNCDIV2)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 4"]
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNCDIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    #[doc = "0: V_REFINT channel disabled"]
    DISABLED = 0,
    #[doc = "1: V_REFINT channel enabled"]
    ENABLED = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub struct VREFEN_R(crate::FieldReader<bool, VREFEN_A>);
impl VREFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::DISABLED,
            true => VREFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VREFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VREFEN_A::ENABLED
    }
}
impl core::ops::Deref for VREFEN_R {
    type Target = crate::FieldReader<bool, VREFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::DISABLED)
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::ENABLED)
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
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_A {
    #[doc = "0: Temperature sensor channel disabled"]
    DISABLED = 0,
    #[doc = "1: Temperature sensor channel enabled"]
    ENABLED = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - Temperature sensor enable"]
pub struct TSEN_R(crate::FieldReader<bool, TSEN_A>);
impl TSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::DISABLED,
            true => TSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSEN_A::ENABLED
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, TSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEN` writer - Temperature sensor enable"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSEN_A::DISABLED)
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSEN_A::ENABLED)
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
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATEN_A {
    #[doc = "0: V_BAT channel disabled"]
    DISABLED = 0,
    #[doc = "1: V_BAT channel enabled"]
    ENABLED = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub struct VBATEN_R(crate::FieldReader<bool, VBATEN_A>);
impl VBATEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::DISABLED,
            true => VBATEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VBATEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VBATEN_A::ENABLED
    }
}
impl core::ops::Deref for VBATEN_R {
    type Target = crate::FieldReader<bool, VBATEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub struct VBATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATEN_A::DISABLED)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATEN_A::ENABLED)
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
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA configuration (for dual ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for dual ADC mode"]
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W { w: self }
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bit 13 - DMA configuration (for dual ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W {
        DMACFG_W { w: self }
    }
    #[doc = "Bits 14:15 - Direct memory access mode for dual ADC mode"]
    #[inline(always)]
    pub fn mdma(&mut self) -> MDMA_W {
        MDMA_W { w: self }
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W {
        VBATEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
