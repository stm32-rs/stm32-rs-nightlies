///Register `EMR` reader
pub type R = crate::R<EMRrs>;
///Register `EMR` writer
pub type W = crate::W<EMRrs>;
/**Event Mask on line %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_MASK {
    ///0: Event request line is masked
    Masked = 0,
    ///1: Event request line is unmasked
    Unmasked = 1,
}
impl From<EVENT_MASK> for bool {
    #[inline(always)]
    fn from(variant: EVENT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `EM(0-23)` reader - Event Mask on line %s
pub type EM_R = crate::BitReader<EVENT_MASK>;
impl EM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_MASK {
        match self.bits {
            false => EVENT_MASK::Masked,
            true => EVENT_MASK::Unmasked,
        }
    }
    ///Event request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EVENT_MASK::Masked
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EVENT_MASK::Unmasked
    }
}
///Field `EM(0-23)` writer - Event Mask on line %s
pub type EM_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> EM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Masked)
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Unmasked)
    }
}
impl R {
    ///Event Mask on line (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EM0` field.</div>
    #[inline(always)]
    pub fn em(&self, n: u8) -> EM_R {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        EM_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Event Mask on line (0-23)
    #[inline(always)]
    pub fn em_iter(&self) -> impl Iterator<Item = EM_R> + '_ {
        (0..24).map(move |n| EM_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Event Mask on line 0
    #[inline(always)]
    pub fn em0(&self) -> EM_R {
        EM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Event Mask on line 1
    #[inline(always)]
    pub fn em1(&self) -> EM_R {
        EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Event Mask on line 2
    #[inline(always)]
    pub fn em2(&self) -> EM_R {
        EM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Event Mask on line 3
    #[inline(always)]
    pub fn em3(&self) -> EM_R {
        EM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Event Mask on line 4
    #[inline(always)]
    pub fn em4(&self) -> EM_R {
        EM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Event Mask on line 5
    #[inline(always)]
    pub fn em5(&self) -> EM_R {
        EM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Event Mask on line 6
    #[inline(always)]
    pub fn em6(&self) -> EM_R {
        EM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Event Mask on line 7
    #[inline(always)]
    pub fn em7(&self) -> EM_R {
        EM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Event Mask on line 8
    #[inline(always)]
    pub fn em8(&self) -> EM_R {
        EM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event Mask on line 9
    #[inline(always)]
    pub fn em9(&self) -> EM_R {
        EM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Event Mask on line 10
    #[inline(always)]
    pub fn em10(&self) -> EM_R {
        EM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Event Mask on line 11
    #[inline(always)]
    pub fn em11(&self) -> EM_R {
        EM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Event Mask on line 12
    #[inline(always)]
    pub fn em12(&self) -> EM_R {
        EM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Event Mask on line 13
    #[inline(always)]
    pub fn em13(&self) -> EM_R {
        EM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Event Mask on line 14
    #[inline(always)]
    pub fn em14(&self) -> EM_R {
        EM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Event Mask on line 15
    #[inline(always)]
    pub fn em15(&self) -> EM_R {
        EM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Event Mask on line 16
    #[inline(always)]
    pub fn em16(&self) -> EM_R {
        EM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Event Mask on line 17
    #[inline(always)]
    pub fn em17(&self) -> EM_R {
        EM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Event Mask on line 18
    #[inline(always)]
    pub fn em18(&self) -> EM_R {
        EM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Event Mask on line 19
    #[inline(always)]
    pub fn em19(&self) -> EM_R {
        EM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Event Mask on line 20
    #[inline(always)]
    pub fn em20(&self) -> EM_R {
        EM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Event Mask on line 21
    #[inline(always)]
    pub fn em21(&self) -> EM_R {
        EM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Event Mask on line 22
    #[inline(always)]
    pub fn em22(&self) -> EM_R {
        EM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Event Mask on line 23
    #[inline(always)]
    pub fn em23(&self) -> EM_R {
        EM_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("em4", &self.em4())
            .field("em5", &self.em5())
            .field("em6", &self.em6())
            .field("em7", &self.em7())
            .field("em8", &self.em8())
            .field("em9", &self.em9())
            .field("em10", &self.em10())
            .field("em11", &self.em11())
            .field("em12", &self.em12())
            .field("em13", &self.em13())
            .field("em14", &self.em14())
            .field("em15", &self.em15())
            .field("em16", &self.em16())
            .field("em17", &self.em17())
            .field("em18", &self.em18())
            .field("em19", &self.em19())
            .field("em20", &self.em20())
            .field("em21", &self.em21())
            .field("em22", &self.em22())
            .field("em23", &self.em23())
            .finish()
    }
}
impl W {
    ///Event Mask on line (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EM0` field.</div>
    #[inline(always)]
    pub fn em(&mut self, n: u8) -> EM_W<'_, EMRrs> {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        EM_W::new(self, n)
    }
    ///Bit 0 - Event Mask on line 0
    #[inline(always)]
    pub fn em0(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 0)
    }
    ///Bit 1 - Event Mask on line 1
    #[inline(always)]
    pub fn em1(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 1)
    }
    ///Bit 2 - Event Mask on line 2
    #[inline(always)]
    pub fn em2(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 2)
    }
    ///Bit 3 - Event Mask on line 3
    #[inline(always)]
    pub fn em3(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 3)
    }
    ///Bit 4 - Event Mask on line 4
    #[inline(always)]
    pub fn em4(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 4)
    }
    ///Bit 5 - Event Mask on line 5
    #[inline(always)]
    pub fn em5(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 5)
    }
    ///Bit 6 - Event Mask on line 6
    #[inline(always)]
    pub fn em6(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 6)
    }
    ///Bit 7 - Event Mask on line 7
    #[inline(always)]
    pub fn em7(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 7)
    }
    ///Bit 8 - Event Mask on line 8
    #[inline(always)]
    pub fn em8(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 8)
    }
    ///Bit 9 - Event Mask on line 9
    #[inline(always)]
    pub fn em9(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 9)
    }
    ///Bit 10 - Event Mask on line 10
    #[inline(always)]
    pub fn em10(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 10)
    }
    ///Bit 11 - Event Mask on line 11
    #[inline(always)]
    pub fn em11(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 11)
    }
    ///Bit 12 - Event Mask on line 12
    #[inline(always)]
    pub fn em12(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 12)
    }
    ///Bit 13 - Event Mask on line 13
    #[inline(always)]
    pub fn em13(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 13)
    }
    ///Bit 14 - Event Mask on line 14
    #[inline(always)]
    pub fn em14(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 14)
    }
    ///Bit 15 - Event Mask on line 15
    #[inline(always)]
    pub fn em15(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 15)
    }
    ///Bit 16 - Event Mask on line 16
    #[inline(always)]
    pub fn em16(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 16)
    }
    ///Bit 17 - Event Mask on line 17
    #[inline(always)]
    pub fn em17(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 17)
    }
    ///Bit 18 - Event Mask on line 18
    #[inline(always)]
    pub fn em18(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 18)
    }
    ///Bit 19 - Event Mask on line 19
    #[inline(always)]
    pub fn em19(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 19)
    }
    ///Bit 20 - Event Mask on line 20
    #[inline(always)]
    pub fn em20(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 20)
    }
    ///Bit 21 - Event Mask on line 21
    #[inline(always)]
    pub fn em21(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 21)
    }
    ///Bit 22 - Event Mask on line 22
    #[inline(always)]
    pub fn em22(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 22)
    }
    ///Bit 23 - Event Mask on line 23
    #[inline(always)]
    pub fn em23(&mut self) -> EM_W<'_, EMRrs> {
        EM_W::new(self, 23)
    }
}
/**Event mask register (EXTI_EMR)

You can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#EXTI:EMR)*/
pub struct EMRrs;
impl crate::RegisterSpec for EMRrs {
    type Ux = u32;
}
///`read()` method returns [`emr::R`](R) reader structure
impl crate::Readable for EMRrs {}
///`write(|w| ..)` method takes [`emr::W`](W) writer structure
impl crate::Writable for EMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR to value 0
impl crate::Resettable for EMRrs {}
