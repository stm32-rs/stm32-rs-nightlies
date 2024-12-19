///Register `RCC_CRRCR` reader
pub type R = crate::R<RCC_CRRCRrs>;
///Register `RCC_CRRCR` writer
pub type W = crate::W<RCC_CRRCRrs>;
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
        f.debug_struct("RCC_CRRCR")
            .field("hsi48on", &self.hsi48on())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("hsi48cal", &self.hsi48cal())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI48 RC oscillator enable<sup>(1)</sup>
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<RCC_CRRCRrs> {
        HSI48ON_W::new(self, 0)
    }
}
/**RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_crrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_CRRCR)*/
pub struct RCC_CRRCRrs;
impl crate::RegisterSpec for RCC_CRRCRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_crrcr::R`](R) reader structure
impl crate::Readable for RCC_CRRCRrs {}
///`write(|w| ..)` method takes [`rcc_crrcr::W`](W) writer structure
impl crate::Writable for RCC_CRRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CRRCR to value 0x8800
impl crate::Resettable for RCC_CRRCRrs {
    const RESET_VALUE: u32 = 0x8800;
}
