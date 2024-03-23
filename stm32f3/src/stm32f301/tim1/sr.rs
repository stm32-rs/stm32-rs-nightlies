#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFR {
    #[doc = "0: No update occurred"]
    NoUpdateOccurred = 0,
    #[doc = "1: Update interrupt pending"]
    UpdatePending = 1,
}
impl From<UIFR> for bool {
    #[inline(always)]
    fn from(variant: UIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader<UIFR>;
impl UIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIFR {
        match self.bits {
            false => UIFR::NoUpdateOccurred,
            true => UIFR::UpdatePending,
        }
    }
    #[doc = "No update occurred"]
    #[inline(always)]
    pub fn is_no_update_occurred(&self) -> bool {
        *self == UIFR::NoUpdateOccurred
    }
    #[doc = "Update interrupt pending"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFR::UpdatePending
    }
}
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<UIFW> for bool {
    #[inline(always)]
    fn from(variant: UIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a, REG> = crate::BitWriter0C<'a, REG, UIFW>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UIFW::Clear)
    }
}
#[doc = "Capture/compare %s interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFR {
    #[doc = "1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register."]
    Match = 1,
}
impl From<CC1IFR> for bool {
    #[inline(always)]
    fn from(variant: CC1IFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIF(1-4)` reader - Capture/compare %s interrupt flag"]
pub type CCIF_R = crate::BitReader<CC1IFR>;
impl CCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1IFR> {
        match self.bits {
            true => Some(CC1IFR::Match),
            _ => None,
        }
    }
    #[doc = "If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IFR::Match
    }
}
#[doc = "Capture/compare %s interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1IFW> for bool {
    #[inline(always)]
    fn from(variant: CC1IFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIF(1-4)` writer - Capture/compare %s interrupt flag"]
pub type CCIF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1IFW>;
impl<'a, REG> CCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IFW::Clear)
    }
}
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFR {
    #[doc = "0: No trigger event occurred"]
    NoTrigger = 0,
    #[doc = "1: Trigger interrupt pending"]
    Trigger = 1,
}
impl From<TIFR> for bool {
    #[inline(always)]
    fn from(variant: TIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TIF_R = crate::BitReader<TIFR>;
impl TIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIFR {
        match self.bits {
            false => TIFR::NoTrigger,
            true => TIFR::Trigger,
        }
    }
    #[doc = "No trigger event occurred"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TIFR::NoTrigger
    }
    #[doc = "Trigger interrupt pending"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TIFR::Trigger
    }
}
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<TIFW> for bool {
    #[inline(always)]
    fn from(variant: TIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TIF_W<'a, REG> = crate::BitWriter0C<'a, REG, TIFW>;
impl<'a, REG> TIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIFW::Clear)
    }
}
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2IF` reader - Break 2 interrupt flag"]
pub type B2IF_R = crate::BitReader;
#[doc = "Field `B2IF` writer - Break 2 interrupt flag"]
pub type B2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture/Compare %s overcapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFR {
    #[doc = "1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    Overcapture = 1,
}
impl From<CC1OFR> for bool {
    #[inline(always)]
    fn from(variant: CC1OFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCOF(1-4)` reader - Capture/Compare %s overcapture flag"]
pub type CCOF_R = crate::BitReader<CC1OFR>;
impl CCOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1OFR> {
        match self.bits {
            true => Some(CC1OFR::Overcapture),
            _ => None,
        }
    }
    #[doc = "The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OFR::Overcapture
    }
}
#[doc = "Capture/Compare %s overcapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1OFW> for bool {
    #[inline(always)]
    fn from(variant: CC1OFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCOF(1-4)` writer - Capture/Compare %s overcapture flag"]
pub type CCOF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1OFW>;
impl<'a, REG> CCOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OFW::Clear)
    }
}
#[doc = "Field `C5IF` reader - Capture/Compare 5 interrupt flag"]
pub type C5IF_R = crate::BitReader;
#[doc = "Field `C5IF` writer - Capture/Compare 5 interrupt flag"]
pub type C5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C6IF` reader - Capture/Compare 6 interrupt flag"]
pub type C6IF_R = crate::BitReader;
#[doc = "Field `C6IF` writer - Capture/Compare 6 interrupt flag"]
pub type C6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Capture/compare (1-4) interrupt flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IF` field"]
    #[inline(always)]
    pub fn ccif(&self, n: u8) -> CCIF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIF_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/compare (1-4) interrupt flag"]
    #[inline(always)]
    pub fn ccif_iter(&self) -> impl Iterator<Item = CCIF_R> + '_ {
        (0..4).map(move |n| CCIF_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag"]
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-4) overcapture flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1OF` field"]
    #[inline(always)]
    pub fn ccof(&self, n: u8) -> CCOF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCOF_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) overcapture flag"]
    #[inline(always)]
    pub fn ccof_iter(&self) -> impl Iterator<Item = CCOF_R> + '_ {
        (0..4).map(move |n| CCOF_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn c5if(&self) -> C5IF_R {
        C5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn c6if(&self) -> C6IF_R {
        C6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    #[doc = "Capture/compare (1-4) interrupt flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IF` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccif(&mut self, n: u8) -> CCIF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIF_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<SRrs> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<SRrs> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<SRrs> {
        B2IF_W::new(self, 8)
    }
    #[doc = "Capture/Compare (1-4) overcapture flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1OF` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccof(&mut self, n: u8) -> CCOF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCOF_W::new(self, n + 9)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 2 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 12)
    }
    #[doc = "Bit 16 - Capture/Compare 5 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c5if(&mut self) -> C5IF_W<SRrs> {
        C5IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/Compare 6 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c6if(&mut self) -> C6IF_W<SRrs> {
        C6IF_W::new(self, 17)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0243;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
