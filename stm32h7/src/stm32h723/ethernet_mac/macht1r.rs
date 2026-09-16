///Register `MACHT1R` reader
pub type R = crate::R<MACHT1Rrs>;
///Register `MACHT1R` writer
pub type W = crate::W<MACHT1Rrs>;
///Field `HT63T32` reader - MAC Hash Table Second 32 Bits
pub type HT63T32_R = crate::FieldReader<u32>;
///Field `HT63T32` writer - MAC Hash Table Second 32 Bits
pub type HT63T32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC Hash Table Second 32 Bits
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHT1R")
            .field("ht63t32", &self.ht63t32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC Hash Table Second 32 Bits
    #[inline(always)]
    pub fn ht63t32(&mut self) -> HT63T32_W<'_, MACHT1Rrs> {
        HT63T32_W::new(self, 0)
    }
}
/**Hash Table 1 register

You can [`read`](crate::Reg::read) this register and get [`macht1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#Ethernet_MAC:MACHT1R)*/
pub struct MACHT1Rrs;
impl crate::RegisterSpec for MACHT1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macht1r::R`](R) reader structure
impl crate::Readable for MACHT1Rrs {}
///`write(|w| ..)` method takes [`macht1r::W`](W) writer structure
impl crate::Writable for MACHT1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACHT1R to value 0
impl crate::Resettable for MACHT1Rrs {}
