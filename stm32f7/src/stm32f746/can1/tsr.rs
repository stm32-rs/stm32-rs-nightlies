///Register `TSR` reader
pub type R = crate::R<TSRrs>;
///Register `TSR` writer
pub type W = crate::W<TSRrs>;
///Field `RQCP(0-2)` reader - RQCP%s
pub type RQCP_R = crate::BitReader;
///Field `RQCP(0-2)` writer - RQCP%s
pub type RQCP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOK(0-2)` reader - TXOK%s
pub type TXOK_R = crate::BitReader;
///Field `TXOK(0-2)` writer - TXOK%s
pub type TXOK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALST(0-2)` reader - ALST%s
pub type ALST_R = crate::BitReader;
///Field `ALST(0-2)` writer - ALST%s
pub type ALST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERR(0-2)` reader - TERR%s
pub type TERR_R = crate::BitReader;
///Field `TERR(0-2)` writer - TERR%s
pub type TERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABRQ(0-2)` reader - ABRQ%s
pub type ABRQ_R = crate::BitReader;
///Field `ABRQ(0-2)` writer - ABRQ%s
pub type ABRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CODE` reader - CODE
pub type CODE_R = crate::FieldReader;
///Field `TME(0-2)` reader - Lowest priority flag for mailbox %s
pub type TME_R = crate::BitReader;
///Field `LOW(0-2)` reader - Lowest priority flag for mailbox %s
pub type LOW_R = crate::BitReader;
impl R {
    ///RQCP(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `RQCP0` field.</div>
    #[inline(always)]
    pub fn rqcp(&self, n: u8) -> RQCP_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        RQCP_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///RQCP(0-2)
    #[inline(always)]
    pub fn rqcp_iter(&self) -> impl Iterator<Item = RQCP_R> + '_ {
        (0..3).map(move |n| RQCP_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    ///Bit 0 - RQCP0
    #[inline(always)]
    pub fn rqcp0(&self) -> RQCP_R {
        RQCP_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - RQCP1
    #[inline(always)]
    pub fn rqcp1(&self) -> RQCP_R {
        RQCP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - RQCP2
    #[inline(always)]
    pub fn rqcp2(&self) -> RQCP_R {
        RQCP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///TXOK(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TXOK0` field.</div>
    #[inline(always)]
    pub fn txok(&self, n: u8) -> TXOK_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TXOK_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///TXOK(0-2)
    #[inline(always)]
    pub fn txok_iter(&self) -> impl Iterator<Item = TXOK_R> + '_ {
        (0..3).map(move |n| TXOK_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0))
    }
    ///Bit 1 - TXOK0
    #[inline(always)]
    pub fn txok0(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - TXOK1
    #[inline(always)]
    pub fn txok1(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - TXOK2
    #[inline(always)]
    pub fn txok2(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///ALST(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALST0` field.</div>
    #[inline(always)]
    pub fn alst(&self, n: u8) -> ALST_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        ALST_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///ALST(0-2)
    #[inline(always)]
    pub fn alst_iter(&self) -> impl Iterator<Item = ALST_R> + '_ {
        (0..3).map(move |n| ALST_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    ///Bit 2 - ALST0
    #[inline(always)]
    pub fn alst0(&self) -> ALST_R {
        ALST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - ALST1
    #[inline(always)]
    pub fn alst1(&self) -> ALST_R {
        ALST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 18 - ALST2
    #[inline(always)]
    pub fn alst2(&self) -> ALST_R {
        ALST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///TERR(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TERR0` field.</div>
    #[inline(always)]
    pub fn terr(&self, n: u8) -> TERR_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TERR_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///TERR(0-2)
    #[inline(always)]
    pub fn terr_iter(&self) -> impl Iterator<Item = TERR_R> + '_ {
        (0..3).map(move |n| TERR_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    ///Bit 3 - TERR0
    #[inline(always)]
    pub fn terr0(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - TERR1
    #[inline(always)]
    pub fn terr1(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 19 - TERR2
    #[inline(always)]
    pub fn terr2(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///ABRQ(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ABRQ0` field.</div>
    #[inline(always)]
    pub fn abrq(&self, n: u8) -> ABRQ_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        ABRQ_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///ABRQ(0-2)
    #[inline(always)]
    pub fn abrq_iter(&self) -> impl Iterator<Item = ABRQ_R> + '_ {
        (0..3).map(move |n| ABRQ_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    ///Bit 7 - ABRQ0
    #[inline(always)]
    pub fn abrq0(&self) -> ABRQ_R {
        ABRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - ABRQ1
    #[inline(always)]
    pub fn abrq1(&self) -> ABRQ_R {
        ABRQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 23 - ABRQ2
    #[inline(always)]
    pub fn abrq2(&self) -> ABRQ_R {
        ABRQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - CODE
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Lowest priority flag for mailbox (0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TME0` field.</div>
    #[inline(always)]
    pub fn tme(&self, n: u8) -> TME_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TME_R::new(((self.bits >> (n + 26)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Lowest priority flag for mailbox (0-2)
    #[inline(always)]
    pub fn tme_iter(&self) -> impl Iterator<Item = TME_R> + '_ {
        (0..3).map(move |n| TME_R::new(((self.bits >> (n + 26)) & 1) != 0))
    }
    ///Bit 26 - Lowest priority flag for mailbox 0
    #[inline(always)]
    pub fn tme0(&self) -> TME_R {
        TME_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Lowest priority flag for mailbox 1
    #[inline(always)]
    pub fn tme1(&self) -> TME_R {
        TME_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Lowest priority flag for mailbox 2
    #[inline(always)]
    pub fn tme2(&self) -> TME_R {
        TME_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Lowest priority flag for mailbox (0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOW0` field.</div>
    #[inline(always)]
    pub fn low(&self, n: u8) -> LOW_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        LOW_R::new(((self.bits >> (n + 29)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Lowest priority flag for mailbox (0-2)
    #[inline(always)]
    pub fn low_iter(&self) -> impl Iterator<Item = LOW_R> + '_ {
        (0..3).map(move |n| LOW_R::new(((self.bits >> (n + 29)) & 1) != 0))
    }
    ///Bit 29 - Lowest priority flag for mailbox 0
    #[inline(always)]
    pub fn low0(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Lowest priority flag for mailbox 1
    #[inline(always)]
    pub fn low1(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Lowest priority flag for mailbox 2
    #[inline(always)]
    pub fn low2(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSR")
            .field("low0", &self.low0())
            .field("low1", &self.low1())
            .field("low2", &self.low2())
            .field("tme0", &self.tme0())
            .field("tme1", &self.tme1())
            .field("tme2", &self.tme2())
            .field("code", &self.code())
            .field("abrq0", &self.abrq0())
            .field("abrq1", &self.abrq1())
            .field("abrq2", &self.abrq2())
            .field("terr0", &self.terr0())
            .field("terr1", &self.terr1())
            .field("terr2", &self.terr2())
            .field("alst0", &self.alst0())
            .field("alst1", &self.alst1())
            .field("alst2", &self.alst2())
            .field("txok0", &self.txok0())
            .field("txok1", &self.txok1())
            .field("txok2", &self.txok2())
            .field("rqcp0", &self.rqcp0())
            .field("rqcp1", &self.rqcp1())
            .field("rqcp2", &self.rqcp2())
            .finish()
    }
}
impl W {
    ///RQCP(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `RQCP0` field.</div>
    #[inline(always)]
    pub fn rqcp(&mut self, n: u8) -> RQCP_W<'_, TSRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        RQCP_W::new(self, n * 8)
    }
    ///Bit 0 - RQCP0
    #[inline(always)]
    pub fn rqcp0(&mut self) -> RQCP_W<'_, TSRrs> {
        RQCP_W::new(self, 0)
    }
    ///Bit 8 - RQCP1
    #[inline(always)]
    pub fn rqcp1(&mut self) -> RQCP_W<'_, TSRrs> {
        RQCP_W::new(self, 8)
    }
    ///Bit 16 - RQCP2
    #[inline(always)]
    pub fn rqcp2(&mut self) -> RQCP_W<'_, TSRrs> {
        RQCP_W::new(self, 16)
    }
    ///TXOK(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TXOK0` field.</div>
    #[inline(always)]
    pub fn txok(&mut self, n: u8) -> TXOK_W<'_, TSRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TXOK_W::new(self, n * 8 + 1)
    }
    ///Bit 1 - TXOK0
    #[inline(always)]
    pub fn txok0(&mut self) -> TXOK_W<'_, TSRrs> {
        TXOK_W::new(self, 1)
    }
    ///Bit 9 - TXOK1
    #[inline(always)]
    pub fn txok1(&mut self) -> TXOK_W<'_, TSRrs> {
        TXOK_W::new(self, 9)
    }
    ///Bit 17 - TXOK2
    #[inline(always)]
    pub fn txok2(&mut self) -> TXOK_W<'_, TSRrs> {
        TXOK_W::new(self, 17)
    }
    ///ALST(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALST0` field.</div>
    #[inline(always)]
    pub fn alst(&mut self, n: u8) -> ALST_W<'_, TSRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        ALST_W::new(self, n * 8 + 2)
    }
    ///Bit 2 - ALST0
    #[inline(always)]
    pub fn alst0(&mut self) -> ALST_W<'_, TSRrs> {
        ALST_W::new(self, 2)
    }
    ///Bit 10 - ALST1
    #[inline(always)]
    pub fn alst1(&mut self) -> ALST_W<'_, TSRrs> {
        ALST_W::new(self, 10)
    }
    ///Bit 18 - ALST2
    #[inline(always)]
    pub fn alst2(&mut self) -> ALST_W<'_, TSRrs> {
        ALST_W::new(self, 18)
    }
    ///TERR(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TERR0` field.</div>
    #[inline(always)]
    pub fn terr(&mut self, n: u8) -> TERR_W<'_, TSRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TERR_W::new(self, n * 8 + 3)
    }
    ///Bit 3 - TERR0
    #[inline(always)]
    pub fn terr0(&mut self) -> TERR_W<'_, TSRrs> {
        TERR_W::new(self, 3)
    }
    ///Bit 11 - TERR1
    #[inline(always)]
    pub fn terr1(&mut self) -> TERR_W<'_, TSRrs> {
        TERR_W::new(self, 11)
    }
    ///Bit 19 - TERR2
    #[inline(always)]
    pub fn terr2(&mut self) -> TERR_W<'_, TSRrs> {
        TERR_W::new(self, 19)
    }
    ///ABRQ(0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ABRQ0` field.</div>
    #[inline(always)]
    pub fn abrq(&mut self, n: u8) -> ABRQ_W<'_, TSRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        ABRQ_W::new(self, n * 8 + 7)
    }
    ///Bit 7 - ABRQ0
    #[inline(always)]
    pub fn abrq0(&mut self) -> ABRQ_W<'_, TSRrs> {
        ABRQ_W::new(self, 7)
    }
    ///Bit 15 - ABRQ1
    #[inline(always)]
    pub fn abrq1(&mut self) -> ABRQ_W<'_, TSRrs> {
        ABRQ_W::new(self, 15)
    }
    ///Bit 23 - ABRQ2
    #[inline(always)]
    pub fn abrq2(&mut self) -> ABRQ_W<'_, TSRrs> {
        ABRQ_W::new(self, 23)
    }
}
/**transmit status register

You can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#CAN1:TSR)*/
pub struct TSRrs;
impl crate::RegisterSpec for TSRrs {
    type Ux = u32;
}
///`read()` method returns [`tsr::R`](R) reader structure
impl crate::Readable for TSRrs {}
///`write(|w| ..)` method takes [`tsr::W`](W) writer structure
impl crate::Writable for TSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSR to value 0x1c00_0000
impl crate::Resettable for TSRrs {
    const RESET_VALUE: u32 = 0x1c00_0000;
}
