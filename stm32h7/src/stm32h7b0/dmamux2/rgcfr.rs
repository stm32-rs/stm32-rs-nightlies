///Register `RGCFR` writer
pub type W = crate::W<RGCFRrs>;
/**Clear trigger overrun event flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COF0W {
    ///1: Clear overrun flag
    Clear = 1,
}
impl From<COF0W> for bool {
    #[inline(always)]
    fn from(variant: COF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `COF(0-7)` writer - Clear trigger overrun event flag
pub type COF_W<'a, REG> = crate::BitWriter1C<'a, REG, COF0W>;
impl<'a, REG> COF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear overrun flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COF0W::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<RGCFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Clear trigger overrun event flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `COF0` field.</div>
    #[inline(always)]
    pub fn cof(&mut self, n: u8) -> COF_W<'_, RGCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        COF_W::new(self, n)
    }
    ///Bit 0 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof0(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 0)
    }
    ///Bit 1 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof1(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 1)
    }
    ///Bit 2 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof2(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 2)
    }
    ///Bit 3 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof3(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 3)
    }
    ///Bit 4 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof4(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 4)
    }
    ///Bit 5 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof5(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 5)
    }
    ///Bit 6 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof6(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 6)
    }
    ///Bit 7 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof7(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 7)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#DMAMUX2:RGCFR)*/
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
///`reset()` method sets RGCFR to value 0
impl crate::Resettable for RGCFRrs {}
