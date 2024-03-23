#[doc = "Register `ADC_CR` reader"]
pub type R = crate::R<ADC_CRrs>;
#[doc = "Register `ADC_CR` writer"]
pub type W = crate::W<ADC_CRrs>;
#[doc = "Field `ADEN` reader - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
pub type ADEN_R = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDIS` reader - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type ADDIS_R = crate::BitReader;
#[doc = "Field `ADDIS` writer - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type ADSTART_R = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type JADSTART_R = crate::BitReader;
#[doc = "Field `JADSTART` writer - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
pub type ADSTP_R = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
pub type JADSTP_R = crate::BitReader;
#[doc = "Field `JADSTP` writer - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALLIN` reader - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADCALLIN_R = crate::BitReader;
#[doc = "Field `ADCALLIN` writer - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALINDEX` reader - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\\[31:0\\]
correspond to the location of CALINDEX\\[3:0\\]
calibration factor data (see for details)."]
pub type CALINDEX_R = crate::FieldReader;
#[doc = "Field `CALINDEX` writer - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\\[31:0\\]
correspond to the location of CALINDEX\\[3:0\\]
calibration factor data (see for details)."]
pub type CALINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADVREGEN_R = crate::BitReader;
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPPWD` reader - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DEEPPWD_R = crate::BitReader;
#[doc = "Field `DEEPPWD` writer - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0."]
pub type ADCAL_R = crate::BitReader;
#[doc = "Field `ADCAL` writer - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0."]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\\[31:0\\]
correspond to the location of CALINDEX\\[3:0\\]
calibration factor data (see for details)."]
    #[inline(always)]
    pub fn calindex(&self) -> CALINDEX_R {
        CALINDEX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0."]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<ADC_CRrs> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<ADC_CRrs> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\\[1:0\\]
= 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<ADC_CRrs> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\\[1:0\\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<ADC_CRrs> {
        JADSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)."]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<ADC_CRrs> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<ADC_CRrs> {
        JADSTP_W::new(self, 5)
    }
    #[doc = "Bit 16 - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn adcallin(&mut self) -> ADCALLIN_W<ADC_CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\\[31:0\\]
correspond to the location of CALINDEX\\[3:0\\]
calibration factor data (see for details)."]
    #[inline(always)]
    #[must_use]
    pub fn calindex(&mut self) -> CALINDEX_W<ADC_CRrs> {
        CALINDEX_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<ADC_CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<ADC_CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<ADC_CRrs> {
        ADCAL_W::new(self, 31)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CRrs;
impl crate::RegisterSpec for ADC_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cr::R`](R) reader structure"]
impl crate::Readable for ADC_CRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_cr::W`](W) writer structure"]
impl crate::Writable for ADC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CR to value 0x2000_0000"]
impl crate::Resettable for ADC_CRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
