///Register `CRRCR` reader
pub type R = crate::R<CRRCRrs>;
///Register `CRRCR` writer
pub type W = crate::W<CRRCRrs>;
///Field `HSI48ON` reader - HSI48 RC oscillator enable<sup>(1)</sup>
pub type HSI48ON_R = crate::BitReader;
///Field `HSI48ON` writer - HSI48 RC oscillator enable<sup>(1)</sup>
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDY` reader - HSI48 clock ready flag<sup>(1)</sup> The flag is set when the HSI48 clock is ready for use.
pub type HSI48RDY_R = crate::BitReader;
///Field `HSI48CAL` reader - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - HSI48 RC oscillator enable<sup>(1)</sup>
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSI48 clock ready flag<sup>(1)</sup> The flag is set when the HSI48 clock is ready for use.
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 7:15 - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRRCR")
            .field("hsi48on", &self.hsi48on())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("hsi48cal", &self.hsi48cal())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI48 RC oscillator enable<sup>(1)</sup>
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<'_, CRRCRrs> {
        HSI48ON_W::new(self, 0)
    }
}
/**RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RCC:CRRCR)*/
pub struct CRRCRrs;
impl crate::RegisterSpec for CRRCRrs {
    type Ux = u32;
}
///`read()` method returns [`crrcr::R`](R) reader structure
impl crate::Readable for CRRCRrs {}
///`write(|w| ..)` method takes [`crrcr::W`](W) writer structure
impl crate::Writable for CRRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRRCR to value 0x8800
impl crate::Resettable for CRRCRrs {
    const RESET_VALUE: u32 = 0x8800;
}
