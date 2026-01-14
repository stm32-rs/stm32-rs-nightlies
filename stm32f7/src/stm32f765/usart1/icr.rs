///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Parity error clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECF {
    ///1: Clears the PE flag in the ISR register
    Clear = 1,
}
impl From<PECF> for bool {
    #[inline(always)]
    fn from(variant: PECF) -> Self {
        variant as u8 != 0
    }
}
///Field `PECF` writer - Parity error clear flag
pub type PECF_W<'a, REG> = crate::BitWriter1C<'a, REG, PECF>;
impl<'a, REG> PECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the PE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECF::Clear)
    }
}
/**Framing error clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FECF {
    ///1: Clears the FE flag in the ISR register
    Clear = 1,
}
impl From<FECF> for bool {
    #[inline(always)]
    fn from(variant: FECF) -> Self {
        variant as u8 != 0
    }
}
///Field `FECF` writer - Framing error clear flag
pub type FECF_W<'a, REG> = crate::BitWriter1C<'a, REG, FECF>;
impl<'a, REG> FECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the FE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FECF::Clear)
    }
}
/**Noise detected clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCF {
    ///1: Clears the NF flag in the ISR register
    Clear = 1,
}
impl From<NCF> for bool {
    #[inline(always)]
    fn from(variant: NCF) -> Self {
        variant as u8 != 0
    }
}
///Field `NCF` writer - Noise detected clear flag
pub type NCF_W<'a, REG> = crate::BitWriter1C<'a, REG, NCF>;
impl<'a, REG> NCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the NF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NCF::Clear)
    }
}
/**Overrun error clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORECF {
    ///1: Clears the ORE flag in the ISR register
    Clear = 1,
}
impl From<ORECF> for bool {
    #[inline(always)]
    fn from(variant: ORECF) -> Self {
        variant as u8 != 0
    }
}
///Field `ORECF` writer - Overrun error clear flag
pub type ORECF_W<'a, REG> = crate::BitWriter1C<'a, REG, ORECF>;
impl<'a, REG> ORECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the ORE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ORECF::Clear)
    }
}
/**Idle line detected clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLECF {
    ///1: Clears the IDLE flag in the ISR register
    Clear = 1,
}
impl From<IDLECF> for bool {
    #[inline(always)]
    fn from(variant: IDLECF) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLECF` writer - Idle line detected clear flag
pub type IDLECF_W<'a, REG> = crate::BitWriter1C<'a, REG, IDLECF>;
impl<'a, REG> IDLECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the IDLE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECF::Clear)
    }
}
/**Transmission complete clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCCF {
    ///1: Clears the TC flag in the ISR register
    Clear = 1,
}
impl From<TCCF> for bool {
    #[inline(always)]
    fn from(variant: TCCF) -> Self {
        variant as u8 != 0
    }
}
///Field `TCCF` writer - Transmission complete clear flag
pub type TCCF_W<'a, REG> = crate::BitWriter1C<'a, REG, TCCF>;
impl<'a, REG> TCCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the TC flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCCF::Clear)
    }
}
/**LIN break detection clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDCF {
    ///1: Clears the LBDF flag in the ISR register
    Clear = 1,
}
impl From<LBDCF> for bool {
    #[inline(always)]
    fn from(variant: LBDCF) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDCF` writer - LIN break detection clear flag
pub type LBDCF_W<'a, REG> = crate::BitWriter1C<'a, REG, LBDCF>;
impl<'a, REG> LBDCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the LBDF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LBDCF::Clear)
    }
}
/**CTS clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCF {
    ///1: Clears the CTSIF flag in the ISR register
    Clear = 1,
}
impl From<CTSCF> for bool {
    #[inline(always)]
    fn from(variant: CTSCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSCF` writer - CTS clear flag
pub type CTSCF_W<'a, REG> = crate::BitWriter1C<'a, REG, CTSCF>;
impl<'a, REG> CTSCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the CTSIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTSCF::Clear)
    }
}
/**Receiver timeout clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOCF {
    ///1: Clears the RTOF flag in the ISR register
    Clear = 1,
}
impl From<RTOCF> for bool {
    #[inline(always)]
    fn from(variant: RTOCF) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOCF` writer - Receiver timeout clear flag
pub type RTOCF_W<'a, REG> = crate::BitWriter1C<'a, REG, RTOCF>;
impl<'a, REG> RTOCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the RTOF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RTOCF::Clear)
    }
}
/**End of block clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBCF {
    ///1: Clears the EOBF flag in the ISR register
    Clear = 1,
}
impl From<EOBCF> for bool {
    #[inline(always)]
    fn from(variant: EOBCF) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBCF` writer - End of block clear flag
pub type EOBCF_W<'a, REG> = crate::BitWriter1C<'a, REG, EOBCF>;
impl<'a, REG> EOBCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the EOBF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOBCF::Clear)
    }
}
/**Character match clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMCF {
    ///1: Clears the CMF flag in the ISR register
    Clear = 1,
}
impl From<CMCF> for bool {
    #[inline(always)]
    fn from(variant: CMCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMCF` writer - Character match clear flag
pub type CMCF_W<'a, REG> = crate::BitWriter1C<'a, REG, CMCF>;
impl<'a, REG> CMCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the CMF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMCF::Clear)
    }
}
/**Wakeup from Stop mode clear flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCF {
    ///1: Clears the WUF flag in the ISR register
    Clear = 1,
}
impl From<WUCF> for bool {
    #[inline(always)]
    fn from(variant: WUCF) -> Self {
        variant as u8 != 0
    }
}
///Field `WUCF` writer - Wakeup from Stop mode clear flag
pub type WUCF_W<'a, REG> = crate::BitWriter1C<'a, REG, WUCF>;
impl<'a, REG> WUCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the WUF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WUCF::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<'_, ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<'_, ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W<'_, ICRrs> {
        NCF_W::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<'_, ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<'_, ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<'_, ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W<'_, ICRrs> {
        LBDCF_W::new(self, 8)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<'_, ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W<'_, ICRrs> {
        RTOCF_W::new(self, 11)
    }
    ///Bit 12 - End of block clear flag
    #[inline(always)]
    pub fn eobcf(&mut self) -> EOBCF_W<'_, ICRrs> {
        EOBCF_W::new(self, 12)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<'_, ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<'_, ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#USART1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0012_1b5f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
