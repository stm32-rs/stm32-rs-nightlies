///Register `WUPB` reader
pub type R = crate::R<WUPBrs>;
///Register `WUPB` writer
pub type W = crate::W<WUPBrs>;
///Field `WUPB` reader - WUPB\[x\] Wake-up Line PB\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
pub type WUPB_R = crate::FieldReader<u16>;
///Field `WUPB` writer - WUPB\[x\] Wake-up Line PB\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
pub type WUPB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WUPB\[x\] Wake-up Line PB\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
    #[inline(always)]
    pub fn wupb(&self) -> WUPB_R {
        WUPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUPB").field("wupb", &self.wupb()).finish()
    }
}
impl W {
    ///Bits 0:15 - WUPB\[x\] Wake-up Line PB\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PB\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
    #[inline(always)]
    pub fn wupb(&mut self) -> WUPB_W<'_, WUPBrs> {
        WUPB_W::new(self, 0)
    }
}
/**WUPB register

You can [`read`](crate::Reg::read) this register and get [`wupb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUPB)*/
pub struct WUPBrs;
impl crate::RegisterSpec for WUPBrs {
    type Ux = u32;
}
///`read()` method returns [`wupb::R`](R) reader structure
impl crate::Readable for WUPBrs {}
///`write(|w| ..)` method takes [`wupb::W`](W) writer structure
impl crate::Writable for WUPBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUPB to value 0
impl crate::Resettable for WUPBrs {}
