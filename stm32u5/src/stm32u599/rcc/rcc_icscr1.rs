///Register `RCC_ICSCR1` reader
pub type R = crate::R<RCC_ICSCR1rs>;
///Register `RCC_ICSCR1` writer
pub type W = crate::W<RCC_ICSCR1rs>;
/**Field `MSICAL3` reader - MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\[4:0\]
and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
pub type MSICAL3_R = crate::FieldReader;
/**Field `MSICAL2` reader - MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\[4:0\]
and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
pub type MSICAL2_R = crate::FieldReader;
/**Field `MSICAL1` reader - MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\[4:0\]
and the factory calibration trim value MSIRC1\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
pub type MSICAL1_R = crate::FieldReader;
/**Field `MSICAL0` reader - MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\[4:0\]
and the factory-programmed calibration trim value MSIRC0\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
pub type MSICAL0_R = crate::FieldReader;
///Field `MSIBIAS` reader - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
pub type MSIBIAS_R = crate::BitReader;
///Field `MSIBIAS` writer - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
pub type MSIBIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `MSIRGSEL` reader - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
and MSIKSRANGE\[3:0\]
in RCC_CSR.*/
pub type MSIRGSEL_R = crate::BitReader;
/**Field `MSIRGSEL` writer - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
and MSIKSRANGE\[3:0\]
in RCC_CSR.*/
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIKRANGE` reader - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
pub type MSIKRANGE_R = crate::FieldReader;
///Field `MSIKRANGE` writer - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
pub type MSIKRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MSISRANGE` reader - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
pub type MSISRANGE_R = crate::FieldReader;
///Field `MSISRANGE` writer - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    /**Bits 0:4 - MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\[4:0\]
    and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
    #[inline(always)]
    pub fn msical3(&self) -> MSICAL3_R {
        MSICAL3_R::new((self.bits & 0x1f) as u8)
    }
    /**Bits 5:9 - MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\[4:0\]
    and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
    #[inline(always)]
    pub fn msical2(&self) -> MSICAL2_R {
        MSICAL2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    /**Bits 10:14 - MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\[4:0\]
    and the factory calibration trim value MSIRC1\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
    #[inline(always)]
    pub fn msical1(&self) -> MSICAL1_R {
        MSICAL1_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    /**Bits 15:19 - MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\[4:0\]
    and the factory-programmed calibration trim value MSIRC0\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.*/
    #[inline(always)]
    pub fn msical0(&self) -> MSICAL0_R {
        MSICAL0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bit 22 - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
    #[inline(always)]
    pub fn msibias(&self) -> MSIBIAS_R {
        MSIBIAS_R::new(((self.bits >> 22) & 1) != 0)
    }
    /**Bit 23 - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
    and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
    and MSIKSRANGE\[3:0\]
    in RCC_CSR.*/
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
    #[inline(always)]
    pub fn msikrange(&self) -> MSIKRANGE_R {
        MSIKRANGE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ICSCR1")
            .field("msical3", &self.msical3())
            .field("msical2", &self.msical2())
            .field("msical1", &self.msical1())
            .field("msical0", &self.msical0())
            .field("msibias", &self.msibias())
            .field("msirgsel", &self.msirgsel())
            .field("msikrange", &self.msikrange())
            .field("msisrange", &self.msisrange())
            .finish()
    }
}
impl W {
    ///Bit 22 - MSI bias mode selection This bit is set by software to select the MSI bias mode. By default, the MSI bias is in�continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption when the regulator is in range 4, or when the device is in Stop 1 or Stop�2 mode, but it�decreases the MSI accuracy
    #[inline(always)]
    #[must_use]
    pub fn msibias(&mut self) -> MSIBIAS_W<RCC_ICSCR1rs> {
        MSIBIAS_W::new(self, 22)
    }
    /**Bit 23 - MSI clock range selection This bit is set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
    and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
    and MSIKSRANGE\[3:0\]
    in RCC_CSR.*/
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<RCC_ICSCR1rs> {
        MSIRGSEL_W::new(self, 23)
    }
    ///Bits 24:27 - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is off (MSISON = 0) or when MSIK is ready (MSIKRDY�=�1). MSIKRANGE must NOT be modified when MSIK is on and NOT ready (MSIKON = 1 and MSIKRDY = 0) Note: MSIKRANGE is kept when the device wakes up from Stop mode, except when the�MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into�range 2 (24 MHz).
    #[inline(always)]
    #[must_use]
    pub fn msikrange(&mut self) -> MSIKRANGE_W<RCC_ICSCR1rs> {
        MSIKRANGE_W::new(self, 24)
    }
    ///Bits 28:31 - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is off (MSISON = 0) or when MSIS is ready (MSISRDY�=�1). MSISRANGE must NOT be modified when MSIS is on and NOT ready (MSISON�=�1 and MSISRDY�=�0) Note: MSISRANGE is kept when the device wakes up from Stop mode, except when the�MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into range 2 (24 MHz).
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<RCC_ICSCR1rs> {
        MSISRANGE_W::new(self, 28)
    }
}
/**RCC internal clock sources calibration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_icscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:RCC_ICSCR1)*/
pub struct RCC_ICSCR1rs;
impl crate::RegisterSpec for RCC_ICSCR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_icscr1::R`](R) reader structure
impl crate::Readable for RCC_ICSCR1rs {}
///`write(|w| ..)` method takes [`rcc_icscr1::W`](W) writer structure
impl crate::Writable for RCC_ICSCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_ICSCR1 to value 0x4400_0000
impl crate::Resettable for RCC_ICSCR1rs {
    const RESET_VALUE: u32 = 0x4400_0000;
}
