///Register `SMPSCR` reader
pub type R = crate::R<SMPSCRrs>;
///Register `SMPSCR` writer
pub type W = crate::W<SMPSCRrs>;
/**Step Down converter clock selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMPSSEL {
    ///0: HSI16 selected as SMPS step-down converter clock
    Hsi16 = 0,
    ///1: MSI selected as SMPS step-down converter clock
    Msi = 1,
    ///2: HSE selected as SMPS step-down converter clock
    Hse = 2,
}
impl From<SMPSSEL> for u8 {
    #[inline(always)]
    fn from(variant: SMPSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMPSSEL {
    type Ux = u8;
}
impl crate::IsEnum for SMPSSEL {}
///Field `SMPSSEL` reader - Step Down converter clock selection
pub type SMPSSEL_R = crate::FieldReader<SMPSSEL>;
impl SMPSSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SMPSSEL> {
        match self.bits {
            0 => Some(SMPSSEL::Hsi16),
            1 => Some(SMPSSEL::Msi),
            2 => Some(SMPSSEL::Hse),
            _ => None,
        }
    }
    ///HSI16 selected as SMPS step-down converter clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SMPSSEL::Hsi16
    }
    ///MSI selected as SMPS step-down converter clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SMPSSEL::Msi
    }
    ///HSE selected as SMPS step-down converter clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SMPSSEL::Hse
    }
}
///Field `SMPSSEL` writer - Step Down converter clock selection
pub type SMPSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SMPSSEL>;
impl<'a, REG> SMPSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSI16 selected as SMPS step-down converter clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSSEL::Hsi16)
    }
    ///MSI selected as SMPS step-down converter clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSSEL::Msi)
    }
    ///HSE selected as SMPS step-down converter clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSSEL::Hse)
    }
}
///Field `SMPSDIV` reader - Step Down converter clock prescaler
pub type SMPSDIV_R = crate::FieldReader;
///Field `SMPSDIV` writer - Step Down converter clock prescaler
pub type SMPSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Step Down converter clock switch status

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMPSSWS {
    ///0: HSI16 oscillator used as SMPS step-down converter clock
    Hsi16 = 0,
    ///1: MSI oscillator used as SMPS step-down converter clock
    Msi = 1,
    ///2: HSE oscillator used as SMPS step-down converter clock
    Hse = 2,
    ///3: No clock is used
    NoClock = 3,
}
impl From<SMPSSWS> for u8 {
    #[inline(always)]
    fn from(variant: SMPSSWS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMPSSWS {
    type Ux = u8;
}
impl crate::IsEnum for SMPSSWS {}
///Field `SMPSSWS` reader - Step Down converter clock switch status
pub type SMPSSWS_R = crate::FieldReader<SMPSSWS>;
impl SMPSSWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSSWS {
        match self.bits {
            0 => SMPSSWS::Hsi16,
            1 => SMPSSWS::Msi,
            2 => SMPSSWS::Hse,
            3 => SMPSSWS::NoClock,
            _ => unreachable!(),
        }
    }
    ///HSI16 oscillator used as SMPS step-down converter clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SMPSSWS::Hsi16
    }
    ///MSI oscillator used as SMPS step-down converter clock
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SMPSSWS::Msi
    }
    ///HSE oscillator used as SMPS step-down converter clock
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SMPSSWS::Hse
    }
    ///No clock is used
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == SMPSSWS::NoClock
    }
}
impl R {
    ///Bits 0:1 - Step Down converter clock selection
    #[inline(always)]
    pub fn smpssel(&self) -> SMPSSEL_R {
        SMPSSEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Step Down converter clock prescaler
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Step Down converter clock switch status
    #[inline(always)]
    pub fn smpssws(&self) -> SMPSSWS_R {
        SMPSSWS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPSCR")
            .field("smpssws", &self.smpssws())
            .field("smpsdiv", &self.smpsdiv())
            .field("smpssel", &self.smpssel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Step Down converter clock selection
    #[inline(always)]
    pub fn smpssel(&mut self) -> SMPSSEL_W<'_, SMPSCRrs> {
        SMPSSEL_W::new(self, 0)
    }
    ///Bits 4:5 - Step Down converter clock prescaler
    #[inline(always)]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W<'_, SMPSCRrs> {
        SMPSDIV_W::new(self, 4)
    }
}
/**Step Down converter control register

You can [`read`](crate::Reg::read) this register and get [`smpscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:SMPSCR)*/
pub struct SMPSCRrs;
impl crate::RegisterSpec for SMPSCRrs {
    type Ux = u32;
}
///`read()` method returns [`smpscr::R`](R) reader structure
impl crate::Readable for SMPSCRrs {}
///`write(|w| ..)` method takes [`smpscr::W`](W) writer structure
impl crate::Writable for SMPSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPSCR to value 0x0301
impl crate::Resettable for SMPSCRrs {
    const RESET_VALUE: u32 = 0x0301;
}
