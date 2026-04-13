///Register `APB_FZ2` reader
pub type R = crate::R<APB_FZ2rs>;
///Register `APB_FZ2` writer
pub type W = crate::W<APB_FZ2rs>;
/**Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM1_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_TIM1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
pub type DBG_TIM1_STOP_R = crate::BitReader<DBG_TIM1_STOP>;
impl DBG_TIM1_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM1_STOP {
        match self.bits {
            false => DBG_TIM1_STOP::B0x0,
            true => DBG_TIM1_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM1_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM1_STOP::B0x1
    }
}
///Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM1_STOP>;
impl<'a, REG> DBG_TIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP::B0x1)
    }
}
/**Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM14_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_TIM14_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM14_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
pub type DBG_TIM14_STOP_R = crate::BitReader<DBG_TIM14_STOP>;
impl DBG_TIM14_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM14_STOP {
        match self.bits {
            false => DBG_TIM14_STOP::B0x0,
            true => DBG_TIM14_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM14_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM14_STOP::B0x1
    }
}
///Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM14_STOP>;
impl<'a, REG> DBG_TIM14_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM14_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM14_STOP::B0x1)
    }
}
/**Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM16_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_TIM16_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM16_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
pub type DBG_TIM16_STOP_R = crate::BitReader<DBG_TIM16_STOP>;
impl DBG_TIM16_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM16_STOP {
        match self.bits {
            false => DBG_TIM16_STOP::B0x0,
            true => DBG_TIM16_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM16_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM16_STOP::B0x1
    }
}
///Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM16_STOP>;
impl<'a, REG> DBG_TIM16_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM16_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM16_STOP::B0x1)
    }
}
/**Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM17_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_TIM17_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM17_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
pub type DBG_TIM17_STOP_R = crate::BitReader<DBG_TIM17_STOP>;
impl DBG_TIM17_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM17_STOP {
        match self.bits {
            false => DBG_TIM17_STOP::B0x0,
            true => DBG_TIM17_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM17_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM17_STOP::B0x1
    }
}
///Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM17_STOP>;
impl<'a, REG> DBG_TIM17_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM17_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM17_STOP::B0x1)
    }
}
impl R {
    ///Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FZ2")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<'_, APB_FZ2rs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<'_, APB_FZ2rs> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    ///Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<'_, APB_FZ2rs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<'_, APB_FZ2rs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
/**DBG APB freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb_fz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DBG:APB_FZ2)*/
pub struct APB_FZ2rs;
impl crate::RegisterSpec for APB_FZ2rs {
    type Ux = u32;
}
///`read()` method returns [`apb_fz2::R`](R) reader structure
impl crate::Readable for APB_FZ2rs {}
///`write(|w| ..)` method takes [`apb_fz2::W`](W) writer structure
impl crate::Writable for APB_FZ2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB_FZ2 to value 0
impl crate::Resettable for APB_FZ2rs {}
