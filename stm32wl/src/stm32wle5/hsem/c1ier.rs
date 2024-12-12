///Register `C1IER` reader
pub type R = crate::R<C1IERrs>;
///Register `C1IER` writer
pub type W = crate::W<C1IERrs>;
/**Interrupt semaphore n enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISE0 {
    ///0: Interrupt generation disabled
    Disabled = 0,
    ///1: Interrupt generation enabled
    Enabled = 1,
}
impl From<ISE0> for bool {
    #[inline(always)]
    fn from(variant: ISE0) -> Self {
        variant as u8 != 0
    }
}
///Field `ISE0` reader - Interrupt semaphore n enable bit
pub type ISE0_R = crate::BitReader<ISE0>;
impl ISE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ISE0 {
        match self.bits {
            false => ISE0::Disabled,
            true => ISE0::Enabled,
        }
    }
    ///Interrupt generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ISE0::Disabled
    }
    ///Interrupt generation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ISE0::Enabled
    }
}
///Field `ISE0` writer - Interrupt semaphore n enable bit
pub type ISE0_W<'a, REG> = crate::BitWriter<'a, REG, ISE0>;
impl<'a, REG> ISE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ISE0::Disabled)
    }
    ///Interrupt generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ISE0::Enabled)
    }
}
///Field `ISE1` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE1_R;
///Field `ISE2` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE2_R;
///Field `ISE3` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE3_R;
///Field `ISE4` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE4_R;
///Field `ISE5` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE5_R;
///Field `ISE6` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE6_R;
///Field `ISE7` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE7_R;
///Field `ISE8` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE8_R;
///Field `ISE9` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE9_R;
///Field `ISE10` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE10_R;
///Field `ISE11` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE11_R;
///Field `ISE12` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE12_R;
///Field `ISE13` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE13_R;
///Field `ISE14` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE14_R;
///Field `ISE15` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE15_R;
///Field `ISE1` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE1_W;
///Field `ISE2` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE2_W;
///Field `ISE3` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE3_W;
///Field `ISE4` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE4_W;
///Field `ISE5` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE5_W;
///Field `ISE6` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE6_W;
///Field `ISE7` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE7_W;
///Field `ISE8` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE8_W;
///Field `ISE9` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE9_W;
///Field `ISE10` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE10_W;
///Field `ISE11` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE11_W;
///Field `ISE12` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE12_W;
///Field `ISE13` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE13_W;
///Field `ISE14` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE14_W;
///Field `ISE15` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE15_W;
impl R {
    ///Bit 0 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise0(&self) -> ISE0_R {
        ISE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise1(&self) -> ISE1_R {
        ISE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise2(&self) -> ISE2_R {
        ISE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise3(&self) -> ISE3_R {
        ISE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise4(&self) -> ISE4_R {
        ISE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise5(&self) -> ISE5_R {
        ISE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise6(&self) -> ISE6_R {
        ISE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise7(&self) -> ISE7_R {
        ISE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise8(&self) -> ISE8_R {
        ISE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise9(&self) -> ISE9_R {
        ISE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise10(&self) -> ISE10_R {
        ISE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise11(&self) -> ISE11_R {
        ISE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise12(&self) -> ISE12_R {
        ISE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise13(&self) -> ISE13_R {
        ISE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise14(&self) -> ISE14_R {
        ISE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise15(&self) -> ISE15_R {
        ISE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1IER")
            .field("ise0", &self.ise0())
            .field("ise1", &self.ise1())
            .field("ise2", &self.ise2())
            .field("ise3", &self.ise3())
            .field("ise4", &self.ise4())
            .field("ise5", &self.ise5())
            .field("ise6", &self.ise6())
            .field("ise7", &self.ise7())
            .field("ise8", &self.ise8())
            .field("ise9", &self.ise9())
            .field("ise10", &self.ise10())
            .field("ise11", &self.ise11())
            .field("ise12", &self.ise12())
            .field("ise13", &self.ise13())
            .field("ise14", &self.ise14())
            .field("ise15", &self.ise15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise0(&mut self) -> ISE0_W<C1IERrs> {
        ISE0_W::new(self, 0)
    }
    ///Bit 1 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise1(&mut self) -> ISE1_W<C1IERrs> {
        ISE1_W::new(self, 1)
    }
    ///Bit 2 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise2(&mut self) -> ISE2_W<C1IERrs> {
        ISE2_W::new(self, 2)
    }
    ///Bit 3 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise3(&mut self) -> ISE3_W<C1IERrs> {
        ISE3_W::new(self, 3)
    }
    ///Bit 4 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise4(&mut self) -> ISE4_W<C1IERrs> {
        ISE4_W::new(self, 4)
    }
    ///Bit 5 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise5(&mut self) -> ISE5_W<C1IERrs> {
        ISE5_W::new(self, 5)
    }
    ///Bit 6 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise6(&mut self) -> ISE6_W<C1IERrs> {
        ISE6_W::new(self, 6)
    }
    ///Bit 7 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise7(&mut self) -> ISE7_W<C1IERrs> {
        ISE7_W::new(self, 7)
    }
    ///Bit 8 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise8(&mut self) -> ISE8_W<C1IERrs> {
        ISE8_W::new(self, 8)
    }
    ///Bit 9 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise9(&mut self) -> ISE9_W<C1IERrs> {
        ISE9_W::new(self, 9)
    }
    ///Bit 10 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise10(&mut self) -> ISE10_W<C1IERrs> {
        ISE10_W::new(self, 10)
    }
    ///Bit 11 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise11(&mut self) -> ISE11_W<C1IERrs> {
        ISE11_W::new(self, 11)
    }
    ///Bit 12 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise12(&mut self) -> ISE12_W<C1IERrs> {
        ISE12_W::new(self, 12)
    }
    ///Bit 13 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise13(&mut self) -> ISE13_W<C1IERrs> {
        ISE13_W::new(self, 13)
    }
    ///Bit 14 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise14(&mut self) -> ISE14_W<C1IERrs> {
        ISE14_W::new(self, 14)
    }
    ///Bit 15 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise15(&mut self) -> ISE15_W<C1IERrs> {
        ISE15_W::new(self, 15)
    }
}
/**HSEM Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c1ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#HSEM:C1IER)*/
pub struct C1IERrs;
impl crate::RegisterSpec for C1IERrs {
    type Ux = u32;
}
///`read()` method returns [`c1ier::R`](R) reader structure
impl crate::Readable for C1IERrs {}
///`write(|w| ..)` method takes [`c1ier::W`](W) writer structure
impl crate::Writable for C1IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1IER to value 0
impl crate::Resettable for C1IERrs {
    const RESET_VALUE: u32 = 0;
}
