///Register `PDR1` reader
pub type R = crate::R<PDR1rs>;
///Register `PDR1` writer
pub type W = crate::W<PDR1rs>;
///Field `UNLOCKED` reader - Debug Register Unlocked.
pub type UNLOCKED_R = crate::BitReader;
///Field `UNLOCKED` writer - Debug Register Unlocked.
pub type UNLOCKED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDFPWMEN` reader - Step down converter force PWM mode
pub type SDFPWMEN_R = crate::BitReader;
///Field `SDFPWMEN` writer - Step down converter force PWM mode
pub type SDFPWMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_ADC` reader - (Non-User bit)
pub type SYNC_ADC_R = crate::BitReader;
///Field `SYNC_ADC` writer - (Non-User bit)
pub type SYNC_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Debug Register Unlocked.
    #[inline(always)]
    pub fn unlocked(&self) -> UNLOCKED_R {
        UNLOCKED_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Step down converter force PWM mode
    #[inline(always)]
    pub fn sdfpwmen(&self) -> SDFPWMEN_R {
        SDFPWMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - (Non-User bit)
    #[inline(always)]
    pub fn sync_adc(&self) -> SYNC_ADC_R {
        SYNC_ADC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDR1")
            .field("unlocked", &self.unlocked())
            .field("sdfpwmen", &self.sdfpwmen())
            .field("sync_adc", &self.sync_adc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug Register Unlocked.
    #[inline(always)]
    pub fn unlocked(&mut self) -> UNLOCKED_W<'_, PDR1rs> {
        UNLOCKED_W::new(self, 0)
    }
    ///Bit 3 - Step down converter force PWM mode
    #[inline(always)]
    pub fn sdfpwmen(&mut self) -> SDFPWMEN_W<'_, PDR1rs> {
        SDFPWMEN_W::new(self, 3)
    }
    ///Bit 16 - (Non-User bit)
    #[inline(always)]
    pub fn sync_adc(&mut self) -> SYNC_ADC_W<'_, PDR1rs> {
        SYNC_ADC_W::new(self, 16)
    }
}
/**PWR debug register 1

You can [`read`](crate::Reg::read) this register and get [`pdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDR1)*/
pub struct PDR1rs;
impl crate::RegisterSpec for PDR1rs {
    type Ux = u32;
}
///`read()` method returns [`pdr1::R`](R) reader structure
impl crate::Readable for PDR1rs {}
///`write(|w| ..)` method takes [`pdr1::W`](W) writer structure
impl crate::Writable for PDR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDR1 to value 0
impl crate::Resettable for PDR1rs {}
