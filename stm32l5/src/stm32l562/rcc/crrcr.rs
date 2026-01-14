///Register `CRRCR` reader
pub type R = crate::R<CRRCRrs>;
///Register `CRRCR` writer
pub type W = crate::W<CRRCRrs>;
///Field `HSI48ON` reader - HSI48 clock enable
pub type HSI48ON_R = crate::BitReader;
///Field `HSI48ON` writer - HSI48 clock enable
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDY` reader - HSI48 clock ready flag
pub type HSI48RDY_R = crate::BitReader;
///Field `HSI48CAL` reader - HSI48 clock calibration
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - HSI48 clock enable
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSI48 clock ready flag
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 7:15 - HSI48 clock calibration
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
    ///Bit 0 - HSI48 clock enable
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<'_, CRRCRrs> {
        HSI48ON_W::new(self, 0)
    }
}
/**Clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CRRCR)*/
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
