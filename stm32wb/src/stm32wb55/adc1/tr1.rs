///Register `TR1` reader
pub type R = crate::R<TR1rs>;
///Register `TR1` writer
pub type W = crate::W<TR1rs>;
///Field `LT1` reader - ADC analog watchdog 1 threshold low
pub type LT1_R = crate::FieldReader<u16>;
///Field `LT1` writer - ADC analog watchdog 1 threshold low
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `HT1` reader - ADC analog watchdog 1 threshold high
pub type HT1_R = crate::FieldReader<u16>;
///Field `HT1` writer - ADC analog watchdog 1 threshold high
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - ADC analog watchdog 1 threshold low
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - ADC analog watchdog 1 threshold high
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR1")
            .field("ht1", &self.ht1())
            .field("lt1", &self.lt1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - ADC analog watchdog 1 threshold low
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W<'_, TR1rs> {
        LT1_W::new(self, 0)
    }
    ///Bits 16:27 - ADC analog watchdog 1 threshold high
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W<'_, TR1rs> {
        HT1_W::new(self, 16)
    }
}
/**ADC analog watchdog 1 threshold register

You can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC1:TR1)*/
pub struct TR1rs;
impl crate::RegisterSpec for TR1rs {
    type Ux = u32;
}
///`read()` method returns [`tr1::R`](R) reader structure
impl crate::Readable for TR1rs {}
///`write(|w| ..)` method takes [`tr1::W`](W) writer structure
impl crate::Writable for TR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR1 to value 0x0fff_0000
impl crate::Resettable for TR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
