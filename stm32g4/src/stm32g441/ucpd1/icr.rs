///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `TXMSGDISCCF` reader - TXMSGDISCCF
pub type TXMSGDISCCF_R = crate::BitReader;
///Field `TXMSGDISCCF` writer - TXMSGDISCCF
pub type TXMSGDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGSENTCF` reader - TXMSGSENTCF
pub type TXMSGSENTCF_R = crate::BitReader;
///Field `TXMSGSENTCF` writer - TXMSGSENTCF
pub type TXMSGSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGABTCF` reader - TXMSGABTCF
pub type TXMSGABTCF_R = crate::BitReader;
///Field `TXMSGABTCF` writer - TXMSGABTCF
pub type TXMSGABTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTDISCCF` reader - HRSTDISCCF
pub type HRSTDISCCF_R = crate::BitReader;
///Field `HRSTDISCCF` writer - HRSTDISCCF
pub type HRSTDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTSENTCF` reader - HRSTSENTCF
pub type HRSTSENTCF_R = crate::BitReader;
///Field `HRSTSENTCF` writer - HRSTSENTCF
pub type HRSTSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNDCF` reader - TXUNDCF
pub type TXUNDCF_R = crate::BitReader;
///Field `TXUNDCF` writer - TXUNDCF
pub type TXUNDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDDETCF` reader - RXORDDETCF
pub type RXORDDETCF_R = crate::BitReader;
///Field `RXORDDETCF` writer - RXORDDETCF
pub type RXORDDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXHRSTDETCF` reader - RXHRSTDETCF
pub type RXHRSTDETCF_R = crate::BitReader;
///Field `RXHRSTDETCF` writer - RXHRSTDETCF
pub type RXHRSTDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRCF` reader - RXOVRCF
pub type RXOVRCF_R = crate::BitReader;
///Field `RXOVRCF` writer - RXOVRCF
pub type RXOVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMSGENDCF` reader - RXMSGENDCF
pub type RXMSGENDCF_R = crate::BitReader;
///Field `RXMSGENDCF` writer - RXMSGENDCF
pub type RXMSGENDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT1CF` reader - TYPECEVT1CF
pub type TYPECEVT1CF_R = crate::BitReader;
///Field `TYPECEVT1CF` writer - TYPECEVT1CF
pub type TYPECEVT1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT2CF` reader - TYPECEVT2CF
pub type TYPECEVT2CF_R = crate::BitReader;
///Field `TYPECEVT2CF` writer - TYPECEVT2CF
pub type TYPECEVT2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSEVTCF` reader - FRSEVTCF
pub type FRSEVTCF_R = crate::BitReader;
///Field `FRSEVTCF` writer - FRSEVTCF
pub type FRSEVTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENTCF
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABTCF
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISCCF
    #[inline(always)]
    #[must_use]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENTCF
    #[inline(always)]
    #[must_use]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    ///Bit 6 - TXUNDCF
    #[inline(always)]
    #[must_use]
    pub fn txundcf(&mut self) -> TXUNDCF_W<ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    ///Bit 9 - RXORDDETCF
    #[inline(always)]
    #[must_use]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDETCF
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    ///Bit 11 - RXOVRCF
    #[inline(always)]
    #[must_use]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    ///Bit 12 - RXMSGENDCF
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    ///Bit 14 - TYPECEVT1CF
    #[inline(always)]
    #[must_use]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2CF
    #[inline(always)]
    #[must_use]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    ///Bit 20 - FRSEVTCF
    #[inline(always)]
    #[must_use]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
/**UCPD Interrupt Clear Register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441xx.html#UCPD1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
