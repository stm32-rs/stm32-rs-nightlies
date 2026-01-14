///Register `TIM17_SR` reader
pub type R = crate::R<TIM17_SRrs>;
///Register `TIM17_SR` writer
pub type W = crate::W<TIM17_SRrs>;
/**Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIF {
    ///0: No update occurred.
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
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
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
    ///No update occurred.
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
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG, UIF>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No update occurred.
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
/**Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).

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
///Field `CC1IF` reader - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
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
///Field `CC1IF` writer - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
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
/**COM interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIF {
    ///0: No COM event occurred
    B0x0 = 0,
    ///1: COM interrupt pending
    B0x1 = 1,
}
impl From<COMIF> for bool {
    #[inline(always)]
    fn from(variant: COMIF) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` reader - COM interrupt flag
pub type COMIF_R = crate::BitReader<COMIF>;
impl COMIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMIF {
        match self.bits {
            false => COMIF::B0x0,
            true => COMIF::B0x1,
        }
    }
    ///No COM event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMIF::B0x0
    }
    ///COM interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMIF::B0x1
    }
}
///Field `COMIF` writer - COM interrupt flag
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG, COMIF>;
impl<'a, REG> COMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No COM event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMIF::B0x0)
    }
    ///COM interrupt pending
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMIF::B0x1)
    }
}
/**Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIF {
    ///0: No break event occurred
    B0x0 = 0,
    ///1: An active level has been detected on the break input
    B0x1 = 1,
}
impl From<BIF> for bool {
    #[inline(always)]
    fn from(variant: BIF) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` reader - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
pub type BIF_R = crate::BitReader<BIF>;
impl BIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIF {
        match self.bits {
            false => BIF::B0x0,
            true => BIF::B0x1,
        }
    }
    ///No break event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIF::B0x0
    }
    ///An active level has been detected on the break input
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIF::B0x1
    }
}
///Field `BIF` writer - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG, BIF>;
impl<'a, REG> BIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No break event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIF::B0x0)
    }
    ///An active level has been detected on the break input
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIF::B0x1)
    }
}
/**Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OF {
    ///0: No overcapture has been detected
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
    ///No overcapture has been detected
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
    ///No overcapture has been detected
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
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17_SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("comif", &self.comif())
            .field("bif", &self.bif())
            .field("cc1of", &self.cc1of())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, TIM17_SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, TIM17_SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<'_, TIM17_SRrs> {
        COMIF_W::new(self, 5)
    }
    ///Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<'_, TIM17_SRrs> {
        BIF_W::new(self, 7)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to 0 .
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, TIM17_SRrs> {
        CC1OF_W::new(self, 9)
    }
}
/**TIM17 status register

You can [`read`](crate::Reg::read) this register and get [`tim17_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM17:TIM17_SR)*/
pub struct TIM17_SRrs;
impl crate::RegisterSpec for TIM17_SRrs {
    type Ux = u16;
}
///`read()` method returns [`tim17_sr::R`](R) reader structure
impl crate::Readable for TIM17_SRrs {}
///`write(|w| ..)` method takes [`tim17_sr::W`](W) writer structure
impl crate::Writable for TIM17_SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM17_SR to value 0
impl crate::Resettable for TIM17_SRrs {}
