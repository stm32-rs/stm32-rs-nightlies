///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `RTCPREEN` reader - RTCPRE divider enable
pub type RTCPREEN_R = crate::FieldReader;
///Field `RTCPREEN` writer - RTCPRE divider enable
pub type RTCPREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - RTCPRE divider enable
    #[inline(always)]
    pub fn rtcpreen(&self) -> RTCPREEN_R {
        RTCPREEN_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("rtcpreen", &self.rtcpreen())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - RTCPRE divider enable
    #[inline(always)]
    pub fn rtcpreen(&mut self) -> RTCPREEN_W<'_, OR1rs> {
        RTCPREEN_W::new(self, 0)
    }
}
/**TIM17 option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM17:OR1)*/
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
///`read()` method returns [`or1::R`](R) reader structure
impl crate::Readable for OR1rs {}
///`write(|w| ..)` method takes [`or1::W`](W) writer structure
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1rs {}
