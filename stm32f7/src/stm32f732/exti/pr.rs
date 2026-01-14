///Register `PR` reader
pub type R = crate::R<PRrs>;
///Register `PR` writer
pub type W = crate::W<PRrs>;
/**Pending bit %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR0R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PR0R> for bool {
    #[inline(always)]
    fn from(variant: PR0R) -> Self {
        variant as u8 != 0
    }
}
///Field `PR(0-23)` reader - Pending bit %s
pub type PR_R = crate::BitReader<PR0R>;
impl PR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR0R {
        match self.bits {
            false => PR0R::NotPending,
            true => PR0R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR0R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR0R::Pending
    }
}
/**Pending bit %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR0W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PR0W> for bool {
    #[inline(always)]
    fn from(variant: PR0W) -> Self {
        variant as u8 != 0
    }
}
///Field `PR(0-23)` writer - Pending bit %s
pub type PR_W<'a, REG> = crate::BitWriter1C<'a, REG, PR0W>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR0W::Clear)
    }
}
impl R {
    ///Pending bit (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PR0` field.</div>
    #[inline(always)]
    pub fn pr(&self, n: u8) -> PR_R {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        PR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Pending bit (0-23)
    #[inline(always)]
    pub fn pr_iter(&self) -> impl Iterator<Item = PR_R> + '_ {
        (0..24).map(move |n| PR_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pr0(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pr1(&self) -> PR_R {
        PR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pr2(&self) -> PR_R {
        PR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pr3(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pr4(&self) -> PR_R {
        PR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pr5(&self) -> PR_R {
        PR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pr6(&self) -> PR_R {
        PR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pr7(&self) -> PR_R {
        PR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pr8(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pr9(&self) -> PR_R {
        PR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pr10(&self) -> PR_R {
        PR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pr11(&self) -> PR_R {
        PR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pr12(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pr13(&self) -> PR_R {
        PR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pr14(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pr15(&self) -> PR_R {
        PR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pr16(&self) -> PR_R {
        PR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Pending bit 17
    #[inline(always)]
    pub fn pr17(&self) -> PR_R {
        PR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Pending bit 18
    #[inline(always)]
    pub fn pr18(&self) -> PR_R {
        PR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pr19(&self) -> PR_R {
        PR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Pending bit 20
    #[inline(always)]
    pub fn pr20(&self) -> PR_R {
        PR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Pending bit 21
    #[inline(always)]
    pub fn pr21(&self) -> PR_R {
        PR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Pending bit 22
    #[inline(always)]
    pub fn pr22(&self) -> PR_R {
        PR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Pending bit 23
    #[inline(always)]
    pub fn pr23(&self) -> PR_R {
        PR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR")
            .field("pr0", &self.pr0())
            .field("pr1", &self.pr1())
            .field("pr2", &self.pr2())
            .field("pr3", &self.pr3())
            .field("pr4", &self.pr4())
            .field("pr5", &self.pr5())
            .field("pr6", &self.pr6())
            .field("pr7", &self.pr7())
            .field("pr8", &self.pr8())
            .field("pr9", &self.pr9())
            .field("pr10", &self.pr10())
            .field("pr11", &self.pr11())
            .field("pr12", &self.pr12())
            .field("pr13", &self.pr13())
            .field("pr14", &self.pr14())
            .field("pr15", &self.pr15())
            .field("pr16", &self.pr16())
            .field("pr17", &self.pr17())
            .field("pr18", &self.pr18())
            .field("pr19", &self.pr19())
            .field("pr20", &self.pr20())
            .field("pr21", &self.pr21())
            .field("pr22", &self.pr22())
            .field("pr23", &self.pr23())
            .finish()
    }
}
impl W {
    ///Pending bit (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PR0` field.</div>
    #[inline(always)]
    pub fn pr(&mut self, n: u8) -> PR_W<'_, PRrs> {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        PR_W::new(self, n)
    }
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pr0(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 0)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pr1(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 1)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pr2(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 2)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pr3(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 3)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pr4(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 4)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pr5(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 5)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pr6(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 6)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pr7(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 7)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pr8(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 8)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pr9(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 9)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pr10(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 10)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pr11(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 11)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pr12(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 12)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pr13(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 13)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pr14(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 14)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pr15(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 15)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pr16(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 16)
    }
    ///Bit 17 - Pending bit 17
    #[inline(always)]
    pub fn pr17(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 17)
    }
    ///Bit 18 - Pending bit 18
    #[inline(always)]
    pub fn pr18(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 18)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pr19(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 19)
    }
    ///Bit 20 - Pending bit 20
    #[inline(always)]
    pub fn pr20(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 20)
    }
    ///Bit 21 - Pending bit 21
    #[inline(always)]
    pub fn pr21(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 21)
    }
    ///Bit 22 - Pending bit 22
    #[inline(always)]
    pub fn pr22(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 22)
    }
    ///Bit 23 - Pending bit 23
    #[inline(always)]
    pub fn pr23(&mut self) -> PR_W<'_, PRrs> {
        PR_W::new(self, 23)
    }
}
/**Pending register (EXTI_PR)

You can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#EXTI:PR)*/
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
///`read()` method returns [`pr::R`](R) reader structure
impl crate::Readable for PRrs {}
///`write(|w| ..)` method takes [`pr::W`](W) writer structure
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
///`reset()` method sets PR to value 0
impl crate::Resettable for PRrs {}
