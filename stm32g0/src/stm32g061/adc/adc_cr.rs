#[doc = "Register `ADC_CR` reader"]
pub struct R(crate::R<ADC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CR` writer"]
pub struct W(crate::W<ADC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CR_SPEC>;
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
impl From<crate::W<ADC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCALÂ =Â 0, ADSTPÂ =Â 0, ADSTARTÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEN_A {
    #[doc = "0: ADC is disabled (OFF state)"]
    B_0X0 = 0,
    #[doc = "1: Write 1 to enable the ADC."]
    B_0X1 = 1,
}
impl From<ADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCALÂ =Â 0, ADSTPÂ =Â 0, ADSTARTÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)"]
pub struct ADEN_R(crate::FieldReader<bool, ADEN_A>);
impl ADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEN_A {
        match self.bits {
            false => ADEN_A::B_0X0,
            true => ADEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADEN_A::B_0X1
    }
}
impl core::ops::Deref for ADEN_R {
    type Target = crate::FieldReader<bool, ADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADEN` writer - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCALÂ =Â 0, ADSTPÂ =Â 0, ADSTARTÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)"]
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC is disabled (OFF state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADEN_A::B_0X0)
    }
    #[doc = "Write 1 to enable the ADC."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to '1â\u{80}\u{99} is only effective when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDIS_A {
    #[doc = "0: No ADDIS command ongoing"]
    B_0X0 = 0,
    #[doc = "1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress. "]
    B_0X1 = 1,
}
impl From<ADDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to '1â\u{80}\u{99} is only effective when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
pub struct ADDIS_R(crate::FieldReader<bool, ADDIS_A>);
impl ADDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDIS_A {
        match self.bits {
            false => ADDIS_A::B_0X0,
            true => ADDIS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADDIS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADDIS_A::B_0X1
    }
}
impl core::ops::Deref for ADDIS_R {
    type Target = crate::FieldReader<bool, ADDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDIS` writer - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to '1â\u{80}\u{99} is only effective when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
pub struct ADDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADDIS command ongoing"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADDIS_A::B_0X0)
    }
    #[doc = "Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADDIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONTÂ =Â 0, DISCENÂ =Â 0), when software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONTÂ =Â 0, DISCENÂ =Â 1), when the software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADENÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and there is no pending request to disable the ADC). After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTART_A {
    #[doc = "0: No ADC conversion is ongoing."]
    B_0X0 = 0,
    #[doc = "1: Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting."]
    B_0X1 = 1,
}
impl From<ADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONTÂ =Â 0, DISCENÂ =Â 0), when software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONTÂ =Â 0, DISCENÂ =Â 1), when the software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADENÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and there is no pending request to disable the ADC). After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored."]
pub struct ADSTART_R(crate::FieldReader<bool, ADSTART_A>);
impl ADSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTART_A {
        match self.bits {
            false => ADSTART_A::B_0X0,
            true => ADSTART_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADSTART_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADSTART_A::B_0X1
    }
}
impl core::ops::Deref for ADSTART_R {
    type Target = crate::FieldReader<bool, ADSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSTART` writer - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONTÂ =Â 0, DISCENÂ =Â 0), when software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONTÂ =Â 0, DISCENÂ =Â 1), when the software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADENÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and there is no pending request to disable the ADC). After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored."]
