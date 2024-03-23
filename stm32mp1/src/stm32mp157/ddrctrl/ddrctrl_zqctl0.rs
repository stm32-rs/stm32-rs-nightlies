#[doc = "Register `DDRCTRL_ZQCTL0` reader"]
pub type R = crate::R<DDRCTRL_ZQCTL0rs>;
#[doc = "Register `DDRCTRL_ZQCTL0` writer"]
pub type W = crate::W<DDRCTRL_ZQCTL0rs>;
#[doc = "Field `T_ZQ_SHORT_NOP` reader - T_ZQ_SHORT_NOP"]
pub type T_ZQ_SHORT_NOP_R = crate::FieldReader<u16>;
#[doc = "Field `T_ZQ_SHORT_NOP` writer - T_ZQ_SHORT_NOP"]
pub type T_ZQ_SHORT_NOP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `T_ZQ_LONG_NOP` reader - T_ZQ_LONG_NOP"]
pub type T_ZQ_LONG_NOP_R = crate::FieldReader<u16>;
#[doc = "Field `T_ZQ_LONG_NOP` writer - T_ZQ_LONG_NOP"]
pub type T_ZQ_LONG_NOP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ZQ_RESISTOR_SHARED` reader - ZQ_RESISTOR_SHARED"]
pub type ZQ_RESISTOR_SHARED_R = crate::BitReader;
#[doc = "Field `ZQ_RESISTOR_SHARED` writer - ZQ_RESISTOR_SHARED"]
pub type ZQ_RESISTOR_SHARED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_SRX_ZQCL` reader - DIS_SRX_ZQCL"]
pub type DIS_SRX_ZQCL_R = crate::BitReader;
#[doc = "Field `DIS_SRX_ZQCL` writer - DIS_SRX_ZQCL"]
pub type DIS_SRX_ZQCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_AUTO_ZQ` reader - DIS_AUTO_ZQ"]
pub type DIS_AUTO_ZQ_R = crate::BitReader;
#[doc = "Field `DIS_AUTO_ZQ` writer - DIS_AUTO_ZQ"]
pub type DIS_AUTO_ZQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    pub fn t_zq_short_nop(&self) -> T_ZQ_SHORT_NOP_R {
        T_ZQ_SHORT_NOP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    pub fn t_zq_long_nop(&self) -> T_ZQ_LONG_NOP_R {
        T_ZQ_LONG_NOP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    pub fn zq_resistor_shared(&self) -> ZQ_RESISTOR_SHARED_R {
        ZQ_RESISTOR_SHARED_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    pub fn dis_srx_zqcl(&self) -> DIS_SRX_ZQCL_R {
        DIS_SRX_ZQCL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    pub fn dis_auto_zq(&self) -> DIS_AUTO_ZQ_R {
        DIS_AUTO_ZQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_short_nop(&mut self) -> T_ZQ_SHORT_NOP_W<DDRCTRL_ZQCTL0rs> {
        T_ZQ_SHORT_NOP_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_long_nop(&mut self) -> T_ZQ_LONG_NOP_W<DDRCTRL_ZQCTL0rs> {
        T_ZQ_LONG_NOP_W::new(self, 16)
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    #[must_use]
    pub fn zq_resistor_shared(&mut self) -> ZQ_RESISTOR_SHARED_W<DDRCTRL_ZQCTL0rs> {
        ZQ_RESISTOR_SHARED_W::new(self, 29)
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    #[must_use]
    pub fn dis_srx_zqcl(&mut self) -> DIS_SRX_ZQCL_W<DDRCTRL_ZQCTL0rs> {
        DIS_SRX_ZQCL_W::new(self, 30)
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_zq(&mut self) -> DIS_AUTO_ZQ_W<DDRCTRL_ZQCTL0rs> {
        DIS_AUTO_ZQ_W::new(self, 31)
    }
}
#[doc = "DDRCTRL ZQ control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_zqctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ZQCTL0rs;
impl crate::RegisterSpec for DDRCTRL_ZQCTL0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_zqctl0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_zqctl0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL0 to value 0x0200_0040"]
impl crate::Resettable for DDRCTRL_ZQCTL0rs {
    const RESET_VALUE: u32 = 0x0200_0040;
}
