#[doc = "Register `RQR` writer"]
pub type W = crate::W<RQRrs>;
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
impl W {
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
}
#[doc = "Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
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
