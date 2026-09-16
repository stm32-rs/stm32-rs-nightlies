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
///Field `IM(0-23)` reader - Interrupt Mask on line %s
pub type IM_R = crate::BitReader<INTERRUPT_MASK>;
impl IM_R {
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
///Field `IM(0-23)` writer - Interrupt Mask on line %s
pub type IM_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> IM_W<'a, REG>
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
    ///Interrupt Mask on line (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IM0` field.</div>
    #[inline(always)]
    pub fn im(&self, n: u8) -> IM_R {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        IM_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt Mask on line (0-23)
    #[inline(always)]
    pub fn im_iter(&self) -> impl Iterator<Item = IM_R> + '_ {
        (0..24).map(move |n| IM_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn im0(&self) -> IM_R {
        IM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn im1(&self) -> IM_R {
        IM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn im2(&self) -> IM_R {
        IM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn im3(&self) -> IM_R {
        IM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn im4(&self) -> IM_R {
        IM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn im5(&self) -> IM_R {
        IM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn im6(&self) -> IM_R {
        IM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn im7(&self) -> IM_R {
        IM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn im8(&self) -> IM_R {
        IM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn im9(&self) -> IM_R {
        IM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn im10(&self) -> IM_R {
        IM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn im11(&self) -> IM_R {
        IM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn im12(&self) -> IM_R {
        IM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn im13(&self) -> IM_R {
        IM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn im14(&self) -> IM_R {
        IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn im15(&self) -> IM_R {
        IM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn im16(&self) -> IM_R {
        IM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn im17(&self) -> IM_R {
        IM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn im18(&self) -> IM_R {
        IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt Mask on line 19
    #[inline(always)]
    pub fn im19(&self) -> IM_R {
        IM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt Mask on line 20
    #[inline(always)]
    pub fn im20(&self) -> IM_R {
        IM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt Mask on line 21
    #[inline(always)]
    pub fn im21(&self) -> IM_R {
        IM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt Mask on line 22
    #[inline(always)]
    pub fn im22(&self) -> IM_R {
        IM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt Mask on line 23
    #[inline(always)]
    pub fn im23(&self) -> IM_R {
        IM_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("im0", &self.im0())
            .field("im1", &self.im1())
            .field("im2", &self.im2())
            .field("im3", &self.im3())
            .field("im4", &self.im4())
            .field("im5", &self.im5())
            .field("im6", &self.im6())
            .field("im7", &self.im7())
            .field("im8", &self.im8())
            .field("im9", &self.im9())
            .field("im10", &self.im10())
            .field("im11", &self.im11())
            .field("im12", &self.im12())
            .field("im13", &self.im13())
            .field("im14", &self.im14())
            .field("im15", &self.im15())
            .field("im16", &self.im16())
            .field("im17", &self.im17())
            .field("im18", &self.im18())
            .field("im19", &self.im19())
            .field("im20", &self.im20())
            .field("im21", &self.im21())
            .field("im22", &self.im22())
            .field("im23", &self.im23())
            .finish()
    }
}
impl W {
    ///Interrupt Mask on line (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IM0` field.</div>
    #[inline(always)]
    pub fn im(&mut self, n: u8) -> IM_W<'_, IMRrs> {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        IM_W::new(self, n)
    }
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn im0(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn im1(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn im2(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn im3(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 3)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn im4(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 4)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn im5(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 5)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn im6(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 6)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn im7(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 7)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn im8(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 8)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn im9(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 9)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn im10(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 10)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn im11(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 11)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn im12(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 12)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn im13(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 13)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn im14(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 14)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn im15(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 15)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn im16(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 16)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn im17(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 17)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn im18(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 18)
    }
    ///Bit 19 - Interrupt Mask on line 19
    #[inline(always)]
    pub fn im19(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 19)
    }
    ///Bit 20 - Interrupt Mask on line 20
    #[inline(always)]
    pub fn im20(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 20)
    }
    ///Bit 21 - Interrupt Mask on line 21
    #[inline(always)]
    pub fn im21(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 21)
    }
    ///Bit 22 - Interrupt Mask on line 22
    #[inline(always)]
    pub fn im22(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 22)
    }
    ///Bit 23 - Interrupt Mask on line 23
    #[inline(always)]
    pub fn im23(&mut self) -> IM_W<'_, IMRrs> {
        IM_W::new(self, 23)
    }
}
/**Interrupt mask register (EXTI_IMR)

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#EXTI:IMR)*/
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
