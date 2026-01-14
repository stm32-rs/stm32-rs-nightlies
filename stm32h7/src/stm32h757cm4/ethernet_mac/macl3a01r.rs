///Register `MACL3A01R` reader
pub type R = crate::R<MACL3A01Rrs>;
///Register `MACL3A01R` writer
pub type W = crate::W<MACL3A01Rrs>;
///Field `L3A01` reader - Layer 3 Address 0 Field
pub type L3A01_R = crate::FieldReader<u32>;
///Field `L3A01` writer - Layer 3 Address 0 Field
pub type L3A01_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 0 Field
    #[inline(always)]
    pub fn l3a01(&self) -> L3A01_R {
        L3A01_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A01R")
            .field("l3a01", &self.l3a01())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 0 Field
    #[inline(always)]
    pub fn l3a01(&mut self) -> L3A01_W<'_, MACL3A01Rrs> {
        L3A01_W::new(self, 0)
    }
}
/**Layer3 address 0 filter 1 Register

You can [`read`](crate::Reg::read) this register and get [`macl3a01r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a01r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_MAC:MACL3A01R)*/
pub struct MACL3A01Rrs;
impl crate::RegisterSpec for MACL3A01Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a01r::R`](R) reader structure
impl crate::Readable for MACL3A01Rrs {}
///`write(|w| ..)` method takes [`macl3a01r::W`](W) writer structure
impl crate::Writable for MACL3A01Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A01R to value 0
impl crate::Resettable for MACL3A01Rrs {}
