///Register `AHB5FZR` reader
pub type R = crate::R<AHB5FZRrs>;
///Register `AHB5FZR` writer
pub type W = crate::W<AHB5FZRrs>;
///Field `DBG_HPDMA_0_STOP` reader - HPDMA channel 0 stop in debug
pub type DBG_HPDMA_0_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_0_STOP` writer - HPDMA channel 0 stop in debug
pub type DBG_HPDMA_0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_1_STOP` reader - HPDMA channel 1 stop in debug
pub type DBG_HPDMA_1_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_1_STOP` writer - HPDMA channel 1 stop in debug
pub type DBG_HPDMA_1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_2_STOP` reader - HPDMA channel 2 stop in debug
pub type DBG_HPDMA_2_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_2_STOP` writer - HPDMA channel 2 stop in debug
pub type DBG_HPDMA_2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_3_STOP` reader - HPDMA channel 3 stop in debug
pub type DBG_HPDMA_3_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_3_STOP` writer - HPDMA channel 3 stop in debug
pub type DBG_HPDMA_3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_4_STOP` reader - HPDMA channel 4 stop in debug
pub type DBG_HPDMA_4_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_4_STOP` writer - HPDMA channel 4 stop in debug
pub type DBG_HPDMA_4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_5_STOP` reader - HPDMA channel 5 stop in debug
pub type DBG_HPDMA_5_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_5_STOP` writer - HPDMA channel 5 stop in debug
pub type DBG_HPDMA_5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_6_STOP` reader - HPDMA channel 6 stop in debug
pub type DBG_HPDMA_6_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_6_STOP` writer - HPDMA channel 6 stop in debug
pub type DBG_HPDMA_6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_7_STOP` reader - HPDMA channel 7 stop in debug
pub type DBG_HPDMA_7_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_7_STOP` writer - HPDMA channel 7 stop in debug
pub type DBG_HPDMA_7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_8_STOP` reader - HPDMA channel 8 stop in debug
pub type DBG_HPDMA_8_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_8_STOP` writer - HPDMA channel 8 stop in debug
pub type DBG_HPDMA_8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_9_STOP` reader - HPDMA channel 9 stop in debug
pub type DBG_HPDMA_9_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_9_STOP` writer - HPDMA channel 9 stop in debug
pub type DBG_HPDMA_9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_10_STOP` reader - HPDMA channel 10 stop in debug
pub type DBG_HPDMA_10_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_10_STOP` writer - HPDMA channel 10 stop in debug
pub type DBG_HPDMA_10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_11_STOP` reader - HPDMA channel 11 stop in debug
pub type DBG_HPDMA_11_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_11_STOP` writer - HPDMA channel 11 stop in debug
pub type DBG_HPDMA_11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_12_STOP` reader - HPDMA channel 12 stop in debug
pub type DBG_HPDMA_12_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_12_STOP` writer - HPDMA channel 12 stop in debug
pub type DBG_HPDMA_12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_13_STOP` reader - HPDMA channel 13 stop in debug
pub type DBG_HPDMA_13_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_13_STOP` writer - HPDMA channel 13 stop in debug
pub type DBG_HPDMA_13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_14_STOP` reader - HPDMA channel 14 stop in debug
pub type DBG_HPDMA_14_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_14_STOP` writer - HPDMA channel 14 stop in debug
pub type DBG_HPDMA_14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HPDMA_15_STOP` reader - HPDMA channel 15 stop in debug
pub type DBG_HPDMA_15_STOP_R = crate::BitReader;
///Field `DBG_HPDMA_15_STOP` writer - HPDMA channel 15 stop in debug
pub type DBG_HPDMA_15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_0_stop(&self) -> DBG_HPDMA_0_STOP_R {
        DBG_HPDMA_0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_1_stop(&self) -> DBG_HPDMA_1_STOP_R {
        DBG_HPDMA_1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_2_stop(&self) -> DBG_HPDMA_2_STOP_R {
        DBG_HPDMA_2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_3_stop(&self) -> DBG_HPDMA_3_STOP_R {
        DBG_HPDMA_3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HPDMA channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_4_stop(&self) -> DBG_HPDMA_4_STOP_R {
        DBG_HPDMA_4_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HPDMA channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_5_stop(&self) -> DBG_HPDMA_5_STOP_R {
        DBG_HPDMA_5_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HPDMA channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_6_stop(&self) -> DBG_HPDMA_6_STOP_R {
        DBG_HPDMA_6_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - HPDMA channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_7_stop(&self) -> DBG_HPDMA_7_STOP_R {
        DBG_HPDMA_7_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HPDMA channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_8_stop(&self) -> DBG_HPDMA_8_STOP_R {
        DBG_HPDMA_8_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HPDMA channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_9_stop(&self) -> DBG_HPDMA_9_STOP_R {
        DBG_HPDMA_9_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HPDMA channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_10_stop(&self) -> DBG_HPDMA_10_STOP_R {
        DBG_HPDMA_10_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HPDMA channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_11_stop(&self) -> DBG_HPDMA_11_STOP_R {
        DBG_HPDMA_11_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HPDMA channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_12_stop(&self) -> DBG_HPDMA_12_STOP_R {
        DBG_HPDMA_12_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HPDMA channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_13_stop(&self) -> DBG_HPDMA_13_STOP_R {
        DBG_HPDMA_13_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HPDMA channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_14_stop(&self) -> DBG_HPDMA_14_STOP_R {
        DBG_HPDMA_14_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - HPDMA channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_15_stop(&self) -> DBG_HPDMA_15_STOP_R {
        DBG_HPDMA_15_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5FZR")
            .field("dbg_hpdma_0_stop", &self.dbg_hpdma_0_stop())
            .field("dbg_hpdma_1_stop", &self.dbg_hpdma_1_stop())
            .field("dbg_hpdma_2_stop", &self.dbg_hpdma_2_stop())
            .field("dbg_hpdma_3_stop", &self.dbg_hpdma_3_stop())
            .field("dbg_hpdma_4_stop", &self.dbg_hpdma_4_stop())
            .field("dbg_hpdma_5_stop", &self.dbg_hpdma_5_stop())
            .field("dbg_hpdma_6_stop", &self.dbg_hpdma_6_stop())
            .field("dbg_hpdma_7_stop", &self.dbg_hpdma_7_stop())
            .field("dbg_hpdma_8_stop", &self.dbg_hpdma_8_stop())
            .field("dbg_hpdma_9_stop", &self.dbg_hpdma_9_stop())
            .field("dbg_hpdma_10_stop", &self.dbg_hpdma_10_stop())
            .field("dbg_hpdma_11_stop", &self.dbg_hpdma_11_stop())
            .field("dbg_hpdma_12_stop", &self.dbg_hpdma_12_stop())
            .field("dbg_hpdma_13_stop", &self.dbg_hpdma_13_stop())
            .field("dbg_hpdma_14_stop", &self.dbg_hpdma_14_stop())
            .field("dbg_hpdma_15_stop", &self.dbg_hpdma_15_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - HPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_0_stop(&mut self) -> DBG_HPDMA_0_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_0_STOP_W::new(self, 0)
    }
    ///Bit 1 - HPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_1_stop(&mut self) -> DBG_HPDMA_1_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_1_STOP_W::new(self, 1)
    }
    ///Bit 2 - HPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_2_stop(&mut self) -> DBG_HPDMA_2_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_2_STOP_W::new(self, 2)
    }
    ///Bit 3 - HPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_3_stop(&mut self) -> DBG_HPDMA_3_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_3_STOP_W::new(self, 3)
    }
    ///Bit 4 - HPDMA channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_4_stop(&mut self) -> DBG_HPDMA_4_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_4_STOP_W::new(self, 4)
    }
    ///Bit 5 - HPDMA channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_5_stop(&mut self) -> DBG_HPDMA_5_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_5_STOP_W::new(self, 5)
    }
    ///Bit 6 - HPDMA channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_6_stop(&mut self) -> DBG_HPDMA_6_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_6_STOP_W::new(self, 6)
    }
    ///Bit 7 - HPDMA channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_7_stop(&mut self) -> DBG_HPDMA_7_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_7_STOP_W::new(self, 7)
    }
    ///Bit 8 - HPDMA channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_8_stop(&mut self) -> DBG_HPDMA_8_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_8_STOP_W::new(self, 8)
    }
    ///Bit 9 - HPDMA channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_9_stop(&mut self) -> DBG_HPDMA_9_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_9_STOP_W::new(self, 9)
    }
    ///Bit 10 - HPDMA channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_10_stop(&mut self) -> DBG_HPDMA_10_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_10_STOP_W::new(self, 10)
    }
    ///Bit 11 - HPDMA channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_11_stop(&mut self) -> DBG_HPDMA_11_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_11_STOP_W::new(self, 11)
    }
    ///Bit 12 - HPDMA channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_12_stop(&mut self) -> DBG_HPDMA_12_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_12_STOP_W::new(self, 12)
    }
    ///Bit 13 - HPDMA channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_13_stop(&mut self) -> DBG_HPDMA_13_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_13_STOP_W::new(self, 13)
    }
    ///Bit 14 - HPDMA channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_14_stop(&mut self) -> DBG_HPDMA_14_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_14_STOP_W::new(self, 14)
    }
    ///Bit 15 - HPDMA channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_hpdma_15_stop(&mut self) -> DBG_HPDMA_15_STOP_W<'_, AHB5FZRrs> {
        DBG_HPDMA_15_STOP_W::new(self, 15)
    }
}
/**DBGMCU AHB5 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb5fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:AHB5FZR)*/
pub struct AHB5FZRrs;
impl crate::RegisterSpec for AHB5FZRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5fzr::R`](R) reader structure
impl crate::Readable for AHB5FZRrs {}
///`write(|w| ..)` method takes [`ahb5fzr::W`](W) writer structure
impl crate::Writable for AHB5FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5FZR to value 0
impl crate::Resettable for AHB5FZRrs {}
