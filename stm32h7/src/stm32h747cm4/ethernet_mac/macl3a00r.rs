///Register `MACL3A00R` reader
pub type R = crate::R<MACL3A00Rrs>;
///Register `MACL3A00R` writer
pub type W = crate::W<MACL3A00Rrs>;
///Field `L3A00` reader - Layer 3 Address 0 Field
pub type L3A00_R = crate::FieldReader<u32>;
///Field `L3A00` writer - Layer 3 Address 0 Field
pub type L3A00_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 0 Field
    #[inline(always)]
    pub fn l3a00(&self) -> L3A00_R {
        L3A00_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A00R")
            .field("l3a00", &self.l3a00())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 0 Field
    #[inline(always)]
    pub fn l3a00(&mut self) -> L3A00_W<MACL3A00Rrs> {
        L3A00_W::new(self, 0)
    }
}
/**MACL3A00R

You can [`read`](crate::Reg::read) this register and get [`macl3a00r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a00r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#Ethernet_MAC:MACL3A00R)*/
pub struct MACL3A00Rrs;
impl crate::RegisterSpec for MACL3A00Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a00r::R`](R) reader structure
impl crate::Readable for MACL3A00Rrs {}
///`write(|w| ..)` method takes [`macl3a00r::W`](W) writer structure
impl crate::Writable for MACL3A00Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A00R to value 0
impl crate::Resettable for MACL3A00Rrs {}
