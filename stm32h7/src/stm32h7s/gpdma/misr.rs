///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Field `MIS(0-15)` reader - masked interrupt status of channel x
pub type MIS_R = crate::BitReader;
impl R {
    ///masked interrupt status of channel x
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MIS0` field.</div>
    #[inline(always)]
    pub fn mis(&self, n: u8) -> MIS_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MIS_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///masked interrupt status of channel x
    #[inline(always)]
    pub fn mis_iter(&self) -> impl Iterator<Item = MIS_R> + '_ {
        (0..16).map(move |n| MIS_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis0(&self) -> MIS_R {
        MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis1(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis2(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis3(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis4(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis5(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis6(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis7(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis8(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis9(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis10(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis11(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis12(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis13(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis14(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis15(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("mis0", &self.mis0())
            .field("mis1", &self.mis1())
            .field("mis2", &self.mis2())
            .field("mis3", &self.mis3())
            .field("mis4", &self.mis4())
            .field("mis5", &self.mis5())
            .field("mis6", &self.mis6())
            .field("mis7", &self.mis7())
            .field("mis8", &self.mis8())
            .field("mis9", &self.mis9())
            .field("mis10", &self.mis10())
            .field("mis11", &self.mis11())
            .field("mis12", &self.mis12())
            .field("mis13", &self.mis13())
            .field("mis14", &self.mis14())
            .field("mis15", &self.mis15())
            .finish()
    }
}
/**GPDMA masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GPDMA:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}