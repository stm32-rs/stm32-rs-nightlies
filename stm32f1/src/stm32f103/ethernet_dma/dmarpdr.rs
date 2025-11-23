///Register `DMARPDR` reader
pub type R = crate::R<DMARPDRrs>;
///Register `DMARPDR` writer
pub type W = crate::W<DMARPDRrs>;
///Field `RPD` reader - Receive poll demand
pub type RPD_R = crate::FieldReader<u32>;
///Field `RPD` writer - Receive poll demand
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Receive poll demand
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARPDR").field("rpd", &self.rpd()).finish()
    }
}
impl W {
    ///Bits 0:31 - Receive poll demand
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W<'_, DMARPDRrs> {
        RPD_W::new(self, 0)
    }
}
/**EHERNET DMA receive poll demand register

You can [`read`](crate::Reg::read) this register and get [`dmarpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_DMA:DMARPDR)*/
pub struct DMARPDRrs;
impl crate::RegisterSpec for DMARPDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmarpdr::R`](R) reader structure
impl crate::Readable for DMARPDRrs {}
///`write(|w| ..)` method takes [`dmarpdr::W`](W) writer structure
impl crate::Writable for DMARPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMARPDR to value 0
impl crate::Resettable for DMARPDRrs {}
