///Register `PTCR1` reader
pub type R = crate::R<PTCR1rs>;
///Register `PTCR1` writer
pub type W = crate::W<PTCR1rs>;
///Field `TDI` reader - Test-interface data in
pub type TDI_R = crate::FieldReader;
///Field `TDI` writer - Test-interface data in
pub type TDI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TWM` reader - Test-interface write mode selector
pub type TWM_R = crate::BitReader;
///Field `TWM` writer - Test-interface write mode selector
pub type TWM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Test-interface data in
    #[inline(always)]
    pub fn tdi(&self) -> TDI_R {
        TDI_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - Test-interface write mode selector
    #[inline(always)]
    pub fn twm(&self) -> TWM_R {
        TWM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTCR1")
            .field("tdi", &self.tdi())
            .field("twm", &self.twm())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Test-interface data in
    #[inline(always)]
    pub fn tdi(&mut self) -> TDI_W<'_, PTCR1rs> {
        TDI_W::new(self, 0)
    }
    ///Bit 16 - Test-interface write mode selector
    #[inline(always)]
    pub fn twm(&mut self) -> TWM_W<'_, PTCR1rs> {
        TWM_W::new(self, 16)
    }
}
/**CSI PHY test control register 1

You can [`read`](crate::Reg::read) this register and get [`ptcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:PTCR1)*/
pub struct PTCR1rs;
impl crate::RegisterSpec for PTCR1rs {
    type Ux = u32;
}
///`read()` method returns [`ptcr1::R`](R) reader structure
impl crate::Readable for PTCR1rs {}
///`write(|w| ..)` method takes [`ptcr1::W`](W) writer structure
impl crate::Writable for PTCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTCR1 to value 0
impl crate::Resettable for PTCR1rs {}
