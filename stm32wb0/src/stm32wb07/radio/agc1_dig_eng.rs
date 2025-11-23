///Register `AGC1_DIG_ENG` reader
pub type R = crate::R<AGC1_DIG_ENGrs>;
///Register `AGC1_DIG_ENG` writer
pub type W = crate::W<AGC1_DIG_ENGrs>;
///Field `AGC_THR_LOW_6` reader - Low threshold for 6dB steps
pub type AGC_THR_LOW_6_R = crate::FieldReader;
///Field `AGC_THR_LOW_6` writer - Low threshold for 6dB steps
pub type AGC_THR_LOW_6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `AGC_AUTOLOCK` reader - AGC locks when level is steady between high threshold and lock threshold
pub type AGC_AUTOLOCK_R = crate::BitReader;
///Field `AGC_AUTOLOCK` writer - AGC locks when level is steady between high threshold and lock threshold
pub type AGC_AUTOLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_LOCK_SYNC` reader - AGC locks when Access Address is detected (recommended)
pub type AGC_LOCK_SYNC_R = crate::BitReader;
///Field `AGC_LOCK_SYNC` writer - AGC locks when Access Address is detected (recommended)
pub type AGC_LOCK_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Low threshold for 6dB steps
    #[inline(always)]
    pub fn agc_thr_low_6(&self) -> AGC_THR_LOW_6_R {
        AGC_THR_LOW_6_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - AGC locks when level is steady between high threshold and lock threshold
    #[inline(always)]
    pub fn agc_autolock(&self) -> AGC_AUTOLOCK_R {
        AGC_AUTOLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AGC locks when Access Address is detected (recommended)
    #[inline(always)]
    pub fn agc_lock_sync(&self) -> AGC_LOCK_SYNC_R {
        AGC_LOCK_SYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC1_DIG_ENG")
            .field("agc_thr_low_6", &self.agc_thr_low_6())
            .field("agc_autolock", &self.agc_autolock())
            .field("agc_lock_sync", &self.agc_lock_sync())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Low threshold for 6dB steps
    #[inline(always)]
    pub fn agc_thr_low_6(&mut self) -> AGC_THR_LOW_6_W<'_, AGC1_DIG_ENGrs> {
        AGC_THR_LOW_6_W::new(self, 0)
    }
    ///Bit 6 - AGC locks when level is steady between high threshold and lock threshold
    #[inline(always)]
    pub fn agc_autolock(&mut self) -> AGC_AUTOLOCK_W<'_, AGC1_DIG_ENGrs> {
        AGC_AUTOLOCK_W::new(self, 6)
    }
    ///Bit 7 - AGC locks when Access Address is detected (recommended)
    #[inline(always)]
    pub fn agc_lock_sync(&mut self) -> AGC_LOCK_SYNC_W<'_, AGC1_DIG_ENGrs> {
        AGC_LOCK_SYNC_W::new(self, 7)
    }
}
/**AGC1_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`agc1_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc1_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:AGC1_DIG_ENG)*/
pub struct AGC1_DIG_ENGrs;
impl crate::RegisterSpec for AGC1_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`agc1_dig_eng::R`](R) reader structure
impl crate::Readable for AGC1_DIG_ENGrs {}
///`write(|w| ..)` method takes [`agc1_dig_eng::W`](W) writer structure
impl crate::Writable for AGC1_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC1_DIG_ENG to value 0x84
impl crate::Resettable for AGC1_DIG_ENGrs {
    const RESET_VALUE: u32 = 0x84;
}
