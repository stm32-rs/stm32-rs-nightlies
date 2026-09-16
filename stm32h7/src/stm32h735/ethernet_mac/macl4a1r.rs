///Register `MACL4A1R` reader
pub type R = crate::R<MACL4A1Rrs>;
///Register `MACL4A1R` writer
pub type W = crate::W<MACL4A1Rrs>;
///Field `L4SP1` reader - Layer 4 Source Port Number Field
pub type L4SP1_R = crate::FieldReader<u16>;
///Field `L4SP1` writer - Layer 4 Source Port Number Field
pub type L4SP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `L4DP1` reader - Layer 4 Destination Port Number Field
pub type L4DP1_R = crate::FieldReader<u16>;
///Field `L4DP1` writer - Layer 4 Destination Port Number Field
pub type L4DP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Layer 4 Source Port Number Field
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Layer 4 Destination Port Number Field
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL4A1R")
            .field("l4sp1", &self.l4sp1())
            .field("l4dp1", &self.l4dp1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Layer 4 Source Port Number Field
    #[inline(always)]
    pub fn l4sp1(&mut self) -> L4SP1_W<'_, MACL4A1Rrs> {
        L4SP1_W::new(self, 0)
    }
    ///Bits 16:31 - Layer 4 Destination Port Number Field
    #[inline(always)]
    pub fn l4dp1(&mut self) -> L4DP1_W<'_, MACL4A1Rrs> {
        L4DP1_W::new(self, 16)
    }
}
/**Layer 4 address filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl4a1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#Ethernet_MAC:MACL4A1R)*/
pub struct MACL4A1Rrs;
impl crate::RegisterSpec for MACL4A1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl4a1r::R`](R) reader structure
impl crate::Readable for MACL4A1Rrs {}
///`write(|w| ..)` method takes [`macl4a1r::W`](W) writer structure
impl crate::Writable for MACL4A1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL4A1R to value 0
impl crate::Resettable for MACL4A1Rrs {}
