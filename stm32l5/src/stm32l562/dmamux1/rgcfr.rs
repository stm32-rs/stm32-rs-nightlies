///Register `RGCFR` reader
pub type R = crate::R<RGCFRrs>;
///Register `RGCFR` writer
pub type W = crate::W<RGCFRrs>;
/**Generator Clear Overrun Flag %s

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
///Field `COF(0-3)` reader - Generator Clear Overrun Flag %s
pub type COF_R = crate::BitReader<COF0W>;
impl COF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<COF0W> {
        match self.bits {
            true => Some(COF0W::Clear),
            _ => None,
        }
    }
    ///Clear overrun flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COF0W::Clear
    }
}
///Field `COF(0-3)` writer - Generator Clear Overrun Flag %s
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
impl R {
    ///Generator Clear Overrun Flag (0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `COF0` field.</div>
    #[inline(always)]
    pub fn cof(&self, n: u8) -> COF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        COF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Generator Clear Overrun Flag (0-3)
    #[inline(always)]
    pub fn cof_iter(&self) -> impl Iterator<Item = COF_R> + '_ {
        (0..4).map(move |n| COF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Generator Clear Overrun Flag 0
    #[inline(always)]
    pub fn cof0(&self) -> COF_R {
        COF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Generator Clear Overrun Flag 1
    #[inline(always)]
    pub fn cof1(&self) -> COF_R {
        COF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Generator Clear Overrun Flag 2
    #[inline(always)]
    pub fn cof2(&self) -> COF_R {
        COF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Generator Clear Overrun Flag 3
    #[inline(always)]
    pub fn cof3(&self) -> COF_R {
        COF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGCFR")
            .field("cof0", &self.cof0())
            .field("cof1", &self.cof1())
            .field("cof2", &self.cof2())
            .field("cof3", &self.cof3())
            .finish()
    }
}
impl W {
    ///Generator Clear Overrun Flag (0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `COF0` field.</div>
    #[inline(always)]
    pub fn cof(&mut self, n: u8) -> COF_W<'_, RGCFRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        COF_W::new(self, n)
    }
    ///Bit 0 - Generator Clear Overrun Flag 0
    #[inline(always)]
    pub fn cof0(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 0)
    }
    ///Bit 1 - Generator Clear Overrun Flag 1
    #[inline(always)]
    pub fn cof1(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 1)
    }
    ///Bit 2 - Generator Clear Overrun Flag 2
    #[inline(always)]
    pub fn cof2(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 2)
    }
    ///Bit 3 - Generator Clear Overrun Flag 3
    #[inline(always)]
    pub fn cof3(&mut self) -> COF_W<'_, RGCFRrs> {
        COF_W::new(self, 3)
    }
}
/**DMA Request Generator Clear Flag Register

You can [`read`](crate::Reg::read) this register and get [`rgcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#DMAMUX1:RGCFR)*/
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
///`read()` method returns [`rgcfr::R`](R) reader structure
impl crate::Readable for RGCFRrs {}
///`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
///`reset()` method sets RGCFR to value 0
impl crate::Resettable for RGCFRrs {}
