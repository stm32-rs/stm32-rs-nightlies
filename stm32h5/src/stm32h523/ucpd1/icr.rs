///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `TXMSGDISCCF` writer - Tx message discard flag (TXMSGDISC) clear
pub type TXMSGDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGSENTCF` writer - Tx message send flag (TXMSGSENT) clear
pub type TXMSGSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGABTCF` writer - Tx message abort flag (TXMSGABT) clear
pub type TXMSGABTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTDISCCF` writer - Hard reset discard flag (HRSTDISC) clear
pub type HRSTDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTSENTCF` writer - Hard reset send flag (HRSTSENT) clear
pub type HRSTSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNDCF` writer - Tx underflow flag (TXUND) clear
pub type TXUNDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDDETCF` writer - Rx ordered set detect flag (RXORDDET) clear
pub type RXORDDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXHRSTDETCF` writer - Rx Hard Reset detect flag (RXHRSTDET) clear
pub type RXHRSTDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRCF` writer - Rx overflow flag (RXOVR) clear
pub type RXOVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMSGENDCF` writer - Rx message received flag (RXMSGEND) clear
pub type RXMSGENDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT1CF` writer - Type-C CC1 event flag (TYPECEVT1) clear
pub type TYPECEVT1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT2CF` writer - Type-C CC2 line event flag (TYPECEVT2) clear
pub type TYPECEVT2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSEVTCF` writer - FRS event flag (FRSEVT) clear
pub type FRSEVTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - Tx message discard flag (TXMSGDISC) clear
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<'_, ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    ///Bit 2 - Tx message send flag (TXMSGSENT) clear
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<'_, ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    ///Bit 3 - Tx message abort flag (TXMSGABT) clear
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<'_, ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    ///Bit 4 - Hard reset discard flag (HRSTDISC) clear
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<'_, ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    ///Bit 5 - Hard reset send flag (HRSTSENT) clear
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<'_, ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    ///Bit 6 - Tx underflow flag (TXUND) clear
    #[inline(always)]
    pub fn txundcf(&mut self) -> TXUNDCF_W<'_, ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    ///Bit 9 - Rx ordered set detect flag (RXORDDET) clear
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<'_, ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    ///Bit 10 - Rx Hard Reset detect flag (RXHRSTDET) clear
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<'_, ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    ///Bit 11 - Rx overflow flag (RXOVR) clear
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<'_, ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    ///Bit 12 - Rx message received flag (RXMSGEND) clear
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<'_, ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    ///Bit 14 - Type-C CC1 event flag (TYPECEVT1) clear
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<'_, ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    ///Bit 15 - Type-C CC2 line event flag (TYPECEVT2) clear
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<'_, ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    ///Bit 20 - FRS event flag (FRSEVT) clear
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<'_, ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
/**UCPD interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#UCPD1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
