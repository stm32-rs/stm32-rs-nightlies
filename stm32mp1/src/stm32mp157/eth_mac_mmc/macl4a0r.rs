///Register `MACL4A0R` reader
pub type R = crate::R<MACL4A0Rrs>;
///Register `MACL4A0R` writer
pub type W = crate::W<MACL4A0Rrs>;
///Field `L4SP0` reader - L4SP0
pub type L4SP0_R = crate::FieldReader<u16>;
///Field `L4SP0` writer - L4SP0
pub type L4SP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `L4DP0` reader - L4DP0
pub type L4DP0_R = crate::FieldReader<u16>;
///Field `L4DP0` writer - L4DP0
pub type L4DP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - L4SP0
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - L4DP0
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL4A0R")
            .field("l4sp0", &self.l4sp0())
            .field("l4dp0", &self.l4dp0())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - L4SP0
    #[inline(always)]
    pub fn l4sp0(&mut self) -> L4SP0_W<'_, MACL4A0Rrs> {
        L4SP0_W::new(self, 0)
    }
    ///Bits 16:31 - L4DP0
    #[inline(always)]
    pub fn l4dp0(&mut self) -> L4DP0_W<'_, MACL4A0Rrs> {
        L4DP0_W::new(self, 16)
    }
}
/**Layer4 address filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl4a0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACL4A0R)*/
pub struct MACL4A0Rrs;
impl crate::RegisterSpec for MACL4A0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl4a0r::R`](R) reader structure
impl crate::Readable for MACL4A0Rrs {}
///`write(|w| ..)` method takes [`macl4a0r::W`](W) writer structure
impl crate::Writable for MACL4A0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL4A0R to value 0
impl crate::Resettable for MACL4A0Rrs {}
