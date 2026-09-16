///Register `DMARDLAR` reader
pub type R = crate::R<DMARDLARrs>;
///Register `DMARDLAR` writer
pub type W = crate::W<DMARDLARrs>;
///Field `SRL` reader - SRL
pub type SRL_R = crate::FieldReader<u32>;
///Field `SRL` writer - SRL
pub type SRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SRL
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARDLAR")
            .field("srl", &self.srl())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SRL
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W<'_, DMARDLARrs> {
        SRL_W::new(self, 0)
    }
}
/**Ethernet DMA receive descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmardlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#Ethernet_DMA:DMARDLAR)*/
pub struct DMARDLARrs;
impl crate::RegisterSpec for DMARDLARrs {
    type Ux = u32;
}
///`read()` method returns [`dmardlar::R`](R) reader structure
impl crate::Readable for DMARDLARrs {}
///`write(|w| ..)` method takes [`dmardlar::W`](W) writer structure
impl crate::Writable for DMARDLARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMARDLAR to value 0
impl crate::Resettable for DMARDLARrs {}
