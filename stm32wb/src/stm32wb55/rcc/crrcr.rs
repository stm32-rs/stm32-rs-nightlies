///Register `CRRCR` reader
pub type R = crate::R<CRRCRrs>;
///Register `CRRCR` writer
pub type W = crate::W<CRRCRrs>;
/**HSI48 oscillator enabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON {
    ///0: HSI48 oscillator off
    Off = 0,
    ///1: HSI48 oscillator on
    On = 1,
}
impl From<HSI48ON> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48ON` reader - HSI48 oscillator enabled
pub type HSI48ON_R = crate::BitReader<HSI48ON>;
impl HSI48ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI48ON {
        match self.bits {
            false => HSI48ON::Off,
            true => HSI48ON::On,
        }
    }
    ///HSI48 oscillator off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSI48ON::Off
    }
    ///HSI48 oscillator on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSI48ON::On
    }
}
///Field `HSI48ON` writer - HSI48 oscillator enabled
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI48ON>;
impl<'a, REG> HSI48ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI48 oscillator off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::Off)
    }
    ///HSI48 oscillator on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::On)
    }
}
/**HSI48 clock ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDY {
    ///0: HSI48 oscillator not ready
    NotReady = 0,
    ///1: HSI48 oscillator ready
    Ready = 1,
}
impl From<HSI48RDY> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48RDY` reader - HSI48 clock ready
pub type HSI48RDY_R = crate::BitReader<HSI48RDY>;
impl HSI48RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDY {
        match self.bits {
            false => HSI48RDY::NotReady,
            true => HSI48RDY::Ready,
        }
    }
    ///HSI48 oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI48RDY::NotReady
    }
    ///HSI48 oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI48RDY::Ready
    }
}
///Field `HSI48CAL` reader - HSI48 clock calibration
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - HSI48 oscillator enabled
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSI48 clock ready
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
            .field("hsi48cal", &self.hsi48cal())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("hsi48on", &self.hsi48on())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI48 oscillator enabled
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W<'_, CRRCRrs> {
        HSI48ON_W::new(self, 0)
    }
}
/**Clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:CRRCR)*/
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
