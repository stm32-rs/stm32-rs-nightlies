///Register `TARG1_FN_MOD_LB` reader
pub type R = crate::R<TARG1_FN_MOD_LBrs>;
///Register `TARG1_FN_MOD_LB` writer
pub type W = crate::W<TARG1_FN_MOD_LBrs>;
///Field `FN_MOD_LB` reader - Controls burst breaking of long bursts
pub type FN_MOD_LB_R = crate::BitReader;
///Field `FN_MOD_LB` writer - Controls burst breaking of long bursts
pub type FN_MOD_LB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Controls burst breaking of long bursts
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARG1_FN_MOD_LB")
            .field("fn_mod_lb", &self.fn_mod_lb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Controls burst breaking of long bursts
    #[inline(always)]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<'_, TARG1_FN_MOD_LBrs> {
        FN_MOD_LB_W::new(self, 0)
    }
}
/**AXI interconnect - TARG x long burst functionality modification

You can [`read`](crate::Reg::read) this register and get [`targ1_fn_mod_lb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`targ1_fn_mod_lb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#AXI:TARG1_FN_MOD_LB)*/
pub struct TARG1_FN_MOD_LBrs;
impl crate::RegisterSpec for TARG1_FN_MOD_LBrs {
    type Ux = u32;
}
///`read()` method returns [`targ1_fn_mod_lb::R`](R) reader structure
impl crate::Readable for TARG1_FN_MOD_LBrs {}
///`write(|w| ..)` method takes [`targ1_fn_mod_lb::W`](W) writer structure
impl crate::Writable for TARG1_FN_MOD_LBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TARG1_FN_MOD_LB to value 0x04
impl crate::Resettable for TARG1_FN_MOD_LBrs {
    const RESET_VALUE: u32 = 0x04;
}
