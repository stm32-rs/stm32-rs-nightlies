#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Parity error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECF {
    #[doc = "1: Clears the PE flag in the ISR register"]
    Clear = 1,
}
impl From<PECF> for bool {
    #[inline(always)]
    fn from(variant: PECF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECF` reader - Parity error clear flag"]
pub type PECF_R = crate::BitReader<PECF>;
impl PECF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PECF> {
        match self.bits {
            true => Some(PECF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the PE flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PECF::Clear
    }
}
#[doc = "Field `PECF` writer - Parity error clear flag"]
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG, PECF>;
impl<'a, REG> PECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the PE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECF::Clear)
    }
}
#[doc = "Framing error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FECF {
    #[doc = "1: Clears the FE flag in the ISR register"]
    Clear = 1,
}
impl From<FECF> for bool {
    #[inline(always)]
    fn from(variant: FECF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FECF` reader - Framing error clear flag"]
pub type FECF_R = crate::BitReader<FECF>;
impl FECF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FECF> {
        match self.bits {
            true => Some(FECF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the FE flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FECF::Clear
    }
}
#[doc = "Field `FECF` writer - Framing error clear flag"]
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG, FECF>;
impl<'a, REG> FECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the FE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FECF::Clear)
    }
}
#[doc = "Noise detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCF {
    #[doc = "1: Clears the NF flag in the ISR register"]
    Clear = 1,
}
impl From<NCF> for bool {
    #[inline(always)]
    fn from(variant: NCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCF` reader - Noise detected clear flag"]
pub type NCF_R = crate::BitReader<NCF>;
impl NCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NCF> {
        match self.bits {
            true => Some(NCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the NF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == NCF::Clear
    }
}
#[doc = "Field `NCF` writer - Noise detected clear flag"]
pub type NCF_W<'a, REG> = crate::BitWriter<'a, REG, NCF>;
impl<'a, REG> NCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the NF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NCF::Clear)
    }
}
#[doc = "Overrun error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORECF {
    #[doc = "1: Clears the ORE flag in the ISR register"]
    Clear = 1,
}
impl From<ORECF> for bool {
    #[inline(always)]
    fn from(variant: ORECF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORECF` reader - Overrun error clear flag"]
pub type ORECF_R = crate::BitReader<ORECF>;
impl ORECF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ORECF> {
        match self.bits {
            true => Some(ORECF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the ORE flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ORECF::Clear
    }
}
#[doc = "Field `ORECF` writer - Overrun error clear flag"]
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG, ORECF>;
impl<'a, REG> ORECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ORE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ORECF::Clear)
    }
}
#[doc = "Idle line detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLECF {
    #[doc = "1: Clears the IDLE flag in the ISR register"]
    Clear = 1,
}
impl From<IDLECF> for bool {
    #[inline(always)]
    fn from(variant: IDLECF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLECF` reader - Idle line detected clear flag"]
pub type IDLECF_R = crate::BitReader<IDLECF>;
impl IDLECF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDLECF> {
        match self.bits {
            true => Some(IDLECF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the IDLE flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IDLECF::Clear
    }
}
#[doc = "Field `IDLECF` writer - Idle line detected clear flag"]
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG, IDLECF>;
impl<'a, REG> IDLECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the IDLE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECF::Clear)
    }
}
#[doc = "Transmission complete clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCCF {
    #[doc = "1: Clears the TC flag in the ISR register"]
    Clear = 1,
}
impl From<TCCF> for bool {
    #[inline(always)]
    fn from(variant: TCCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCCF` reader - Transmission complete clear flag"]
pub type TCCF_R = crate::BitReader<TCCF>;
impl TCCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCF> {
        match self.bits {
            true => Some(TCCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the TC flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TCCF::Clear
    }
}
#[doc = "Field `TCCF` writer - Transmission complete clear flag"]
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG, TCCF>;
impl<'a, REG> TCCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the TC flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCCF::Clear)
    }
}
#[doc = "LIN break detection clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDCF {
    #[doc = "1: Clears the LBDF flag in the ISR register"]
    Clear = 1,
}
impl From<LBDCF> for bool {
    #[inline(always)]
    fn from(variant: LBDCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDCF` reader - LIN break detection clear flag"]
pub type LBDCF_R = crate::BitReader<LBDCF>;
impl LBDCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LBDCF> {
        match self.bits {
            true => Some(LBDCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the LBDF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LBDCF::Clear
    }
}
#[doc = "Field `LBDCF` writer - LIN break detection clear flag"]
pub type LBDCF_W<'a, REG> = crate::BitWriter<'a, REG, LBDCF>;
impl<'a, REG> LBDCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the LBDF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LBDCF::Clear)
    }
}
#[doc = "CTS clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCF {
    #[doc = "1: Clears the CTSIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTSCF> for bool {
    #[inline(always)]
    fn from(variant: CTSCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCF` reader - CTS clear flag"]
pub type CTSCF_R = crate::BitReader<CTSCF>;
impl CTSCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSCF> {
        match self.bits {
            true => Some(CTSCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the CTSIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTSCF::Clear
    }
}
#[doc = "Field `CTSCF` writer - CTS clear flag"]
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG, CTSCF>;
impl<'a, REG> CTSCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the CTSIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTSCF::Clear)
    }
}
#[doc = "Receiver timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOCF {
    #[doc = "1: Clears the RTOF flag in the ISR register"]
    Clear = 1,
}
impl From<RTOCF> for bool {
    #[inline(always)]
    fn from(variant: RTOCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOCF` reader - Receiver timeout clear flag"]
pub type RTOCF_R = crate::BitReader<RTOCF>;
impl RTOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTOCF> {
        match self.bits {
            true => Some(RTOCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the RTOF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RTOCF::Clear
    }
}
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag"]
pub type RTOCF_W<'a, REG> = crate::BitWriter<'a, REG, RTOCF>;
impl<'a, REG> RTOCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the RTOF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RTOCF::Clear)
    }
}
#[doc = "End of timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBCF {
    #[doc = "1: Clears the EOBF flag in the ISR register"]
    Clear = 1,
}
impl From<EOBCF> for bool {
    #[inline(always)]
    fn from(variant: EOBCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBCF` reader - End of timeout clear flag"]
pub type EOBCF_R = crate::BitReader<EOBCF>;
impl EOBCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EOBCF> {
        match self.bits {
            true => Some(EOBCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the EOBF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EOBCF::Clear
    }
}
#[doc = "Field `EOBCF` writer - End of timeout clear flag"]
pub type EOBCF_W<'a, REG> = crate::BitWriter<'a, REG, EOBCF>;
impl<'a, REG> EOBCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the EOBF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOBCF::Clear)
    }
}
#[doc = "Character match clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMCF {
    #[doc = "1: Clears the CMF flag in the ISR register"]
    Clear = 1,
}
impl From<CMCF> for bool {
    #[inline(always)]
    fn from(variant: CMCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMCF` reader - Character match clear flag"]
pub type CMCF_R = crate::BitReader<CMCF>;
impl CMCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMCF> {
        match self.bits {
            true => Some(CMCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the CMF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMCF::Clear
    }
}
#[doc = "Field `CMCF` writer - Character match clear flag"]
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG, CMCF>;
impl<'a, REG> CMCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the CMF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMCF::Clear)
    }
}
#[doc = "Wakeup from Stop mode clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCF {
    #[doc = "1: Clears the WUF flag in the ISR register"]
    Clear = 1,
}
impl From<WUCF> for bool {
    #[inline(always)]
    fn from(variant: WUCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUCF` reader - Wakeup from Stop mode clear flag"]
pub type WUCF_R = crate::BitReader<WUCF>;
impl WUCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUCF> {
        match self.bits {
            true => Some(WUCF::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the WUF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUCF::Clear
    }
}
#[doc = "Field `WUCF` writer - Wakeup from Stop mode clear flag"]
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG, WUCF>;
impl<'a, REG> WUCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the WUF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WUCF::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    pub fn pecf(&self) -> PECF_R {
        PECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    pub fn fecf(&self) -> FECF_R {
        FECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    pub fn ncf(&self) -> NCF_R {
        NCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    pub fn orecf(&self) -> ORECF_R {
        ORECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    pub fn idlecf(&self) -> IDLECF_R {
        IDLECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    pub fn tccf(&self) -> TCCF_R {
        TCCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    pub fn lbdcf(&self) -> LBDCF_R {
        LBDCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    pub fn ctscf(&self) -> CTSCF_R {
        CTSCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    pub fn rtocf(&self) -> RTOCF_R {
        RTOCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline(always)]
    pub fn eobcf(&self) -> EOBCF_R {
        EOBCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    pub fn cmcf(&self) -> CMCF_R {
        CMCF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    pub fn wucf(&self) -> WUCF_R {
        WUCF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PECF_W<ICRrs> {
        PECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FECF_W<ICRrs> {
        FECF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncf(&mut self) -> NCF_W<ICRrs> {
        NCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> ORECF_W<ICRrs> {
        ORECF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IDLECF_W<ICRrs> {
        IDLECF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TCCF_W<ICRrs> {
        TCCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LBDCF_W<ICRrs> {
        LBDCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<ICRrs> {
        CTSCF_W::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RTOCF_W<ICRrs> {
        RTOCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EOBCF_W<ICRrs> {
        EOBCF_W::new(self, 12)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CMCF_W<ICRrs> {
        CMCF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WUCF_W<ICRrs> {
        WUCF_W::new(self, 20)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
