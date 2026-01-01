///Register `ZQCTL0` reader
pub type R = crate::R<ZQCTL0rs>;
///Register `ZQCTL0` writer
pub type W = crate::W<ZQCTL0rs>;
///Field `T_ZQ_SHORT_NOP` reader - T_ZQ_SHORT_NOP
pub type T_ZQ_SHORT_NOP_R = crate::FieldReader<u16>;
///Field `T_ZQ_SHORT_NOP` writer - T_ZQ_SHORT_NOP
pub type T_ZQ_SHORT_NOP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `T_ZQ_LONG_NOP` reader - T_ZQ_LONG_NOP
pub type T_ZQ_LONG_NOP_R = crate::FieldReader<u16>;
///Field `T_ZQ_LONG_NOP` writer - T_ZQ_LONG_NOP
pub type T_ZQ_LONG_NOP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `ZQ_RESISTOR_SHARED` reader - ZQ_RESISTOR_SHARED
pub type ZQ_RESISTOR_SHARED_R = crate::BitReader;
///Field `ZQ_RESISTOR_SHARED` writer - ZQ_RESISTOR_SHARED
pub type ZQ_RESISTOR_SHARED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_SRX_ZQCL` reader - DIS_SRX_ZQCL
pub type DIS_SRX_ZQCL_R = crate::BitReader;
///Field `DIS_SRX_ZQCL` writer - DIS_SRX_ZQCL
pub type DIS_SRX_ZQCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_AUTO_ZQ` reader - DIS_AUTO_ZQ
pub type DIS_AUTO_ZQ_R = crate::BitReader;
///Field `DIS_AUTO_ZQ` writer - DIS_AUTO_ZQ
pub type DIS_AUTO_ZQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - T_ZQ_SHORT_NOP
    #[inline(always)]
    pub fn t_zq_short_nop(&self) -> T_ZQ_SHORT_NOP_R {
        T_ZQ_SHORT_NOP_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:26 - T_ZQ_LONG_NOP
    #[inline(always)]
    pub fn t_zq_long_nop(&self) -> T_ZQ_LONG_NOP_R {
        T_ZQ_LONG_NOP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 29 - ZQ_RESISTOR_SHARED
    #[inline(always)]
    pub fn zq_resistor_shared(&self) -> ZQ_RESISTOR_SHARED_R {
        ZQ_RESISTOR_SHARED_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DIS_SRX_ZQCL
    #[inline(always)]
    pub fn dis_srx_zqcl(&self) -> DIS_SRX_ZQCL_R {
        DIS_SRX_ZQCL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DIS_AUTO_ZQ
    #[inline(always)]
    pub fn dis_auto_zq(&self) -> DIS_AUTO_ZQ_R {
        DIS_AUTO_ZQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQCTL0")
            .field("t_zq_short_nop", &self.t_zq_short_nop())
            .field("t_zq_long_nop", &self.t_zq_long_nop())
            .field("zq_resistor_shared", &self.zq_resistor_shared())
            .field("dis_srx_zqcl", &self.dis_srx_zqcl())
            .field("dis_auto_zq", &self.dis_auto_zq())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - T_ZQ_SHORT_NOP
    #[inline(always)]
    pub fn t_zq_short_nop(&mut self) -> T_ZQ_SHORT_NOP_W<'_, ZQCTL0rs> {
        T_ZQ_SHORT_NOP_W::new(self, 0)
    }
    ///Bits 16:26 - T_ZQ_LONG_NOP
    #[inline(always)]
    pub fn t_zq_long_nop(&mut self) -> T_ZQ_LONG_NOP_W<'_, ZQCTL0rs> {
        T_ZQ_LONG_NOP_W::new(self, 16)
    }
    ///Bit 29 - ZQ_RESISTOR_SHARED
    #[inline(always)]
    pub fn zq_resistor_shared(&mut self) -> ZQ_RESISTOR_SHARED_W<'_, ZQCTL0rs> {
        ZQ_RESISTOR_SHARED_W::new(self, 29)
    }
    ///Bit 30 - DIS_SRX_ZQCL
    #[inline(always)]
    pub fn dis_srx_zqcl(&mut self) -> DIS_SRX_ZQCL_W<'_, ZQCTL0rs> {
        DIS_SRX_ZQCL_W::new(self, 30)
    }
    ///Bit 31 - DIS_AUTO_ZQ
    #[inline(always)]
    pub fn dis_auto_zq(&mut self) -> DIS_AUTO_ZQ_W<'_, ZQCTL0rs> {
        DIS_AUTO_ZQ_W::new(self, 31)
    }
}
/**DDRCTRL ZQ control register 0

You can [`read`](crate::Reg::read) this register and get [`zqctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zqctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQCTL0)*/
pub struct ZQCTL0rs;
impl crate::RegisterSpec for ZQCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`zqctl0::R`](R) reader structure
impl crate::Readable for ZQCTL0rs {}
///`write(|w| ..)` method takes [`zqctl0::W`](W) writer structure
impl crate::Writable for ZQCTL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ZQCTL0 to value 0x0200_0040
impl crate::Resettable for ZQCTL0rs {
    const RESET_VALUE: u32 = 0x0200_0040;
}
