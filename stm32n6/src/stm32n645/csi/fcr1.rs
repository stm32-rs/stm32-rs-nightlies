///Register `FCR1` writer
pub type W = crate::W<FCR1rs>;
///Field `CESOTDL0F` writer - Clear SOT error flag on lane 0
pub type CESOTDL0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CESOTSYNCDL0F` writer - Clear SOT synchronization error flag on lane 0
pub type CESOTSYNCDL0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEESCDL0F` writer - Clear D-PHY_RX lane 0 escape entry error flag
pub type CEESCDL0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CESYNCESCDL0F` writer - Clear D-PHY_RX lane 0 low-power data transmission synchronization error flag
pub type CESYNCESCDL0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CECTRLDL0F` writer - Clear D-PHY_RX lane 0 control error flag
pub type CECTRLDL0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CESOTDL1F` writer - Clear SOT error flag on lane 1
pub type CESOTDL1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CESOTSYNCDL1F` writer - Clear SOT synchronization error flag on lane 1
pub type CESOTSYNCDL1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEESCDL1F` writer - Clear D-PHY_RX lane 1 escape entry error flag
pub type CEESCDL1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CESYNCESCDL1F` writer - Clear D-PHY_RX lane 1 low-power data transmission synchronization error flag
pub type CESYNCESCDL1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CECTRLDL1F` writer - Clear D-PHY_RX lane 1 control error flag
pub type CECTRLDL1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear SOT error flag on lane 0
    #[inline(always)]
    pub fn cesotdl0f(&mut self) -> CESOTDL0F_W<'_, FCR1rs> {
        CESOTDL0F_W::new(self, 0)
    }
    ///Bit 1 - Clear SOT synchronization error flag on lane 0
    #[inline(always)]
    pub fn cesotsyncdl0f(&mut self) -> CESOTSYNCDL0F_W<'_, FCR1rs> {
        CESOTSYNCDL0F_W::new(self, 1)
    }
    ///Bit 2 - Clear D-PHY_RX lane 0 escape entry error flag
    #[inline(always)]
    pub fn ceescdl0f(&mut self) -> CEESCDL0F_W<'_, FCR1rs> {
        CEESCDL0F_W::new(self, 2)
    }
    ///Bit 3 - Clear D-PHY_RX lane 0 low-power data transmission synchronization error flag
    #[inline(always)]
    pub fn cesyncescdl0f(&mut self) -> CESYNCESCDL0F_W<'_, FCR1rs> {
        CESYNCESCDL0F_W::new(self, 3)
    }
    ///Bit 4 - Clear D-PHY_RX lane 0 control error flag
    #[inline(always)]
    pub fn cectrldl0f(&mut self) -> CECTRLDL0F_W<'_, FCR1rs> {
        CECTRLDL0F_W::new(self, 4)
    }
    ///Bit 8 - Clear SOT error flag on lane 1
    #[inline(always)]
    pub fn cesotdl1f(&mut self) -> CESOTDL1F_W<'_, FCR1rs> {
        CESOTDL1F_W::new(self, 8)
    }
    ///Bit 9 - Clear SOT synchronization error flag on lane 1
    #[inline(always)]
    pub fn cesotsyncdl1f(&mut self) -> CESOTSYNCDL1F_W<'_, FCR1rs> {
        CESOTSYNCDL1F_W::new(self, 9)
    }
    ///Bit 10 - Clear D-PHY_RX lane 1 escape entry error flag
    #[inline(always)]
    pub fn ceescdl1f(&mut self) -> CEESCDL1F_W<'_, FCR1rs> {
        CEESCDL1F_W::new(self, 10)
    }
    ///Bit 11 - Clear D-PHY_RX lane 1 low-power data transmission synchronization error flag
    #[inline(always)]
    pub fn cesyncescdl1f(&mut self) -> CESYNCESCDL1F_W<'_, FCR1rs> {
        CESYNCESCDL1F_W::new(self, 11)
    }
    ///Bit 12 - Clear D-PHY_RX lane 1 control error flag
    #[inline(always)]
    pub fn cectrldl1f(&mut self) -> CECTRLDL1F_W<'_, FCR1rs> {
        CECTRLDL1F_W::new(self, 12)
    }
}
/**CSI-2 Host flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CSI:FCR1)*/
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr1::W`](W) writer structure
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1rs {}
