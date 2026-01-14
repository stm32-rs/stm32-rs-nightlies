///Register `M5_FN_MOD2` reader
pub type R = crate::R<M5_FN_MOD2rs>;
///Register `M5_FN_MOD2` writer
pub type W = crate::W<M5_FN_MOD2rs>;
///Field `BYPASS_MERGE` reader - BYPASS_MERGE
pub type BYPASS_MERGE_R = crate::BitReader;
///Field `BYPASS_MERGE` writer - BYPASS_MERGE
pub type BYPASS_MERGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BYPASS_MERGE
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5_FN_MOD2")
            .field("bypass_merge", &self.bypass_merge())
            .finish()
    }
}
impl W {
    ///Bit 0 - BYPASS_MERGE
    #[inline(always)]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W<'_, M5_FN_MOD2rs> {
        BYPASS_MERGE_W::new(self, 0)
    }
}
/**AXIMC master 5 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m5_fn_mod2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5_fn_mod2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#AXIMC_Mx:M5_FN_MOD2)*/
pub struct M5_FN_MOD2rs;
impl crate::RegisterSpec for M5_FN_MOD2rs {
    type Ux = u32;
}
///`read()` method returns [`m5_fn_mod2::R`](R) reader structure
impl crate::Readable for M5_FN_MOD2rs {}
///`write(|w| ..)` method takes [`m5_fn_mod2::W`](W) writer structure
impl crate::Writable for M5_FN_MOD2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5_FN_MOD2 to value 0
impl crate::Resettable for M5_FN_MOD2rs {}
