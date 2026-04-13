///Register `INI3_FN_MOD_AHB` reader
pub type R = crate::R<INI3_FN_MOD_AHBrs>;
///Register `INI3_FN_MOD_AHB` writer
pub type W = crate::W<INI3_FN_MOD_AHBrs>;
///Field `RD_INC_OVERRIDE` reader - Converts all AHB-Lite write transactions to a series of single beat AXI
pub type RD_INC_OVERRIDE_R = crate::BitReader;
///Field `RD_INC_OVERRIDE` writer - Converts all AHB-Lite write transactions to a series of single beat AXI
pub type RD_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_INC_OVERRIDE` reader - Converts all AHB-Lite read transactions to a series of single beat AXI
pub type WR_INC_OVERRIDE_R = crate::BitReader;
///Field `WR_INC_OVERRIDE` writer - Converts all AHB-Lite read transactions to a series of single beat AXI
pub type WR_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INI3_FN_MOD_AHB")
            .field("rd_inc_override", &self.rd_inc_override())
            .field("wr_inc_override", &self.wr_inc_override())
            .finish()
    }
}
impl W {
    ///Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI
    #[inline(always)]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W<'_, INI3_FN_MOD_AHBrs> {
        RD_INC_OVERRIDE_W::new(self, 0)
    }
    ///Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI
    #[inline(always)]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W<'_, INI3_FN_MOD_AHBrs> {
        WR_INC_OVERRIDE_W::new(self, 1)
    }
}
/**AXI interconnect - INI x AHB functionality modification register

You can [`read`](crate::Reg::read) this register and get [`ini3_fn_mod_ahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ini3_fn_mod_ahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#AXI:INI3_FN_MOD_AHB)*/
pub struct INI3_FN_MOD_AHBrs;
impl crate::RegisterSpec for INI3_FN_MOD_AHBrs {
    type Ux = u32;
}
///`read()` method returns [`ini3_fn_mod_ahb::R`](R) reader structure
impl crate::Readable for INI3_FN_MOD_AHBrs {}
///`write(|w| ..)` method takes [`ini3_fn_mod_ahb::W`](W) writer structure
impl crate::Writable for INI3_FN_MOD_AHBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INI3_FN_MOD_AHB to value 0x04
impl crate::Resettable for INI3_FN_MOD_AHBrs {
    const RESET_VALUE: u32 = 0x04;
}
