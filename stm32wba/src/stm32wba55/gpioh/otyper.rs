///Register `OTYPER` reader
pub type R = crate::R<OTYPERrs>;
///Register `OTYPER` writer
pub type W = crate::W<OTYPERrs>;
///Field `OT3` reader - Port H configuration I/O pin 3 This bit is written by software to configure the I/O output type. Access can be protected by GPIOH SEC3.
pub type OT3_R = crate::BitReader;
///Field `OT3` writer - Port H configuration I/O pin 3 This bit is written by software to configure the I/O output type. Access can be protected by GPIOH SEC3.
pub type OT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Port H configuration I/O pin 3 This bit is written by software to configure the I/O output type. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTYPER").field("ot3", &self.ot3()).finish()
    }
}
impl W {
    ///Bit 3 - Port H configuration I/O pin 3 This bit is written by software to configure the I/O output type. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W<OTYPERrs> {
        OT3_W::new(self, 3)
    }
}
/**GPIO port H output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOH:OTYPER)*/
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
///`read()` method returns [`otyper::R`](R) reader structure
impl crate::Readable for OTYPERrs {}
///`write(|w| ..)` method takes [`otyper::W`](W) writer structure
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPERrs {}
