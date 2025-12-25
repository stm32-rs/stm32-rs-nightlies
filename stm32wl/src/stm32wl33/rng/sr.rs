///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `RNGRDY` reader - New Random Value Ready.
pub type RNGRDY_R = crate::BitReader;
///Field `REVCLK` reader - RNGCLK Clock Reveal bit.
pub type REVCLK_R = crate::BitReader;
///Field `FAULT` reader - Fault Reveal bit.
pub type FAULT_R = crate::BitReader;
///Field `FAULT` writer - Fault Reveal bit.
pub type FAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - New Random Value Ready.
    #[inline(always)]
    pub fn rngrdy(&self) -> RNGRDY_R {
        RNGRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RNGCLK Clock Reveal bit.
    #[inline(always)]
    pub fn revclk(&self) -> REVCLK_R {
        REVCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault Reveal bit.
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rngrdy", &self.rngrdy())
            .field("revclk", &self.revclk())
            .field("fault", &self.fault())
            .finish()
    }
}
impl W {
    ///Bit 2 - Fault Reveal bit.
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W<'_, SRrs> {
        FAULT_W::new(self, 2)
    }
}
/**RNG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
