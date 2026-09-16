///Register `MTLESTFSER` reader
pub type R = crate::R<MTLESTFSERrs>;
///Register `MTLESTFSER` writer
pub type W = crate::W<MTLESTFSERrs>;
///Field `FEQN` reader - Frame Size Error Queue Number
pub type FEQN_R = crate::FieldReader;
///Field `FEQN` writer - Frame Size Error Queue Number
pub type FEQN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Frame Size Error Queue Number
    #[inline(always)]
    pub fn feqn(&self) -> FEQN_R {
        FEQN_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTFSER")
            .field("feqn", &self.feqn())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Frame Size Error Queue Number
    #[inline(always)]
    pub fn feqn(&mut self) -> FEQN_W<'_, MTLESTFSERrs> {
        FEQN_W::new(self, 0)
    }
}
/**EST Frame size Error Register

You can [`read`](crate::Reg::read) this register and get [`mtlestfser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestfser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLESTFSER)*/
pub struct MTLESTFSERrs;
impl crate::RegisterSpec for MTLESTFSERrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestfser::R`](R) reader structure
impl crate::Readable for MTLESTFSERrs {}
///`write(|w| ..)` method takes [`mtlestfser::W`](W) writer structure
impl crate::Writable for MTLESTFSERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTFSER to value 0
impl crate::Resettable for MTLESTFSERrs {}
