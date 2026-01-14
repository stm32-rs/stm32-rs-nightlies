///Register `TIM3_SR` reader
pub type R = crate::R<TIM3_SRrs>;
///Register `TIM3_SR` writer
pub type W = crate::W<TIM3_SRrs>;
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIF {
    ///0: No update occurred
    B0x0 = 0,
    ///1: Update interrupt pending. This bit is set by hardware when the registers are updated:
    B0x1 = 1,
}
impl From<UIF> for bool {
    #[inline(always)]
    fn from(variant: UIF) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub type UIF_R = crate::BitReader<UIF>;
impl UIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIF {
        match self.bits {
            false => UIF::B0x0,
            true => UIF::B0x1,
        }
    }
    ///No update occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIF::B0x0
    }
    ///Update interrupt pending. This bit is set by hardware when the registers are updated:
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UIF::B0x1
    }
}
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG, UIF>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No update occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIF::B0x0)
    }
    ///Update interrupt pending. This bit is set by hardware when the registers are updated:
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIF::B0x1)
    }
}
/**Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IF {
    ///0: No compare match / No input capture occurred
    B0x0 = 0,
    ///1: A compare match or an input capture occurred
    B0x1 = 1,
}
impl From<CC1IF> for bool {
    #[inline(always)]
    fn from(variant: CC1IF) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
pub type CC1IF_R = crate::BitReader<CC1IF>;
impl CC1IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IF {
        match self.bits {
            false => CC1IF::B0x0,
            true => CC1IF::B0x1,
        }
    }
    ///No compare match / No input capture occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IF::B0x0
    }
    ///A compare match or an input capture occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1IF::B0x1
    }
}
///Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG, CC1IF>;
impl<'a, REG> CC1IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match / No input capture occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IF::B0x0)
    }
    ///A compare match or an input capture occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IF::B0x1)
    }
}
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag Refer to CC1IF description
pub type CC2IF_R = crate::BitReader;
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag Refer to CC1IF description
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IF` reader - Capture/Compare 3 interrupt flag Refer to CC1IF description
pub type CC3IF_R = crate::BitReader;
///Field `CC3IF` writer - Capture/Compare 3 interrupt flag Refer to CC1IF description
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IF` reader - Capture/Compare 4 interrupt flag Refer to CC1IF description
pub type CC4IF_R = crate::BitReader;
///Field `CC4IF` writer - Capture/Compare 4 interrupt flag Refer to CC1IF description
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIF {
    ///0: No trigger event occurred.
    B0x0 = 0,
    ///1: Trigger interrupt pending.
    B0x1 = 1,
}
impl From<TIF> for bool {
    #[inline(always)]
    fn from(variant: TIF) -> Self {
        variant as u8 != 0
    }
}
///Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub type TIF_R = crate::BitReader<TIF>;
impl TIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIF {
        match self.bits {
            false => TIF::B0x0,
            true => TIF::B0x1,
        }
    }
    ///No trigger event occurred.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIF::B0x0
    }
    ///Trigger interrupt pending.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIF::B0x1
    }
}
///Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG, TIF>;
impl<'a, REG> TIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No trigger event occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIF::B0x0)
    }
    ///Trigger interrupt pending.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIF::B0x1)
    }
}
/**Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OF {
    ///0: No overcapture has been detected.
    B0x0 = 0,
    ///1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    B0x1 = 1,
}
impl From<CC1OF> for bool {
    #[inline(always)]
    fn from(variant: CC1OF) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .
pub type CC1OF_R = crate::BitReader<CC1OF>;
impl CC1OF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1OF {
        match self.bits {
            false => CC1OF::B0x0,
            true => CC1OF::B0x1,
        }
    }
    ///No overcapture has been detected.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1OF::B0x0
    }
    ///The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1OF::B0x1
    }
}
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG, CC1OF>;
impl<'a, REG> CC1OF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overcapture has been detected.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OF::B0x0)
    }
    ///The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OF::B0x1)
    }
}
///Field `CC2OF` reader - Capture/compare 2 overcapture flag refer to CC1OF description
pub type CC2OF_R = crate::BitReader;
///Field `CC2OF` writer - Capture/compare 2 overcapture flag refer to CC1OF description
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3OF` reader - Capture/Compare 3 overcapture flag refer to CC1OF description
pub type CC3OF_R = crate::BitReader;
///Field `CC3OF` writer - Capture/Compare 3 overcapture flag refer to CC1OF description
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4OF` reader - Capture/Compare 4 overcapture flag refer to CC1OF description
pub type CC4OF_R = crate::BitReader;
///Field `CC4OF` writer - Capture/Compare 4 overcapture flag refer to CC1OF description
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("tif", &self.tif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow and if UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to the synchro control register description), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, TIM3_SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, TIM3_SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<'_, TIM3_SRrs> {
        CC2IF_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W<'_, TIM3_SRrs> {
        CC3IF_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W<'_, TIM3_SRrs> {
        CC4IF_W::new(self, 4)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<'_, TIM3_SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, TIM3_SRrs> {
        CC1OF_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<'_, TIM3_SRrs> {
        CC2OF_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W<'_, TIM3_SRrs> {
        CC3OF_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag refer to CC1OF description
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W<'_, TIM3_SRrs> {
        CC4OF_W::new(self, 12)
    }
}
/**TIM3 status register

You can [`read`](crate::Reg::read) this register and get [`tim3_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM3:TIM3_SR)*/
pub struct TIM3_SRrs;
impl crate::RegisterSpec for TIM3_SRrs {
    type Ux = u16;
}
///`read()` method returns [`tim3_sr::R`](R) reader structure
impl crate::Readable for TIM3_SRrs {}
///`write(|w| ..)` method takes [`tim3_sr::W`](W) writer structure
impl crate::Writable for TIM3_SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_SR to value 0
impl crate::Resettable for TIM3_SRrs {}
