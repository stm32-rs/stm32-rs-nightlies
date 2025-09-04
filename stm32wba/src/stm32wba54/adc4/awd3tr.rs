///Register `AWD3TR` reader
pub type R = crate::R<AWD3TRrs>;
///Register `AWD3TR` writer
pub type W = crate::W<AWD3TRrs>;
///Field `LT3` reader - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
pub type LT3_R = crate::FieldReader<u16>;
///Field `LT3` writer - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
pub type LT3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HT3` reader - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
pub type HT3_R = crate::FieldReader<u16>;
///Field `HT3` writer - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
pub type HT3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3TR")
            .field("lt3", &self.lt3())
            .field("ht3", &self.ht3())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog 3lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W<AWD3TRrs> {
        LT3_W::new(self, 0)
    }
    ///Bits 16:27 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section 20.4.25: Analog window watchdog on page 638.
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W<AWD3TRrs> {
        HT3_W::new(self, 16)
    }
}
/**ADC watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`awd3tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#ADC4:AWD3TR)*/
pub struct AWD3TRrs;
impl crate::RegisterSpec for AWD3TRrs {
    type Ux = u32;
}
///`read()` method returns [`awd3tr::R`](R) reader structure
impl crate::Readable for AWD3TRrs {}
///`write(|w| ..)` method takes [`awd3tr::W`](W) writer structure
impl crate::Writable for AWD3TRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD3TR to value 0x0fff_0000
impl crate::Resettable for AWD3TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
