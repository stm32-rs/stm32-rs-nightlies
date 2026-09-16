///Register `MTLISR` reader
pub type R = crate::R<MTLISRrs>;
///Register `MTLISR` writer
pub type W = crate::W<MTLISRrs>;
///Field `Q0IS` reader - Queue interrupt status
pub type Q0IS_R = crate::BitReader;
///Field `Q0IS` writer - Queue interrupt status
pub type Q0IS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Queue interrupt status
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLISR")
            .field("q0is", &self.q0is())
            .finish()
    }
}
impl W {
    ///Bit 0 - Queue interrupt status
    #[inline(always)]
    pub fn q0is(&mut self) -> Q0IS_W<'_, MTLISRrs> {
        Q0IS_W::new(self, 0)
    }
}
/**Interrupt status Register

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#Ethernet_MTL:MTLISR)*/
pub struct MTLISRrs;
impl crate::RegisterSpec for MTLISRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlisr::R`](R) reader structure
impl crate::Readable for MTLISRrs {}
///`write(|w| ..)` method takes [`mtlisr::W`](W) writer structure
impl crate::Writable for MTLISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLISR to value 0
impl crate::Resettable for MTLISRrs {}
