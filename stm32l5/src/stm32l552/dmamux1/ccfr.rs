///Register `CCFR` reader
pub type R = crate::R<CCFRrs>;
///Register `CCFR` writer
pub type W = crate::W<CCFRrs>;
/**Synchronization Clear Overrun Flag %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSOF0W {
    ///1: Clear synchronization flag
    Clear = 1,
}
impl From<CSOF0W> for bool {
    #[inline(always)]
    fn from(variant: CSOF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `CSOF(0-15)` reader - Synchronization Clear Overrun Flag %s
pub type CSOF_R = crate::BitReader<CSOF0W>;
impl CSOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSOF0W> {
        match self.bits {
            true => Some(CSOF0W::Clear),
            _ => None,
        }
    }
    ///Clear synchronization flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSOF0W::Clear
    }
}
///Field `CSOF(0-15)` writer - Synchronization Clear Overrun Flag %s
pub type CSOF_W<'a, REG> = crate::BitWriter1C<'a, REG, CSOF0W>;
impl<'a, REG> CSOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear synchronization flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSOF0W::Clear)
    }
}
impl R {
    ///Synchronization Clear Overrun Flag (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CSOF0` field.</div>
    #[inline(always)]
    pub fn csof(&self, n: u8) -> CSOF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        CSOF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Synchronization Clear Overrun Flag (0-15)
    #[inline(always)]
    pub fn csof_iter(&self) -> impl Iterator<Item = CSOF_R> + '_ {
        (0..16).map(move |n| CSOF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Synchronization Clear Overrun Flag 0
    #[inline(always)]
    pub fn csof0(&self) -> CSOF_R {
        CSOF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Synchronization Clear Overrun Flag 1
    #[inline(always)]
    pub fn csof1(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization Clear Overrun Flag 2
    #[inline(always)]
    pub fn csof2(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization Clear Overrun Flag 3
    #[inline(always)]
    pub fn csof3(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization Clear Overrun Flag 4
    #[inline(always)]
    pub fn csof4(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization Clear Overrun Flag 5
    #[inline(always)]
    pub fn csof5(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization Clear Overrun Flag 6
    #[inline(always)]
    pub fn csof6(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronization Clear Overrun Flag 7
    #[inline(always)]
    pub fn csof7(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Synchronization Clear Overrun Flag 8
    #[inline(always)]
    pub fn csof8(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Synchronization Clear Overrun Flag 9
    #[inline(always)]
    pub fn csof9(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Synchronization Clear Overrun Flag 10
    #[inline(always)]
    pub fn csof10(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Clear Overrun Flag 11
    #[inline(always)]
    pub fn csof11(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Synchronization Clear Overrun Flag 12
    #[inline(always)]
    pub fn csof12(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Synchronization Clear Overrun Flag 13
    #[inline(always)]
    pub fn csof13(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Synchronization Clear Overrun Flag 14
    #[inline(always)]
    pub fn csof14(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Synchronization Clear Overrun Flag 15
    #[inline(always)]
    pub fn csof15(&self) -> CSOF_R {
        CSOF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFR")
            .field("csof0", &self.csof0())
            .field("csof1", &self.csof1())
            .field("csof2", &self.csof2())
            .field("csof3", &self.csof3())
            .field("csof4", &self.csof4())
            .field("csof5", &self.csof5())
            .field("csof6", &self.csof6())
            .field("csof7", &self.csof7())
            .field("csof8", &self.csof8())
            .field("csof9", &self.csof9())
            .field("csof10", &self.csof10())
            .field("csof11", &self.csof11())
            .field("csof12", &self.csof12())
            .field("csof13", &self.csof13())
            .field("csof14", &self.csof14())
            .field("csof15", &self.csof15())
            .finish()
    }
}
impl W {
    ///Synchronization Clear Overrun Flag (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CSOF0` field.</div>
    #[inline(always)]
    pub fn csof(&mut self, n: u8) -> CSOF_W<'_, CCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        CSOF_W::new(self, n)
    }
    ///Bit 0 - Synchronization Clear Overrun Flag 0
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 0)
    }
    ///Bit 1 - Synchronization Clear Overrun Flag 1
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 1)
    }
    ///Bit 2 - Synchronization Clear Overrun Flag 2
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 2)
    }
    ///Bit 3 - Synchronization Clear Overrun Flag 3
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 3)
    }
    ///Bit 4 - Synchronization Clear Overrun Flag 4
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 4)
    }
    ///Bit 5 - Synchronization Clear Overrun Flag 5
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 5)
    }
    ///Bit 6 - Synchronization Clear Overrun Flag 6
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 6)
    }
    ///Bit 7 - Synchronization Clear Overrun Flag 7
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 7)
    }
    ///Bit 8 - Synchronization Clear Overrun Flag 8
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 8)
    }
    ///Bit 9 - Synchronization Clear Overrun Flag 9
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 9)
    }
    ///Bit 10 - Synchronization Clear Overrun Flag 10
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Clear Overrun Flag 11
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 11)
    }
    ///Bit 12 - Synchronization Clear Overrun Flag 12
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 12)
    }
    ///Bit 13 - Synchronization Clear Overrun Flag 13
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 13)
    }
    ///Bit 14 - Synchronization Clear Overrun Flag 14
    #[inline(always)]
    pub fn csof14(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 14)
    }
    ///Bit 15 - Synchronization Clear Overrun Flag 15
    #[inline(always)]
    pub fn csof15(&mut self) -> CSOF_W<'_, CCFRrs> {
        CSOF_W::new(self, 15)
    }
}
/**DMA Channel Clear Flag Register

You can [`read`](crate::Reg::read) this register and get [`ccfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DMAMUX1:CCFR)*/
pub struct CCFRrs;
impl crate::RegisterSpec for CCFRrs {
    type Ux = u32;
}
///`read()` method returns [`ccfr::R`](R) reader structure
impl crate::Readable for CCFRrs {}
///`write(|w| ..)` method takes [`ccfr::W`](W) writer structure
impl crate::Writable for CCFRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
///`reset()` method sets CCFR to value 0
impl crate::Resettable for CCFRrs {}
