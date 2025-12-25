///Register `PTCR0` reader
pub type R = crate::R<PTCR0rs>;
///Register `PTCR0` writer
pub type W = crate::W<PTCR0rs>;
///Field `TCKEN` reader - Test-interface clock enable for the TDI bus into the PHY
pub type TCKEN_R = crate::BitReader;
///Field `TCKEN` writer - Test-interface clock enable for the TDI bus into the PHY
pub type TCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRSEN` reader - Test-interface reset enable for the TDI bus into the PHY
pub type TRSEN_R = crate::BitReader;
///Field `TRSEN` writer - Test-interface reset enable for the TDI bus into the PHY
pub type TRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Test-interface clock enable for the TDI bus into the PHY
    #[inline(always)]
    pub fn tcken(&self) -> TCKEN_R {
        TCKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Test-interface reset enable for the TDI bus into the PHY
    #[inline(always)]
    pub fn trsen(&self) -> TRSEN_R {
        TRSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTCR0")
            .field("tcken", &self.tcken())
            .field("trsen", &self.trsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Test-interface clock enable for the TDI bus into the PHY
    #[inline(always)]
    pub fn tcken(&mut self) -> TCKEN_W<'_, PTCR0rs> {
        TCKEN_W::new(self, 0)
    }
    ///Bit 1 - Test-interface reset enable for the TDI bus into the PHY
    #[inline(always)]
    pub fn trsen(&mut self) -> TRSEN_W<'_, PTCR0rs> {
        TRSEN_W::new(self, 1)
    }
}
/**CSI PHY test control register 0

You can [`read`](crate::Reg::read) this register and get [`ptcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PTCR0)*/
pub struct PTCR0rs;
impl crate::RegisterSpec for PTCR0rs {
    type Ux = u32;
}
///`read()` method returns [`ptcr0::R`](R) reader structure
impl crate::Readable for PTCR0rs {}
///`write(|w| ..)` method takes [`ptcr0::W`](W) writer structure
impl crate::Writable for PTCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTCR0 to value 0x01
impl crate::Resettable for PTCR0rs {
    const RESET_VALUE: u32 = 0x01;
}
