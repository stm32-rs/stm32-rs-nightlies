///Register `ICR` reader
pub type R = crate::R<ICRrs>;
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
///Field `PECF` reader - Parity error clear flag
pub type PECF_R = crate::BitReader<PECF>;
impl PECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PECF> {
        match self.bits {
            true => Some(PECF::Clear),
            _ => None,
        }
    }
    ///Clears the PE flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PECF::Clear
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
///Field `FECF` reader - Framing error clear flag
pub type FECF_R = crate::BitReader<FECF>;
impl FECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FECF> {
        match self.bits {
            true => Some(FECF::Clear),
            _ => None,
        }
    }
    ///Clears the FE flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FECF::Clear
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
///Field `NCF` reader - Noise detected clear flag
pub type NCF_R = crate::BitReader<NCF>;
impl NCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<NCF> {
        match self.bits {
            true => Some(NCF::Clear),
            _ => None,
        }
    }
    ///Clears the NF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == NCF::Clear
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
///Field `ORECF` reader - Overrun error clear flag
pub type ORECF_R = crate::BitReader<ORECF>;
impl ORECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ORECF> {
        match self.bits {
            true => Some(ORECF::Clear),
            _ => None,
        }
    }
    ///Clears the ORE flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ORECF::Clear
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
///Field `IDLECF` reader - Idle line detected clear flag
pub type IDLECF_R = crate::BitReader<IDLECF>;
impl IDLECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDLECF> {
        match self.bits {
            true => Some(IDLECF::Clear),
            _ => None,
        }
    }
    ///Clears the IDLE flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IDLECF::Clear
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
///Field `TCCF` reader - Transmission complete clear flag
pub type TCCF_R = crate::BitReader<TCCF>;
impl TCCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCF> {
        match self.bits {
            true => Some(TCCF::Clear),
            _ => None,
        }
    }
    ///Clears the TC flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TCCF::Clear
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
///Field `LBDCF` reader - LIN break detection clear flag
pub type LBDCF_R = crate::BitReader<LBDCF>;
impl LBDCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LBDCF> {
        match self.bits {
            true => Some(LBDCF::Clear),
            _ => None,
        }
    }
    ///Clears the LBDF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LBDCF::Clear
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
///Field `CTSCF` reader - CTS clear flag
pub type CTSCF_R = crate::BitReader<CTSCF>;
impl CTSCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSCF> {
        match self.bits {
            true => Some(CTSCF::Clear),
            _ => None,
        }
    }
    ///Clears the CTSIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTSCF::Clear
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
///Field `RTOCF` reader - Receiver timeout clear flag
pub type RTOCF_R = crate::BitReader<RTOCF>;
impl RTOCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTOCF> {
        match self.bits {
            true => Some(RTOCF::Clear),
            _ => None,
        }
    }
    ///Clears the RTOF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RTOCF::Clear
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
/**End of timeout clear flag

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
///Field `EOBCF` reader - End of timeout clear flag
pub type EOBCF_R = crate::BitReader<EOBCF>;
impl EOBCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EOBCF> {
        match self.bits {
            true => Some(EOBCF::Clear),
            _ => None,
        }
    }
    ///Clears the EOBF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EOBCF::Clear
    }
}
///Field `EOBCF` writer - End of timeout clear flag
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
///Field `CMCF` reader - Character match clear flag
pub type CMCF_R = crate::BitReader<CMCF>;
impl CMCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMCF> {
        match self.bits {
            true => Some(CMCF::Clear),
            _ => None,
        }
    }
    ///Clears the CMF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMCF::Clear
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
///Field `WUCF` reader - Wakeup from Stop mode clear flag
pub type WUCF_R = crate::BitReader<WUCF>;
impl WUCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUCF> {
        match self.bits {
            true => Some(WUCF::Clear),
            _ => None,
        }
    }
    ///Clears the WUF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUCF::Clear
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
impl R {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&self) -> PECF_R {
        PECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&self) -> FECF_R {
        FECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&self) -> NCF_R {
        NCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&self) -> ORECF_R {
        ORECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&self) -> IDLECF_R {
        IDLECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&self) -> TCCF_R {
        TCCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&self) -> LBDCF_R {
        LBDCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&self) -> CTSCF_R {
        CTSCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&self) -> RTOCF_R {
        RTOCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - End of timeout clear flag
    #[inline(always)]
    pub fn eobcf(&self) -> EOBCF_R {
        EOBCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&self) -> CMCF_R {
        CMCF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    pub fn wucf(&self) -> WUCF_R {
        WUCF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("wucf", &self.wucf())
            .field("cmcf", &self.cmcf())
            .field("eobcf", &self.eobcf())
            .field("rtocf", &self.rtocf())
            .field("ctscf", &self.ctscf())
            .field("lbdcf", &self.lbdcf())
            .field("tccf", &self.tccf())
            .field("idlecf", &self.idlecf())
            .field("orecf", &self.orecf())
            .field("ncf", &self.ncf())
            .field("fecf", &self.fecf())
            .field("pecf", &self.pecf())
            .finish()
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
    ///Bit 12 - End of timeout clear flag
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

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#USART1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0012_1b5f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
