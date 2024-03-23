#[doc = "Register `INI3_FN_MOD_AHB` reader"]
pub type R = crate::R<INI3_FN_MOD_AHBrs>;
#[doc = "Register `INI3_FN_MOD_AHB` writer"]
pub type W = crate::W<INI3_FN_MOD_AHBrs>;
#[doc = "Field `RD_INC_OVERRIDE` reader - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RD_INC_OVERRIDE_R = crate::BitReader;
#[doc = "Field `RD_INC_OVERRIDE` writer - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RD_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_INC_OVERRIDE` reader - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WR_INC_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WR_INC_OVERRIDE` writer - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WR_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    #[must_use]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W<INI3_FN_MOD_AHBrs> {
        RD_INC_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    #[must_use]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W<INI3_FN_MOD_AHBrs> {
        WR_INC_OVERRIDE_W::new(self, 1)
    }
}
#[doc = "AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini3_fn_mod_ahb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini3_fn_mod_ahb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INI3_FN_MOD_AHBrs;
impl crate::RegisterSpec for INI3_FN_MOD_AHBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ini3_fn_mod_ahb::R`](R) reader structure"]
impl crate::Readable for INI3_FN_MOD_AHBrs {}
#[doc = "`write(|w| ..)` method takes [`ini3_fn_mod_ahb::W`](W) writer structure"]
impl crate::Writable for INI3_FN_MOD_AHBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INI3_FN_MOD_AHB to value 0x04"]
impl crate::Resettable for INI3_FN_MOD_AHBrs {
    const RESET_VALUE: u32 = 0x04;
}
