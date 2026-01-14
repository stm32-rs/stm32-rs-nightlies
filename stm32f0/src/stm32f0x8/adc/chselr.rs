///Register `CHSELR` reader
pub type R = crate::R<CHSELRrs>;
///Register `CHSELR` writer
pub type W = crate::W<CHSELRrs>;
/**Channel-x selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0 {
    ///0: Input Channel is not selected for conversion
    NotSelected = 0,
    ///1: Input Channel is selected for conversion
    Selected = 1,
}
impl From<CHSEL0> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL(0-18)` reader - Channel-x selection
pub type CHSEL_R = crate::BitReader<CHSEL0>;
impl CHSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL0 {
        match self.bits {
            false => CHSEL0::NotSelected,
            true => CHSEL0::Selected,
        }
    }
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == CHSEL0::NotSelected
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == CHSEL0::Selected
    }
}
///Field `CHSEL(0-18)` writer - Channel-x selection
pub type CHSEL_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL0>;
impl<'a, REG> CHSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::NotSelected)
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::Selected)
    }
}
impl R {
    ///Channel-x selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CHSEL0` field.</div>
    #[inline(always)]
    pub fn chsel(&self, n: u8) -> CHSEL_R {
        #[allow(clippy::no_effect)]
        [(); 19][n as usize];
        CHSEL_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel-x selection
    #[inline(always)]
    pub fn chsel_iter(&self) -> impl Iterator<Item = CHSEL_R> + '_ {
        (0..19).map(move |n| CHSEL_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Channel-x selection
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel-x selection
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel-x selection
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel-x selection
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel-x selection
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel-x selection
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel-x selection
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel-x selection
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel-x selection
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x selection
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x selection
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x selection
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x selection
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x selection
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x selection
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x selection
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x selection
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x selection
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x selection
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR")
            .field("chsel0", &self.chsel0())
            .field("chsel1", &self.chsel1())
            .field("chsel2", &self.chsel2())
            .field("chsel3", &self.chsel3())
            .field("chsel4", &self.chsel4())
            .field("chsel5", &self.chsel5())
            .field("chsel6", &self.chsel6())
            .field("chsel7", &self.chsel7())
            .field("chsel8", &self.chsel8())
            .field("chsel9", &self.chsel9())
            .field("chsel10", &self.chsel10())
            .field("chsel11", &self.chsel11())
            .field("chsel12", &self.chsel12())
            .field("chsel13", &self.chsel13())
            .field("chsel14", &self.chsel14())
            .field("chsel15", &self.chsel15())
            .field("chsel16", &self.chsel16())
            .field("chsel17", &self.chsel17())
            .field("chsel18", &self.chsel18())
            .finish()
    }
}
impl W {
    ///Channel-x selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CHSEL0` field.</div>
    #[inline(always)]
    pub fn chsel(&mut self, n: u8) -> CHSEL_W<'_, CHSELRrs> {
        #[allow(clippy::no_effect)]
        [(); 19][n as usize];
        CHSEL_W::new(self, n)
    }
    ///Bit 0 - Channel-x selection
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 0)
    }
    ///Bit 1 - Channel-x selection
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 1)
    }
    ///Bit 2 - Channel-x selection
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 2)
    }
    ///Bit 3 - Channel-x selection
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 3)
    }
    ///Bit 4 - Channel-x selection
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 4)
    }
    ///Bit 5 - Channel-x selection
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 5)
    }
    ///Bit 6 - Channel-x selection
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 6)
    }
    ///Bit 7 - Channel-x selection
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 7)
    }
    ///Bit 8 - Channel-x selection
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 8)
    }
    ///Bit 9 - Channel-x selection
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 9)
    }
    ///Bit 10 - Channel-x selection
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 10)
    }
    ///Bit 11 - Channel-x selection
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 11)
    }
    ///Bit 12 - Channel-x selection
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 12)
    }
    ///Bit 13 - Channel-x selection
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 13)
    }
    ///Bit 14 - Channel-x selection
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 14)
    }
    ///Bit 15 - Channel-x selection
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 15)
    }
    ///Bit 16 - Channel-x selection
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 16)
    }
    ///Bit 17 - Channel-x selection
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 17)
    }
    ///Bit 18 - Channel-x selection
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL_W<'_, CHSELRrs> {
        CHSEL_W::new(self, 18)
    }
}
/**channel selection register

You can [`read`](crate::Reg::read) this register and get [`chselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#ADC:CHSELR)*/
pub struct CHSELRrs;
impl crate::RegisterSpec for CHSELRrs {
    type Ux = u32;
}
///`read()` method returns [`chselr::R`](R) reader structure
impl crate::Readable for CHSELRrs {}
///`write(|w| ..)` method takes [`chselr::W`](W) writer structure
impl crate::Writable for CHSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CHSELR to value 0
impl crate::Resettable for CHSELRrs {}
