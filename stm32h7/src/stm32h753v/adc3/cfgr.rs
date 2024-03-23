#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "ADC DMA transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMNGT {
    #[doc = "0: Store output data in DR only"]
    Dr = 0,
    #[doc = "1: DMA One Shot Mode selected"]
    DmaOneShot = 1,
    #[doc = "2: DFSDM mode selected"]
    Dfsdm = 2,
    #[doc = "3: DMA Circular Mode selected"]
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
#[doc = "Field `DMNGT` reader - ADC DMA transfer enable"]
pub type DMNGT_R = crate::FieldReader<DMNGT>;
impl DMNGT_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Store output data in DR only"]
    #[inline(always)]
    pub fn is_dr(&self) -> bool {
        *self == DMNGT::Dr
    }
    #[doc = "DMA One Shot Mode selected"]
    #[inline(always)]
    pub fn is_dma_one_shot(&self) -> bool {
        *self == DMNGT::DmaOneShot
    }
    #[doc = "DFSDM mode selected"]
    #[inline(always)]
    pub fn is_dfsdm(&self) -> bool {
        *self == DMNGT::Dfsdm
    }
    #[doc = "DMA Circular Mode selected"]
    #[inline(always)]
    pub fn is_dma_circular(&self) -> bool {
        *self == DMNGT::DmaCircular
    }
}
#[doc = "Field `DMNGT` writer - ADC DMA transfer enable"]
pub type DMNGT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMNGT>;
impl<'a, REG> DMNGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Store output data in DR only"]
    #[inline(always)]
    pub fn dr(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::Dr)
    }
    #[doc = "DMA One Shot Mode selected"]
    #[inline(always)]
    pub fn dma_one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::DmaOneShot)
    }
    #[doc = "DFSDM mode selected"]
    #[inline(always)]
    pub fn dfsdm(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::Dfsdm)
    }
    #[doc = "DMA Circular Mode selected"]
    #[inline(always)]
    pub fn dma_circular(self) -> &'a mut crate::W<REG> {
        self.variant(DMNGT::DmaCircular)
    }
}
#[doc = "ADC data resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    #[doc = "0: 16-bit resolution"]
    SixteenBit = 0,
    #[doc = "1: 14-bit resolution in legacy mode (not optimized power consumption)"]
    FourteenBit = 1,
    #[doc = "2: 12-bit resolution in legacy mode (not optimized power consumption)"]
    TwelveBit = 2,
    #[doc = "3: 10-bit resolution"]
    TenBit = 3,
    #[doc = "5: 14-bit resolution"]
    FourteenBitV = 5,
    #[doc = "6: 12-bit resolution"]
    TwelveBitV = 6,
    #[doc = "7: 8-bit resolution"]
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
#[doc = "Field `RES` reader - ADC data resolution"]
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "16-bit resolution"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == RES::SixteenBit
    }
    #[doc = "14-bit resolution in legacy mode (not optimized power consumption)"]
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == RES::FourteenBit
    }
    #[doc = "12-bit resolution in legacy mode (not optimized power consumption)"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES::TwelveBit
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES::TenBit
    }
    #[doc = "14-bit resolution"]
    #[inline(always)]
    pub fn is_fourteen_bit_v(&self) -> bool {
        *self == RES::FourteenBitV
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_twelve_bit_v(&self) -> bool {
        *self == RES::TwelveBitV
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES::EightBit
    }
}
#[doc = "Field `RES` writer - ADC data resolution"]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RES>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit resolution"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::SixteenBit)
    }
    #[doc = "14-bit resolution in legacy mode (not optimized power consumption)"]
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::FourteenBit)
    }
    #[doc = "12-bit resolution in legacy mode (not optimized power consumption)"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBit)
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TenBit)
    }
    #[doc = "14-bit resolution"]
    #[inline(always)]
    pub fn fourteen_bit_v(self) -> &'a mut crate::W<REG> {
        self.variant(RES::FourteenBitV)
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn twelve_bit_v(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBitV)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::EightBit)
    }
}
#[doc = "ADC group regular external trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    #[doc = "0: Timer 1 CC1 event"]
    Tim1Cc1 = 0,
    #[doc = "1: Timer 1 CC2 event"]
    Tim1Cc2 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    Tim1Cc3 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    Tim2Cc2 = 3,
    #[doc = "4: Timer 3 TRGO event"]
    Tim3Trgo = 4,
    #[doc = "5: Timer 4 CC4 event"]
    Tim4Cc4 = 5,
    #[doc = "6: EXTI line 11"]
    Exti11 = 6,
    #[doc = "7: Timer 8 TRGO event"]
    Tim8Trgo = 7,
    #[doc = "8: Timer 8 TRGO2 event"]
    Tim8Trgo2 = 8,
    #[doc = "9: Timer 1 TRGO event"]
    Tim1Trgo = 9,
    #[doc = "10: Timer 1 TRGO2 event"]
    Tim1Trgo2 = 10,
    #[doc = "11: Timer 2 TRGO event"]
    Tim2Trgo = 11,
    #[doc = "12: Timer 4 TRGO event"]
    Tim4Trgo = 12,
    #[doc = "13: Timer 6 TRGO event"]
    Tim6Trgo = 13,
    #[doc = "14: Timer 15 TRGO event"]
    Tim15Trgo = 14,
    #[doc = "15: Timer 3 CC4 event"]
    Tim3Cc4 = 15,
    #[doc = "16: HRTIM1_ADCTRG1 event"]
    Hrtim1Adctrg1 = 16,
    #[doc = "17: HRTIM1_ADCTRG3 event"]
    Hrtim1Adctrg3 = 17,
    #[doc = "18: LPTIM1_OUT event"]
    Lptim1Out = 18,
    #[doc = "19: LPTIM2_OUT event"]
    Lptim2Out = 19,
    #[doc = "20: LPTIM3_OUT event"]
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
#[doc = "Field `EXTSEL` reader - ADC group regular external trigger source"]
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        *self == EXTSEL::Tim1Cc1
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        *self == EXTSEL::Tim1Cc2
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        *self == EXTSEL::Tim1Cc3
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        *self == EXTSEL::Tim2Cc2
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL::Tim3Trgo
    }
    #[doc = "Timer 4 CC4 event"]
    #[inline(always)]
    pub fn is_tim4_cc4(&self) -> bool {
        *self == EXTSEL::Tim4Cc4
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == EXTSEL::Tim8Trgo
    }
    #[doc = "Timer 8 TRGO2 event"]
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == EXTSEL::Tim8Trgo2
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL::Tim1Trgo
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL::Tim1Trgo2
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL::Tim2Trgo
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == EXTSEL::Tim4Trgo
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL::Tim6Trgo
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL::Tim15Trgo
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == EXTSEL::Tim3Cc4
    }
    #[doc = "HRTIM1_ADCTRG1 event"]
    #[inline(always)]
    pub fn is_hrtim1_adctrg1(&self) -> bool {
        *self == EXTSEL::Hrtim1Adctrg1
    }
    #[doc = "HRTIM1_ADCTRG3 event"]
    #[inline(always)]
    pub fn is_hrtim1_adctrg3(&self) -> bool {
        *self == EXTSEL::Hrtim1Adctrg3
    }
    #[doc = "LPTIM1_OUT event"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == EXTSEL::Lptim1Out
    }
    #[doc = "LPTIM2_OUT event"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == EXTSEL::Lptim2Out
    }
    #[doc = "LPTIM3_OUT event"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == EXTSEL::Lptim3Out
    }
}
#[doc = "Field `EXTSEL` writer - ADC group regular external trigger source"]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Cc2)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Trgo)
    }
    #[doc = "Timer 4 CC4 event"]
    #[inline(always)]
    pub fn tim4_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4Cc4)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8Trgo)
    }
    #[doc = "Timer 8 TRGO2 event"]
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8Trgo2)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo)
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo2)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Trgo)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim4Trgo)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim6Trgo)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim15Trgo)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Cc4)
    }
    #[doc = "HRTIM1_ADCTRG1 event"]
    #[inline(always)]
    pub fn hrtim1_adctrg1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Hrtim1Adctrg1)
    }
    #[doc = "HRTIM1_ADCTRG3 event"]
    #[inline(always)]
    pub fn hrtim1_adctrg3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Hrtim1Adctrg3)
    }
    #[doc = "LPTIM1_OUT event"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim1Out)
    }
    #[doc = "LPTIM2_OUT event"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim2Out)
    }
    #[doc = "LPTIM3_OUT event"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Lptim3Out)
    }
}
#[doc = "ADC group regular external trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    #[doc = "0: Trigger detection disabled"]
    Disabled = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RisingEdge = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FallingEdge = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
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
#[doc = "Field `EXTEN` reader - ADC group regular external trigger polarity"]
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
#[doc = "Field `EXTEN` writer - ADC group regular external trigger polarity"]
pub type EXTEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTEN>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
#[doc = "ADC group regular overrun configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    #[doc = "0: Preserve DR register when an overrun is detected"]
    Preserve = 0,
    #[doc = "1: Overwrite DR register when an overrun is detected"]
    Overwrite = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRMOD` reader - ADC group regular overrun configuration"]
pub type OVRMOD_R = crate::BitReader<OVRMOD>;
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD {
        match self.bits {
            false => OVRMOD::Preserve,
            true => OVRMOD::Overwrite,
        }
    }
    #[doc = "Preserve DR register when an overrun is detected"]
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD::Preserve
    }
    #[doc = "Overwrite DR register when an overrun is detected"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD::Overwrite
    }
}
#[doc = "Field `OVRMOD` writer - ADC group regular overrun configuration"]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preserve DR register when an overrun is detected"]
    #[inline(always)]
    pub fn preserve(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Preserve)
    }
    #[doc = "Overwrite DR register when an overrun is detected"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Overwrite)
    }
}
#[doc = "ADC group regular continuous conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    #[doc = "0: Single conversion mode"]
    Single = 0,
    #[doc = "1: Continuous conversion mode"]
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - ADC group regular continuous conversion mode"]
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::Single,
            true => CONT::Continuous,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT::Single
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
#[doc = "Field `CONT` writer - ADC group regular continuous conversion mode"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Single)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
#[doc = "ADC low power auto wait\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY {
    #[doc = "0: Auto delayed conversion mode off"]
    Off = 0,
    #[doc = "1: Auto delayed conversion mode on"]
    On = 1,
}
impl From<AUTDLY> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTDLY` reader - ADC low power auto wait"]
pub type AUTDLY_R = crate::BitReader<AUTDLY>;
impl AUTDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTDLY {
        match self.bits {
            false => AUTDLY::Off,
            true => AUTDLY::On,
        }
    }
    #[doc = "Auto delayed conversion mode off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTDLY::Off
    }
    #[doc = "Auto delayed conversion mode on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == AUTDLY::On
    }
}
#[doc = "Field `AUTDLY` writer - ADC low power auto wait"]
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG, AUTDLY>;
impl<'a, REG> AUTDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto delayed conversion mode off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::Off)
    }
    #[doc = "Auto delayed conversion mode on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::On)
    }
}
#[doc = "ADC group regular sequencer discontinuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - ADC group regular sequencer discontinuous mode"]
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::Disabled,
            true => DISCEN::Enabled,
        }
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
#[doc = "Field `DISCEN` writer - ADC group regular sequencer discontinuous mode"]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
#[doc = "Field `DISCNUM` reader - ADC group regular sequencer discontinuous number of ranks"]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - ADC group regular sequencer discontinuous number of ranks"]
pub type DISCNUM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "ADC group injected sequencer discontinuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    Enabled = 1,
}
impl From<JDISCEN> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDISCEN` reader - ADC group injected sequencer discontinuous mode"]
pub type JDISCEN_R = crate::BitReader<JDISCEN>;
impl JDISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JDISCEN {
        match self.bits {
            false => JDISCEN::Disabled,
            true => JDISCEN::Enabled,
        }
    }
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN::Disabled
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN::Enabled
    }
}
#[doc = "Field `JDISCEN` writer - ADC group injected sequencer discontinuous mode"]
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG, JDISCEN>;
impl<'a, REG> JDISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Disabled)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Enabled)
    }
}
#[doc = "ADC group injected contexts queue mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQM {
    #[doc = "0: JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    Mode0 = 0,
    #[doc = "1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    Mode1 = 1,
}
impl From<JQM> for bool {
    #[inline(always)]
    fn from(variant: JQM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQM` reader - ADC group injected contexts queue mode"]
