///Register `WUPA` reader
pub type R = crate::R<WUPArs>;
///Register `WUPA` writer
pub type W = crate::W<WUPArs>;
///Field `WUPA` reader - WUPA\[x\] Wake-up Line PA\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
pub type WUPA_R = crate::FieldReader<u16>;
///Field `WUPA` writer - WUPA\[x\] Wake-up Line PA\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
pub type WUPA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WUPA\[x\] Wake-up Line PA\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
    #[inline(always)]
    pub fn wupa(&self) -> WUPA_R {
        WUPA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUPA").field("wupa", &self.wupa()).finish()
    }
}
impl W {
    ///Bits 0:15 - WUPA\[x\] Wake-up Line PA\[x\] Polarity This bit defines the polarity used for event detection on external wake-up line PA\[x\] - 0: Detection on high level (rising edge) - 1: Detection on low level (falling edge)
    #[inline(always)]
    pub fn wupa(&mut self) -> WUPA_W<'_, WUPArs> {
        WUPA_W::new(self, 0)
    }
}
/**WUPA register

You can [`read`](crate::Reg::read) this register and get [`wupa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUPA)*/
pub struct WUPArs;
impl crate::RegisterSpec for WUPArs {
    type Ux = u32;
}
///`read()` method returns [`wupa::R`](R) reader structure
impl crate::Readable for WUPArs {}
///`write(|w| ..)` method takes [`wupa::W`](W) writer structure
impl crate::Writable for WUPArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUPA to value 0
impl crate::Resettable for WUPArs {}
