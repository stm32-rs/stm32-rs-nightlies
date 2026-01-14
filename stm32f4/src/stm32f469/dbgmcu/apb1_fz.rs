///Register `APB1_FZ` reader
pub type R = crate::R<APB1_FZrs>;
///Register `APB1_FZ` writer
pub type W = crate::W<APB1_FZrs>;
///Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3_STOP` reader - DBG_TIM3 _STOP
pub type DBG_TIM3_STOP_R = crate::BitReader;
///Field `DBG_TIM3_STOP` writer - DBG_TIM3 _STOP
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP
pub type DBG_TIM4_STOP_R = crate::BitReader;
///Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP
pub type DBG_TIM5_STOP_R = crate::BitReader;
///Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM6_STOP` reader - DBG_TIM6_STOP
pub type DBG_TIM6_STOP_R = crate::BitReader;
///Field `DBG_TIM6_STOP` writer - DBG_TIM6_STOP
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM7_STOP` reader - DBG_TIM7_STOP
pub type DBG_TIM7_STOP_R = crate::BitReader;
///Field `DBG_TIM7_STOP` writer - DBG_TIM7_STOP
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM12_STOP` reader - DBG_TIM12_STOP
pub type DBG_TIM12_STOP_R = crate::BitReader;
///Field `DBG_TIM12_STOP` writer - DBG_TIM12_STOP
pub type DBG_TIM12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM13_STOP` reader - DBG_TIM13_STOP
pub type DBG_TIM13_STOP_R = crate::BitReader;
///Field `DBG_TIM13_STOP` writer - DBG_TIM13_STOP
pub type DBG_TIM13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14_STOP` reader - DBG_TIM14_STOP
pub type DBG_TIM14_STOP_R = crate::BitReader;
///Field `DBG_TIM14_STOP` writer - DBG_TIM14_STOP
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP
pub type DBG_WWDG_STOP_R = crate::BitReader;
///Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - DBG_IWDEG_STOP
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - DBG_IWDEG_STOP
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_J2C1_SMBUS_TIMEOUT` reader - DBG_J2C1_SMBUS_TIMEOUT
pub type DBG_J2C1_SMBUS_TIMEOUT_R = crate::BitReader;
///Field `DBG_J2C1_SMBUS_TIMEOUT` writer - DBG_J2C1_SMBUS_TIMEOUT
pub type DBG_J2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_J2C2_SMBUS_TIMEOUT` reader - DBG_J2C2_SMBUS_TIMEOUT
pub type DBG_J2C2_SMBUS_TIMEOUT_R = crate::BitReader;
///Field `DBG_J2C2_SMBUS_TIMEOUT` writer - DBG_J2C2_SMBUS_TIMEOUT
pub type DBG_J2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_J2C3SMBUS_TIMEOUT` reader - DBG_J2C3SMBUS_TIMEOUT
pub type DBG_J2C3SMBUS_TIMEOUT_R = crate::BitReader;
///Field `DBG_J2C3SMBUS_TIMEOUT` writer - DBG_J2C3SMBUS_TIMEOUT
pub type DBG_J2C3SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_CAN1_STOP` reader - DBG_CAN1_STOP
pub type DBG_CAN1_STOP_R = crate::BitReader;
///Field `DBG_CAN1_STOP` writer - DBG_CAN1_STOP
pub type DBG_CAN1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_CAN2_STOP` reader - DBG_CAN2_STOP
pub type DBG_CAN2_STOP_R = crate::BitReader;
///Field `DBG_CAN2_STOP` writer - DBG_CAN2_STOP
pub type DBG_CAN2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DBG_TIM3 _STOP
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DBG_TIM4_STOP
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DBG_TIM5_STOP
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DBG_TIM6_STOP
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DBG_TIM7_STOP
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DBG_TIM12_STOP
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DBG_TIM13_STOP
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DBG_TIM14_STOP
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - DBG_WWDG_STOP
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DBG_IWDEG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - DBG_J2C1_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c1_smbus_timeout(&self) -> DBG_J2C1_SMBUS_TIMEOUT_R {
        DBG_J2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DBG_J2C2_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c2_smbus_timeout(&self) -> DBG_J2C2_SMBUS_TIMEOUT_R {
        DBG_J2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - DBG_J2C3SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c3smbus_timeout(&self) -> DBG_J2C3SMBUS_TIMEOUT_R {
        DBG_J2C3SMBUS_TIMEOUT_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - DBG_CAN1_STOP
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DBG_CAN1_STOP_R {
        DBG_CAN1_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - DBG_CAN2_STOP
    #[inline(always)]
    pub fn dbg_can2_stop(&self) -> DBG_CAN2_STOP_R {
        DBG_CAN2_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1_FZ")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_tim4_stop", &self.dbg_tim4_stop())
            .field("dbg_tim5_stop", &self.dbg_tim5_stop())
            .field("dbg_tim6_stop", &self.dbg_tim6_stop())
            .field("dbg_tim7_stop", &self.dbg_tim7_stop())
            .field("dbg_tim12_stop", &self.dbg_tim12_stop())
            .field("dbg_tim13_stop", &self.dbg_tim13_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_j2c1_smbus_timeout", &self.dbg_j2c1_smbus_timeout())
            .field("dbg_j2c2_smbus_timeout", &self.dbg_j2c2_smbus_timeout())
            .field("dbg_j2c3smbus_timeout", &self.dbg_j2c3smbus_timeout())
            .field("dbg_can1_stop", &self.dbg_can1_stop())
            .field("dbg_can2_stop", &self.dbg_can2_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<'_, APB1_FZrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 1 - DBG_TIM3 _STOP
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<'_, APB1_FZrs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 2 - DBG_TIM4_STOP
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<'_, APB1_FZrs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    ///Bit 3 - DBG_TIM5_STOP
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<'_, APB1_FZrs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    ///Bit 4 - DBG_TIM6_STOP
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<'_, APB1_FZrs> {
        DBG_TIM6_STOP_W::new(self, 4)
    }
    ///Bit 5 - DBG_TIM7_STOP
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<'_, APB1_FZrs> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    ///Bit 6 - DBG_TIM12_STOP
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<'_, APB1_FZrs> {
        DBG_TIM12_STOP_W::new(self, 6)
    }
    ///Bit 7 - DBG_TIM13_STOP
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<'_, APB1_FZrs> {
        DBG_TIM13_STOP_W::new(self, 7)
    }
    ///Bit 8 - DBG_TIM14_STOP
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<'_, APB1_FZrs> {
        DBG_TIM14_STOP_W::new(self, 8)
    }
    ///Bit 11 - DBG_WWDG_STOP
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<'_, APB1_FZrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - DBG_IWDEG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB1_FZrs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - DBG_J2C1_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c1_smbus_timeout(&mut self) -> DBG_J2C1_SMBUS_TIMEOUT_W<'_, APB1_FZrs> {
        DBG_J2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
    ///Bit 22 - DBG_J2C2_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c2_smbus_timeout(&mut self) -> DBG_J2C2_SMBUS_TIMEOUT_W<'_, APB1_FZrs> {
        DBG_J2C2_SMBUS_TIMEOUT_W::new(self, 22)
    }
    ///Bit 23 - DBG_J2C3SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_j2c3smbus_timeout(&mut self) -> DBG_J2C3SMBUS_TIMEOUT_W<'_, APB1_FZrs> {
        DBG_J2C3SMBUS_TIMEOUT_W::new(self, 23)
    }
    ///Bit 25 - DBG_CAN1_STOP
    #[inline(always)]
    pub fn dbg_can1_stop(&mut self) -> DBG_CAN1_STOP_W<'_, APB1_FZrs> {
        DBG_CAN1_STOP_W::new(self, 25)
    }
    ///Bit 26 - DBG_CAN2_STOP
    #[inline(always)]
    pub fn dbg_can2_stop(&mut self) -> DBG_CAN2_STOP_W<'_, APB1_FZrs> {
        DBG_CAN2_STOP_W::new(self, 26)
    }
}
/**Debug MCU APB1 Freeze registe

You can [`read`](crate::Reg::read) this register and get [`apb1_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DBGMCU:APB1_FZ)*/
pub struct APB1_FZrs;
impl crate::RegisterSpec for APB1_FZrs {
    type Ux = u32;
}
///`read()` method returns [`apb1_fz::R`](R) reader structure
impl crate::Readable for APB1_FZrs {}
///`write(|w| ..)` method takes [`apb1_fz::W`](W) writer structure
impl crate::Writable for APB1_FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1_FZ to value 0
impl crate::Resettable for APB1_FZrs {}
