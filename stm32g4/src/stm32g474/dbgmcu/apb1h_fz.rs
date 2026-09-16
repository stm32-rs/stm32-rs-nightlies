///Register `APB1H_FZ` reader
pub type R = crate::R<APB1H_FZrs>;
///Register `APB1H_FZ` writer
pub type W = crate::W<APB1H_FZrs>;
///Field `DBG_I2C4_STOP` reader - DBG_I2C4_STOP
pub type DBG_I2C4_STOP_R = crate::BitReader;
///Field `DBG_I2C4_STOP` writer - DBG_I2C4_STOP
pub type DBG_I2C4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - DBG_I2C4_STOP
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1H_FZ")
            .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
            .finish()
    }
}
impl W {
    ///Bit 1 - DBG_I2C4_STOP
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<'_, APB1H_FZrs> {
        DBG_I2C4_STOP_W::new(self, 1)
    }
}
/**APB Low Freeze Register 2

You can [`read`](crate::Reg::read) this register and get [`apb1h_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1h_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#DBGMCU:APB1H_FZ)*/
pub struct APB1H_FZrs;
impl crate::RegisterSpec for APB1H_FZrs {
    type Ux = u32;
}
///`read()` method returns [`apb1h_fz::R`](R) reader structure
impl crate::Readable for APB1H_FZrs {}
///`write(|w| ..)` method takes [`apb1h_fz::W`](W) writer structure
impl crate::Writable for APB1H_FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1H_FZ to value 0
impl crate::Resettable for APB1H_FZrs {}
