///Register `CRRCR` reader
pub type R = crate::R<CRRCRrs>;
///Register `CRRCR` writer
pub type W = crate::W<CRRCRrs>;
///Field `HSI48ON` reader - 48MHz HSI clock enable bit
pub type HSI48ON_R = crate::BitReader;
///Field `HSI48ON` writer - 48MHz HSI clock enable bit
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDY` reader - 48MHz HSI clock ready flag
pub type HSI48RDY_R = crate::BitReader;
///Field `HSI48DIV6EN` reader - 48 MHz HSI clock divided by 6 output enable
pub type HSI48DIV6EN_R = crate::BitReader;
///Field `HSI48DIV6EN` writer - 48 MHz HSI clock divided by 6 output enable
pub type HSI48DIV6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48CAL` reader - 48 MHz HSI clock calibration
pub type HSI48CAL_R = crate::FieldReader;
impl R {
    ///Bit 0 - 48MHz HSI clock enable bit
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 48MHz HSI clock ready flag
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 48 MHz HSI clock divided by 6 output enable
    #[inline(always)]
    pub fn hsi48div6en(&self) -> HSI48DIV6EN_R {
        HSI48DIV6EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:15 - 48 MHz HSI clock calibration
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRRCR")
            .field("hsi48cal", &self.hsi48cal())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("hsi48on", &self.hsi48on())
            .field("hsi48div6en", &self.hsi48div6en())
            .finish()
    }
}
impl W {
    ///Bit 0 - 48MHz HSI clock enable bit
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<'_, CRRCRrs> {
        HSI48ON_W::new(self, 0)
    }
    ///Bit 2 - 48 MHz HSI clock divided by 6 output enable
    #[inline(always)]
    pub fn hsi48div6en(&mut self) -> HSI48DIV6EN_W<'_, CRRCRrs> {
        HSI48DIV6EN_W::new(self, 2)
    }
}
/**Clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#RCC:CRRCR)*/
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
///`reset()` method sets CRRCR to value 0
impl crate::Resettable for CRRCRrs {}
