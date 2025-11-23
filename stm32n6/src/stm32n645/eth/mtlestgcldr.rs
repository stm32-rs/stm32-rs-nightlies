///Register `MTLESTGCLDR` reader
pub type R = crate::R<MTLESTGCLDRrs>;
///Register `MTLESTGCLDR` writer
pub type W = crate::W<MTLESTGCLDRrs>;
///Field `GCD` reader - Gate Control Data
pub type GCD_R = crate::FieldReader<u32>;
///Field `GCD` writer - Gate Control Data
pub type GCD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Gate Control Data
    #[inline(always)]
    pub fn gcd(&self) -> GCD_R {
        GCD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTGCLDR")
            .field("gcd", &self.gcd())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Gate Control Data
    #[inline(always)]
    pub fn gcd(&mut self) -> GCD_W<'_, MTLESTGCLDRrs> {
        GCD_W::new(self, 0)
    }
}
/**EST Gate Control List Data Register

You can [`read`](crate::Reg::read) this register and get [`mtlestgcldr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestgcldr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLESTGCLDR)*/
pub struct MTLESTGCLDRrs;
impl crate::RegisterSpec for MTLESTGCLDRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestgcldr::R`](R) reader structure
impl crate::Readable for MTLESTGCLDRrs {}
///`write(|w| ..)` method takes [`mtlestgcldr::W`](W) writer structure
impl crate::Writable for MTLESTGCLDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTGCLDR to value 0
impl crate::Resettable for MTLESTGCLDRrs {}
