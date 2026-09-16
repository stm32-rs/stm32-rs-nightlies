///Register `ZQCTL1` reader
pub type R = crate::R<ZQCTL1rs>;
///Register `ZQCTL1` writer
pub type W = crate::W<ZQCTL1rs>;
///Field `T_ZQ_SHORT_INTERVAL_X1024` reader - T_ZQ_SHORT_INTERVAL_X1024
pub type T_ZQ_SHORT_INTERVAL_X1024_R = crate::FieldReader<u32>;
///Field `T_ZQ_SHORT_INTERVAL_X1024` writer - T_ZQ_SHORT_INTERVAL_X1024
pub type T_ZQ_SHORT_INTERVAL_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `T_ZQ_RESET_NOP` reader - T_ZQ_RESET_NOP
pub type T_ZQ_RESET_NOP_R = crate::FieldReader<u16>;
///Field `T_ZQ_RESET_NOP` writer - T_ZQ_RESET_NOP
pub type T_ZQ_RESET_NOP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&self) -> T_ZQ_SHORT_INTERVAL_X1024_R {
        T_ZQ_SHORT_INTERVAL_X1024_R::new(self.bits & 0x000f_ffff)
    }
    ///Bits 20:29 - T_ZQ_RESET_NOP
    #[inline(always)]
    pub fn t_zq_reset_nop(&self) -> T_ZQ_RESET_NOP_R {
        T_ZQ_RESET_NOP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQCTL1")
            .field(
                "t_zq_short_interval_x1024",
                &self.t_zq_short_interval_x1024(),
            )
            .field("t_zq_reset_nop", &self.t_zq_reset_nop())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&mut self) -> T_ZQ_SHORT_INTERVAL_X1024_W<'_, ZQCTL1rs> {
        T_ZQ_SHORT_INTERVAL_X1024_W::new(self, 0)
    }
    ///Bits 20:29 - T_ZQ_RESET_NOP
    #[inline(always)]
    pub fn t_zq_reset_nop(&mut self) -> T_ZQ_RESET_NOP_W<'_, ZQCTL1rs> {
        T_ZQ_RESET_NOP_W::new(self, 20)
    }
}
/**DDRCTRL ZQ control register 1

You can [`read`](crate::Reg::read) this register and get [`zqctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zqctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ZQCTL1)*/
pub struct ZQCTL1rs;
impl crate::RegisterSpec for ZQCTL1rs {
    type Ux = u32;
}
///`read()` method returns [`zqctl1::R`](R) reader structure
impl crate::Readable for ZQCTL1rs {}
///`write(|w| ..)` method takes [`zqctl1::W`](W) writer structure
impl crate::Writable for ZQCTL1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ZQCTL1 to value 0x0200_0100
impl crate::Resettable for ZQCTL1rs {
    const RESET_VALUE: u32 = 0x0200_0100;
}
