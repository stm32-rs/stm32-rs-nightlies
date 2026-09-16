///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
///Field `ESOTDL0IE` reader - SOT error interrupt enable on lane 0
pub type ESOTDL0IE_R = crate::BitReader;
///Field `ESOTDL0IE` writer - SOT error interrupt enable on lane 0
pub type ESOTDL0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESOTSYNCDL0IE` reader - SOT synchronization interrupt error enable on lane 0
pub type ESOTSYNCDL0IE_R = crate::BitReader;
///Field `ESOTSYNCDL0IE` writer - SOT synchronization interrupt error enable on lane 0
pub type ESOTSYNCDL0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EESCDL0IE` reader - D-PHY_RX lane 0 escape entry error interrupt enable
pub type EESCDL0IE_R = crate::BitReader;
///Field `EESCDL0IE` writer - D-PHY_RX lane 0 escape entry error interrupt enable
pub type EESCDL0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESYNCESCDL0IE` reader - D-PHY_RX lane 0 low power data transmission synchronization error interrupt enable
pub type ESYNCESCDL0IE_R = crate::BitReader;
///Field `ESYNCESCDL0IE` writer - D-PHY_RX lane 0 low power data transmission synchronization error interrupt enable
pub type ESYNCESCDL0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECTRLDL0IE` reader - D-PHY_RX lane 0 control error interrupt enable
pub type ECTRLDL0IE_R = crate::BitReader;
///Field `ECTRLDL0IE` writer - D-PHY_RX lane 0 control error interrupt enable
pub type ECTRLDL0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESOTDL1IE` reader - SOT error interrupt enable on lane 1
pub type ESOTDL1IE_R = crate::BitReader;
///Field `ESOTDL1IE` writer - SOT error interrupt enable on lane 1
pub type ESOTDL1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESOTSYNCDL1IE` reader - SOT synchronization interrupt error enable on lane 1
pub type ESOTSYNCDL1IE_R = crate::BitReader;
///Field `ESOTSYNCDL1IE` writer - SOT synchronization interrupt error enable on lane 1
pub type ESOTSYNCDL1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EESCDL1IE` reader - D-PHY_RX lane 1 escape entry error interrupt enable
pub type EESCDL1IE_R = crate::BitReader;
///Field `EESCDL1IE` writer - D-PHY_RX lane 1 escape entry error interrupt enable
pub type EESCDL1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESYNCESCDL1IE` reader - D-PHY_RX lane 1 low-power data transmission synchronization error interrupt enable
pub type ESYNCESCDL1IE_R = crate::BitReader;
///Field `ESYNCESCDL1IE` writer - D-PHY_RX lane 1 low-power data transmission synchronization error interrupt enable
pub type ESYNCESCDL1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECTRLDL1IE` reader - D-PHY_RX lane 1 control error interrupt enable
pub type ECTRLDL1IE_R = crate::BitReader;
///Field `ECTRLDL1IE` writer - D-PHY_RX lane 1 control error interrupt enable
pub type ECTRLDL1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SOT error interrupt enable on lane 0
    #[inline(always)]
    pub fn esotdl0ie(&self) -> ESOTDL0IE_R {
        ESOTDL0IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SOT synchronization interrupt error enable on lane 0
    #[inline(always)]
    pub fn esotsyncdl0ie(&self) -> ESOTSYNCDL0IE_R {
        ESOTSYNCDL0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - D-PHY_RX lane 0 escape entry error interrupt enable
    #[inline(always)]
    pub fn eescdl0ie(&self) -> EESCDL0IE_R {
        EESCDL0IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - D-PHY_RX lane 0 low power data transmission synchronization error interrupt enable
    #[inline(always)]
    pub fn esyncescdl0ie(&self) -> ESYNCESCDL0IE_R {
        ESYNCESCDL0IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - D-PHY_RX lane 0 control error interrupt enable
    #[inline(always)]
    pub fn ectrldl0ie(&self) -> ECTRLDL0IE_R {
        ECTRLDL0IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SOT error interrupt enable on lane 1
    #[inline(always)]
    pub fn esotdl1ie(&self) -> ESOTDL1IE_R {
        ESOTDL1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SOT synchronization interrupt error enable on lane 1
    #[inline(always)]
    pub fn esotsyncdl1ie(&self) -> ESOTSYNCDL1IE_R {
        ESOTSYNCDL1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - D-PHY_RX lane 1 escape entry error interrupt enable
    #[inline(always)]
    pub fn eescdl1ie(&self) -> EESCDL1IE_R {
        EESCDL1IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - D-PHY_RX lane 1 low-power data transmission synchronization error interrupt enable
    #[inline(always)]
    pub fn esyncescdl1ie(&self) -> ESYNCESCDL1IE_R {
        ESYNCESCDL1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - D-PHY_RX lane 1 control error interrupt enable
    #[inline(always)]
    pub fn ectrldl1ie(&self) -> ECTRLDL1IE_R {
        ECTRLDL1IE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("esotdl0ie", &self.esotdl0ie())
            .field("esotsyncdl0ie", &self.esotsyncdl0ie())
            .field("eescdl0ie", &self.eescdl0ie())
            .field("esyncescdl0ie", &self.esyncescdl0ie())
            .field("ectrldl0ie", &self.ectrldl0ie())
            .field("esotdl1ie", &self.esotdl1ie())
            .field("esotsyncdl1ie", &self.esotsyncdl1ie())
            .field("eescdl1ie", &self.eescdl1ie())
            .field("esyncescdl1ie", &self.esyncescdl1ie())
            .field("ectrldl1ie", &self.ectrldl1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - SOT error interrupt enable on lane 0
    #[inline(always)]
    pub fn esotdl0ie(&mut self) -> ESOTDL0IE_W<'_, IER1rs> {
        ESOTDL0IE_W::new(self, 0)
    }
    ///Bit 1 - SOT synchronization interrupt error enable on lane 0
    #[inline(always)]
    pub fn esotsyncdl0ie(&mut self) -> ESOTSYNCDL0IE_W<'_, IER1rs> {
        ESOTSYNCDL0IE_W::new(self, 1)
    }
    ///Bit 2 - D-PHY_RX lane 0 escape entry error interrupt enable
    #[inline(always)]
    pub fn eescdl0ie(&mut self) -> EESCDL0IE_W<'_, IER1rs> {
        EESCDL0IE_W::new(self, 2)
    }
    ///Bit 3 - D-PHY_RX lane 0 low power data transmission synchronization error interrupt enable
    #[inline(always)]
    pub fn esyncescdl0ie(&mut self) -> ESYNCESCDL0IE_W<'_, IER1rs> {
        ESYNCESCDL0IE_W::new(self, 3)
    }
    ///Bit 4 - D-PHY_RX lane 0 control error interrupt enable
    #[inline(always)]
    pub fn ectrldl0ie(&mut self) -> ECTRLDL0IE_W<'_, IER1rs> {
        ECTRLDL0IE_W::new(self, 4)
    }
    ///Bit 8 - SOT error interrupt enable on lane 1
    #[inline(always)]
    pub fn esotdl1ie(&mut self) -> ESOTDL1IE_W<'_, IER1rs> {
        ESOTDL1IE_W::new(self, 8)
    }
    ///Bit 9 - SOT synchronization interrupt error enable on lane 1
    #[inline(always)]
    pub fn esotsyncdl1ie(&mut self) -> ESOTSYNCDL1IE_W<'_, IER1rs> {
        ESOTSYNCDL1IE_W::new(self, 9)
    }
    ///Bit 10 - D-PHY_RX lane 1 escape entry error interrupt enable
    #[inline(always)]
    pub fn eescdl1ie(&mut self) -> EESCDL1IE_W<'_, IER1rs> {
        EESCDL1IE_W::new(self, 10)
    }
    ///Bit 11 - D-PHY_RX lane 1 low-power data transmission synchronization error interrupt enable
    #[inline(always)]
    pub fn esyncescdl1ie(&mut self) -> ESYNCESCDL1IE_W<'_, IER1rs> {
        ESYNCESCDL1IE_W::new(self, 11)
    }
    ///Bit 12 - D-PHY_RX lane 1 control error interrupt enable
    #[inline(always)]
    pub fn ectrldl1ie(&mut self) -> ECTRLDL1IE_W<'_, IER1rs> {
        ECTRLDL1IE_W::new(self, 12)
    }
}
/**CSI-2 Host interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1rs {}
