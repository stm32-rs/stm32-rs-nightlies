///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**Data Management configuration This bit is set and cleared by software to select how ADC interface output data are managed. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DAMDF of the ADCx_CCR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMNGT {
    ///0: Store output data in DR only
    Dr = 0,
    ///1: DMA One Shot Mode selected
    DmaOneShot = 1,
    ///2: DFSDM mode selected
    Dfsdm = 2,
    ///3: DMA Circular Mode selected
    DmaCircular = 3,
}
impl From<DMNGT> for u8 {
    #[inline(always)]
    fn from(variant: DMNGT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMNGT {
    type Ux = u8;
}
impl crate::IsEnum for DMNGT {}
///Field `DMNGT` reader - Data Management configuration This bit is set and cleared by software to select how ADC interface output data are managed. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DAMDF of the ADCx_CCR register.
pub type DMNGT_R = crate::FieldReader<DMNGT>;
impl DMNGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMNGT {
        match self.bits {
            0 => DMNGT::Dr,
            1 => DMNGT::DmaOneShot,
            2 => DMNGT::Dfsdm,
            3 => DMNGT::DmaCircular,
            _ => unreachable!(),
        }
    }
    ///Store output data in DR only
    #[inline(always)]
    pub fn is_dr(&self) -> bool {
        *self == DMNGT::Dr
    }
    ///DMA One Shot Mode selected
    #[inline(always)]
    pub fn is_dma_one_shot(&self) -> bool {
        *self == DMNGT::DmaOneShot
    }
    ///DFSDM mode selected
    #[inline(always)]
    pub fn is_dfsdm(&self) -> bool {
        *self == DMNGT::Dfsdm
    }
    ///DMA Circular Mode selected
    #[inline(always)]
    pub fn is_dma_circular(&self) -> bool {
        *self == DMNGT::DmaCircular
    }
}
///Field `DMNGT` writer - Data Management configuration This bit is set and cleared by software to select how ADC interface output data are managed. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DAMDF of the ADCx_CCR register.
pub type DMNGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMNGT, crate::Safe>;
impl<'a, REG> DMNGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Store output data in DR only
    #[inline(always)]
    pub fn dr(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::Dr)
    }
    ///DMA One Shot Mode selected
    #[inline(always)]
    pub fn dma_one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::DmaOneShot)
    }
    ///DFSDM mode selected
    #[inline(always)]
    pub fn dfsdm(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::Dfsdm)
    }
    ///DMA Circular Mode selected
    #[inline(always)]
    pub fn dma_circular(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::DmaCircular)
    }
}
/**Data resolution These bits are written by software to select the resolution of the conversion. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 16-bit resolution
    SixteenBit = 0,
    ///1: 14-bit resolution in legacy mode (not optimized power consumption)
    FourteenBit = 1,
    ///2: 12-bit resolution in legacy mode (not optimized power consumption)
    TwelveBit = 2,
    ///3: 10-bit resolution
    TenBit = 3,
    ///5: 14-bit resolution
    FourteenBitV = 5,
    ///6: 12-bit resolution
    TwelveBitV = 6,
    ///7: 8-bit resolution
    EightBit = 7,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
impl crate::IsEnum for RES {}
///Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RES> {
        match self.bits {
            0 => Some(RES::SixteenBit),
            1 => Some(RES::FourteenBit),
            2 => Some(RES::TwelveBit),
            3 => Some(RES::TenBit),
            5 => Some(RES::FourteenBitV),
            6 => Some(RES::TwelveBitV),
            7 => Some(RES::EightBit),
            _ => None,
        }
    }
    ///16-bit resolution
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == RES::SixteenBit
    }
    ///14-bit resolution in legacy mode (not optimized power consumption)
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == RES::FourteenBit
    }
    ///12-bit resolution in legacy mode (not optimized power consumption)
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES::TwelveBit
    }
    ///10-bit resolution
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES::TenBit
    }
    ///14-bit resolution
    #[inline(always)]
    pub fn is_fourteen_bit_v(&self) -> bool {
        *self == RES::FourteenBitV
    }
    ///12-bit resolution
    #[inline(always)]
    pub fn is_twelve_bit_v(&self) -> bool {
        *self == RES::TwelveBitV
    }
    ///8-bit resolution
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES::EightBit
    }
}
///Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RES>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16-bit resolution
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::SixteenBit)
    }
    ///14-bit resolution in legacy mode (not optimized power consumption)
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::FourteenBit)
    }
    ///12-bit resolution in legacy mode (not optimized power consumption)
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBit)
    }
    ///10-bit resolution
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TenBit)
    }
    ///14-bit resolution
    #[inline(always)]
    pub fn fourteen_bit_v(self) -> &'a mut crate::W<REG> {
        self.variant(RES::FourteenBitV)
    }
    ///12-bit resolution
    #[inline(always)]
    pub fn twelve_bit_v(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBitV)
    }
    ///8-bit resolution
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::EightBit)
    }
}
/**External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 1 CC1 event
    Tim1Cc1 = 0,
    ///1: Timer 1 CC2 event
    Tim1Cc2 = 1,
    ///2: Timer 1 CC3 event
    Tim1Cc3 = 2,
    ///3: Timer 2 CC2 event
    Tim2Cc2 = 3,
    ///4: Timer 3 TRGO event
    Tim3Trgo = 4,
    ///5: Timer 4 CC4 event
    Tim4Cc4 = 5,
    ///6: EXTI line 11
    Exti11 = 6,
    ///7: Timer 8 TRGO event
    Tim8Trgo = 7,
    ///8: Timer 8 TRGO2 event
    Tim8Trgo2 = 8,
    ///9: Timer 1 TRGO event
    Tim1Trgo = 9,
    ///10: Timer 1 TRGO2 event
    Tim1Trgo2 = 10,
    ///11: Timer 2 TRGO event
    Tim2Trgo = 11,
    ///12: Timer 4 TRGO event
    Tim4Trgo = 12,
    ///13: Timer 6 TRGO event
    Tim6Trgo = 13,
    ///14: Timer 15 TRGO event
    Tim15Trgo = 14,
    ///15: Timer 3 CC4 event
    Tim3Cc4 = 15,
    ///16: HRTIM1_ADCTRG1 event
    Hrtim1Adctrg1 = 16,
    ///17: HRTIM1_ADCTRG3 event
    Hrtim1Adctrg3 = 17,
    ///18: LPTIM1_OUT event
    Lptim1Out = 18,
    ///19: LPTIM2_OUT event
    Lptim2Out = 19,
    ///20: LPTIM3_OUT event
    Lptim3Out = 20,
}
impl From<EXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL {}
///Field `EXTSEL` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTSEL> {
        match self.bits {
            0 => Some(EXTSEL::Tim1Cc1),
            1 => Some(EXTSEL::Tim1Cc2),
            2 => Some(EXTSEL::Tim1Cc3),
            3 => Some(EXTSEL::Tim2Cc2),
            4 => Some(EXTSEL::Tim3Trgo),
            5 => Some(EXTSEL::Tim4Cc4),
            6 => Some(EXTSEL::Exti11),
            7 => Some(EXTSEL::Tim8Trgo),
            8 => Some(EXTSEL::Tim8Trgo2),
            9 => Some(EXTSEL::Tim1Trgo),
            10 => Some(EXTSEL::Tim1Trgo2),
            11 => Some(EXTSEL::Tim2Trgo),
            12 => Some(EXTSEL::Tim4Trgo),
            13 => Some(EXTSEL::Tim6Trgo),
            14 => Some(EXTSEL::Tim15Trgo),
            15 => Some(EXTSEL::Tim3Cc4),
            16 => Some(EXTSEL::Hrtim1Adctrg1),
            17 => Some(EXTSEL::Hrtim1Adctrg3),
            18 => Some(EXTSEL::Lptim1Out),
            19 => Some(EXTSEL::Lptim2Out),
            20 => Some(EXTSEL::Lptim3Out),
            _ => None,
        }
    }
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        *self == EXTSEL::Tim1Cc1
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        *self == EXTSEL::Tim1Cc2
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        *self == EXTSEL::Tim1Cc3
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        *self == EXTSEL::Tim2Cc2
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL::Tim3Trgo
    }
    ///Timer 4 CC4 event
    #[inline(always)]
    pub fn is_tim4_cc4(&self) -> bool {
        *self == EXTSEL::Tim4Cc4
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == EXTSEL::Tim8Trgo
    }
    ///Timer 8 TRGO2 event
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == EXTSEL::Tim8Trgo2
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL::Tim1Trgo
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL::Tim1Trgo2
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL::Tim2Trgo
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == EXTSEL::Tim4Trgo
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL::Tim6Trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL::Tim15Trgo
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == EXTSEL::Tim3Cc4
    }
    ///HRTIM1_ADCTRG1 event
    #[inline(always)]
    pub fn is_hrtim1_adctrg1(&self) -> bool {
        *self == EXTSEL::Hrtim1Adctrg1
    }
    ///HRTIM1_ADCTRG3 event
    #[inline(always)]
    pub fn is_hrtim1_adctrg3(&self) -> bool {
        *self == EXTSEL::Hrtim1Adctrg3
    }
    ///LPTIM1_OUT event
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == EXTSEL::Lptim1Out
    }
    ///LPTIM2_OUT event
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == EXTSEL::Lptim2Out
    }
    ///LPTIM3_OUT event
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == EXTSEL::Lptim3Out
    }
}
///Field `EXTSEL` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc1)
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc2)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc3)
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Cc2)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Trgo)
    }
    ///Timer 4 CC4 event
    #[inline(always)]
    pub fn tim4_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4Cc4)
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8Trgo)
    }
    ///Timer 8 TRGO2 event
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8Trgo2)
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo)
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo2)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4Trgo)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim6Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim15Trgo)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Cc4)
    }
    ///HRTIM1_ADCTRG1 event
    #[inline(always)]
    pub fn hrtim1_adctrg1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Hrtim1Adctrg1)
    }
    ///HRTIM1_ADCTRG3 event
    #[inline(always)]
    pub fn hrtim1_adctrg3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Hrtim1Adctrg3)
    }
    ///LPTIM1_OUT event
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim1Out)
    }
    ///LPTIM2_OUT event
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim2Out)
    }
    ///LPTIM3_OUT event
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim3Out)
    }
}
/**External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<EXTEN> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN {
    type Ux = u8;
}
impl crate::IsEnum for EXTEN {}
///Field `EXTEN` reader - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN {
        match self.bits {
            0 => EXTEN::Disabled,
            1 => EXTEN::RisingEdge,
            2 => EXTEN::FallingEdge,
            3 => EXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
///Field `EXTEN` writer - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
/**Overrun Mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    ///0: Preserve DR register when an overrun is detected
    Preserve = 0,
    ///1: Overwrite DR register when an overrun is detected
    Overwrite = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - Overrun Mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type OVRMOD_R = crate::BitReader<OVRMOD>;
impl OVRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD {
        match self.bits {
            false => OVRMOD::Preserve,
            true => OVRMOD::Overwrite,
        }
    }
    ///Preserve DR register when an overrun is detected
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD::Preserve
    }
    ///Overwrite DR register when an overrun is detected
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD::Overwrite
    }
}
///Field `OVRMOD` writer - Overrun Mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preserve DR register when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Preserve)
    }
    ///Overwrite DR register when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Overwrite)
    }
}
/**Single / continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: Single conversion mode
    Single = 0,
    ///1: Continuous conversion mode
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Single / continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::Single,
            true => CONT::Continuous,
        }
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT::Single
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
///Field `CONT` writer - Single / continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Single)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
/**Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY {
    ///0: Auto delayed conversion mode off
    Off = 0,
    ///1: Auto delayed conversion mode on
    On = 1,
}
impl From<AUTDLY> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTDLY` reader - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
pub type AUTDLY_R = crate::BitReader<AUTDLY>;
impl AUTDLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTDLY {
        match self.bits {
            false => AUTDLY::Off,
            true => AUTDLY::On,
        }
    }
    ///Auto delayed conversion mode off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTDLY::Off
    }
    ///Auto delayed conversion mode on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == AUTDLY::On
    }
}
///Field `AUTDLY` writer - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG, AUTDLY>;
impl<'a, REG> AUTDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto delayed conversion mode off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::Off)
    }
    ///Auto delayed conversion mode on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::On)
    }
}
/**Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    ///0: Discontinuous mode on regular channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on regular channels enabled
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::Disabled,
            true => DISCEN::Enabled,
        }
    }
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
///Field `DISCEN` writer - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
///Field `DISCNUM` reader - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DAMDF of ADCx_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN {
    ///0: Discontinuous mode on injected channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on injected channels enabled
    Enabled = 1,
}
impl From<JDISCEN> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JDISCEN` reader - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DAMDF of ADCx_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
pub type JDISCEN_R = crate::BitReader<JDISCEN>;
impl JDISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JDISCEN {
        match self.bits {
            false => JDISCEN::Disabled,
            true => JDISCEN::Enabled,
        }
    }
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN::Disabled
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN::Enabled
    }
}
///Field `JDISCEN` writer - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DAMDF of ADCx_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG, JDISCEN>;
impl<'a, REG> JDISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Disabled)
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Enabled)
    }
}
/**JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQM {
    ///0: JSQR Mode 0: Queue maintains the last written configuration into JSQR
    Mode0 = 0,
    ///1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    Mode1 = 1,
}
impl From<JQM> for bool {
    #[inline(always)]
    fn from(variant: JQM) -> Self {
        variant as u8 != 0
    }
}
///Field `JQM` reader - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
pub type JQM_R = crate::BitReader<JQM>;
impl JQM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQM {
        match self.bits {
            false => JQM::Mode0,
            true => JQM::Mode1,
        }
    }
    ///JSQR Mode 0: Queue maintains the last written configuration into JSQR
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == JQM::Mode0
    }
    ///JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == JQM::Mode1
    }
}
///Field `JQM` writer - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG, JQM>;
impl<'a, REG> JQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///JSQR Mode 0: Queue maintains the last written configuration into JSQR
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(JQM::Mode0)
    }
    ///JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(JQM::Mode1)
    }
}
/**Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL {
    ///0: Analog watchdog 1 enabled on all channels
    All = 0,
    ///1: Analog watchdog 1 enabled on single channel selected in AWD1CH
    Single = 1,
}
impl From<AWD1SGL> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1SGL` reader - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD1SGL_R = crate::BitReader<AWD1SGL>;
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL {
        match self.bits {
            false => AWD1SGL::All,
            true => AWD1SGL::Single,
        }
    }
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWD1SGL::All
    }
    ///Analog watchdog 1 enabled on single channel selected in AWD1CH
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWD1SGL::Single
    }
}
///Field `AWD1SGL` writer - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::All)
    }
    ///Analog watchdog 1 enabled on single channel selected in AWD1CH
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::Single)
    }
}
/**Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN {
    ///0: Analog watchdog 1 disabled on regular channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on regular channels
    Enabled = 1,
}
impl From<AWD1EN> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1EN` reader - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type AWD1EN_R = crate::BitReader<AWD1EN>;
impl AWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN {
        match self.bits {
            false => AWD1EN::Disabled,
            true => AWD1EN::Enabled,
        }
    }
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN::Enabled
    }
}
///Field `AWD1EN` writer - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Enabled)
    }
}
/**Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN {
    ///0: Analog watchdog 1 disabled on injected channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on injected channels
    Enabled = 1,
}
impl From<JAWD1EN> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `JAWD1EN` reader - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing).
pub type JAWD1EN_R = crate::BitReader<JAWD1EN>;
impl JAWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAWD1EN {
        match self.bits {
            false => JAWD1EN::Disabled,
            true => JAWD1EN::Enabled,
        }
    }
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN::Enabled
    }
}
///Field `JAWD1EN` writer - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing).
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, JAWD1EN>;
impl<'a, REG> JAWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Enabled)
    }
}
/**Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO {
    ///0: Automatic injected group conversion disabled
    Disabled = 0,
    ///1: Automatic injected group conversion enabled
    Enabled = 1,
}
impl From<JAUTO> for bool {
    #[inline(always)]
    fn from(variant: JAUTO) -> Self {
        variant as u8 != 0
    }
}
///Field `JAUTO` reader - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
pub type JAUTO_R = crate::BitReader<JAUTO>;
impl JAUTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAUTO {
        match self.bits {
            false => JAUTO::Disabled,
            true => JAUTO::Enabled,
        }
    }
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO::Disabled
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO::Enabled
    }
}
///Field `JAUTO` writer - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG, JAUTO>;
impl<'a, REG> JAUTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Disabled)
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Enabled)
    }
}
///Field `AWD1CH` reader - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: Reserved, must not be used Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: Reserved, must not be used Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism: Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQDIS {
    ///0: Injected Queue enabled
    Enabled = 0,
    ///1: Injected Queue disabled
    Disabled = 1,
}
impl From<JQDIS> for bool {
    #[inline(always)]
    fn from(variant: JQDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `JQDIS` reader - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism: Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
pub type JQDIS_R = crate::BitReader<JQDIS>;
impl JQDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQDIS {
        match self.bits {
            false => JQDIS::Enabled,
            true => JQDIS::Disabled,
        }
    }
    ///Injected Queue enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JQDIS::Enabled
    }
    ///Injected Queue disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JQDIS::Disabled
    }
}
///Field `JQDIS` writer - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism: Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG, JQDIS>;
impl<'a, REG> JQDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected Queue enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQDIS::Enabled)
    }
    ///Injected Queue disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQDIS::Disabled)
    }
}
impl R {
    ///Bits 0:1 - Data Management configuration This bit is set and cleared by software to select how ADC interface output data are managed. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DAMDF of the ADCx_CCR register.
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - Data resolution These bits are written by software to select the resolution of the conversion. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun Mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DAMDF of ADCx_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: Reserved, must not be used Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism: Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("dmngt", &self.dmngt())
            .field("res", &self.res())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("autdly", &self.autdly())
            .field("discen", &self.discen())
            .field("discnum", &self.discnum())
            .field("jdiscen", &self.jdiscen())
            .field("jqm", &self.jqm())
            .field("awd1sgl", &self.awd1sgl())
            .field("awd1en", &self.awd1en())
            .field("jawd1en", &self.jawd1en())
            .field("jauto", &self.jauto())
            .field("awd1ch", &self.awd1ch())
            .field("jqdis", &self.jqdis())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Data Management configuration This bit is set and cleared by software to select how ADC interface output data are managed. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). In dual-ADC modes, this bit is not relevant and replaced by control bit DAMDF of the ADCx_CCR register.
    #[inline(always)]
    pub fn dmngt(&mut self) -> DMNGT_W<'_, CFGRrs> {
        DMNGT_W::new(self, 0)
    }
    ///Bits 2:4 - Data resolution These bits are written by software to select the resolution of the conversion. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGRrs> {
        RES_W::new(self, 2)
    }
    ///Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGRrs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun Mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit CONT of the slave ADC is no more writable and its content is equal to the bit CONT of the master ADC.
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGRrs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit AUTDLY of the slave ADC is no more writable and its content is equal to the bit AUTDLY of the master ADC.
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<'_, CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 16 - Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN=1 and CONT=1. It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit DISCEN of the slave ADC is no more writable and its content is equal to the bit DISCEN of the master ADC.
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART=0 (which ensures that no regular conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bits DISCNUM\[2:0\] of the slave ADC are no more writable and their content is equal to the bits DISCNUM\[2:0\] of the master ADC.
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. When dual mode is enabled (bits DAMDF of ADCx_CCR register are not equal to zero), the bit JDISCEN of the slave ADC is no more writable and its content is equal to the bit JDISCEN of the master ADC.
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 21 - JSQR queue mode This bit is set and cleared by software. It defines how an empty Queue is managed. Refer to for more information. Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JQM of the slave ADC is no more writable and its content is equal to the bit JQM of the master ADC.
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<'_, CFGRrs> {
        JQM_W::new(self, 21)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH\[4:0\] bits or on all the channels Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART=0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<'_, CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). When dual mode is enabled (DAMDF bits in ADCx_CCR register are not equal to zero), the bit JAUTO of the slave ADC is no more writable and its content is equal to the bit JAUTO of the master ADC.
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... others: Reserved, must not be used Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    ///Bit 31 - Injected Queue disable These bits are set and cleared by software to disable the Injected Queue mechanism: Note: The software is allowed to write this bit only when ADSTART=0 and JADSTART=0 (which ensures that no regular nor injected conversion is ongoing). A set or reset of JQDIS bit causes the injected queue to be flushed and the JSQR register is cleared.
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W<'_, CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#ADC1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0x8000_0000
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
