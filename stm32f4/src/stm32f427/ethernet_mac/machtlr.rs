///Register `MACHTLR` reader
pub type R = crate::R<MACHTLRrs>;
///Register `MACHTLR` writer
pub type W = crate::W<MACHTLRrs>;
///Field `HTL` reader - Lower 32 bits of hash table
pub type HTL_R = crate::FieldReader<u32>;
///Field `HTL` writer - Lower 32 bits of hash table
pub type HTL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Lower 32 bits of hash table
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHTLR").field("htl", &self.htl()).finish()
    }
}
impl W {
    ///Bits 0:31 - Lower 32 bits of hash table
    #[inline(always)]
    pub fn htl(&mut self) -> HTL_W<'_, MACHTLRrs> {
        HTL_W::new(self, 0)
    }
}
/**Ethernet MAC hash table low register

You can [`read`](crate::Reg::read) this register and get [`machtlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machtlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#Ethernet_MAC:MACHTLR)*/
pub struct MACHTLRrs;
impl crate::RegisterSpec for MACHTLRrs {
    type Ux = u32;
}
///`read()` method returns [`machtlr::R`](R) reader structure
impl crate::Readable for MACHTLRrs {}
///`write(|w| ..)` method takes [`machtlr::W`](W) writer structure
impl crate::Writable for MACHTLRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets MACHTLR to value 0
impl crate::Resettable for MACHTLRrs {}
