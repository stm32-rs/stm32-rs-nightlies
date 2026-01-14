///Register `C1IER` reader
pub type R = crate::R<C1IERrs>;
///Register `C1IER` writer
pub type W = crate::W<C1IERrs>;
/**Interrupt semaphore %s enable bit

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
///Field `ISE(0-31)` reader - Interrupt semaphore %s enable bit
pub type ISE_R = crate::BitReader<ISE0>;
impl ISE_R {
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
///Field `ISE(0-31)` writer - Interrupt semaphore %s enable bit
pub type ISE_W<'a, REG> = crate::BitWriter<'a, REG, ISE0>;
impl<'a, REG> ISE_W<'a, REG>
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
impl R {
    ///Interrupt semaphore (0-31) enable bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISE0` field.</div>
    #[inline(always)]
    pub fn ise(&self, n: u8) -> ISE_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt semaphore (0-31) enable bit
    #[inline(always)]
    pub fn ise_iter(&self) -> impl Iterator<Item = ISE_R> + '_ {
        (0..32).map(move |n| ISE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt semaphore 0 enable bit
    #[inline(always)]
    pub fn ise0(&self) -> ISE_R {
        ISE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt semaphore 1 enable bit
    #[inline(always)]
    pub fn ise1(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt semaphore 2 enable bit
    #[inline(always)]
    pub fn ise2(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt semaphore 3 enable bit
    #[inline(always)]
    pub fn ise3(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt semaphore 4 enable bit
    #[inline(always)]
    pub fn ise4(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt semaphore 5 enable bit
    #[inline(always)]
    pub fn ise5(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt semaphore 6 enable bit
    #[inline(always)]
    pub fn ise6(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt semaphore 7 enable bit
    #[inline(always)]
    pub fn ise7(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt semaphore 8 enable bit
    #[inline(always)]
    pub fn ise8(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt semaphore 9 enable bit
    #[inline(always)]
    pub fn ise9(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt semaphore 10 enable bit
    #[inline(always)]
    pub fn ise10(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt semaphore 11 enable bit
    #[inline(always)]
    pub fn ise11(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt semaphore 12 enable bit
    #[inline(always)]
    pub fn ise12(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt semaphore 13 enable bit
    #[inline(always)]
    pub fn ise13(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt semaphore 14 enable bit
    #[inline(always)]
    pub fn ise14(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt semaphore 15 enable bit
    #[inline(always)]
    pub fn ise15(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt semaphore 16 enable bit
    #[inline(always)]
    pub fn ise16(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt semaphore 17 enable bit
    #[inline(always)]
    pub fn ise17(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt semaphore 18 enable bit
    #[inline(always)]
    pub fn ise18(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt semaphore 19 enable bit
    #[inline(always)]
    pub fn ise19(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt semaphore 20 enable bit
    #[inline(always)]
    pub fn ise20(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt semaphore 21 enable bit
    #[inline(always)]
    pub fn ise21(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt semaphore 22 enable bit
    #[inline(always)]
    pub fn ise22(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt semaphore 23 enable bit
    #[inline(always)]
    pub fn ise23(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Interrupt semaphore 24 enable bit
    #[inline(always)]
    pub fn ise24(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Interrupt semaphore 25 enable bit
    #[inline(always)]
    pub fn ise25(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Interrupt semaphore 26 enable bit
    #[inline(always)]
    pub fn ise26(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Interrupt semaphore 27 enable bit
    #[inline(always)]
    pub fn ise27(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Interrupt semaphore 28 enable bit
    #[inline(always)]
    pub fn ise28(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt semaphore 29 enable bit
    #[inline(always)]
    pub fn ise29(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Interrupt semaphore 30 enable bit
    #[inline(always)]
    pub fn ise30(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Interrupt semaphore 31 enable bit
    #[inline(always)]
    pub fn ise31(&self) -> ISE_R {
        ISE_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("ise16", &self.ise16())
            .field("ise17", &self.ise17())
            .field("ise18", &self.ise18())
            .field("ise19", &self.ise19())
            .field("ise20", &self.ise20())
            .field("ise21", &self.ise21())
            .field("ise22", &self.ise22())
            .field("ise23", &self.ise23())
            .field("ise24", &self.ise24())
            .field("ise25", &self.ise25())
            .field("ise26", &self.ise26())
            .field("ise27", &self.ise27())
            .field("ise28", &self.ise28())
            .field("ise29", &self.ise29())
            .field("ise30", &self.ise30())
            .field("ise31", &self.ise31())
            .finish()
    }
}
impl W {
    ///Interrupt semaphore (0-31) enable bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISE0` field.</div>
    #[inline(always)]
    pub fn ise(&mut self, n: u8) -> ISE_W<'_, C1IERrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISE_W::new(self, n)
    }
    ///Bit 0 - Interrupt semaphore 0 enable bit
    #[inline(always)]
    pub fn ise0(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 0)
    }
    ///Bit 1 - Interrupt semaphore 1 enable bit
    #[inline(always)]
    pub fn ise1(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 1)
    }
    ///Bit 2 - Interrupt semaphore 2 enable bit
    #[inline(always)]
    pub fn ise2(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 2)
    }
    ///Bit 3 - Interrupt semaphore 3 enable bit
    #[inline(always)]
    pub fn ise3(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 3)
    }
    ///Bit 4 - Interrupt semaphore 4 enable bit
    #[inline(always)]
    pub fn ise4(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 4)
    }
    ///Bit 5 - Interrupt semaphore 5 enable bit
    #[inline(always)]
    pub fn ise5(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 5)
    }
    ///Bit 6 - Interrupt semaphore 6 enable bit
    #[inline(always)]
    pub fn ise6(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 6)
    }
    ///Bit 7 - Interrupt semaphore 7 enable bit
    #[inline(always)]
    pub fn ise7(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 7)
    }
    ///Bit 8 - Interrupt semaphore 8 enable bit
    #[inline(always)]
    pub fn ise8(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 8)
    }
    ///Bit 9 - Interrupt semaphore 9 enable bit
    #[inline(always)]
    pub fn ise9(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 9)
    }
    ///Bit 10 - Interrupt semaphore 10 enable bit
    #[inline(always)]
    pub fn ise10(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 10)
    }
    ///Bit 11 - Interrupt semaphore 11 enable bit
    #[inline(always)]
    pub fn ise11(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 11)
    }
    ///Bit 12 - Interrupt semaphore 12 enable bit
    #[inline(always)]
    pub fn ise12(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 12)
    }
    ///Bit 13 - Interrupt semaphore 13 enable bit
    #[inline(always)]
    pub fn ise13(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 13)
    }
    ///Bit 14 - Interrupt semaphore 14 enable bit
    #[inline(always)]
    pub fn ise14(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 14)
    }
    ///Bit 15 - Interrupt semaphore 15 enable bit
    #[inline(always)]
    pub fn ise15(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 15)
    }
    ///Bit 16 - Interrupt semaphore 16 enable bit
    #[inline(always)]
    pub fn ise16(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 16)
    }
    ///Bit 17 - Interrupt semaphore 17 enable bit
    #[inline(always)]
    pub fn ise17(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 17)
    }
    ///Bit 18 - Interrupt semaphore 18 enable bit
    #[inline(always)]
    pub fn ise18(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 18)
    }
    ///Bit 19 - Interrupt semaphore 19 enable bit
    #[inline(always)]
    pub fn ise19(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 19)
    }
    ///Bit 20 - Interrupt semaphore 20 enable bit
    #[inline(always)]
    pub fn ise20(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 20)
    }
    ///Bit 21 - Interrupt semaphore 21 enable bit
    #[inline(always)]
    pub fn ise21(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 21)
    }
    ///Bit 22 - Interrupt semaphore 22 enable bit
    #[inline(always)]
    pub fn ise22(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 22)
    }
    ///Bit 23 - Interrupt semaphore 23 enable bit
    #[inline(always)]
    pub fn ise23(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 23)
    }
    ///Bit 24 - Interrupt semaphore 24 enable bit
    #[inline(always)]
    pub fn ise24(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 24)
    }
    ///Bit 25 - Interrupt semaphore 25 enable bit
    #[inline(always)]
    pub fn ise25(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 25)
    }
    ///Bit 26 - Interrupt semaphore 26 enable bit
    #[inline(always)]
    pub fn ise26(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 26)
    }
    ///Bit 27 - Interrupt semaphore 27 enable bit
    #[inline(always)]
    pub fn ise27(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 27)
    }
    ///Bit 28 - Interrupt semaphore 28 enable bit
    #[inline(always)]
    pub fn ise28(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 28)
    }
    ///Bit 29 - Interrupt semaphore 29 enable bit
    #[inline(always)]
    pub fn ise29(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 29)
    }
    ///Bit 30 - Interrupt semaphore 30 enable bit
    #[inline(always)]
    pub fn ise30(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 30)
    }
    ///Bit 31 - Interrupt semaphore 31 enable bit
    #[inline(always)]
    pub fn ise31(&mut self) -> ISE_W<'_, C1IERrs> {
        ISE_W::new(self, 31)
    }
}
/**HSEM Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c1ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HSEM:C1IER)*/
pub struct C1IERrs;
impl crate::RegisterSpec for C1IERrs {
    type Ux = u32;
}
///`read()` method returns [`c1ier::R`](R) reader structure
impl crate::Readable for C1IERrs {}
///`write(|w| ..)` method takes [`c1ier::W`](W) writer structure
impl crate::Writable for C1IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1IER to value 0
impl crate::Resettable for C1IERrs {}
