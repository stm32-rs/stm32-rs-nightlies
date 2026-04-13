///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
/**Clear synchronization overrun event flag

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
///Field `CSOF(0-15)` writer - Clear synchronization overrun event flag
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFR").finish()
    }
}
impl W {
    ///Clear synchronization overrun event flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CSOF0` field.</div>
    #[inline(always)]
    pub fn csof(&mut self, n: u8) -> CSOF_W<'_, CFRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        CSOF_W::new(self, n)
    }
    ///Bit 0 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 0)
    }
    ///Bit 1 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 1)
    }
    ///Bit 2 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 2)
    }
    ///Bit 3 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 3)
    }
    ///Bit 4 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 4)
    }
    ///Bit 5 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 5)
    }
    ///Bit 6 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 6)
    }
    ///Bit 7 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 7)
    }
    ///Bit 8 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 8)
    }
    ///Bit 9 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 9)
    }
    ///Bit 10 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 10)
    }
    ///Bit 11 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 11)
    }
    ///Bit 12 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 12)
    }
    ///Bit 13 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 13)
    }
    ///Bit 14 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof14(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 14)
    }
    ///Bit 15 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof15(&mut self) -> CSOF_W<'_, CFRrs> {
        CSOF_W::new(self, 15)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DMAMUX1:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
///`reset()` method sets CFR to value 0
impl crate::Resettable for CFRrs {}
