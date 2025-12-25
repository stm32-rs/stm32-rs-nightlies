///Register `MACL3A30` reader
pub type R = crate::R<MACL3A30rs>;
///Register `MACL3A30` writer
pub type W = crate::W<MACL3A30rs>;
///Field `L3A30` reader - Layer 3 Address 3 Field
pub type L3A30_R = crate::FieldReader<u32>;
///Field `L3A30` writer - Layer 3 Address 3 Field
pub type L3A30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 3 Field
    #[inline(always)]
    pub fn l3a30(&self) -> L3A30_R {
        L3A30_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A30")
            .field("l3a30", &self.l3a30())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 3 Field
    #[inline(always)]
    pub fn l3a30(&mut self) -> L3A30_W<'_, MACL3A30rs> {
        L3A30_W::new(self, 0)
    }
}
/**Layer3 Address 3 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#Ethernet_MAC:MACL3A30)*/
pub struct MACL3A30rs;
impl crate::RegisterSpec for MACL3A30rs {
    type Ux = u32;
}
///`read()` method returns [`macl3a30::R`](R) reader structure
impl crate::Readable for MACL3A30rs {}
///`write(|w| ..)` method takes [`macl3a30::W`](W) writer structure
impl crate::Writable for MACL3A30rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A30 to value 0
impl crate::Resettable for MACL3A30rs {}
