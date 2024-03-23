#[doc = "Register `TSR` reader"]
pub type R = crate::R<TSRrs>;
#[doc = "Register `TSR` writer"]
pub type W = crate::W<TSRrs>;
#[doc = "Field `RQCP0` reader - RQCP0"]
pub type RQCP0_R = crate::BitReader;
#[doc = "Field `RQCP0` writer - RQCP0"]
pub type RQCP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK0` reader - TXOK0"]
pub type TXOK0_R = crate::BitReader;
#[doc = "Field `TXOK0` writer - TXOK0"]
pub type TXOK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALST0` reader - ALST0"]
pub type ALST0_R = crate::BitReader;
#[doc = "Field `ALST0` writer - ALST0"]
pub type ALST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR0` reader - TERR0"]
pub type TERR0_R = crate::BitReader;
#[doc = "Field `TERR0` writer - TERR0"]
pub type TERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ0` reader - ABRQ0"]
pub type ABRQ0_R = crate::BitReader;
#[doc = "Field `ABRQ0` writer - ABRQ0"]
pub type ABRQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RQCP1` reader - RQCP1"]
pub type RQCP1_R = crate::BitReader;
#[doc = "Field `RQCP1` writer - RQCP1"]
pub type RQCP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK1` reader - TXOK1"]
pub type TXOK1_R = crate::BitReader;
#[doc = "Field `TXOK1` writer - TXOK1"]
pub type TXOK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALST1` reader - ALST1"]
pub type ALST1_R = crate::BitReader;
#[doc = "Field `ALST1` writer - ALST1"]
pub type ALST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR1` reader - TERR1"]
pub type TERR1_R = crate::BitReader;
#[doc = "Field `TERR1` writer - TERR1"]
pub type TERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ1` reader - ABRQ1"]
pub type ABRQ1_R = crate::BitReader;
#[doc = "Field `ABRQ1` writer - ABRQ1"]
pub type ABRQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RQCP2` reader - RQCP2"]
pub type RQCP2_R = crate::BitReader;
#[doc = "Field `RQCP2` writer - RQCP2"]
pub type RQCP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK2` reader - TXOK2"]
pub type TXOK2_R = crate::BitReader;
#[doc = "Field `TXOK2` writer - TXOK2"]
pub type TXOK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALST2` reader - ALST2"]
pub type ALST2_R = crate::BitReader;
#[doc = "Field `ALST2` writer - ALST2"]
pub type ALST2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR2` reader - TERR2"]
pub type TERR2_R = crate::BitReader;
#[doc = "Field `TERR2` writer - TERR2"]
pub type TERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ2` reader - ABRQ2"]
pub type ABRQ2_R = crate::BitReader;
#[doc = "Field `ABRQ2` writer - ABRQ2"]
pub type ABRQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE` reader - CODE"]
pub type CODE_R = crate::FieldReader;
#[doc = "Field `TME(0-2)` reader - Lowest priority flag for mailbox %s"]
pub type TME_R = crate::BitReader;
#[doc = "Field `LOW(0-2)` reader - Lowest priority flag for mailbox %s"]
pub type LOW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RQCP0"]
    #[inline(always)]
    pub fn rqcp0(&self) -> RQCP0_R {
        RQCP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXOK0"]
    #[inline(always)]
    pub fn txok0(&self) -> TXOK0_R {
        TXOK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ALST0"]
    #[inline(always)]
    pub fn alst0(&self) -> ALST0_R {
        ALST0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TERR0"]
    #[inline(always)]
    pub fn terr0(&self) -> TERR0_R {
        TERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ABRQ0"]
    #[inline(always)]
    pub fn abrq0(&self) -> ABRQ0_R {
        ABRQ0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RQCP1"]
    #[inline(always)]
    pub fn rqcp1(&self) -> RQCP1_R {
        RQCP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXOK1"]
    #[inline(always)]
    pub fn txok1(&self) -> TXOK1_R {
        TXOK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ALST1"]
    #[inline(always)]
    pub fn alst1(&self) -> ALST1_R {
        ALST1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TERR1"]
    #[inline(always)]
    pub fn terr1(&self) -> TERR1_R {
        TERR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ABRQ1"]
    #[inline(always)]
    pub fn abrq1(&self) -> ABRQ1_R {
        ABRQ1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RQCP2"]
    #[inline(always)]
    pub fn rqcp2(&self) -> RQCP2_R {
        RQCP2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXOK2"]
    #[inline(always)]
    pub fn txok2(&self) -> TXOK2_R {
        TXOK2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALST2"]
    #[inline(always)]
    pub fn alst2(&self) -> ALST2_R {
        ALST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TERR2"]
    #[inline(always)]
    pub fn terr2(&self) -> TERR2_R {
        TERR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - ABRQ2"]
    #[inline(always)]
    pub fn abrq2(&self) -> ABRQ2_R {
        ABRQ2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - CODE"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Lowest priority flag for mailbox (0-2)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TME0` field"]
    #[inline(always)]
    pub fn tme(&self, n: u8) -> TME_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TME_R::new(((self.bits >> (n + 26)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Lowest priority flag for mailbox (0-2)"]
    #[inline(always)]
    pub fn tme_iter(&self) -> impl Iterator<Item = TME_R> + '_ {
        (0..3).map(move |n| TME_R::new(((self.bits >> (n + 26)) & 1) != 0))
    }
    #[doc = "Bit 26 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn tme0(&self) -> TME_R {
        TME_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn tme1(&self) -> TME_R {
        TME_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn tme2(&self) -> TME_R {
        TME_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Lowest priority flag for mailbox (0-2)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LOW0` field"]
    #[inline(always)]
    pub fn low(&self, n: u8) -> LOW_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        LOW_R::new(((self.bits >> (n + 29)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Lowest priority flag for mailbox (0-2)"]
    #[inline(always)]
    pub fn low_iter(&self) -> impl Iterator<Item = LOW_R> + '_ {
        (0..3).map(move |n| LOW_R::new(((self.bits >> (n + 29)) & 1) != 0))
    }
    #[doc = "Bit 29 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn low0(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn low1(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn low2(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RQCP0"]
    #[inline(always)]
    #[must_use]
    pub fn rqcp0(&mut self) -> RQCP0_W<TSRrs> {
        RQCP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXOK0"]
    #[inline(always)]
    #[must_use]
    pub fn txok0(&mut self) -> TXOK0_W<TSRrs> {
        TXOK0_W::new(self, 1)
    }
    #[doc = "Bit 2 - ALST0"]
    #[inline(always)]
    #[must_use]
    pub fn alst0(&mut self) -> ALST0_W<TSRrs> {
        ALST0_W::new(self, 2)
    }
    #[doc = "Bit 3 - TERR0"]
    #[inline(always)]
    #[must_use]
    pub fn terr0(&mut self) -> TERR0_W<TSRrs> {
        TERR0_W::new(self, 3)
    }
    #[doc = "Bit 7 - ABRQ0"]
    #[inline(always)]
    #[must_use]
    pub fn abrq0(&mut self) -> ABRQ0_W<TSRrs> {
        ABRQ0_W::new(self, 7)
    }
    #[doc = "Bit 8 - RQCP1"]
    #[inline(always)]
    #[must_use]
    pub fn rqcp1(&mut self) -> RQCP1_W<TSRrs> {
        RQCP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - TXOK1"]
    #[inline(always)]
    #[must_use]
    pub fn txok1(&mut self) -> TXOK1_W<TSRrs> {
        TXOK1_W::new(self, 9)
    }
    #[doc = "Bit 10 - ALST1"]
    #[inline(always)]
    #[must_use]
    pub fn alst1(&mut self) -> ALST1_W<TSRrs> {
        ALST1_W::new(self, 10)
    }
    #[doc = "Bit 11 - TERR1"]
    #[inline(always)]
    #[must_use]
    pub fn terr1(&mut self) -> TERR1_W<TSRrs> {
        TERR1_W::new(self, 11)
    }
    #[doc = "Bit 15 - ABRQ1"]
    #[inline(always)]
    #[must_use]
    pub fn abrq1(&mut self) -> ABRQ1_W<TSRrs> {
        ABRQ1_W::new(self, 15)
    }
    #[doc = "Bit 16 - RQCP2"]
    #[inline(always)]
    #[must_use]
    pub fn rqcp2(&mut self) -> RQCP2_W<TSRrs> {
        RQCP2_W::new(self, 16)
    }
    #[doc = "Bit 17 - TXOK2"]
    #[inline(always)]
    #[must_use]
    pub fn txok2(&mut self) -> TXOK2_W<TSRrs> {
        TXOK2_W::new(self, 17)
    }
    #[doc = "Bit 18 - ALST2"]
    #[inline(always)]
    #[must_use]
    pub fn alst2(&mut self) -> ALST2_W<TSRrs> {
        ALST2_W::new(self, 18)
    }
    #[doc = "Bit 19 - TERR2"]
    #[inline(always)]
    #[must_use]
    pub fn terr2(&mut self) -> TERR2_W<TSRrs> {
        TERR2_W::new(self, 19)
    }
    #[doc = "Bit 23 - ABRQ2"]
    #[inline(always)]
    #[must_use]
    pub fn abrq2(&mut self) -> ABRQ2_W<TSRrs> {
        ABRQ2_W::new(self, 23)
    }
}
#[doc = "CAN_TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSRrs;
impl crate::RegisterSpec for TSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TSRrs {}
#[doc = "`write(|w| ..)` method takes [`tsr::W`](W) writer structure"]
impl crate::Writable for TSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSRrs {
    const RESET_VALUE: u32 = 0;
}
