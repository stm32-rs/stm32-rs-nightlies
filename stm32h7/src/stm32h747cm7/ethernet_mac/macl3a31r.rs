///Register `MACL3A31R` reader
pub type R = crate::R<MACL3A31Rrs>;
///Register `MACL3A31R` writer
pub type W = crate::W<MACL3A31Rrs>;
///Field `L3A31` reader - Layer 3 Address 3 Field
pub type L3A31_R = crate::FieldReader<u32>;
///Field `L3A31` writer - Layer 3 Address 3 Field
pub type L3A31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 3 Field
    #[inline(always)]
    pub fn l3a31(&self) -> L3A31_R {
        L3A31_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A31R")
            .field("l3a31", &self.l3a31())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 3 Field
    #[inline(always)]
    pub fn l3a31(&mut self) -> L3A31_W<'_, MACL3A31Rrs> {
        L3A31_W::new(self, 0)
    }
}
/**Layer3 address 3 filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3a31r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a31r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#Ethernet_MAC:MACL3A31R)*/
pub struct MACL3A31Rrs;
impl crate::RegisterSpec for MACL3A31Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a31r::R`](R) reader structure
impl crate::Readable for MACL3A31Rrs {}
///`write(|w| ..)` method takes [`macl3a31r::W`](W) writer structure
impl crate::Writable for MACL3A31Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A31R to value 0
impl crate::Resettable for MACL3A31Rrs {}
