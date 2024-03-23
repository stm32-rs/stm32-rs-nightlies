#[doc = "Register `RQR` reader"]
pub type R = crate::R<RQRrs>;
#[doc = "Register `RQR` writer"]
pub type W = crate::W<RQRrs>;
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABRRQ {
    #[doc = "1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    Request = 1,
}
impl From<ABRRQ> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRRQ` reader - Auto baud rate request"]
pub type ABRRQ_R = crate::BitReader<ABRRQ>;
impl ABRRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ABRRQ> {
        match self.bits {
            true => Some(ABRRQ::Request),
            _ => None,
        }
    }
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == ABRRQ::Request
    }
}
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub type ABRRQ_W<'a, REG> = crate::BitWriter<'a, REG, ABRRQ>;
impl<'a, REG> ABRRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut crate::W<REG> {
        self.variant(ABRRQ::Request)
    }
}
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKRQ {
    #[doc = "1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    Break = 1,
}
impl From<SBKRQ> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKRQ` reader - Send break request"]
pub type SBKRQ_R = crate::BitReader<SBKRQ>;
impl SBKRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SBKRQ> {
        match self.bits {
            true => Some(SBKRQ::Break),
            _ => None,
        }
    }
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBKRQ::Break
    }
}
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG, SBKRQ>;
impl<'a, REG> SBKRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(SBKRQ::Break)
    }
}
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMRQ {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    Mute = 1,
}
impl From<MMRQ> for bool {
    #[inline(always)]
    fn from(variant: MMRQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMRQ` reader - Mute mode request"]
pub type MMRQ_R = crate::BitReader<MMRQ>;
impl MMRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MMRQ> {
        match self.bits {
            true => Some(MMRQ::Mute),
            _ => None,
        }
    }
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MMRQ::Mute
    }
}
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG, MMRQ>;
impl<'a, REG> MMRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(MMRQ::Mute)
    }
}
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRQ {
    #[doc = "1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    Discard = 1,
}
impl From<RXFRQ> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRQ` reader - Receive data flush request"]
pub type RXFRQ_R = crate::BitReader<RXFRQ>;
impl RXFRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXFRQ> {
        match self.bits {
            true => Some(RXFRQ::Discard),
            _ => None,
        }
    }
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == RXFRQ::Discard
    }
}
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG, RXFRQ>;
impl<'a, REG> RXFRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(RXFRQ::Discard)
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRQ {
    #[doc = "1: Set the TXE flags. This allows to discard the transmit data"]
    Discard = 1,
}
impl From<TXFRQ> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRQ` reader - Transmit data flush request"]
pub type TXFRQ_R = crate::BitReader<TXFRQ>;
impl TXFRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXFRQ> {
        match self.bits {
            true => Some(TXFRQ::Discard),
            _ => None,
        }
    }
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == TXFRQ::Discard
    }
}
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG, TXFRQ>;
impl<'a, REG> TXFRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(TXFRQ::Discard)
    }
}
impl R {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&self) -> ABRRQ_R {
        ABRRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&self) -> SBKRQ_R {
        SBKRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&self) -> MMRQ_R {
        MMRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&self) -> RXFRQ_R {
        RXFRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&self) -> TXFRQ_R {
        TXFRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    #[must_use]
    pub fn abrrq(&mut self) -> ABRRQ_W<RQRrs> {
        ABRRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    #[must_use]
    pub fn sbkrq(&mut self) -> SBKRQ_W<RQRrs> {
        SBKRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mmrq(&mut self) -> MMRQ_W<RQRrs> {
        MMRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrq(&mut self) -> RXFRQ_W<RQRrs> {
        RXFRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn txfrq(&mut self) -> TXFRQ_W<RQRrs> {
        TXFRQ_W::new(self, 4)
    }
}
#[doc = "request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rqr::R`](R) reader structure"]
impl crate::Readable for RQRrs {}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQRrs {
    const RESET_VALUE: u32 = 0;
}
