#[doc = "Register `DDRCTRL_ZQCTL1` reader"]
pub type R = crate::R<DDRCTRL_ZQCTL1rs>;
#[doc = "Register `DDRCTRL_ZQCTL1` writer"]
pub type W = crate::W<DDRCTRL_ZQCTL1rs>;
#[doc = "Field `T_ZQ_SHORT_INTERVAL_X1024` reader - T_ZQ_SHORT_INTERVAL_X1024"]
pub type T_ZQ_SHORT_INTERVAL_X1024_R = crate::FieldReader<u32>;
#[doc = "Field `T_ZQ_SHORT_INTERVAL_X1024` writer - T_ZQ_SHORT_INTERVAL_X1024"]
pub type T_ZQ_SHORT_INTERVAL_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `T_ZQ_RESET_NOP` reader - T_ZQ_RESET_NOP"]
pub type T_ZQ_RESET_NOP_R = crate::FieldReader<u16>;
#[doc = "Field `T_ZQ_RESET_NOP` writer - T_ZQ_RESET_NOP"]
pub type T_ZQ_RESET_NOP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024"]
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&self) -> T_ZQ_SHORT_INTERVAL_X1024_R {
        T_ZQ_SHORT_INTERVAL_X1024_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:29 - T_ZQ_RESET_NOP"]
    #[inline(always)]
    pub fn t_zq_reset_nop(&self) -> T_ZQ_RESET_NOP_R {
        T_ZQ_RESET_NOP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_short_interval_x1024(&mut self) -> T_ZQ_SHORT_INTERVAL_X1024_W<DDRCTRL_ZQCTL1rs> {
        T_ZQ_SHORT_INTERVAL_X1024_W::new(self, 0)
    }
    #[doc = "Bits 20:29 - T_ZQ_RESET_NOP"]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_reset_nop(&mut self) -> T_ZQ_RESET_NOP_W<DDRCTRL_ZQCTL1rs> {
        T_ZQ_RESET_NOP_W::new(self, 20)
    }
}
#[doc = "DDRCTRL ZQ control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_zqctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ZQCTL1rs;
impl crate::RegisterSpec for DDRCTRL_ZQCTL1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_zqctl1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_zqctl1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL1 to value 0x0200_0100"]
impl crate::Resettable for DDRCTRL_ZQCTL1rs {
    const RESET_VALUE: u32 = 0x0200_0100;
}
