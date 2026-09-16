///Register `TR` reader
pub type R = crate::R<TRrs>;
///Register `TR` writer
pub type W = crate::W<TRrs>;
///Field `LT` reader - Analog watchdog lower threshold
pub type LT_R = crate::FieldReader<u16>;
///Field `LT` writer - Analog watchdog lower threshold
pub type LT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HT` reader - Analog watchdog higher threshold
pub type HT_R = crate::FieldReader<u16>;
///Field `HT` writer - Analog watchdog higher threshold
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Analog watchdog lower threshold
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR")
            .field("ht", &self.ht())
            .field("lt", &self.lt())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog lower threshold
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W<'_, TRrs> {
        LT_W::new(self, 0)
    }
    ///Bits 16:27 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<'_, TRrs> {
        HT_W::new(self, 16)
    }
}
/**watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#ADC:TR)*/
pub struct TRrs;
impl crate::RegisterSpec for TRrs {
    type Ux = u32;
}
///`read()` method returns [`tr::R`](R) reader structure
impl crate::Readable for TRrs {}
///`write(|w| ..)` method takes [`tr::W`](W) writer structure
impl crate::Writable for TRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR to value 0x0fff_0000
impl crate::Resettable for TRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
