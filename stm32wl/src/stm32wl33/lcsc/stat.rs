///Register `STAT` reader
pub type R = crate::R<STATrs>;
///Register `STAT` writer
pub type W = crate::W<STATrs>;
///Field `MIN_LCAB_CNT` reader - The Minimum of CMP_LCA_CNT, CMP_LCB_CNT reached during the
pub type MIN_LCAB_CNT_R = crate::FieldReader;
///Field `MAX_LCAB_CNT` reader - The Maximum of CMP_LCA_CNT, CMP_LCB_CNT reached during
pub type MAX_LCAB_CNT_R = crate::FieldReader;
///Field `MIN_LCAB_CNT_BOUND` reader - The Minimum bound of CMP_LCA_COUNT,
pub type MIN_LCAB_CNT_BOUND_R = crate::FieldReader;
///Field `MIN_LCAB_CNT_BOUND` writer - The Minimum bound of CMP_LCA_COUNT,
pub type MIN_LCAB_CNT_BOUND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MAX_LCAB_CNT_BOUND` reader - The Maximum bound of CMP_LCA_COUNT,
pub type MAX_LCAB_CNT_BOUND_R = crate::FieldReader;
///Field `MAX_LCAB_CNT_BOUND` writer - The Maximum bound of CMP_LCA_COUNT,
pub type MAX_LCAB_CNT_BOUND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The Minimum of CMP_LCA_CNT, CMP_LCB_CNT reached during the
    #[inline(always)]
    pub fn min_lcab_cnt(&self) -> MIN_LCAB_CNT_R {
        MIN_LCAB_CNT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - The Maximum of CMP_LCA_CNT, CMP_LCB_CNT reached during
    #[inline(always)]
    pub fn max_lcab_cnt(&self) -> MAX_LCAB_CNT_R {
        MAX_LCAB_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - The Minimum bound of CMP_LCA_COUNT,
    #[inline(always)]
    pub fn min_lcab_cnt_bound(&self) -> MIN_LCAB_CNT_BOUND_R {
        MIN_LCAB_CNT_BOUND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - The Maximum bound of CMP_LCA_COUNT,
    #[inline(always)]
    pub fn max_lcab_cnt_bound(&self) -> MAX_LCAB_CNT_BOUND_R {
        MAX_LCAB_CNT_BOUND_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("min_lcab_cnt", &self.min_lcab_cnt())
            .field("max_lcab_cnt", &self.max_lcab_cnt())
            .field("min_lcab_cnt_bound", &self.min_lcab_cnt_bound())
            .field("max_lcab_cnt_bound", &self.max_lcab_cnt_bound())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - The Minimum bound of CMP_LCA_COUNT,
    #[inline(always)]
    pub fn min_lcab_cnt_bound(&mut self) -> MIN_LCAB_CNT_BOUND_W<'_, STATrs> {
        MIN_LCAB_CNT_BOUND_W::new(self, 16)
    }
    ///Bits 24:31 - The Maximum bound of CMP_LCA_COUNT,
    #[inline(always)]
    pub fn max_lcab_cnt_bound(&mut self) -> MAX_LCAB_CNT_BOUND_W<'_, STATrs> {
        MAX_LCAB_CNT_BOUND_W::new(self, 24)
    }
}
/**LCSC_STAT register

You can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:STAT)*/
pub struct STATrs;
impl crate::RegisterSpec for STATrs {
    type Ux = u32;
}
///`read()` method returns [`stat::R`](R) reader structure
impl crate::Readable for STATrs {}
///`write(|w| ..)` method takes [`stat::W`](W) writer structure
impl crate::Writable for STATrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STAT to value 0xff00_00ff
impl crate::Resettable for STATrs {
    const RESET_VALUE: u32 = 0xff00_00ff;
}
