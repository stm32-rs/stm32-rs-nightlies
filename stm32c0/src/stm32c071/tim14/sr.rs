///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow and if UDIS= 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS= 0 and UDIS= 0 in the TIMx_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFR {
    ///0: No update occurred
    NoUpdateOccurred = 0,
    ///1: Update interrupt pending
    UpdatePending = 1,
}
impl From<UIFR> for bool {
    #[inline(always)]
    fn from(variant: UIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow and if UDIS= 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS= 0 and UDIS= 0 in the TIMx_CR1 register.
pub type UIF_R = crate::BitReader<UIFR>;
impl UIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIFR {
        match self.bits {
            false => UIFR::NoUpdateOccurred,
            true => UIFR::UpdatePending,
        }
    }
    ///No update occurred
    #[inline(always)]
    pub fn is_no_update_occurred(&self) -> bool {
        *self == UIFR::NoUpdateOccurred
    }
    ///Update interrupt pending
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFR::UpdatePending
    }
}
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow and if UDIS= 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS= 0 and UDIS= 0 in the TIMx_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<UIFW> for bool {
    #[inline(always)]
    fn from(variant: UIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow and if UDIS= 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS= 0 and UDIS= 0 in the TIMx_CR1 register.
pub type UIF_W<'a, REG> = crate::BitWriter0C<'a, REG, UIFW>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UIFW::Clear)
    }
}
/**Capture/compare %s interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFR {
    ///0: No campture/compare has been detected
    NoMatch = 0,
    ///1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
    Match = 1,
}
impl From<CC1IFR> for bool {
    #[inline(always)]
    fn from(variant: CC1IFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CCIF(1-1)` reader - Capture/compare %s interrupt flag
pub type CCIF_R = crate::BitReader<CC1IFR>;
impl CCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IFR {
        match self.bits {
            false => CC1IFR::NoMatch,
            true => CC1IFR::Match,
        }
    }
    ///No campture/compare has been detected
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CC1IFR::NoMatch
    }
    ///If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IFR::Match
    }
}
/**Capture/compare %s interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CC1IFW> for bool {
    #[inline(always)]
    fn from(variant: CC1IFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCIF(1-1)` writer - Capture/compare %s interrupt flag
pub type CCIF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1IFW>;
impl<'a, REG> CCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IFW::Clear)
    }
}
/**Capture/Compare %s overcapture flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFR {
    ///0: No overcapture has been detected
    NoOvercapture = 0,
    ///1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
    Overcapture = 1,
}
impl From<CC1OFR> for bool {
    #[inline(always)]
    fn from(variant: CC1OFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CCOF(1-1)` reader - Capture/Compare %s overcapture flag
pub type CCOF_R = crate::BitReader<CC1OFR>;
impl CCOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1OFR {
        match self.bits {
            false => CC1OFR::NoOvercapture,
            true => CC1OFR::Overcapture,
        }
    }
    ///No overcapture has been detected
    #[inline(always)]
    pub fn is_no_overcapture(&self) -> bool {
        *self == CC1OFR::NoOvercapture
    }
    ///The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OFR::Overcapture
    }
}
/**Capture/Compare %s overcapture flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CC1OFW> for bool {
    #[inline(always)]
    fn from(variant: CC1OFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCOF(1-1)` writer - Capture/Compare %s overcapture flag
pub type CCOF_W<'a, REG> = crate::BitWriter0C<'a, REG, CC1OFW>;
impl<'a, REG> CCOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OFW::Clear)
    }
}
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow and if UDIS= 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS= 0 and UDIS= 0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Capture/compare (1-1) interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IF` field.</div>
    #[inline(always)]
    pub fn ccif(&self, n: u8) -> CCIF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIF_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/compare (1-1) interrupt flag
    #[inline(always)]
    pub fn ccif_iter(&self) -> impl Iterator<Item = CCIF_R> + '_ {
        (0..1).map(move |n| CCIF_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Capture/Compare (1-1) overcapture flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1OF` field.</div>
    #[inline(always)]
    pub fn ccof(&self, n: u8) -> CCOF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCOF_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) overcapture flag
    #[inline(always)]
    pub fn ccof_iter(&self) -> impl Iterator<Item = CCOF_R> + '_ {
        (0..1).map(move |n| CCOF_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc1of", &self.cc1of())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow and if UDIS= 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS= 0 and UDIS= 0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    ///Capture/compare (1-1) interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IF` field.</div>
    #[inline(always)]
    pub fn ccif(&mut self, n: u8) -> CCIF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCIF_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 1)
    }
    ///Capture/Compare (1-1) overcapture flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1OF` field.</div>
    #[inline(always)]
    pub fn ccof(&mut self, n: u8) -> CCOF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCOF_W::new(self, n * 0 + 9)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 9)
    }
}
/**TIM14 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#TIM14:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x0203;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u16 = 0;
}
