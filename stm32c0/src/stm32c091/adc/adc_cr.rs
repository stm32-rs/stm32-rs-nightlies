///Register `ADC_CR` reader
pub type R = crate::R<ADC_CRrs>;
///Register `ADC_CR` writer
pub type W = crate::W<ADC_CRrs>;
/**ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEN {
    ///0: ADC is disabled (OFF state)
    B0x0 = 0,
    ///1: Write 1 to enable the ADC.
    B0x1 = 1,
}
impl From<ADEN> for bool {
    #[inline(always)]
    fn from(variant: ADEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` reader - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)
pub type ADEN_R = crate::BitReader<ADEN>;
impl ADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADEN {
        match self.bits {
            false => ADEN::B0x0,
            true => ADEN::B0x1,
        }
    }
    ///ADC is disabled (OFF state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADEN::B0x0
    }
    ///Write 1 to enable the ADC.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADEN::B0x1
    }
}
///Field `ADEN` writer - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG, ADEN>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC is disabled (OFF state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADEN::B0x0)
    }
    ///Write 1 to enable the ADC.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADEN::B0x1)
    }
}
/**ADC disable command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDIS {
    ///0: No ADDIS command ongoing
    B0x0 = 0,
    ///1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress.
    B0x1 = 1,
}
impl From<ADDIS> for bool {
    #[inline(always)]
    fn from(variant: ADDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` reader - ADC disable command
pub type ADDIS_R = crate::BitReader<ADDIS>;
impl ADDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDIS {
        match self.bits {
            false => ADDIS::B0x0,
            true => ADDIS::B0x1,
        }
    }
    ///No ADDIS command ongoing
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADDIS::B0x0
    }
    ///Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADDIS::B0x1
    }
}
///Field `ADDIS` writer - ADC disable command
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG, ADDIS>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ADDIS command ongoing
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDIS::B0x0)
    }
    ///Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDIS::B0x1)
    }
}
/**ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \[1:0\] configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTART {
    ///0: No ADC conversion is ongoing.
    B0x0 = 0,
    ///1: Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting.
    B0x1 = 1,
}
impl From<ADSTART> for bool {
    #[inline(always)]
    fn from(variant: ADSTART) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` reader - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \[1:0\] configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored.
pub type ADSTART_R = crate::BitReader<ADSTART>;
impl ADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTART {
        match self.bits {
            false => ADSTART::B0x0,
            true => ADSTART::B0x1,
        }
    }
    ///No ADC conversion is ongoing.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADSTART::B0x0
    }
    ///Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADSTART::B0x1
    }
}
///Field `ADSTART` writer - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \[1:0\] configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored.
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG, ADSTART>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ADC conversion is ongoing.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTART::B0x0)
    }
    ///Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTART::B0x1)
    }
}
/**ADC stop conversion command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTP {
    ///0: No ADC stop conversion command ongoing
    B0x0 = 0,
    ///1: Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress.
    B0x1 = 1,
}
impl From<ADSTP> for bool {
    #[inline(always)]
    fn from(variant: ADSTP) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` reader - ADC stop conversion command
pub type ADSTP_R = crate::BitReader<ADSTP>;
impl ADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTP {
        match self.bits {
            false => ADSTP::B0x0,
            true => ADSTP::B0x1,
        }
    }
    ///No ADC stop conversion command ongoing
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADSTP::B0x0
    }
    ///Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADSTP::B0x1
    }
}
///Field `ADSTP` writer - ADC stop conversion command
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG, ADSTP>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ADC stop conversion command ongoing
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTP::B0x0)
    }
    ///Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTP::B0x1)
    }
}
/**ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after t<sub>ADCVREG_STUP</sub>. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN {
    ///0: ADC voltage regulator disabled
    B0x0 = 0,
    ///1: ADC voltage regulator enabled
    B0x1 = 1,
}
impl From<ADVREGEN> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADVREGEN` reader - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after t<sub>ADCVREG_STUP</sub>. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_R = crate::BitReader<ADVREGEN>;
impl ADVREGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADVREGEN {
        match self.bits {
            false => ADVREGEN::B0x0,
            true => ADVREGEN::B0x1,
        }
    }
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADVREGEN::B0x0
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADVREGEN::B0x1
    }
}
///Field `ADVREGEN` writer - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after t<sub>ADCVREG_STUP</sub>. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG, ADVREGEN>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::B0x0)
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::B0x1)
    }
}
/**ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 (ADC enabled and no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCAL {
    ///0: Calibration complete
    B0x0 = 0,
    ///1: Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress.
    B0x1 = 1,
}
impl From<ADCAL> for bool {
    #[inline(always)]
    fn from(variant: ADCAL) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 (ADC enabled and no conversion is ongoing).
pub type ADCAL_R = crate::BitReader<ADCAL>;
impl ADCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCAL {
        match self.bits {
            false => ADCAL::B0x0,
            true => ADCAL::B0x1,
        }
    }
    ///Calibration complete
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADCAL::B0x0
    }
    ///Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADCAL::B0x1
    }
}
///Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 (ADC enabled and no conversion is ongoing).
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCAL>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration complete
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL::B0x0)
    }
    ///Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL::B0x1)
    }
}
impl R {
    ///Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \[1:0\] configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored.
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - ADC stop conversion command
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 28 - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after t<sub>ADCVREG_STUP</sub>. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 (ADC enabled and no conversion is ongoing).
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CR")
            .field("aden", &self.aden())
            .field("addis", &self.addis())
            .field("adstart", &self.adstart())
            .field("adstp", &self.adstp())
            .field("advregen", &self.advregen())
            .field("adcal", &self.adcal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<'_, ADC_CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<'_, ADC_CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC start conversion command This bit is set by software to start ADC conversion. Depending on the EXTEN \[1:0\] configuration bits, a conversion either starts immediately (software trigger configuration) or once a hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag. In discontinuous conversion mode(CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN = 00): at the assertion of the end of Conversion (EOC) flag. In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). Note: After writing to ADC_CHSELR register or changing CHSELRMOD or SCANDIRW, it is mandatory to wait until CCRDY flag is asserted before setting ADSTART, otherwise, the value written to ADSTART is ignored.
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<'_, ADC_CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 4 - ADC stop conversion command
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<'_, ADC_CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 28 - ADC Voltage Regulator Enable This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output is available after t<sub>ADCVREG_STUP</sub>. It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0. Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W<'_, ADC_CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC. It is cleared by hardware after calibration is complete. Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0). Note: The software is allowed to update the calibration factor by writing ADC_CALFACT only when ADEN = 1 and ADSTART = 0 (ADC enabled and no conversion is ongoing).
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<'_, ADC_CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`adc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#ADC:ADC_CR)*/
pub struct ADC_CRrs;
impl crate::RegisterSpec for ADC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_cr::R`](R) reader structure
impl crate::Readable for ADC_CRrs {}
///`write(|w| ..)` method takes [`adc_cr::W`](W) writer structure
impl crate::Writable for ADC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CR to value 0
impl crate::Resettable for ADC_CRrs {}
