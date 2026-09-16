///Register `TR2` reader
pub type R = crate::R<TR2rs>;
///Register `TR2` writer
pub type W = crate::W<TR2rs>;
///Field `LT2` reader - LT2
pub type LT2_R = crate::FieldReader;
///Field `LT2` writer - LT2
pub type LT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `HT2` reader - HT2
pub type HT2_R = crate::FieldReader;
///Field `HT2` writer - HT2
pub type HT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - LT2
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - HT2
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR2")
            .field("ht2", &self.ht2())
            .field("lt2", &self.lt2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - LT2
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W<'_, TR2rs> {
        LT2_W::new(self, 0)
    }
    ///Bits 16:23 - HT2
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W<'_, TR2rs> {
        HT2_W::new(self, 16)
    }
}
/**watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#ADC1:TR2)*/
pub struct TR2rs;
impl crate::RegisterSpec for TR2rs {
    type Ux = u32;
}
///`read()` method returns [`tr2::R`](R) reader structure
impl crate::Readable for TR2rs {}
///`write(|w| ..)` method takes [`tr2::W`](W) writer structure
impl crate::Writable for TR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR2 to value 0x0fff_0000
impl crate::Resettable for TR2rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
