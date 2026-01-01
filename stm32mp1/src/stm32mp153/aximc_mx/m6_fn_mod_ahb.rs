///Register `M6_FN_MOD_AHB` reader
pub type R = crate::R<M6_FN_MOD_AHBrs>;
///Register `M6_FN_MOD_AHB` writer
pub type W = crate::W<M6_FN_MOD_AHBrs>;
///Field `RD_INC_OVERRIDE` reader - RD_INC_OVERRIDE
pub type RD_INC_OVERRIDE_R = crate::BitReader;
///Field `RD_INC_OVERRIDE` writer - RD_INC_OVERRIDE
pub type RD_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_INC_OVERRIDE` reader - WR_INC_OVERRIDE
pub type WR_INC_OVERRIDE_R = crate::BitReader;
///Field `WR_INC_OVERRIDE` writer - WR_INC_OVERRIDE
pub type WR_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RD_INC_OVERRIDE
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WR_INC_OVERRIDE
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M6_FN_MOD_AHB")
            .field("rd_inc_override", &self.rd_inc_override())
            .field("wr_inc_override", &self.wr_inc_override())
            .finish()
    }
}
impl W {
    ///Bit 0 - RD_INC_OVERRIDE
    #[inline(always)]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W<'_, M6_FN_MOD_AHBrs> {
        RD_INC_OVERRIDE_W::new(self, 0)
    }
    ///Bit 1 - WR_INC_OVERRIDE
    #[inline(always)]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W<'_, M6_FN_MOD_AHBrs> {
        WR_INC_OVERRIDE_W::new(self, 1)
    }
}
/**AXIMC master 6 AHB conversion override functionality register

You can [`read`](crate::Reg::read) this register and get [`m6_fn_mod_ahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6_fn_mod_ahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M6_FN_MOD_AHB)*/
pub struct M6_FN_MOD_AHBrs;
impl crate::RegisterSpec for M6_FN_MOD_AHBrs {
    type Ux = u32;
}
///`read()` method returns [`m6_fn_mod_ahb::R`](R) reader structure
impl crate::Readable for M6_FN_MOD_AHBrs {}
///`write(|w| ..)` method takes [`m6_fn_mod_ahb::W`](W) writer structure
impl crate::Writable for M6_FN_MOD_AHBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M6_FN_MOD_AHB to value 0
impl crate::Resettable for M6_FN_MOD_AHBrs {}
