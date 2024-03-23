#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1 {
    #[doc = "0: DAC Channel X disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X enabled"]
    Enabled = 1,
}
impl From<EN1> for bool {
    #[inline(always)]
    fn from(variant: EN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub type EN1_R = crate::BitReader<EN1>;
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN1 {
        match self.bits {
            false => EN1::Disabled,
            true => EN1::Enabled,
        }
    }
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1::Disabled
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1::Enabled
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Disabled)
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Enabled)
    }
}
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1 {
    #[doc = "0: DAC Channel X trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X trigger enabled"]
    Enabled = 1,
}
impl From<TEN1> for bool {
    #[inline(always)]
    fn from(variant: TEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader<TEN1>;
impl TEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEN1 {
        match self.bits {
            false => TEN1::Disabled,
            true => TEN1::Enabled,
        }
    }
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1::Disabled
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1::Enabled
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, REG> = crate::BitWriter<'a, REG, TEN1>;
impl<'a, REG> TEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Disabled)
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Enabled)
    }
}
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1 {
    #[doc = "0: SWTRIG1"]
    Swtrig = 0,
    #[doc = "1: dac_chx_trg1"]
    Tim1Trgo = 1,
    #[doc = "2: dac_chx_trg2"]
    Tim2Trgo = 2,
    #[doc = "3: dac_chx_trg3"]
    Trg3 = 3,
    #[doc = "4: dac_chx_trg4"]
    Trg4 = 4,
    #[doc = "5: dac_chx_trg5"]
    Trg5 = 5,
    #[doc = "6: dac_chx_trg6"]
    Trg6 = 6,
    #[doc = "7: dac_chx_trg7"]
    Trg7 = 7,
    #[doc = "8: dac_chx_trg8"]
    Trg8 = 8,
    #[doc = "9: dac_chx_trg9"]
    Trg9 = 9,
    #[doc = "10: dac_chx_trg10"]
    Trg10 = 10,
    #[doc = "11: dac_chx_trg11"]
    Lptim1Out = 11,
    #[doc = "12: dac_chx_trg12"]
    Lptim2Out = 12,
    #[doc = "13: dac_chx_trg13"]
    Lptim3Out = 13,
    #[doc = "14: dac_chx_trg14"]
    Exti9 = 14,
    #[doc = "15: dac_chx_trg15"]
    Trg15 = 15,
}
impl From<TSEL1> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL1 {
    type Ux = u8;
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader<TSEL1>;
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEL1 {
        match self.bits {
            0 => TSEL1::Swtrig,
            1 => TSEL1::Tim1Trgo,
            2 => TSEL1::Tim2Trgo,
            3 => TSEL1::Trg3,
            4 => TSEL1::Trg4,
            5 => TSEL1::Trg5,
            6 => TSEL1::Trg6,
            7 => TSEL1::Trg7,
            8 => TSEL1::Trg8,
            9 => TSEL1::Trg9,
            10 => TSEL1::Trg10,
            11 => TSEL1::Lptim1Out,
            12 => TSEL1::Lptim2Out,
            13 => TSEL1::Lptim3Out,
            14 => TSEL1::Exti9,
            15 => TSEL1::Trg15,
            _ => unreachable!(),
        }
    }
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == TSEL1::Swtrig
    }
    #[doc = "dac_chx_trg1"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == TSEL1::Tim1Trgo
    }
    #[doc = "dac_chx_trg2"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1::Tim2Trgo
    }
    #[doc = "dac_chx_trg3"]
    #[inline(always)]
    pub fn is_trg3(&self) -> bool {
        *self == TSEL1::Trg3
    }
    #[doc = "dac_chx_trg4"]
    #[inline(always)]
    pub fn is_trg4(&self) -> bool {
        *self == TSEL1::Trg4
    }
    #[doc = "dac_chx_trg5"]
    #[inline(always)]
    pub fn is_trg5(&self) -> bool {
        *self == TSEL1::Trg5
    }
    #[doc = "dac_chx_trg6"]
    #[inline(always)]
    pub fn is_trg6(&self) -> bool {
        *self == TSEL1::Trg6
    }
    #[doc = "dac_chx_trg7"]
    #[inline(always)]
    pub fn is_trg7(&self) -> bool {
        *self == TSEL1::Trg7
    }
    #[doc = "dac_chx_trg8"]
    #[inline(always)]
    pub fn is_trg8(&self) -> bool {
        *self == TSEL1::Trg8
    }
    #[doc = "dac_chx_trg9"]
    #[inline(always)]
    pub fn is_trg9(&self) -> bool {
        *self == TSEL1::Trg9
    }
    #[doc = "dac_chx_trg10"]
    #[inline(always)]
    pub fn is_trg10(&self) -> bool {
        *self == TSEL1::Trg10
    }
    #[doc = "dac_chx_trg11"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == TSEL1::Lptim1Out
    }
    #[doc = "dac_chx_trg12"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == TSEL1::Lptim2Out
    }
    #[doc = "dac_chx_trg13"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == TSEL1::Lptim3Out
    }
    #[doc = "dac_chx_trg14"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1::Exti9
    }
    #[doc = "dac_chx_trg15"]
    #[inline(always)]
    pub fn is_trg15(&self) -> bool {
        *self == TSEL1::Trg15
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, TSEL1>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Swtrig)
    }
    #[doc = "dac_chx_trg1"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim1Trgo)
    }
    #[doc = "dac_chx_trg2"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim2Trgo)
    }
    #[doc = "dac_chx_trg3"]
    #[inline(always)]
    pub fn trg3(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg3)
    }
    #[doc = "dac_chx_trg4"]
    #[inline(always)]
    pub fn trg4(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg4)
    }
    #[doc = "dac_chx_trg5"]
    #[inline(always)]
    pub fn trg5(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg5)
    }
    #[doc = "dac_chx_trg6"]
    #[inline(always)]
    pub fn trg6(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg6)
    }
    #[doc = "dac_chx_trg7"]
    #[inline(always)]
    pub fn trg7(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg7)
    }
    #[doc = "dac_chx_trg8"]
    #[inline(always)]
    pub fn trg8(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg8)
    }
    #[doc = "dac_chx_trg9"]
    #[inline(always)]
    pub fn trg9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg9)
    }
    #[doc = "dac_chx_trg10"]
    #[inline(always)]
    pub fn trg10(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg10)
    }
    #[doc = "dac_chx_trg11"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Lptim1Out)
    }
    #[doc = "dac_chx_trg12"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Lptim2Out)
    }
    #[doc = "dac_chx_trg13"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Lptim3Out)
    }
    #[doc = "dac_chx_trg14"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Exti9)
    }
    #[doc = "dac_chx_trg15"]
    #[inline(always)]
    pub fn trg15(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Trg15)
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1 {
    #[doc = "0: Wave generation disabled"]
    Disabled = 0,
    #[doc = "1: Noise wave generation enabled"]
    Noise = 1,
    #[doc = "2: Triangle wave generation enabled"]
    Triangle = 2,
}
impl From<WAVE1> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAVE1 {
    type Ux = u8;
}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::FieldReader<WAVE1>;
impl WAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAVE1> {
        match self.bits {
            0 => Some(WAVE1::Disabled),
            1 => Some(WAVE1::Noise),
            2 => Some(WAVE1::Triangle),
            _ => None,
        }
    }
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1::Disabled
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1::Noise
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1::Triangle
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAVE1>;
impl<'a, REG> WAVE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Disabled)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Noise)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Triangle)
    }
}
#[doc = "DAC channel1 mask/amplitude selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAMP1 {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    Amp1 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    Amp3 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    Amp7 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    Amp15 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    Amp31 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    Amp63 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    Amp127 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    Amp255 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    Amp511 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    Amp1023 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    Amp2047 = 10,
    #[doc = "11: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    Amp4095 = 11,
}
impl From<MAMP1> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAMP1 {
    type Ux = u8;
}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP1_R = crate::FieldReader<MAMP1>;
impl MAMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAMP1> {
        match self.bits {
            0 => Some(MAMP1::Amp1),
            1 => Some(MAMP1::Amp3),
            2 => Some(MAMP1::Amp7),
            3 => Some(MAMP1::Amp15),
            4 => Some(MAMP1::Amp31),
            5 => Some(MAMP1::Amp63),
            6 => Some(MAMP1::Amp127),
            7 => Some(MAMP1::Amp255),
            8 => Some(MAMP1::Amp511),
            9 => Some(MAMP1::Amp1023),
            10 => Some(MAMP1::Amp2047),
            11 => Some(MAMP1::Amp4095),
            _ => None,
        }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn is_amp1(&self) -> bool {
        *self == MAMP1::Amp1
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn is_amp3(&self) -> bool {
        *self == MAMP1::Amp3
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn is_amp7(&self) -> bool {
        *self == MAMP1::Amp7
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn is_amp15(&self) -> bool {
        *self == MAMP1::Amp15
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn is_amp31(&self) -> bool {
        *self == MAMP1::Amp31
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn is_amp63(&self) -> bool {
        *self == MAMP1::Amp63
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn is_amp127(&self) -> bool {
        *self == MAMP1::Amp127
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn is_amp255(&self) -> bool {
        *self == MAMP1::Amp255
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn is_amp511(&self) -> bool {
        *self == MAMP1::Amp511
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn is_amp1023(&self) -> bool {
        *self == MAMP1::Amp1023
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn is_amp2047(&self) -> bool {
        *self == MAMP1::Amp2047
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn is_amp4095(&self) -> bool {
        *self == MAMP1::Amp4095
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAMP1>;
impl<'a, REG> MAMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn amp1(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp1)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn amp3(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp3)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn amp7(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp7)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn amp15(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp15)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn amp31(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp31)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn amp63(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp63)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn amp127(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp127)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn amp255(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp255)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn amp511(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp511)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn amp1023(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp1023)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn amp2047(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp2047)
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn amp4095(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1::Amp4095)
    }
}
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1 {
    #[doc = "0: DAC Channel X DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA mode enabled"]
    Enabled = 1,
}
impl From<DMAEN1> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader<DMAEN1>;
impl DMAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN1 {
        match self.bits {
            false => DMAEN1::Disabled,
            true => DMAEN1::Enabled,
        }
    }
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1::Disabled
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1::Enabled
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN1>;
impl<'a, REG> DMAEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Disabled)
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Enabled)
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1 {
    #[doc = "0: DAC Channel X DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA Underrun Interrupt enabled"]
    Enabled = 1,
}
impl From<DMAUDRIE1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1>;
impl DMAUDRIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDRIE1 {
        match self.bits {
            false => DMAUDRIE1::Disabled,
            true => DMAUDRIE1::Enabled,
        }
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1::Disabled
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1::Enabled
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDRIE1>;
impl<'a, REG> DMAUDRIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Disabled)
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Enabled)
    }
}
#[doc = "DAC Channel 1 calibration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN1 {
    #[doc = "0: DAC Channel X Normal operating mode"]
    Normal = 0,
    #[doc = "1: DAC Channel X calibration mode"]
    Calibration = 1,
}
impl From<CEN1> for bool {
    #[inline(always)]
    fn from(variant: CEN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable"]
pub type CEN1_R = crate::BitReader<CEN1>;
impl CEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN1 {
        match self.bits {
            false => CEN1::Normal,
            true => CEN1::Calibration,
        }
    }
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CEN1::Normal
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == CEN1::Calibration
    }
}
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable"]
pub type CEN1_W<'a, REG> = crate::BitWriter<'a, REG, CEN1>;
impl<'a, REG> CEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1::Normal)
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1::Calibration)
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CRrs> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<CRrs> {
        TEN1_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<CRrs> {
        TSEL1_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<CRrs> {
        MAMP1_W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<CRrs> {
        DMAEN1_W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<CRrs> {
        DMAUDRIE1_W::new(self, 13)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen1(&mut self) -> CEN1_W<CRrs> {
        CEN1_W::new(self, 14)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
