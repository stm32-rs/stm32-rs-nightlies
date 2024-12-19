///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.

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
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
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
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.

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
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
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
///Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag Refer to CC1IF description
pub type CC2IF_R = crate::BitReader;
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag Refer to CC1IF description
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub type TIF_R = crate::BitReader;
///Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.
pub type CC1OF_R = crate::BitReader;
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OF` reader - Capture/compare 2 overcapture flag refer to CC1OF description
pub type CC2OF_R = crate::BitReader;
///Field `CC2OF` writer - Capture/compare 2 overcapture flag refer to CC1OF description
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("tif", &self.tif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<SRrs> {
        CC2IF_W::new(self, 2)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to ‘0’.
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<SRrs> {
        CC1OF_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<SRrs> {
        CC2OF_W::new(self, 10)
    }
}
/**TIM2 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#TIM12:SR)*/
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