pub struct ADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADC conversion is ongoing."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADSTART_A::B_0X0)
    }
    #[doc = "Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADSTART_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: Setting ADSTP to '1â\u{80}\u{99} is only effective when ADSTARTÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and may be converting and there is no pending request to disable the ADC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTP_A {
    #[doc = "0: No ADC stop conversion command ongoing"]
    B_0X0 = 0,
    #[doc = "1: Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress."]
    B_0X1 = 1,
}
impl From<ADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: Setting ADSTP to '1â\u{80}\u{99} is only effective when ADSTARTÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and may be converting and there is no pending request to disable the ADC)"]
pub struct ADSTP_R(crate::FieldReader<bool, ADSTP_A>);
impl ADSTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTP_A {
        match self.bits {
            false => ADSTP_A::B_0X0,
            true => ADSTP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADSTP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADSTP_A::B_0X1
    }
}
impl core::ops::Deref for ADSTP_R {
    type Target = crate::FieldReader<bool, ADSTP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSTP` writer - ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: Setting ADSTP to '1â\u{80}\u{99} is only effective when ADSTARTÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and may be converting and there is no pending request to disable the ADC)"]
pub struct ADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ADC stop conversion command ongoing"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADSTP_A::B_0X0)
    }
    #[doc = "Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADSTP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tADCVREG_SETUP. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVREGEN_A {
    #[doc = "0: ADC voltage regulator disabled"]
    B_0X0 = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    B_0X1 = 1,
}
impl From<ADVREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVREGEN` reader - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tADCVREG_SETUP. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
pub struct ADVREGEN_R(crate::FieldReader<bool, ADVREGEN_A>);
impl ADVREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADVREGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVREGEN_A {
        match self.bits {
            false => ADVREGEN_A::B_0X0,
            true => ADVREGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADVREGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADVREGEN_A::B_0X1
    }
}
impl core::ops::Deref for ADVREGEN_R {
    type Target = crate::FieldReader<bool, ADVREGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVREGEN` writer - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tADCVREG_SETUP. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
pub struct ADVREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVREGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVREGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADVREGEN_A::B_0X0)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADVREGEN_A::B_0X1)
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
#[doc = "ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0). The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (ADC enabled and no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCAL_A {
    #[doc = "0: Calibration complete"]
    B_0X0 = 0,
    #[doc = "1: Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress."]
    B_0X1 = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0). The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (ADC enabled and no conversion is ongoing)."]
pub struct ADCAL_R(crate::FieldReader<bool, ADCAL_A>);
impl ADCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::B_0X0,
            true => ADCAL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADCAL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADCAL_A::B_0X1
    }
}
impl core::ops::Deref for ADCAL_R {
    type Target = crate::FieldReader<bool, ADCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0). The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (ADC enabled and no conversion is ongoing)."]
pub struct ADCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADCAL_A::B_0X0)
    }
    #[doc = "Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADCAL_A::B_0X1)
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
    #[doc = "Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCALÂ =Â 0, ADSTPÂ =Â 0, ADSTARTÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to '1â\u{80}\u{99} is only effective when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONTÂ =Â 0, DISCENÂ =Â 0), when software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONTÂ =Â 0, DISCENÂ =Â 1), when the software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADENÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and there is no pending request to disable the ADC). After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored."]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: Setting ADSTP to '1â\u{80}\u{99} is only effective when ADSTARTÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and may be converting and there is no pending request to disable the ADC)"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tADCVREG_SETUP. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0). The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (ADC enabled and no conversion is ongoing)."]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCALÂ =Â 0, ADSTPÂ =Â 0, ADSTARTÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)"]
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
    #[doc = "Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: Setting ADDIS to '1â\u{80}\u{99} is only effective when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W {
        ADDIS_W { w: self }
    }
    #[doc = "Bit 2 - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \\[1:0\\]
configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONTÂ =Â 0, DISCENÂ =Â 0), when software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONTÂ =Â 0, DISCENÂ =Â 1), when the software trigger is selected (EXTENÂ =Â 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADENÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and there is no pending request to disable the ADC). After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored."]
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W {
        ADSTART_W { w: self }
    }
    #[doc = "Bit 4 - ADC stop conversion command This bit is set by software to stop and discard an ongoing conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to accept a new start conversion command. Note: Setting ADSTP to '1â\u{80}\u{99} is only effective when ADSTARTÂ =Â 1 and ADDISÂ =Â 0 (ADC is enabled and may be converting and there is no pending request to disable the ADC)"]
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W {
        ADSTP_W { w: self }
    }
    #[doc = "Bit 28 - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after tADCVREG_SETUP. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W {
        ADVREGEN_W { w: self }
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0). The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADENÂ =Â 1 and ADSTARTÂ =Â 0 (ADC enabled and no conversion is ongoing)."]
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W {
        ADCAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cr](index.html) module"]
pub struct ADC_CR_SPEC;
impl crate::RegisterSpec for ADC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cr::R](R) reader structure"]
impl crate::Readable for ADC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cr::W](W) writer structure"]
impl crate::Writable for ADC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CR to value 0"]
impl crate::Resettable for ADC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
