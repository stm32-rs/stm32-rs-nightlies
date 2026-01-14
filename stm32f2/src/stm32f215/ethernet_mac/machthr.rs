///Register `MACHTHR` reader
pub type R = crate::R<MACHTHRrs>;
///Register `MACHTHR` writer
pub type W = crate::W<MACHTHRrs>;
///Field `HTH` reader - Hash table high
pub type HTH_R = crate::FieldReader<u32>;
///Field `HTH` writer - Hash table high
pub type HTH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Hash table high
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHTHR").field("hth", &self.hth()).finish()
    }
}
impl W {
    ///Bits 0:31 - Hash table high
    #[inline(always)]
    pub fn hth(&mut self) -> HTH_W<'_, MACHTHRrs> {
        HTH_W::new(self, 0)
    }
}
/**Ethernet MAC hash table high register

You can [`read`](crate::Reg::read) this register and get [`machthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`machthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#Ethernet_MAC:MACHTHR)*/
pub struct MACHTHRrs;
impl crate::RegisterSpec for MACHTHRrs {
    type Ux = u32;
}
///`read()` method returns [`machthr::R`](R) reader structure
impl crate::Readable for MACHTHRrs {}
///`write(|w| ..)` method takes [`machthr::W`](W) writer structure
impl crate::Writable for MACHTHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACHTHR to value 0
impl crate::Resettable for MACHTHRrs {}
