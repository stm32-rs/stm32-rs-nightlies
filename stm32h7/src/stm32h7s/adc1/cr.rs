///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ADEN` reader - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
pub type ADEN_R = crate::BitReader;
///Field `ADEN` writer - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDIS` reader - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ADDIS_R = crate::BitReader;
///Field `ADDIS` writer - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTART` reader - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type ADSTART_R = crate::BitReader;
///Field `ADSTART` writer - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JADSTART` reader - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type JADSTART_R = crate::BitReader;
///Field `JADSTART` writer - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSTP` reader - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
pub type ADSTP_R = crate::BitReader;
///Field `ADSTP` writer - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JADSTP` reader - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) Note: In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
pub type JADSTP_R = crate::BitReader;
///Field `JADSTP` writer - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) Note: In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADVREGEN` reader - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to Section 25.4.6: ADC Deep-power-down mode (DEEPPWD) and ADC voltage regulator (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_R = crate::BitReader;
///Field `ADVREGEN` writer - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to Section 25.4.6: ADC Deep-power-down mode (DEEPPWD) and ADC voltage regulator (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEEPPWD` reader - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DEEPPWD_R = crate::BitReader;
///Field `DEEPPWD` writer - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCALDIF` reader - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADCALDIF_R = crate::BitReader;
///Field `ADCALDIF` writer - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
pub type ADCAL_R = crate::BitReader;
///Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) Note: In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to Section 25.4.6: ADC Deep-power-down mode (DEEPPWD) and ADC voltage regulator (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("aden", &self.aden())
            .field("addis", &self.addis())
            .field("adstart", &self.adstart())
            .field("jadstart", &self.jadstart())
            .field("adstp", &self.adstp())
            .field("jadstp", &self.jadstp())
            .field("advregen", &self.advregen())
            .field("deeppwd", &self.deeppwd())
            .field("adcaldif", &self.adcaldif())
            .field("adcal", &self.adcal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<'_, CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<'_, CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<'_, CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W<'_, CRrs> {
        JADSTART_W::new(self, 3)
    }
    ///Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<'_, CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) Note: In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W<'_, CRrs> {
        JADSTP_W::new(self, 5)
    }
    ///Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to Section 25.4.6: ADC Deep-power-down mode (DEEPPWD) and ADC voltage regulator (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W<'_, CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    ///Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<'_, CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    ///Bit 30 - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<'_, CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<'_, CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ADC1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x2000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
