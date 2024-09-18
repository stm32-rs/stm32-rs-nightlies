///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.

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
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
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
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.

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
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
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
///Field `CCIF(1-4)` reader - Capture/compare %s interrupt flag
pub type CCIF_R = crate::BitReader;
///Field `CCIF(1-4)` writer - Capture/compare %s interrupt flag
pub type CCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMIF` reader - COM interrupt flag
pub type COMIF_R = crate::BitReader;
///Field `COMIF` writer - COM interrupt flag
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub type TIF_R = crate::BitReader;
///Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIF` reader - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
pub type BIF_R = crate::BitReader;
///Field `BIF` writer - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2IF` reader - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
pub type B2IF_R = crate::BitReader;
///Field `B2IF` writer - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
pub type B2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCOF(1-4)` reader - Capture/Compare %s overcapture flag
pub type CCOF_R = crate::BitReader;
///Field `CCOF(1-4)` writer - Capture/Compare %s overcapture flag
pub type CCOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBIF` reader - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
pub type SBIF_R = crate::BitReader;
///Field `SBIF` writer - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
pub type SBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5IF` reader - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
pub type CC5IF_R = crate::BitReader;
///Field `CC5IF` writer - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
pub type CC5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6IF` reader - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
pub type CC6IF_R = crate::BitReader;
///Field `CC6IF` writer - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
pub type CC6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Capture/compare (1-4) interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IF` field.</div>
    #[inline(always)]
    pub fn ccif(&self, n: u8) -> CCIF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIF_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/compare (1-4) interrupt flag
    #[inline(always)]
    pub fn ccif_iter(&self) -> impl Iterator<Item = CCIF_R> + '_ {
        (0..4).map(move |n| CCIF_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Capture/Compare (1-4) overcapture flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1OF` field.</div>
    #[inline(always)]
    pub fn ccof(&self, n: u8) -> CCOF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCOF_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-4) overcapture flag
    #[inline(always)]
    pub fn ccof_iter(&self) -> impl Iterator<Item = CCOF_R> + '_ {
        (0..4).map(move |n| CCOF_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&self) -> CCOF_R {
        CCOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("comif", &self.comif())
            .field("tif", &self.tif())
            .field("bif", &self.bif())
            .field("b2if", &self.b2if())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .field("sbif", &self.sbif())
            .field("cc5if", &self.cc5if())
            .field("cc6if", &self.cc6if())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    ///Capture/compare (1-4) interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IF` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ccif(&mut self, n: u8) -> CCIF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIF_W::new(self, n + 1)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CCIF_W<SRrs> {
        CCIF_W::new(self, 4)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<SRrs> {
        COMIF_W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<SRrs> {
        BIF_W::new(self, 7)
    }
    ///Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<SRrs> {
        B2IF_W::new(self, 8)
    }
    ///Capture/Compare (1-4) overcapture flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1OF` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ccof(&mut self, n: u8) -> CCOF_W<SRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCOF_W::new(self, n + 9)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CCOF_W<SRrs> {
        CCOF_W::new(self, 12)
    }
    ///Bit 13 - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<SRrs> {
        SBIF_W::new(self, 13)
    }
    ///Bit 16 - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<SRrs> {
        CC5IF_W::new(self, 16)
    }
    ///Bit 17 - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<SRrs> {
        CC6IF_W::new(self, 17)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#TIM1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
