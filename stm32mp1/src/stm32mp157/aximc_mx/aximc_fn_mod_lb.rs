///Register `AXIMC_FN_MOD_LB` reader
pub type R = crate::R<AXIMC_FN_MOD_LBrs>;
///Register `AXIMC_FN_MOD_LB` writer
pub type W = crate::W<AXIMC_FN_MOD_LBrs>;
///Field `FN_MOD_LB` reader - FN_MOD_LB
pub type FN_MOD_LB_R = crate::BitReader;
///Field `FN_MOD_LB` writer - FN_MOD_LB
pub type FN_MOD_LB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FN_MOD_LB
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXIMC_FN_MOD_LB")
            .field("fn_mod_lb", &self.fn_mod_lb())
            .finish()
    }
}
impl W {
    ///Bit 0 - FN_MOD_LB
    #[inline(always)]
    #[must_use]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<AXIMC_FN_MOD_LBrs> {
        FN_MOD_LB_W::new(self, 0)
    }
}
/**AXIMC long burst capability inhibition register

You can [`read`](crate::Reg::read) this register and get [`aximc_fn_mod_lb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aximc_fn_mod_lb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#AXIMC_Mx:AXIMC_FN_MOD_LB)*/
pub struct AXIMC_FN_MOD_LBrs;
impl crate::RegisterSpec for AXIMC_FN_MOD_LBrs {
    type Ux = u32;
}
///`read()` method returns [`aximc_fn_mod_lb::R`](R) reader structure
impl crate::Readable for AXIMC_FN_MOD_LBrs {}
///`write(|w| ..)` method takes [`aximc_fn_mod_lb::W`](W) writer structure
impl crate::Writable for AXIMC_FN_MOD_LBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AXIMC_FN_MOD_LB to value 0
impl crate::Resettable for AXIMC_FN_MOD_LBrs {
    const RESET_VALUE: u32 = 0;
}
