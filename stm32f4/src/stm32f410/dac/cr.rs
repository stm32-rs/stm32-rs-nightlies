#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1 {
    #[doc = "0: DAC channel X disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X enabled"]
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
    #[doc = "DAC channel X disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1::Disabled
    }
    #[doc = "DAC channel X enabled"]
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
    #[doc = "DAC channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Disabled)
    }
    #[doc = "DAC channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN1::Enabled)
    }
}
#[doc = "DAC channel1 output buffer disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFF1 {
    #[doc = "0: DAC channel X output buffer enabled"]
    Enabled = 0,
    #[doc = "1: DAC channel X output buffer disabled"]
    Disabled = 1,
}
impl From<BOFF1> for bool {
    #[inline(always)]
    fn from(variant: BOFF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFF1` reader - DAC channel1 output buffer disable"]
pub type BOFF1_R = crate::BitReader<BOFF1>;
impl BOFF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOFF1 {
        match self.bits {
            false => BOFF1::Enabled,
            true => BOFF1::Disabled,
        }
    }
    #[doc = "DAC channel X output buffer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF1::Enabled
    }
    #[doc = "DAC channel X output buffer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF1::Disabled
    }
}
#[doc = "Field `BOFF1` writer - DAC channel1 output buffer disable"]
pub type BOFF1_W<'a, REG> = crate::BitWriter<'a, REG, BOFF1>;
impl<'a, REG> BOFF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFF1::Enabled)
    }
    #[doc = "DAC channel X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BOFF1::Disabled)
    }
}
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1 {
    #[doc = "0: DAC channel X trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X trigger enabled"]
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
    #[doc = "DAC channel X trigger disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1::Disabled
    }
    #[doc = "DAC channel X trigger enabled"]
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
    #[doc = "DAC channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Disabled)
    }
    #[doc = "DAC channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1::Enabled)
    }
}
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1 {
    #[doc = "3: Timer 5 TRGO event"]
    Tim5Trgo = 3,
    #[doc = "6: EXTI line9"]
    Exti9 = 6,
    #[doc = "7: Software trigger"]
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
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader<TSEL1>;
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL1> {
        match self.bits {
            3 => Some(TSEL1::Tim5Trgo),
            6 => Some(TSEL1::Exti9),
            7 => Some(TSEL1::Software),
            _ => None,
        }
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn is_tim5_trgo(&self) -> bool {
        *self == TSEL1::Tim5Trgo
    }
    #[doc = "EXTI line9"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1::Exti9
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TSEL1::Software
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSEL1>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn tim5_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim5Trgo)
    }
    #[doc = "EXTI line9"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Exti9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Software)
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
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP1_R = crate::FieldReader;
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1 {
    #[doc = "0: DAC channel X DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X DMA mode enabled"]
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
    #[doc = "DAC channel X DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1::Disabled
    }
    #[doc = "DAC channel X DMA mode enabled"]
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
    #[doc = "DAC channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Disabled)
    }
    #[doc = "DAC channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1::Enabled)
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1 {
    #[doc = "0: DAC channel X DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X DMA Underrun Interrupt enabled"]
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
    #[doc = "DAC channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1::Disabled
    }
    #[doc = "DAC channel X DMA Underrun Interrupt enabled"]
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
    #[doc = "DAC channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Disabled)
    }
    #[doc = "DAC channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
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
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CRrs> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn boff1(&mut self) -> BOFF1_W<CRrs> {
        BOFF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<CRrs> {
        TEN1_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<CRrs> {
        TSEL1_W::new(self, 3)
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
