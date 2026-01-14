///Register `INI1_FN_MOD2` reader
pub type R = crate::R<INI1_FN_MOD2rs>;
///Register `INI1_FN_MOD2` writer
pub type W = crate::W<INI1_FN_MOD2rs>;
///Field `BYPASS_MERGE` reader - Disables alteration of transactions by the up-sizer unless required by the protocol
pub type BYPASS_MERGE_R = crate::BitReader;
///Field `BYPASS_MERGE` writer - Disables alteration of transactions by the up-sizer unless required by the protocol
pub type BYPASS_MERGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INI1_FN_MOD2")
            .field("bypass_merge", &self.bypass_merge())
            .finish()
    }
}
impl W {
    ///Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol
    #[inline(always)]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W<'_, INI1_FN_MOD2rs> {
        BYPASS_MERGE_W::new(self, 0)
    }
}
/**AXI interconnect - INI x functionality modification 2 register

You can [`read`](crate::Reg::read) this register and get [`ini1_fn_mod2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ini1_fn_mod2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#AXI:INI1_FN_MOD2)*/
pub struct INI1_FN_MOD2rs;
impl crate::RegisterSpec for INI1_FN_MOD2rs {
    type Ux = u32;
}
///`read()` method returns [`ini1_fn_mod2::R`](R) reader structure
impl crate::Readable for INI1_FN_MOD2rs {}
///`write(|w| ..)` method takes [`ini1_fn_mod2::W`](W) writer structure
impl crate::Writable for INI1_FN_MOD2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INI1_FN_MOD2 to value 0x04
impl crate::Resettable for INI1_FN_MOD2rs {
    const RESET_VALUE: u32 = 0x04;
}
