///Register `DHTMEM%s` reader
pub type R = crate::R<DHTMEMrs>;
///Register `DHTMEM%s` writer
pub type W = crate::W<DHTMEMrs>;
///Field `DHTMem_RAM` reader - DHTMem RAM
pub type DHTMEM_RAM_R = crate::FieldReader<u32>;
///Field `DHTMem_RAM` writer - DHTMem RAM
pub type DHTMEM_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM")
            .field("dhtmem_ram", &self.dhtmem_ram())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DHTMem RAM
    #[inline(always)]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W<'_, DHTMEMrs> {
        DHTMEM_RAM_W::new(self, 0)
    }
}
/**JPEG DHTMem tables

You can [`read`](crate::Reg::read) this register and get [`dhtmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#JPEG:DHTMEM[0])*/
pub struct DHTMEMrs;
impl crate::RegisterSpec for DHTMEMrs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem::R`](R) reader structure
impl crate::Readable for DHTMEMrs {}
///`write(|w| ..)` method takes [`dhtmem::W`](W) writer structure
impl crate::Writable for DHTMEMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM%s to value 0
impl crate::Resettable for DHTMEMrs {}
