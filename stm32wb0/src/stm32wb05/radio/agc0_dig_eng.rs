///Register `AGC0_DIG_ENG` reader
pub type R = crate::R<AGC0_DIG_ENGrs>;
///Register `AGC0_DIG_ENG` writer
pub type W = crate::W<AGC0_DIG_ENGrs>;
///Field `AGC_THR_HIGH` reader - High AGC threshold
pub type AGC_THR_HIGH_R = crate::FieldReader;
///Field `AGC_THR_HIGH` writer - High AGC threshold
pub type AGC_THR_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `AGC_ENABLE` reader - Enable AGC
pub type AGC_ENABLE_R = crate::BitReader;
///Field `AGC_ENABLE` writer - Enable AGC
pub type AGC_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - High AGC threshold
    #[inline(always)]
    pub fn agc_thr_high(&self) -> AGC_THR_HIGH_R {
        AGC_THR_HIGH_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Enable AGC
    #[inline(always)]
    pub fn agc_enable(&self) -> AGC_ENABLE_R {
        AGC_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC0_DIG_ENG")
            .field("agc_thr_high", &self.agc_thr_high())
            .field("agc_enable", &self.agc_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - High AGC threshold
    #[inline(always)]
    pub fn agc_thr_high(&mut self) -> AGC_THR_HIGH_W<'_, AGC0_DIG_ENGrs> {
        AGC_THR_HIGH_W::new(self, 0)
    }
    ///Bit 6 - Enable AGC
    #[inline(always)]
    pub fn agc_enable(&mut self) -> AGC_ENABLE_W<'_, AGC0_DIG_ENGrs> {
        AGC_ENABLE_W::new(self, 6)
    }
}
/**AGC0_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc0_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc0_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:AGC0_DIG_ENG)*/
pub struct AGC0_DIG_ENGrs;
impl crate::RegisterSpec for AGC0_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`agc0_dig_eng::R`](R) reader structure
impl crate::Readable for AGC0_DIG_ENGrs {}
///`write(|w| ..)` method takes [`agc0_dig_eng::W`](W) writer structure
impl crate::Writable for AGC0_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC0_DIG_ENG to value 0x4a
impl crate::Resettable for AGC0_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x4a;
}
