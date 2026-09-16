///Register `IMR` reader
pub type R = crate::R<IMRrs>;
///Register `IMR` writer
pub type W = crate::W<IMRrs>;
/**Interrupt Mask on line %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `MR(0-22)` reader - Interrupt Mask on line %s
pub type MR_R = crate::BitReader<INTERRUPT_MASK>;
impl MR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK {
        match self.bits {
            false => INTERRUPT_MASK::Masked,
            true => INTERRUPT_MASK::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `MR(0-22)` writer - Interrupt Mask on line %s
pub type MR_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> MR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
impl R {
    ///Interrupt Mask on line (0-22)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MR0` field.</div>
    #[inline(always)]
    pub fn mr(&self, n: u8) -> MR_R {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        MR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt Mask on line (0-22)
    #[inline(always)]
    pub fn mr_iter(&self) -> impl Iterator<Item = MR_R> + '_ {
        (0..23).map(move |n| MR_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn mr0(&self) -> MR_R {
        MR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn mr1(&self) -> MR_R {
        MR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn mr2(&self) -> MR_R {
        MR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn mr3(&self) -> MR_R {
        MR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn mr4(&self) -> MR_R {
        MR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn mr5(&self) -> MR_R {
        MR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn mr6(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn mr7(&self) -> MR_R {
        MR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn mr8(&self) -> MR_R {
        MR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn mr9(&self) -> MR_R {
        MR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn mr10(&self) -> MR_R {
        MR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn mr11(&self) -> MR_R {
        MR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn mr12(&self) -> MR_R {
        MR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn mr13(&self) -> MR_R {
        MR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn mr14(&self) -> MR_R {
        MR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn mr15(&self) -> MR_R {
        MR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn mr16(&self) -> MR_R {
        MR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn mr17(&self) -> MR_R {
        MR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn mr18(&self) -> MR_R {
        MR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt Mask on line 19
    #[inline(always)]
    pub fn mr19(&self) -> MR_R {
        MR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt Mask on line 20
    #[inline(always)]
    pub fn mr20(&self) -> MR_R {
        MR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt Mask on line 21
    #[inline(always)]
    pub fn mr21(&self) -> MR_R {
        MR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt Mask on line 22
    #[inline(always)]
    pub fn mr22(&self) -> MR_R {
        MR_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("mr0", &self.mr0())
            .field("mr1", &self.mr1())
            .field("mr2", &self.mr2())
            .field("mr3", &self.mr3())
            .field("mr4", &self.mr4())
            .field("mr5", &self.mr5())
            .field("mr6", &self.mr6())
            .field("mr7", &self.mr7())
            .field("mr8", &self.mr8())
            .field("mr9", &self.mr9())
            .field("mr10", &self.mr10())
            .field("mr11", &self.mr11())
            .field("mr12", &self.mr12())
            .field("mr13", &self.mr13())
            .field("mr14", &self.mr14())
            .field("mr15", &self.mr15())
            .field("mr16", &self.mr16())
            .field("mr17", &self.mr17())
            .field("mr18", &self.mr18())
            .field("mr19", &self.mr19())
            .field("mr20", &self.mr20())
            .field("mr21", &self.mr21())
            .field("mr22", &self.mr22())
            .finish()
    }
}
impl W {
    ///Interrupt Mask on line (0-22)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MR0` field.</div>
    #[inline(always)]
    pub fn mr(&mut self, n: u8) -> MR_W<'_, IMRrs> {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        MR_W::new(self, n)
    }
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn mr0(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn mr1(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn mr2(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn mr3(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 3)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn mr4(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 4)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn mr5(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 5)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn mr6(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 6)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn mr7(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 7)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn mr8(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 8)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn mr9(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 9)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn mr10(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 10)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn mr11(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 11)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn mr12(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 12)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn mr13(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 13)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn mr14(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 14)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn mr15(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 15)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn mr16(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 16)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn mr17(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 17)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn mr18(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 18)
    }
    ///Bit 19 - Interrupt Mask on line 19
    #[inline(always)]
    pub fn mr19(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 19)
    }
    ///Bit 20 - Interrupt Mask on line 20
    #[inline(always)]
    pub fn mr20(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 20)
    }
    ///Bit 21 - Interrupt Mask on line 21
    #[inline(always)]
    pub fn mr21(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 21)
    }
    ///Bit 22 - Interrupt Mask on line 22
    #[inline(always)]
    pub fn mr22(&mut self) -> MR_W<'_, IMRrs> {
        MR_W::new(self, 22)
    }
}
/**Interrupt mask register (EXTI_IMR)

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#EXTI:IMR)*/
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
///`read()` method returns [`imr::R`](R) reader structure
impl crate::Readable for IMRrs {}
///`write(|w| ..)` method takes [`imr::W`](W) writer structure
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMRrs {}
