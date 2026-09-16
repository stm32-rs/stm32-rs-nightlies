///Register `SMISR` reader
pub type R = crate::R<SMISRrs>;
///Field `MIS(0-7)` reader - masked interrupt status of the secure channel x (x = 7 to 0)
pub type MIS_R = crate::BitReader;
impl R {
    ///masked interrupt status of the secure channel x (x = 7 to 0)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MIS0` field.</div>
    #[inline(always)]
    pub fn mis(&self, n: u8) -> MIS_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MIS_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis_iter(&self) -> impl Iterator<Item = MIS_R> + '_ {
        (0..8).map(move |n| MIS_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis0(&self) -> MIS_R {
        MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis1(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis2(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis3(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis4(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis5(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis6(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt status of the secure channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis7(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMISR")
            .field("mis0", &self.mis0())
            .field("mis1", &self.mis1())
            .field("mis2", &self.mis2())
            .field("mis3", &self.mis3())
            .field("mis4", &self.mis4())
            .field("mis5", &self.mis5())
            .field("mis6", &self.mis6())
            .field("mis7", &self.mis7())
            .finish()
    }
}
/**GPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:SMISR)*/
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
///`read()` method returns [`smisr::R`](R) reader structure
impl crate::Readable for SMISRrs {}
///`reset()` method sets SMISR to value 0
impl crate::Resettable for SMISRrs {}
