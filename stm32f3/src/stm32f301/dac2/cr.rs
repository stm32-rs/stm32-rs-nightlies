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
    #[doc = "0: Timer 6 TRGO event"]
    Tim6Trgo = 0,
    #[doc = "1: Timer 3 TRGO event"]
    Tim3Trgo = 1,
    #[doc = "2: Timer 7 TRGO event"]
    Tim7Trgo = 2,
    #[doc = "3: Timer 15 TRGO event"]
    Tim15Trgo = 3,
    #[doc = "4: Timer 2 TRGO event"]
    Tim2Trgo = 4,
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
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1::Tim6Trgo
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == TSEL1::Tim3Trgo
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1::Tim7Trgo
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == TSEL1::Tim15Trgo
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1::Tim2Trgo
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
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim6Trgo)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim3Trgo)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim7Trgo)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim15Trgo)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1::Tim2Trgo)
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
#[doc = "Field `WAVE2` reader - WAVE2"]
pub type WAVE2_R = crate::BitReader;
#[doc = "Field `WAVE2` writer - WAVE2"]
pub type WAVE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::BitReader;
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP10` reader - MAMP10"]
pub type MAMP10_R = crate::BitReader;
#[doc = "Field `MAMP10` writer - MAMP10"]
pub type MAMP10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP11` reader - MAMP11"]
pub type MAMP11_R = crate::BitReader;
#[doc = "Field `MAMP11` writer - MAMP11"]
pub type MAMP11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP12` reader - MAMP12"]
pub type MAMP12_R = crate::BitReader;
#[doc = "Field `MAMP12` writer - MAMP12"]
pub type MAMP12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMP13` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP13_R = crate::BitReader;
#[doc = "Field `MAMP13` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP13_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - WAVE2"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MAMP10"]
    #[inline(always)]
    pub fn mamp10(&self) -> MAMP10_R {
        MAMP10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MAMP11"]
    #[inline(always)]
    pub fn mamp11(&self) -> MAMP11_R {
        MAMP11_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MAMP12"]
    #[inline(always)]
    pub fn mamp12(&self) -> MAMP12_R {
        MAMP12_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp13(&self) -> MAMP13_R {
        MAMP13_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 6 - WAVE2"]
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<CRrs> {
        WAVE2_W::new(self, 6)
    }
    #[doc = "Bit 7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 7)
    }
    #[doc = "Bit 8 - MAMP10"]
    #[inline(always)]
    #[must_use]
    pub fn mamp10(&mut self) -> MAMP10_W<CRrs> {
        MAMP10_W::new(self, 8)
    }
    #[doc = "Bit 9 - MAMP11"]
    #[inline(always)]
    #[must_use]
    pub fn mamp11(&mut self) -> MAMP11_W<CRrs> {
        MAMP11_W::new(self, 9)
    }
    #[doc = "Bit 10 - MAMP12"]
    #[inline(always)]
    #[must_use]
    pub fn mamp12(&mut self) -> MAMP12_W<CRrs> {
        MAMP12_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp13(&mut self) -> MAMP13_W<CRrs> {
        MAMP13_W::new(self, 11)
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
