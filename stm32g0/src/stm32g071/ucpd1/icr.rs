///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**TXMSGDISCCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMSGDISCCFW {
    ///1: Clear flag in UCPD_SR
    Clear = 1,
}
impl From<TXMSGDISCCFW> for bool {
    #[inline(always)]
    fn from(variant: TXMSGDISCCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGDISCCF` reader - TXMSGDISCCF
pub type TXMSGDISCCF_R = crate::BitReader<TXMSGDISCCFW>;
impl TXMSGDISCCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXMSGDISCCFW> {
        match self.bits {
            true => Some(TXMSGDISCCFW::Clear),
            _ => None,
        }
    }
    ///Clear flag in UCPD_SR
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TXMSGDISCCFW::Clear
    }
}
///Field `TXMSGDISCCF` writer - TXMSGDISCCF
pub type TXMSGDISCCF_W<'a, REG> = crate::BitWriter<'a, REG, TXMSGDISCCFW>;
impl<'a, REG> TXMSGDISCCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag in UCPD_SR
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TXMSGDISCCFW::Clear)
    }
}
///Field `TXMSGSENTCF` reader - TXMSGSENTCF
pub use TXMSGDISCCF_R as TXMSGSENTCF_R;
///Field `TXMSGABTCF` reader - TXMSGABTCF
pub use TXMSGDISCCF_R as TXMSGABTCF_R;
///Field `HRSTDISCCF` reader - HRSTDISCCF
pub use TXMSGDISCCF_R as HRSTDISCCF_R;
///Field `HRSTSENTCF` reader - HRSTSENTCF
pub use TXMSGDISCCF_R as HRSTSENTCF_R;
///Field `TXUNDCF` reader - TXUNDCF
pub use TXMSGDISCCF_R as TXUNDCF_R;
///Field `RXORDDETCF` reader - RXORDDETCF
pub use TXMSGDISCCF_R as RXORDDETCF_R;
///Field `RXHRSTDETCF` reader - RXHRSTDETCF
pub use TXMSGDISCCF_R as RXHRSTDETCF_R;
///Field `RXOVRCF` reader - RXOVRCF
pub use TXMSGDISCCF_R as RXOVRCF_R;
///Field `RXMSGENDCF` reader - RXMSGENDCF
pub use TXMSGDISCCF_R as RXMSGENDCF_R;
///Field `TYPECEVT1CF` reader - TYPECEVT1CF
pub use TXMSGDISCCF_R as TYPECEVT1CF_R;
///Field `TYPECEVT2CF` reader - TYPECEVT2CF
pub use TXMSGDISCCF_R as TYPECEVT2CF_R;
///Field `FRSEVTCF` reader - FRSEVTCF
pub use TXMSGDISCCF_R as FRSEVTCF_R;
///Field `TXMSGSENTCF` writer - TXMSGSENTCF
pub use TXMSGDISCCF_W as TXMSGSENTCF_W;
///Field `TXMSGABTCF` writer - TXMSGABTCF
pub use TXMSGDISCCF_W as TXMSGABTCF_W;
///Field `HRSTDISCCF` writer - HRSTDISCCF
pub use TXMSGDISCCF_W as HRSTDISCCF_W;
///Field `HRSTSENTCF` writer - HRSTSENTCF
pub use TXMSGDISCCF_W as HRSTSENTCF_W;
///Field `TXUNDCF` writer - TXUNDCF
pub use TXMSGDISCCF_W as TXUNDCF_W;
///Field `RXORDDETCF` writer - RXORDDETCF
pub use TXMSGDISCCF_W as RXORDDETCF_W;
///Field `RXHRSTDETCF` writer - RXHRSTDETCF
pub use TXMSGDISCCF_W as RXHRSTDETCF_W;
///Field `RXOVRCF` writer - RXOVRCF
pub use TXMSGDISCCF_W as RXOVRCF_W;
///Field `RXMSGENDCF` writer - RXMSGENDCF
pub use TXMSGDISCCF_W as RXMSGENDCF_W;
///Field `TYPECEVT1CF` writer - TYPECEVT1CF
pub use TXMSGDISCCF_W as TYPECEVT1CF_W;
///Field `TYPECEVT2CF` writer - TYPECEVT2CF
pub use TXMSGDISCCF_W as TYPECEVT2CF_W;
///Field `FRSEVTCF` writer - FRSEVTCF
pub use TXMSGDISCCF_W as FRSEVTCF_W;
impl R {
    ///Bit 1 - TXMSGDISCCF
    #[inline(always)]
    pub fn txmsgdisccf(&self) -> TXMSGDISCCF_R {
        TXMSGDISCCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXMSGSENTCF
    #[inline(always)]
    pub fn txmsgsentcf(&self) -> TXMSGSENTCF_R {
        TXMSGSENTCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXMSGABTCF
    #[inline(always)]
    pub fn txmsgabtcf(&self) -> TXMSGABTCF_R {
        TXMSGABTCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRSTDISCCF
    #[inline(always)]
    pub fn hrstdisccf(&self) -> HRSTDISCCF_R {
        HRSTDISCCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRSTSENTCF
    #[inline(always)]
    pub fn hrstsentcf(&self) -> HRSTSENTCF_R {
        HRSTSENTCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXUNDCF
    #[inline(always)]
    pub fn txundcf(&self) -> TXUNDCF_R {
        TXUNDCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - RXORDDETCF
    #[inline(always)]
    pub fn rxorddetcf(&self) -> RXORDDETCF_R {
        RXORDDETCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RXHRSTDETCF
    #[inline(always)]
    pub fn rxhrstdetcf(&self) -> RXHRSTDETCF_R {
        RXHRSTDETCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RXOVRCF
    #[inline(always)]
    pub fn rxovrcf(&self) -> RXOVRCF_R {
        RXOVRCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RXMSGENDCF
    #[inline(always)]
    pub fn rxmsgendcf(&self) -> RXMSGENDCF_R {
        RXMSGENDCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TYPECEVT1CF
    #[inline(always)]
    pub fn typecevt1cf(&self) -> TYPECEVT1CF_R {
        TYPECEVT1CF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TYPECEVT2CF
    #[inline(always)]
    pub fn typecevt2cf(&self) -> TYPECEVT2CF_R {
        TYPECEVT2CF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - FRSEVTCF
    #[inline(always)]
    pub fn frsevtcf(&self) -> FRSEVTCF_R {
        FRSEVTCF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("txmsgdisccf", &self.txmsgdisccf())
            .field("txmsgsentcf", &self.txmsgsentcf())
            .field("txmsgabtcf", &self.txmsgabtcf())
            .field("hrstdisccf", &self.hrstdisccf())
            .field("hrstsentcf", &self.hrstsentcf())
            .field("txundcf", &self.txundcf())
            .field("rxorddetcf", &self.rxorddetcf())
            .field("rxhrstdetcf", &self.rxhrstdetcf())
            .field("rxovrcf", &self.rxovrcf())
            .field("rxmsgendcf", &self.rxmsgendcf())
            .field("typecevt1cf", &self.typecevt1cf())
            .field("typecevt2cf", &self.typecevt2cf())
            .field("frsevtcf", &self.frsevtcf())
            .finish()
    }
}
impl W {
    ///Bit 1 - TXMSGDISCCF
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<'_, ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENTCF
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<'_, ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABTCF
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<'_, ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISCCF
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<'_, ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENTCF
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<'_, ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    ///Bit 6 - TXUNDCF
    #[inline(always)]
    pub fn txundcf(&mut self) -> TXUNDCF_W<'_, ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    ///Bit 9 - RXORDDETCF
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<'_, ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDETCF
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<'_, ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    ///Bit 11 - RXOVRCF
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<'_, ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    ///Bit 12 - RXMSGENDCF
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<'_, ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    ///Bit 14 - TYPECEVT1CF
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<'_, ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2CF
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<'_, ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    ///Bit 20 - FRSEVTCF
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<'_, ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
/**UCPD Interrupt Clear Register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
