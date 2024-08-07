///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**DAC channel1 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1 {
    ///0: DAC channel X disabled
    Disabled = 0,
    ///1: DAC channel X enabled
    Enabled = 1,
}
impl From<EN1> for bool {
    #[inline(always)]
    fn from(variant: EN1) -> Self {
        variant as u8 != 0
    }
}
///Field `EN1` reader - DAC channel1 enable
pub type EN1_R = crate::BitReader<EN1>;
impl EN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN1 {
        match self.bits {
            false => EN1::Disabled,
            true => EN1::Enabled,
        }
    }
    ///DAC channel X disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1::Disabled
    }
    ///DAC channel X enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1::Enabled
    }
}
///Field `EN1` writer - DAC channel1 enable
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Disabled)
    }
    ///DAC channel X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Enabled)
    }
}
/**DAC channel1 output buffer disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFF1 {
    ///0: DAC channel X output buffer enabled
    Enabled = 0,
    ///1: DAC channel X output buffer disabled
    Disabled = 1,
}
impl From<BOFF1> for bool {
    #[inline(always)]
    fn from(variant: BOFF1) -> Self {
        variant as u8 != 0
    }
}
///Field `BOFF1` reader - DAC channel1 output buffer disable
pub type BOFF1_R = crate::BitReader<BOFF1>;
impl BOFF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOFF1 {
        match self.bits {
            false => BOFF1::Enabled,
            true => BOFF1::Disabled,
        }
    }
    ///DAC channel X output buffer enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF1::Enabled
    }
    ///DAC channel X output buffer disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF1::Disabled
    }
}
///Field `BOFF1` writer - DAC channel1 output buffer disable
pub type BOFF1_W<'a, REG> = crate::BitWriter<'a, REG, BOFF1>;
impl<'a, REG> BOFF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X output buffer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFF1::Enabled)
    }
    ///DAC channel X output buffer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFF1::Disabled)
    }
}
/**DAC channel1 trigger enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1 {
    ///0: DAC channel X trigger disabled
    Disabled = 0,
    ///1: DAC channel X trigger enabled
    Enabled = 1,
}
impl From<TEN1> for bool {
    #[inline(always)]
    fn from(variant: TEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `TEN1` reader - DAC channel1 trigger enable
pub type TEN1_R = crate::BitReader<TEN1>;
impl TEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEN1 {
        match self.bits {
            false => TEN1::Disabled,
            true => TEN1::Enabled,
        }
    }
    ///DAC channel X trigger disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1::Disabled
    }
    ///DAC channel X trigger enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1::Enabled
    }
}
///Field `TEN1` writer - DAC channel1 trigger enable
pub type TEN1_W<'a, REG> = crate::BitWriter<'a, REG, TEN1>;
impl<'a, REG> TEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Disabled)
    }
    ///DAC channel X trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Enabled)
    }
}
/**DAC channel1 trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1 {
    ///0: Timer 6 TRGO event
    Tim6Trgo = 0,
    ///1: Timer 3 TRGO event
    Tim3Trgo = 1,
    ///2: Timer 7 TRGO event
    Tim7Trgo = 2,
    ///3: Timer 15 TRGO event
    Tim15Trgo = 3,
    ///4: Timer 2 TRGO event
    Tim2Trgo = 4,
    ///6: EXTI line9
    Exti9 = 6,
    ///7: Software trigger
    Software = 7,
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
impl crate::IsEnum for TSEL1 {}
///Field `TSEL1` reader - DAC channel1 trigger selection
pub type TSEL1_R = crate::FieldReader<TSEL1>;
impl TSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL1> {
        match self.bits {
            0 => Some(TSEL1::Tim6Trgo),
            1 => Some(TSEL1::Tim3Trgo),
            2 => Some(TSEL1::Tim7Trgo),
            3 => Some(TSEL1::Tim15Trgo),
            4 => Some(TSEL1::Tim2Trgo),
            6 => Some(TSEL1::Exti9),
            7 => Some(TSEL1::Software),
            _ => None,
        }
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1::Tim6Trgo
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == TSEL1::Tim3Trgo
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1::Tim7Trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == TSEL1::Tim15Trgo
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1::Tim2Trgo
    }
    ///EXTI line9
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1::Exti9
    }
    ///Software trigger
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TSEL1::Software
    }
}
///Field `TSEL1` writer - DAC channel1 trigger selection
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSEL1>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim6Trgo)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim3Trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim7Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim15Trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim2Trgo)
    }
    ///EXTI line9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Exti9)
    }
    ///Software trigger
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Software)
    }
}
/**DAC channel1 noise/triangle wave generation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1 {
    ///0: Wave generation disabled
    Disabled = 0,
    ///1: Noise wave generation enabled
    Noise = 1,
    ///2: Triangle wave generation enabled
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
impl crate::IsEnum for WAVE1 {}
///Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable
pub type WAVE1_R = crate::FieldReader<WAVE1>;
impl WAVE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAVE1> {
        match self.bits {
            0 => Some(WAVE1::Disabled),
            1 => Some(WAVE1::Noise),
            2 => Some(WAVE1::Triangle),
            _ => None,
        }
    }
    ///Wave generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1::Disabled
    }
    ///Noise wave generation enabled
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1::Noise
    }
    ///Triangle wave generation enabled
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1::Triangle
    }
}
///Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable
pub type WAVE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAVE1>;
impl<'a, REG> WAVE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Wave generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Disabled)
    }
    ///Noise wave generation enabled
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Noise)
    }
    ///Triangle wave generation enabled
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1::Triangle)
    }
}
///Field `MAMP1` reader - DAC channel1 mask/amplitude selector
pub type MAMP1_R = crate::FieldReader;
///Field `MAMP1` writer - DAC channel1 mask/amplitude selector
pub type MAMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**DAC channel1 DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1 {
    ///0: DAC channel X DMA mode disabled
    Disabled = 0,
    ///1: DAC channel X DMA mode enabled
    Enabled = 1,
}
impl From<DMAEN1> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN1` reader - DAC channel1 DMA enable
pub type DMAEN1_R = crate::BitReader<DMAEN1>;
impl DMAEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN1 {
        match self.bits {
            false => DMAEN1::Disabled,
            true => DMAEN1::Enabled,
        }
    }
    ///DAC channel X DMA mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1::Disabled
    }
    ///DAC channel X DMA mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1::Enabled
    }
}
///Field `DMAEN1` writer - DAC channel1 DMA enable
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN1>;
impl<'a, REG> DMAEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Disabled)
    }
    ///DAC channel X DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Enabled)
    }
}
/**DAC channel1 DMA Underrun Interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1 {
    ///0: DAC channel X DMA Underrun Interrupt disabled
    Disabled = 0,
    ///1: DAC channel X DMA Underrun Interrupt enabled
    Enabled = 1,
}
impl From<DMAUDRIE1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1>;
impl DMAUDRIE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDRIE1 {
        match self.bits {
            false => DMAUDRIE1::Disabled,
            true => DMAUDRIE1::Enabled,
        }
    }
    ///DAC channel X DMA Underrun Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1::Disabled
    }
    ///DAC channel X DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1::Enabled
    }
}
///Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDRIE1>;
impl<'a, REG> DMAUDRIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X DMA Underrun Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Disabled)
    }
    ///DAC channel X DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Enabled)
    }
}
///Field `BOFF2` reader - DAC channel2 output buffer disable
pub use BOFF1_R as BOFF2_R;
///Field `BOFF2` writer - DAC channel2 output buffer disable
pub use BOFF1_W as BOFF2_W;
///Field `EN2` reader - DAC channel2 enable
pub use EN1_R as EN2_R;
///Field `EN2` writer - DAC channel2 enable
pub use EN1_W as EN2_W;
///Field `TEN2` reader - DAC channel2 trigger enable
pub use TEN1_R as TEN2_R;
///Field `TEN2` writer - DAC channel2 trigger enable
pub use TEN1_W as TEN2_W;
/**DAC channel2 trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL2 {
    ///0: Timer 6 TRGO event
    Tim6Trgo = 0,
    ///1: Timer 8 TRGO event
    Tim8Trgo = 1,
    ///2: Timer 7 TRGO event
    Tim7Trgo = 2,
    ///3: Timer 5 TRGO event
    Tim5Trgo = 3,
    ///4: Timer 2 TRGO event
    Tim2Trgo = 4,
    ///5: Timer 4 TRGO event
    Tim4Trgo = 5,
    ///6: EXTI line9
    Exti9 = 6,
    ///7: Software trigger
    Software = 7,
}
impl From<TSEL2> for u8 {
    #[inline(always)]
    fn from(variant: TSEL2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL2 {
    type Ux = u8;
}
impl crate::IsEnum for TSEL2 {}
///Field `TSEL2` reader - DAC channel2 trigger selection
pub type TSEL2_R = crate::FieldReader<TSEL2>;
impl TSEL2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEL2 {
        match self.bits {
            0 => TSEL2::Tim6Trgo,
            1 => TSEL2::Tim8Trgo,
            2 => TSEL2::Tim7Trgo,
            3 => TSEL2::Tim5Trgo,
            4 => TSEL2::Tim2Trgo,
            5 => TSEL2::Tim4Trgo,
            6 => TSEL2::Exti9,
            7 => TSEL2::Software,
            _ => unreachable!(),
        }
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL2::Tim6Trgo
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == TSEL2::Tim8Trgo
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL2::Tim7Trgo
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn is_tim5_trgo(&self) -> bool {
        *self == TSEL2::Tim5Trgo
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL2::Tim2Trgo
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == TSEL2::Tim4Trgo
    }
    ///EXTI line9
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL2::Exti9
    }
    ///Software trigger
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TSEL2::Software
    }
}
///Field `TSEL2` writer - DAC channel2 trigger selection
pub type TSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSEL2, crate::Safe>;
impl<'a, REG> TSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Tim6Trgo)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Tim8Trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Tim7Trgo)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Tim5Trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Tim2Trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Tim4Trgo)
    }
    ///EXTI line9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Exti9)
    }
    ///Software trigger
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2::Software)
    }
}
///Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable
pub use WAVE1_R as WAVE2_R;
///Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable
pub use WAVE1_W as WAVE2_W;
///Field `MAMP2` reader - DAC channel2 mask/amplitude selector
pub type MAMP2_R = crate::FieldReader;
///Field `MAMP2` writer - DAC channel2 mask/amplitude selector
pub type MAMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `DMAEN2` reader - DAC channel2 DMA enable
pub use DMAEN1_R as DMAEN2_R;
///Field `DMAEN2` writer - DAC channel2 DMA enable
pub use DMAEN1_W as DMAEN2_W;
///Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable
pub use DMAUDRIE1_R as DMAUDRIE2_R;
///Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable
pub use DMAUDRIE1_W as DMAUDRIE2_W;
impl R {
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DAC channel1 output buffer disable
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - DAC channel1 trigger selection
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - DAC channel2 enable
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC channel2 output buffer disable
    #[inline(always)]
    pub fn boff2(&self) -> BOFF2_R {
        BOFF2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:21 - DAC channel2 trigger selection
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - DAC channel2 DMA enable
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun interrupt enable
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dmaudrie1", &self.dmaudrie1())
            .field("dmaudrie2", &self.dmaudrie2())
            .field("dmaen1", &self.dmaen1())
            .field("dmaen2", &self.dmaen2())
            .field("mamp2", &self.mamp2())
            .field("wave1", &self.wave1())
            .field("wave2", &self.wave2())
            .field("tsel2", &self.tsel2())
            .field("ten1", &self.ten1())
            .field("ten2", &self.ten2())
            .field("boff1", &self.boff1())
            .field("boff2", &self.boff2())
            .field("en1", &self.en1())
            .field("en2", &self.en2())
            .field("mamp1", &self.mamp1())
            .field("tsel1", &self.tsel1())
            .finish()
    }
}
impl W {
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CRrs> {
        EN1_W::new(self, 0)
    }
    ///Bit 1 - DAC channel1 output buffer disable
    #[inline(always)]
    #[must_use]
    pub fn boff1(&mut self) -> BOFF1_W<CRrs> {
        BOFF1_W::new(self, 1)
    }
    ///Bit 2 - DAC channel1 trigger enable
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<CRrs> {
        TEN1_W::new(self, 2)
    }
    ///Bits 3:5 - DAC channel1 trigger selection
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<CRrs> {
        TSEL1_W::new(self, 3)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 6)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<CRrs> {
        MAMP1_W::new(self, 8)
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<CRrs> {
        DMAEN1_W::new(self, 12)
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<CRrs> {
        DMAUDRIE1_W::new(self, 13)
    }
    ///Bit 16 - DAC channel2 enable
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<CRrs> {
        EN2_W::new(self, 16)
    }
    ///Bit 17 - DAC channel2 output buffer disable
    #[inline(always)]
    #[must_use]
    pub fn boff2(&mut self) -> BOFF2_W<CRrs> {
        BOFF2_W::new(self, 17)
    }
    ///Bit 18 - DAC channel2 trigger enable
    #[inline(always)]
    #[must_use]
    pub fn ten2(&mut self) -> TEN2_W<CRrs> {
        TEN2_W::new(self, 18)
    }
    ///Bits 19:21 - DAC channel2 trigger selection
    #[inline(always)]
    #[must_use]
    pub fn tsel2(&mut self) -> TSEL2_W<CRrs> {
        TSEL2_W::new(self, 19)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<CRrs> {
        WAVE2_W::new(self, 22)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector
    #[inline(always)]
    #[must_use]
    pub fn mamp2(&mut self) -> MAMP2_W<CRrs> {
        MAMP2_W::new(self, 24)
    }
    ///Bit 28 - DAC channel2 DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen2(&mut self) -> DMAEN2_W<CRrs> {
        DMAEN2_W::new(self, 28)
    }
    ///Bit 29 - DAC channel2 DMA underrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<CRrs> {
        DMAUDRIE2_W::new(self, 29)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#DAC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
