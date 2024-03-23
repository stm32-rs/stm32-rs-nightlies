#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `TXMSGDISCCF` writer - Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register."]
pub type TXMSGDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGSENTCF` writer - Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the UCPD_SR register."]
pub type TXMSGSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGABTCF` writer - Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the UCPD_SR register."]
pub type TXMSGABTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTDISCCF` writer - Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the UCPD_SR register."]
pub type HRSTDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSENTCF` writer - Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the UCPD_SR register."]
pub type HRSTSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDCF` writer - Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the UCPD_SR register."]
pub type TXUNDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORDDETCF` writer - Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the UCPD_SR register."]
pub type RXORDDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHRSTDETCF` writer - Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the UCPD_SR register."]
pub type RXHRSTDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRCF` writer - Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the UCPD_SR register."]
pub type RXOVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMSGENDCF` writer - Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the UCPD_SR register."]
pub type RXMSGENDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT1CF` writer - Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register"]
pub type TYPECEVT1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT2CF` writer - Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register"]
pub type TYPECEVT2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSEVTCF` writer - FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the UCPD_SR register."]
pub type FRSEVTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txundcf(&mut self) -> TXUNDCF_W<ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    #[doc = "Bit 9 - Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    #[doc = "Bit 14 - Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    #[doc = "Bit 20 - FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
#[doc = "UCPD interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
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
