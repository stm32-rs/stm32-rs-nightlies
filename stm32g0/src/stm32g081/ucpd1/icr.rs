#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `TXMSGDISCCF` reader - TXMSGDISCCF"]
pub type TXMSGDISCCF_R = crate::BitReader;
#[doc = "Field `TXMSGDISCCF` writer - TXMSGDISCCF"]
pub type TXMSGDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGSENTCF` reader - TXMSGSENTCF"]
pub type TXMSGSENTCF_R = crate::BitReader;
#[doc = "Field `TXMSGSENTCF` writer - TXMSGSENTCF"]
pub type TXMSGSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMSGABTCF` reader - TXMSGABTCF"]
pub type TXMSGABTCF_R = crate::BitReader;
#[doc = "Field `TXMSGABTCF` writer - TXMSGABTCF"]
pub type TXMSGABTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTDISCCF` reader - HRSTDISCCF"]
pub type HRSTDISCCF_R = crate::BitReader;
#[doc = "Field `HRSTDISCCF` writer - HRSTDISCCF"]
pub type HRSTDISCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSENTCF` reader - HRSTSENTCF"]
pub type HRSTSENTCF_R = crate::BitReader;
#[doc = "Field `HRSTSENTCF` writer - HRSTSENTCF"]
pub type HRSTSENTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDCF` reader - TXUNDCF"]
pub type TXUNDCF_R = crate::BitReader;
#[doc = "Field `TXUNDCF` writer - TXUNDCF"]
pub type TXUNDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORDDETCF` reader - RXORDDETCF"]
pub type RXORDDETCF_R = crate::BitReader;
#[doc = "Field `RXORDDETCF` writer - RXORDDETCF"]
pub type RXORDDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHRSTDETCF` reader - RXHRSTDETCF"]
pub type RXHRSTDETCF_R = crate::BitReader;
#[doc = "Field `RXHRSTDETCF` writer - RXHRSTDETCF"]
pub type RXHRSTDETCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRCF` reader - RXOVRCF"]
pub type RXOVRCF_R = crate::BitReader;
#[doc = "Field `RXOVRCF` writer - RXOVRCF"]
pub type RXOVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMSGENDCF` reader - RXMSGENDCF"]
pub type RXMSGENDCF_R = crate::BitReader;
#[doc = "Field `RXMSGENDCF` writer - RXMSGENDCF"]
pub type RXMSGENDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT1CF` reader - TYPECEVT1CF"]
pub type TYPECEVT1CF_R = crate::BitReader;
#[doc = "Field `TYPECEVT1CF` writer - TYPECEVT1CF"]
pub type TYPECEVT1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPECEVT2CF` reader - TYPECEVT2CF"]
pub type TYPECEVT2CF_R = crate::BitReader;
#[doc = "Field `TYPECEVT2CF` writer - TYPECEVT2CF"]
pub type TYPECEVT2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSEVTCF` reader - FRSEVTCF"]
pub type FRSEVTCF_R = crate::BitReader;
#[doc = "Field `FRSEVTCF` writer - FRSEVTCF"]
pub type FRSEVTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&self) -> TXMSGDISCCF_R {
        TXMSGDISCCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&self) -> TXMSGSENTCF_R {
        TXMSGSENTCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&self) -> TXMSGABTCF_R {
        TXMSGABTCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&self) -> HRSTDISCCF_R {
        HRSTDISCCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&self) -> HRSTSENTCF_R {
        HRSTSENTCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&self) -> TXUNDCF_R {
        TXUNDCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&self) -> RXORDDETCF_R {
        RXORDDETCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&self) -> RXHRSTDETCF_R {
        RXHRSTDETCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&self) -> RXOVRCF_R {
        RXOVRCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&self) -> RXMSGENDCF_R {
        RXMSGENDCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&self) -> TYPECEVT1CF_R {
        TYPECEVT1CF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&self) -> TYPECEVT2CF_R {
        TYPECEVT2CF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&self) -> FRSEVTCF_R {
        FRSEVTCF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<ICRrs> {
        TXMSGDISCCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<ICRrs> {
        TXMSGSENTCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<ICRrs> {
        TXMSGABTCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    #[must_use]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<ICRrs> {
        HRSTDISCCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    #[must_use]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<ICRrs> {
        HRSTSENTCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    #[must_use]
    pub fn txundcf(&mut self) -> TXUNDCF_W<ICRrs> {
        TXUNDCF_W::new(self, 6)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<ICRrs> {
        RXORDDETCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<ICRrs> {
        RXHRSTDETCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<ICRrs> {
        RXOVRCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<ICRrs> {
        RXMSGENDCF_W::new(self, 12)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<ICRrs> {
        TYPECEVT1CF_W::new(self, 14)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<ICRrs> {
        TYPECEVT2CF_W::new(self, 15)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    #[must_use]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<ICRrs> {
        FRSEVTCF_W::new(self, 20)
    }
}
#[doc = "UCPD Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
