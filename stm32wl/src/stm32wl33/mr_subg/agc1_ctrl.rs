///Register `AGC1_CTRL` reader
pub type R = crate::R<AGC1_CTRLrs>;
///Register `AGC1_CTRL` writer
pub type W = crate::W<AGC1_CTRLrs>;
///Field `AGC_MIN_THR` reader - Minimum signal threshold.
pub type AGC_MIN_THR_R = crate::FieldReader;
///Field `AGC_MIN_THR` writer - Minimum signal threshold.
pub type AGC_MIN_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AGC_MAX_THR` reader - Maximum signal threshold.
pub type AGC_MAX_THR_R = crate::FieldReader;
///Field `AGC_MAX_THR` writer - Maximum signal threshold.
pub type AGC_MAX_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Minimum signal threshold.
    #[inline(always)]
    pub fn agc_min_thr(&self) -> AGC_MIN_THR_R {
        AGC_MIN_THR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Maximum signal threshold.
    #[inline(always)]
    pub fn agc_max_thr(&self) -> AGC_MAX_THR_R {
        AGC_MAX_THR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC1_CTRL")
            .field("agc_min_thr", &self.agc_min_thr())
            .field("agc_max_thr", &self.agc_max_thr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Minimum signal threshold.
    #[inline(always)]
    pub fn agc_min_thr(&mut self) -> AGC_MIN_THR_W<'_, AGC1_CTRLrs> {
        AGC_MIN_THR_W::new(self, 0)
    }
    ///Bits 4:7 - Maximum signal threshold.
    #[inline(always)]
    pub fn agc_max_thr(&mut self) -> AGC_MAX_THR_W<'_, AGC1_CTRLrs> {
        AGC_MAX_THR_W::new(self, 4)
    }
}
/**AGC1_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC1_CTRL)*/
pub struct AGC1_CTRLrs;
impl crate::RegisterSpec for AGC1_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`agc1_ctrl::R`](R) reader structure
impl crate::Readable for AGC1_CTRLrs {}
///`write(|w| ..)` method takes [`agc1_ctrl::W`](W) writer structure
impl crate::Writable for AGC1_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC1_CTRL to value 0x62
impl crate::Resettable for AGC1_CTRLrs {
    const RESET_VALUE: u32 = 0x62;
}
