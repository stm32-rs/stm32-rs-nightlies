///Register `LR_PD_THR_DIG_ENG` reader
pub type R = crate::R<LR_PD_THR_DIG_ENGrs>;
///Register `LR_PD_THR_DIG_ENG` writer
pub type W = crate::W<LR_PD_THR_DIG_ENGrs>;
///Field `LR_PD_THR` reader - preamble detect threshold value
pub type LR_PD_THR_R = crate::FieldReader;
///Field `LR_PD_THR` writer - preamble detect threshold value
pub type LR_PD_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - preamble detect threshold value
    #[inline(always)]
    pub fn lr_pd_thr(&self) -> LR_PD_THR_R {
        LR_PD_THR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LR_PD_THR_DIG_ENG")
            .field("lr_pd_thr", &self.lr_pd_thr())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - preamble detect threshold value
    #[inline(always)]
    pub fn lr_pd_thr(&mut self) -> LR_PD_THR_W<'_, LR_PD_THR_DIG_ENGrs> {
        LR_PD_THR_W::new(self, 0)
    }
}
/**LR_PD_THR_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`lr_pd_thr_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr_pd_thr_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:LR_PD_THR_DIG_ENG)*/
pub struct LR_PD_THR_DIG_ENGrs;
impl crate::RegisterSpec for LR_PD_THR_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`lr_pd_thr_dig_eng::R`](R) reader structure
impl crate::Readable for LR_PD_THR_DIG_ENGrs {}
///`write(|w| ..)` method takes [`lr_pd_thr_dig_eng::W`](W) writer structure
impl crate::Writable for LR_PD_THR_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LR_PD_THR_DIG_ENG to value 0x50
impl crate::Resettable for LR_PD_THR_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x50;
}
