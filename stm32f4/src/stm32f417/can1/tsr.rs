///Register `TSR` reader
pub type R = crate::R<TSRrs>;
///Register `TSR` writer
pub type W = crate::W<TSRrs>;
///Field `RQCP0` reader - RQCP0
pub type RQCP0_R = crate::BitReader;
///Field `RQCP0` writer - RQCP0
pub type RQCP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOK0` reader - TXOK0
pub type TXOK0_R = crate::BitReader;
///Field `TXOK0` writer - TXOK0
pub type TXOK0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALST0` reader - ALST0
pub type ALST0_R = crate::BitReader;
///Field `ALST0` writer - ALST0
pub type ALST0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERR0` reader - TERR0
pub type TERR0_R = crate::BitReader;
///Field `TERR0` writer - TERR0
pub type TERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABRQ0` reader - ABRQ0
pub type ABRQ0_R = crate::BitReader;
///Field `ABRQ0` writer - ABRQ0
pub type ABRQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RQCP1` reader - RQCP1
pub type RQCP1_R = crate::BitReader;
///Field `RQCP1` writer - RQCP1
pub type RQCP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOK1` reader - TXOK1
pub type TXOK1_R = crate::BitReader;
///Field `TXOK1` writer - TXOK1
pub type TXOK1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALST1` reader - ALST1
pub type ALST1_R = crate::BitReader;
///Field `ALST1` writer - ALST1
pub type ALST1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERR1` reader - TERR1
pub type TERR1_R = crate::BitReader;
///Field `TERR1` writer - TERR1
pub type TERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABRQ1` reader - ABRQ1
pub type ABRQ1_R = crate::BitReader;
///Field `ABRQ1` writer - ABRQ1
pub type ABRQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RQCP2` reader - RQCP2
pub type RQCP2_R = crate::BitReader;
///Field `RQCP2` writer - RQCP2
pub type RQCP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOK2` reader - TXOK2
pub type TXOK2_R = crate::BitReader;
///Field `TXOK2` writer - TXOK2
pub type TXOK2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALST2` reader - ALST2
pub type ALST2_R = crate::BitReader;
///Field `ALST2` writer - ALST2
pub type ALST2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERR2` reader - TERR2
pub type TERR2_R = crate::BitReader;
///Field `TERR2` writer - TERR2
pub type TERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABRQ2` reader - ABRQ2
pub type ABRQ2_R = crate::BitReader;
///Field `ABRQ2` writer - ABRQ2
pub type ABRQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CODE` reader - CODE
pub type CODE_R = crate::FieldReader;
///Field `TME0` reader - Lowest priority flag for mailbox 0
pub type TME0_R = crate::BitReader;
///Field `TME1` reader - Lowest priority flag for mailbox 1
pub type TME1_R = crate::BitReader;
///Field `TME2` reader - Lowest priority flag for mailbox 2
pub type TME2_R = crate::BitReader;
///Field `LOW0` reader - Lowest priority flag for mailbox 0
pub type LOW0_R = crate::BitReader;
///Field `LOW1` reader - Lowest priority flag for mailbox 1
pub type LOW1_R = crate::BitReader;
///Field `LOW2` reader - Lowest priority flag for mailbox 2
pub type LOW2_R = crate::BitReader;
impl R {
    ///Bit 0 - RQCP0
    #[inline(always)]
    pub fn rqcp0(&self) -> RQCP0_R {
        RQCP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXOK0
    #[inline(always)]
    pub fn txok0(&self) -> TXOK0_R {
        TXOK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ALST0
    #[inline(always)]
    pub fn alst0(&self) -> ALST0_R {
        ALST0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TERR0
    #[inline(always)]
    pub fn terr0(&self) -> TERR0_R {
        TERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - ABRQ0
    #[inline(always)]
    pub fn abrq0(&self) -> ABRQ0_R {
        ABRQ0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RQCP1
    #[inline(always)]
    pub fn rqcp1(&self) -> RQCP1_R {
        RQCP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TXOK1
    #[inline(always)]
    pub fn txok1(&self) -> TXOK1_R {
        TXOK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ALST1
    #[inline(always)]
    pub fn alst1(&self) -> ALST1_R {
        ALST1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TERR1
    #[inline(always)]
    pub fn terr1(&self) -> TERR1_R {
        TERR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - ABRQ1
    #[inline(always)]
    pub fn abrq1(&self) -> ABRQ1_R {
        ABRQ1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RQCP2
    #[inline(always)]
    pub fn rqcp2(&self) -> RQCP2_R {
        RQCP2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TXOK2
    #[inline(always)]
    pub fn txok2(&self) -> TXOK2_R {
        TXOK2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ALST2
    #[inline(always)]
    pub fn alst2(&self) -> ALST2_R {
        ALST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TERR2
    #[inline(always)]
    pub fn terr2(&self) -> TERR2_R {
        TERR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - ABRQ2
    #[inline(always)]
    pub fn abrq2(&self) -> ABRQ2_R {
        ABRQ2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - CODE
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Lowest priority flag for mailbox 0
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Lowest priority flag for mailbox 1
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Lowest priority flag for mailbox 2
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Lowest priority flag for mailbox 0
    #[inline(always)]
    pub fn low0(&self) -> LOW0_R {
        LOW0_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Lowest priority flag for mailbox 1
    #[inline(always)]
    pub fn low1(&self) -> LOW1_R {
        LOW1_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Lowest priority flag for mailbox 2
    #[inline(always)]
    pub fn low2(&self) -> LOW2_R {
        LOW2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSR")
            .field("low2", &self.low2())
            .field("low1", &self.low1())
            .field("low0", &self.low0())
            .field("tme2", &self.tme2())
            .field("tme1", &self.tme1())
            .field("tme0", &self.tme0())
            .field("code", &self.code())
            .field("abrq2", &self.abrq2())
            .field("terr2", &self.terr2())
            .field("alst2", &self.alst2())
            .field("txok2", &self.txok2())
            .field("rqcp2", &self.rqcp2())
            .field("abrq1", &self.abrq1())
            .field("terr1", &self.terr1())
            .field("alst1", &self.alst1())
            .field("txok1", &self.txok1())
            .field("rqcp1", &self.rqcp1())
            .field("abrq0", &self.abrq0())
            .field("terr0", &self.terr0())
            .field("alst0", &self.alst0())
            .field("txok0", &self.txok0())
            .field("rqcp0", &self.rqcp0())
            .finish()
    }
}
impl W {
    ///Bit 0 - RQCP0
    #[inline(always)]
    pub fn rqcp0(&mut self) -> RQCP0_W<'_, TSRrs> {
        RQCP0_W::new(self, 0)
    }
    ///Bit 1 - TXOK0
    #[inline(always)]
    pub fn txok0(&mut self) -> TXOK0_W<'_, TSRrs> {
        TXOK0_W::new(self, 1)
    }
    ///Bit 2 - ALST0
    #[inline(always)]
    pub fn alst0(&mut self) -> ALST0_W<'_, TSRrs> {
        ALST0_W::new(self, 2)
    }
    ///Bit 3 - TERR0
    #[inline(always)]
    pub fn terr0(&mut self) -> TERR0_W<'_, TSRrs> {
        TERR0_W::new(self, 3)
    }
    ///Bit 7 - ABRQ0
    #[inline(always)]
    pub fn abrq0(&mut self) -> ABRQ0_W<'_, TSRrs> {
        ABRQ0_W::new(self, 7)
    }
    ///Bit 8 - RQCP1
    #[inline(always)]
    pub fn rqcp1(&mut self) -> RQCP1_W<'_, TSRrs> {
        RQCP1_W::new(self, 8)
    }
    ///Bit 9 - TXOK1
    #[inline(always)]
    pub fn txok1(&mut self) -> TXOK1_W<'_, TSRrs> {
        TXOK1_W::new(self, 9)
    }
    ///Bit 10 - ALST1
    #[inline(always)]
    pub fn alst1(&mut self) -> ALST1_W<'_, TSRrs> {
        ALST1_W::new(self, 10)
    }
    ///Bit 11 - TERR1
    #[inline(always)]
    pub fn terr1(&mut self) -> TERR1_W<'_, TSRrs> {
        TERR1_W::new(self, 11)
    }
    ///Bit 15 - ABRQ1
    #[inline(always)]
    pub fn abrq1(&mut self) -> ABRQ1_W<'_, TSRrs> {
        ABRQ1_W::new(self, 15)
    }
    ///Bit 16 - RQCP2
    #[inline(always)]
    pub fn rqcp2(&mut self) -> RQCP2_W<'_, TSRrs> {
        RQCP2_W::new(self, 16)
    }
    ///Bit 17 - TXOK2
    #[inline(always)]
    pub fn txok2(&mut self) -> TXOK2_W<'_, TSRrs> {
        TXOK2_W::new(self, 17)
    }
    ///Bit 18 - ALST2
    #[inline(always)]
    pub fn alst2(&mut self) -> ALST2_W<'_, TSRrs> {
        ALST2_W::new(self, 18)
    }
    ///Bit 19 - TERR2
    #[inline(always)]
    pub fn terr2(&mut self) -> TERR2_W<'_, TSRrs> {
        TERR2_W::new(self, 19)
    }
    ///Bit 23 - ABRQ2
    #[inline(always)]
    pub fn abrq2(&mut self) -> ABRQ2_W<'_, TSRrs> {
        ABRQ2_W::new(self, 23)
    }
}
/**transmit status register

You can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#CAN1:TSR)*/
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
