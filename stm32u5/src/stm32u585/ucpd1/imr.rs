///Register `IMR` reader
pub type R = crate::R<IMRrs>;
///Register `IMR` writer
pub type W = crate::W<IMRrs>;
/**TXIS interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXISIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<TXISIE> for bool {
    #[inline(always)]
    fn from(variant: TXISIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXISIE` reader - TXIS interrupt enable
pub type TXISIE_R = crate::BitReader<TXISIE>;
impl TXISIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXISIE {
        match self.bits {
            false => TXISIE::Disabled,
            true => TXISIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXISIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXISIE::Enabled
    }
}
///Field `TXISIE` writer - TXIS interrupt enable
pub type TXISIE_W<'a, REG> = crate::BitWriter<'a, REG, TXISIE>;
impl<'a, REG> TXISIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXISIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXISIE::Enabled)
    }
}
///Field `TXMSGDISCIE` reader - TXMSGDISC interrupt enable
pub use TXISIE_R as TXMSGDISCIE_R;
///Field `TXMSGSENTIE` reader - TXMSGSENT interrupt enable
pub use TXISIE_R as TXMSGSENTIE_R;
///Field `TXMSGABTIE` reader - TXMSGABT interrupt enable
pub use TXISIE_R as TXMSGABTIE_R;
///Field `HRSTDISCIE` reader - HRSTDISC interrupt enable
pub use TXISIE_R as HRSTDISCIE_R;
///Field `HRSTSENTIE` reader - HRSTSENT interrupt enable
pub use TXISIE_R as HRSTSENTIE_R;
///Field `TXUNDIE` reader - TXUND interrupt enable
pub use TXISIE_R as TXUNDIE_R;
///Field `RXNEIE` reader - RXNE interrupt enable
pub use TXISIE_R as RXNEIE_R;
///Field `RXORDDETIE` reader - RXORDDET interrupt enable
pub use TXISIE_R as RXORDDETIE_R;
///Field `RXHRSTDETIE` reader - RXHRSTDET interrupt enable
pub use TXISIE_R as RXHRSTDETIE_R;
///Field `RXOVRIE` reader - RXOVR interrupt enable
pub use TXISIE_R as RXOVRIE_R;
///Field `RXMSGENDIE` reader - RXMSGEND interrupt enable
pub use TXISIE_R as RXMSGENDIE_R;
///Field `TYPECEVT1IE` reader - TYPECEVT1 interrupt enable
pub use TXISIE_R as TYPECEVT1IE_R;
///Field `TYPECEVT2IE` reader - TYPECEVT2 interrupt enable
pub use TXISIE_R as TYPECEVT2IE_R;
///Field `FRSEVTIE` reader - FRSEVT interrupt enable
pub use TXISIE_R as FRSEVTIE_R;
///Field `TXMSGDISCIE` writer - TXMSGDISC interrupt enable
pub use TXISIE_W as TXMSGDISCIE_W;
///Field `TXMSGSENTIE` writer - TXMSGSENT interrupt enable
pub use TXISIE_W as TXMSGSENTIE_W;
///Field `TXMSGABTIE` writer - TXMSGABT interrupt enable
pub use TXISIE_W as TXMSGABTIE_W;
///Field `HRSTDISCIE` writer - HRSTDISC interrupt enable
pub use TXISIE_W as HRSTDISCIE_W;
///Field `HRSTSENTIE` writer - HRSTSENT interrupt enable
pub use TXISIE_W as HRSTSENTIE_W;
///Field `TXUNDIE` writer - TXUND interrupt enable
pub use TXISIE_W as TXUNDIE_W;
///Field `RXNEIE` writer - RXNE interrupt enable
pub use TXISIE_W as RXNEIE_W;
///Field `RXORDDETIE` writer - RXORDDET interrupt enable
pub use TXISIE_W as RXORDDETIE_W;
///Field `RXHRSTDETIE` writer - RXHRSTDET interrupt enable
pub use TXISIE_W as RXHRSTDETIE_W;
///Field `RXOVRIE` writer - RXOVR interrupt enable
pub use TXISIE_W as RXOVRIE_W;
///Field `RXMSGENDIE` writer - RXMSGEND interrupt enable
pub use TXISIE_W as RXMSGENDIE_W;
///Field `TYPECEVT1IE` writer - TYPECEVT1 interrupt enable
pub use TXISIE_W as TYPECEVT1IE_W;
///Field `TYPECEVT2IE` writer - TYPECEVT2 interrupt enable
pub use TXISIE_W as TYPECEVT2IE_W;
impl R {
    ///Bit 0 - TXIS interrupt enable
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXMSGDISC interrupt enable
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXMSGSENT interrupt enable
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXMSGABT interrupt enable
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRSTDISC interrupt enable
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRSTSENT interrupt enable
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXUND interrupt enable
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXORDDET interrupt enable
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RXHRSTDET interrupt enable
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RXOVR interrupt enable
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RXMSGEND interrupt enable
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TYPECEVT1 interrupt enable
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TYPECEVT2 interrupt enable
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - FRSEVT interrupt enable
    #[inline(always)]
    pub fn frsevtie(&self) -> FRSEVTIE_R {
        FRSEVTIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("txisie", &self.txisie())
            .field("txmsgdiscie", &self.txmsgdiscie())
            .field("txmsgsentie", &self.txmsgsentie())
            .field("txmsgabtie", &self.txmsgabtie())
            .field("hrstdiscie", &self.hrstdiscie())
            .field("hrstsentie", &self.hrstsentie())
            .field("txundie", &self.txundie())
            .field("rxneie", &self.rxneie())
            .field("rxorddetie", &self.rxorddetie())
            .field("rxhrstdetie", &self.rxhrstdetie())
            .field("rxovrie", &self.rxovrie())
            .field("rxmsgendie", &self.rxmsgendie())
            .field("typecevt1ie", &self.typecevt1ie())
            .field("typecevt2ie", &self.typecevt2ie())
            .field("frsevtie", &self.frsevtie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXIS interrupt enable
    #[inline(always)]
    pub fn txisie(&mut self) -> TXISIE_W<IMRrs> {
        TXISIE_W::new(self, 0)
    }
    ///Bit 1 - TXMSGDISC interrupt enable
    #[inline(always)]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W<IMRrs> {
        TXMSGDISCIE_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENT interrupt enable
    #[inline(always)]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W<IMRrs> {
        TXMSGSENTIE_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABT interrupt enable
    #[inline(always)]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W<IMRrs> {
        TXMSGABTIE_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISC interrupt enable
    #[inline(always)]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W<IMRrs> {
        HRSTDISCIE_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENT interrupt enable
    #[inline(always)]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W<IMRrs> {
        HRSTSENTIE_W::new(self, 5)
    }
    ///Bit 6 - TXUND interrupt enable
    #[inline(always)]
    pub fn txundie(&mut self) -> TXUNDIE_W<IMRrs> {
        TXUNDIE_W::new(self, 6)
    }
    ///Bit 8 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<IMRrs> {
        RXNEIE_W::new(self, 8)
    }
    ///Bit 9 - RXORDDET interrupt enable
    #[inline(always)]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W<IMRrs> {
        RXORDDETIE_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDET interrupt enable
    #[inline(always)]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W<IMRrs> {
        RXHRSTDETIE_W::new(self, 10)
    }
    ///Bit 11 - RXOVR interrupt enable
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<IMRrs> {
        RXOVRIE_W::new(self, 11)
    }
    ///Bit 12 - RXMSGEND interrupt enable
    #[inline(always)]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W<IMRrs> {
        RXMSGENDIE_W::new(self, 12)
    }
    ///Bit 14 - TYPECEVT1 interrupt enable
    #[inline(always)]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W<IMRrs> {
        TYPECEVT1IE_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2 interrupt enable
    #[inline(always)]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W<IMRrs> {
        TYPECEVT2IE_W::new(self, 15)
    }
}
/**UCPD interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#UCPD1:IMR)*/
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
///`read()` method returns [`imr::R`](R) reader structure
impl crate::Readable for IMRrs {}
///`write(|w| ..)` method takes [`imr::W`](W) writer structure
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMRrs {
    const RESET_VALUE: u32 = 0;
}