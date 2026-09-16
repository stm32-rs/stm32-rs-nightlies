///Register `MACHT0R` reader
pub type R = crate::R<MACHT0Rrs>;
///Register `MACHT0R` writer
pub type W = crate::W<MACHT0Rrs>;
///Field `HT31T0` reader - MAC Hash Table First 32 Bits
pub type HT31T0_R = crate::FieldReader<u32>;
///Field `HT31T0` writer - MAC Hash Table First 32 Bits
pub type HT31T0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC Hash Table First 32 Bits
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHT0R")
            .field("ht31t0", &self.ht31t0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC Hash Table First 32 Bits
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W<'_, MACHT0Rrs> {
        HT31T0_W::new(self, 0)
    }
}
/**Hash Table 0 register

You can [`read`](crate::Reg::read) this register and get [`macht0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACHT0R)*/
pub struct MACHT0Rrs;
impl crate::RegisterSpec for MACHT0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macht0r::R`](R) reader structure
impl crate::Readable for MACHT0Rrs {}
///`write(|w| ..)` method takes [`macht0r::W`](W) writer structure
impl crate::Writable for MACHT0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACHT0R to value 0
impl crate::Resettable for MACHT0Rrs {}
