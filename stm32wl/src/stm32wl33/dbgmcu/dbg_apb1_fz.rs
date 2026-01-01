///Register `DBG_APB1_FZ` reader
pub type R = crate::R<DBG_APB1_FZrs>;
///Register `DBG_APB1_FZ` writer
pub type W = crate::W<DBG_APB1_FZrs>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode.
pub type DBG_I2C1_STOP_R = crate::BitReader;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode.
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode.
pub type DBG_I2C2_STOP_R = crate::BitReader;
///Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode.
pub type DBG_I2C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 21 - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_APB1_FZ")
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
            .finish()
    }
}
impl W {
    ///Bit 21 - I2C1 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C1 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C1 SMBUS timeou is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, DBG_APB1_FZrs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    ///Bit 23 - I2C2 SMBUS timeout stop in CPU debug - 0: Normal operation. I2C2 SMBUS timeout continues to operate while the CPU is in debug mode - 1: Stop in debug. I2C2 SMBUS timeou is frozen while the CPU is in debug mode.
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<'_, DBG_APB1_FZrs> {
        DBG_I2C2_STOP_W::new(self, 23)
    }
}
/**DBG_APB1_FZ register

You can [`read`](crate::Reg::read) this register and get [`dbg_apb1_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_apb1_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DBGMCU:DBG_APB1_FZ)*/
pub struct DBG_APB1_FZrs;
impl crate::RegisterSpec for DBG_APB1_FZrs {
    type Ux = u32;
}
///`read()` method returns [`dbg_apb1_fz::R`](R) reader structure
impl crate::Readable for DBG_APB1_FZrs {}
///`write(|w| ..)` method takes [`dbg_apb1_fz::W`](W) writer structure
impl crate::Writable for DBG_APB1_FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG_APB1_FZ to value 0
impl crate::Resettable for DBG_APB1_FZrs {}
