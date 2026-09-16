///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `DUAL` reader - Dual ADC mode selection
pub type DUAL_R = crate::FieldReader;
///Field `DUAL` writer - Dual ADC mode selection
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DELAY` reader - Delay between two sampling phases
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - Delay between two sampling phases
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DAMDF` reader - Dual ADC mode data format
pub type DAMDF_R = crate::FieldReader;
///Field `DAMDF` writer - Dual ADC mode data format
pub type DAMDF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VREFEN` reader - V less than sub>REFINT less than /sub> enable
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - V less than sub>REFINT less than /sub> enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBATEN` reader - VBAT enable
pub type VBATEN_R = crate::BitReader;
///Field `VBATEN` writer - VBAT enable
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between two sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 14:15 - Dual ADC mode data format
    #[inline(always)]
    pub fn damdf(&self) -> DAMDF_R {
        DAMDF_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 22 - V less than sub>REFINT less than /sub> enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dual", &self.dual())
            .field("delay", &self.delay())
            .field("damdf", &self.damdf())
            .field("vrefen", &self.vrefen())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<'_, CCRrs> {
        DUAL_W::new(self, 0)
    }
    ///Bits 8:11 - Delay between two sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<'_, CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bits 14:15 - Dual ADC mode data format
    #[inline(always)]
    pub fn damdf(&mut self) -> DAMDF_W<'_, CCRrs> {
        DAMDF_W::new(self, 14)
    }
    ///Bit 22 - V less than sub>REFINT less than /sub> enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<'_, CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC12 common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ADC12:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
