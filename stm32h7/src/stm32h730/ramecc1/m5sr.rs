///Register `M5SR` reader
pub type R = crate::R<M5SRrs>;
///Register `M5SR` writer
pub type W = crate::W<M5SRrs>;
///Field `FEC` reader - Failing error code
pub type FEC_R = crate::FieldReader<u32>;
///Field `FEC` writer - Failing error code
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5SR").field("fec", &self.fec()).finish()
    }
}
impl W {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<M5SRrs> {
        FEC_W::new(self, 0)
    }
}
/**RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`m5sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RAMECC1:M5SR)*/
pub struct M5SRrs;
impl crate::RegisterSpec for M5SRrs {
    type Ux = u32;
}
///`read()` method returns [`m5sr::R`](R) reader structure
impl crate::Readable for M5SRrs {}
///`write(|w| ..)` method takes [`m5sr::W`](W) writer structure
impl crate::Writable for M5SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5SR to value 0
impl crate::Resettable for M5SRrs {}
