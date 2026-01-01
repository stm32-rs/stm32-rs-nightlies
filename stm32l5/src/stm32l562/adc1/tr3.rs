///Register `TR3` reader
pub type R = crate::R<TR3rs>;
///Register `TR3` writer
pub type W = crate::W<TR3rs>;
///Field `LT3` reader - LT3
pub type LT3_R = crate::FieldReader;
///Field `LT3` writer - LT3
pub type LT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `HT3` reader - HT3
pub type HT3_R = crate::FieldReader;
///Field `HT3` writer - HT3
pub type HT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - LT3
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - HT3
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR3")
            .field("ht3", &self.ht3())
            .field("lt3", &self.lt3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - LT3
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W<'_, TR3rs> {
        LT3_W::new(self, 0)
    }
    ///Bits 16:23 - HT3
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W<'_, TR3rs> {
        HT3_W::new(self, 16)
    }
}
/**watchdog threshold register 3

You can [`read`](crate::Reg::read) this register and get [`tr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#ADC1:TR3)*/
pub struct TR3rs;
impl crate::RegisterSpec for TR3rs {
    type Ux = u32;
}
///`read()` method returns [`tr3::R`](R) reader structure
impl crate::Readable for TR3rs {}
///`write(|w| ..)` method takes [`tr3::W`](W) writer structure
impl crate::Writable for TR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR3 to value 0x0fff_0000
impl crate::Resettable for TR3rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