pub type JQM_R = crate::BitReader<JQM>;
impl JQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQM {
        match self.bits {
            false => JQM::Mode0,
            true => JQM::Mode1,
        }
    }
    #[doc = "JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == JQM::Mode0
    }
    #[doc = "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == JQM::Mode1
    }
}
#[doc = "Field `JQM` writer - ADC group injected contexts queue mode"]
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG, JQM>;
impl<'a, REG> JQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(JQM::Mode0)
    }
    #[doc = "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(JQM::Mode1)
    }
}
#[doc = "ADC analog watchdog 1 monitoring a single channel or all channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    All = 0,
    #[doc = "1: Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    Single = 1,
}
impl From<AWD1SGL> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1SGL` reader - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub type AWD1SGL_R = crate::BitReader<AWD1SGL>;
impl AWD1SGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL {
        match self.bits {
            false => AWD1SGL::All,
            true => AWD1SGL::Single,
        }
    }
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWD1SGL::All
    }
    #[doc = "Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWD1SGL::Single
    }
}
#[doc = "Field `AWD1SGL` writer - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::All)
    }
    #[doc = "Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::Single)
    }
}
#[doc = "ADC analog watchdog 1 enable on scope ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN {
    #[doc = "0: Analog watchdog 1 disabled on regular channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog 1 enabled on regular channels"]
    Enabled = 1,
}
impl From<AWD1EN> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group regular"]
pub type AWD1EN_R = crate::BitReader<AWD1EN>;
impl AWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN {
        match self.bits {
            false => AWD1EN::Disabled,
            true => AWD1EN::Enabled,
        }
    }
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN::Disabled
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN::Enabled
    }
}
#[doc = "Field `AWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group regular"]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Disabled)
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Enabled)
    }
}
#[doc = "ADC analog watchdog 1 enable on scope ADC group injected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN {
    #[doc = "0: Analog watchdog 1 disabled on injected channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog 1 enabled on injected channels"]
    Enabled = 1,
}
impl From<JAWD1EN> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group injected"]
pub type JAWD1EN_R = crate::BitReader<JAWD1EN>;
impl JAWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JAWD1EN {
        match self.bits {
            false => JAWD1EN::Disabled,
            true => JAWD1EN::Enabled,
        }
    }
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN::Disabled
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN::Enabled
    }
}
#[doc = "Field `JAWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group injected"]
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, JAWD1EN>;
impl<'a, REG> JAWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Disabled)
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Enabled)
    }
}
#[doc = "ADC group injected automatic trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO {
    #[doc = "0: Automatic injected group conversion disabled"]
    Disabled = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    Enabled = 1,
}
impl From<JAUTO> for bool {
    #[inline(always)]
    fn from(variant: JAUTO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAUTO` reader - ADC group injected automatic trigger mode"]
pub type JAUTO_R = crate::BitReader<JAUTO>;
impl JAUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JAUTO {
        match self.bits {
            false => JAUTO::Disabled,
            true => JAUTO::Enabled,
        }
    }
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO::Disabled
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO::Enabled
    }
}
#[doc = "Field `JAUTO` writer - ADC group injected automatic trigger mode"]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG, JAUTO>;
impl<'a, REG> JAUTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Disabled)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Enabled)
    }
}
#[doc = "Field `AWD1CH` reader - ADC analog watchdog 1 monitored channel selection"]
pub type AWD1CH_R = crate::FieldReader;
#[doc = "Field `AWD1CH` writer - ADC analog watchdog 1 monitored channel selection"]
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "ADC group injected contexts queue disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQDIS {
    #[doc = "0: Injected Queue enabled"]
    Enabled = 0,
    #[doc = "1: Injected Queue disabled"]
    Disabled = 1,
}
impl From<JQDIS> for bool {
    #[inline(always)]
    fn from(variant: JQDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQDIS` reader - ADC group injected contexts queue disable"]
pub type JQDIS_R = crate::BitReader<JQDIS>;
impl JQDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQDIS {
        match self.bits {
            false => JQDIS::Enabled,
            true => JQDIS::Disabled,
        }
    }
    #[doc = "Injected Queue enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JQDIS::Enabled
    }
    #[doc = "Injected Queue disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JQDIS::Disabled
    }
}
#[doc = "Field `JQDIS` writer - ADC group injected contexts queue disable"]
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG, JQDIS>;
impl<'a, REG> JQDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected Queue enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQDIS::Enabled)
    }
    #[doc = "Injected Queue disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JQDIS::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - ADC data resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:9 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC DMA transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmngt(&mut self) -> DMNGT_W<CFGRrs> {
        DMNGT_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - ADC data resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CFGRrs> {
        RES_W::new(self, 2)
    }
    #[doc = "Bits 5:9 - ADC group regular external trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<CFGRrs> {
        EXTSEL_W::new(self, 5)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CFGRrs> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    #[must_use]
    pub fn jqm(&mut self) -> JQM_W<CFGRrs> {
        JQM_W::new(self, 21)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    #[must_use]
    pub fn jqdis(&mut self) -> JQDIS_W<CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
