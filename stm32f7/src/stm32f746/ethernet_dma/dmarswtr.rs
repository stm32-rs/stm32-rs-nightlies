///Register `DMARSWTR` reader
pub type R = crate::R<DMARSWTRrs>;
///Register `DMARSWTR` writer
pub type W = crate::W<DMARSWTRrs>;
///Field `RSWTC` reader - Receive status watchdog timer count
pub type RSWTC_R = crate::FieldReader;
///Field `RSWTC` writer - Receive status watchdog timer count
pub type RSWTC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Receive status watchdog timer count
    #[inline(always)]
    pub fn rswtc(&self) -> RSWTC_R {
        RSWTC_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARSWTR")
            .field("rswtc", &self.rswtc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Receive status watchdog timer count
    #[inline(always)]
    pub fn rswtc(&mut self) -> RSWTC_W<'_, DMARSWTRrs> {
        RSWTC_W::new(self, 0)
    }
}
/**Ethernet DMA receive status watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmarswtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarswtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_DMA:DMARSWTR)*/
pub struct DMARSWTRrs;
impl crate::RegisterSpec for DMARSWTRrs {
    type Ux = u32;
}
///`read()` method returns [`dmarswtr::R`](R) reader structure
impl crate::Readable for DMARSWTRrs {}
///`write(|w| ..)` method takes [`dmarswtr::W`](W) writer structure
impl crate::Writable for DMARSWTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMARSWTR to value 0
impl crate::Resettable for DMARSWTRrs {}
