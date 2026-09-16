///Register `MTLESTSCHER` reader
pub type R = crate::R<MTLESTSCHERrs>;
///Register `MTLESTSCHER` writer
pub type W = crate::W<MTLESTSCHERrs>;
///Field `SEQN` reader - Schedule Error Queue Number
pub type SEQN_R = crate::FieldReader;
///Field `SEQN` writer - Schedule Error Queue Number
pub type SEQN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Schedule Error Queue Number
    #[inline(always)]
    pub fn seqn(&self) -> SEQN_R {
        SEQN_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTSCHER")
            .field("seqn", &self.seqn())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Schedule Error Queue Number
    #[inline(always)]
    pub fn seqn(&mut self) -> SEQN_W<'_, MTLESTSCHERrs> {
        SEQN_W::new(self, 0)
    }
}
/**EST Schedule Error Register

You can [`read`](crate::Reg::read) this register and get [`mtlestscher::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestscher::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLESTSCHER)*/
pub struct MTLESTSCHERrs;
impl crate::RegisterSpec for MTLESTSCHERrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestscher::R`](R) reader structure
impl crate::Readable for MTLESTSCHERrs {}
///`write(|w| ..)` method takes [`mtlestscher::W`](W) writer structure
impl crate::Writable for MTLESTSCHERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTSCHER to value 0
impl crate::Resettable for MTLESTSCHERrs {}
