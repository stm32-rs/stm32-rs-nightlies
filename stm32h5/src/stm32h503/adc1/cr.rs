///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR {
    ///0: ADC disabled
    Disabled = 0,
    ///1: ADC enabled
    Enabled = 1,
}
impl From<ADENR> for bool {
    #[inline(always)]
    fn from(variant: ADENR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` reader - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
pub type ADEN_R = crate::BitReader<ADENR>;
impl ADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADENR {
        match self.bits {
            false => ADENR::Disabled,
            true => ADENR::Enabled,
        }
    }
    ///ADC disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR::Disabled
    }
    ///ADC enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR::Enabled
    }
}
/**ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW {
    ///1: Enable the ADC
    Enabled = 1,
}
impl From<ADENW> for bool {
    #[inline(always)]
    fn from(variant: ADENW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` writer - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
pub type ADEN_W<'a, REG> = crate::BitWriter1S<'a, REG, ADENW>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable the ADC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADENW::Enabled)
    }
}
/**ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR {
    ///0: No disable command active
    NotDisabling = 0,
    ///1: ADC disabling
    Disabling = 1,
}
impl From<ADDISR> for bool {
    #[inline(always)]
    fn from(variant: ADDISR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` reader - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ADDIS_R = crate::BitReader<ADDISR>;
impl ADDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDISR {
        match self.bits {
            false => ADDISR::NotDisabling,
            true => ADDISR::Disabling,
        }
    }
    ///No disable command active
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR::NotDisabling
    }
    ///ADC disabling
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR::Disabling
    }
}
/**ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW {
    ///1: Disable the ADC
    Disable = 1,
}
impl From<ADDISW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` writer - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ADDIS_W<'a, REG> = crate::BitWriter1S<'a, REG, ADDISW>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the ADC
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADDISW::Disable)
    }
}
/**ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR {
    ///0: No conversion ongoing
    NotActive = 0,
    ///1: ADC operating and may be converting
    Active = 1,
}
impl From<ADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` reader - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type ADSTART_R = crate::BitReader<ADSTARTR>;
impl ADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTARTR {
        match self.bits {
            false => ADSTARTR::NotActive,
            true => ADSTARTR::Active,
        }
    }
    ///No conversion ongoing
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR::NotActive
    }
    ///ADC operating and may be converting
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR::Active
    }
}
/**ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW {
    ///1: Start the ADC conversion (may be delayed for hardware triggers)
    StartConversion = 1,
}
impl From<ADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` writer - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type ADSTART_W<'a, REG> = crate::BitWriter1S<'a, REG, ADSTARTW>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the ADC conversion (may be delayed for hardware triggers)
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTARTW::StartConversion)
    }
}
///Field `JADSTART` reader - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub use ADSTART_R as JADSTART_R;
///Field `JADSTART` writer - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub use ADSTART_W as JADSTART_W;
/**ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR {
    ///0: No stop command active
    NotStopping = 0,
    ///1: ADC stopping conversion
    Stopping = 1,
}
impl From<ADSTPR> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` reader - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
pub type ADSTP_R = crate::BitReader<ADSTPR>;
impl ADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTPR {
        match self.bits {
            false => ADSTPR::NotStopping,
            true => ADSTPR::Stopping,
        }
    }
    ///No stop command active
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR::NotStopping
    }
    ///ADC stopping conversion
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR::Stopping
    }
}
/**ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW {
    ///1: Stop the active conversion
    StopConversion = 1,
}
impl From<ADSTPW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` writer - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
pub type ADSTP_W<'a, REG> = crate::BitWriter1S<'a, REG, ADSTPW>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop the active conversion
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTPW::StopConversion)
    }
}
///Field `JADSTP` reader - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
pub use ADSTP_R as JADSTP_R;
///Field `JADSTP` writer - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
pub use ADSTP_W as JADSTP_W;
/**ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN {
    ///0: ADC Voltage regulator disabled
    Disabled = 0,
    ///1: ADC Voltage regulator enabled
    Enabled = 1,
}
impl From<ADVREGEN> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADVREGEN` reader - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_R = crate::BitReader<ADVREGEN>;
impl ADVREGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADVREGEN {
        match self.bits {
            false => ADVREGEN::Disabled,
            true => ADVREGEN::Enabled,
        }
    }
    ///ADC Voltage regulator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN::Disabled
    }
    ///ADC Voltage regulator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN::Enabled
    }
}
///Field `ADVREGEN` writer - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG, ADVREGEN>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC Voltage regulator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Disabled)
    }
    ///ADC Voltage regulator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Enabled)
    }
}
/**Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPWD {
    ///0: ADC not in Deep-power down
    NotDeepPowerDown = 0,
    ///1: ADC in Deep-power-down (default reset state)
    DeepPowerDown = 1,
}
impl From<DEEPPWD> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD) -> Self {
        variant as u8 != 0
    }
}
///Field `DEEPPWD` reader - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DEEPPWD_R = crate::BitReader<DEEPPWD>;
impl DEEPPWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEEPPWD {
        match self.bits {
            false => DEEPPWD::NotDeepPowerDown,
            true => DEEPPWD::DeepPowerDown,
        }
    }
    ///ADC not in Deep-power down
    #[inline(always)]
    pub fn is_not_deep_power_down(&self) -> bool {
        *self == DEEPPWD::NotDeepPowerDown
    }
    ///ADC in Deep-power-down (default reset state)
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        *self == DEEPPWD::DeepPowerDown
    }
}
///Field `DEEPPWD` writer - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG, DEEPPWD>;
impl<'a, REG> DEEPPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC not in Deep-power down
    #[inline(always)]
    pub fn not_deep_power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::NotDeepPowerDown)
    }
    ///ADC in Deep-power-down (default reset state)
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::DeepPowerDown)
    }
}
/**Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF {
    ///0: Calibration for single-ended mode
    SingleEnded = 0,
    ///1: Calibration for differential mode
    Differential = 1,
}
impl From<ADCALDIF> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCALDIF` reader - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADCALDIF_R = crate::BitReader<ADCALDIF>;
impl ADCALDIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALDIF {
        match self.bits {
            false => ADCALDIF::SingleEnded,
            true => ADCALDIF::Differential,
        }
    }
    ///Calibration for single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF::SingleEnded
    }
    ///Calibration for differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF::Differential
    }
}
///Field `ADCALDIF` writer - Differential mode for calibration This bit is set and cleared by software to configure the Single-ended or Differential inputs mode for the calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG, ADCALDIF>;
impl<'a, REG> ADCALDIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration for single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::SingleEnded)
    }
    ///Calibration for differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::Differential)
    }
}
/**ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR {
    ///0: ADC calibration either not yet performed or completed
    NotCalibrating = 0,
    ///1: ADC calibration in progress
    Calibrating = 1,
}
impl From<ADCALR> for bool {
    #[inline(always)]
    fn from(variant: ADCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
pub type ADCAL_R = crate::BitReader<ADCALR>;
impl ADCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALR {
        match self.bits {
            false => ADCALR::NotCalibrating,
            true => ADCALR::Calibrating,
        }
    }
    ///ADC calibration either not yet performed or completed
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        *self == ADCALR::NotCalibrating
    }
    ///ADC calibration in progress
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        *self == ADCALR::Calibrating
    }
}
/**ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW {
    ///1: Start the ADC calibration sequence
    StartCalibration = 1,
}
impl From<ADCALW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
pub type ADCAL_W<'a, REG> = crate::BitWriter1S<'a, REG, ADCALW>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the ADC calibration sequence
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW::StartCalibration)
    }
}
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
    ///Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
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
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
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
    ///Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN, a conversion immediately starts (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (EXTSEL = 0x0): at the assertion of the End of Regular Conversion Sequence (EOS) flag. in all cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<'_, CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN, a conversion immediately starts (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the End of Injected Conversion Sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time that JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W<'_, CRrs> {
        JADSTART_W::new(self, 3)
    }
    ///Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<'_, CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W<'_, CRrs> {
        JADSTP_W::new(self, 5)
    }
    ///Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
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
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. Program first the bit ADCALDIF to determine if this calibration applies for Single-ended or Differential inputs mode. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0. The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing)
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<'_, CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#ADC1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_003f;
}
///`reset()` method sets CR to value 0x2000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
