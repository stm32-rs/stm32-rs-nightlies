///Register `AGC4_CTRL` reader
pub type R = crate::R<AGC4_CTRLrs>;
///Register `AGC4_CTRL` writer
pub type W = crate::W<AGC4_CTRLrs>;
///Field `AGC_FREEZE_THR` reader - Signal threshold for the autofreeze feature.
pub type AGC_FREEZE_THR_R = crate::FieldReader;
///Field `AGC_FREEZE_THR` writer - Signal threshold for the autofreeze feature.
pub type AGC_FREEZE_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Signal threshold for the autofreeze feature.
    #[inline(always)]
    pub fn agc_freeze_thr(&self) -> AGC_FREEZE_THR_R {
        AGC_FREEZE_THR_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC4_CTRL")
            .field("agc_freeze_thr", &self.agc_freeze_thr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Signal threshold for the autofreeze feature.
    #[inline(always)]
    pub fn agc_freeze_thr(&mut self) -> AGC_FREEZE_THR_W<'_, AGC4_CTRLrs> {
        AGC_FREEZE_THR_W::new(self, 0)
    }
}
/**AGC4_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC4_CTRL)*/
pub struct AGC4_CTRLrs;
impl crate::RegisterSpec for AGC4_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`agc4_ctrl::R`](R) reader structure
impl crate::Readable for AGC4_CTRLrs {}
///`write(|w| ..)` method takes [`agc4_ctrl::W`](W) writer structure
impl crate::Writable for AGC4_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC4_CTRL to value 0x02
impl crate::Resettable for AGC4_CTRLrs {
    const RESET_VALUE: u32 = 0x02;
}
