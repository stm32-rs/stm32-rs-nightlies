///Register `ICSCR` reader
pub type R = crate::R<ICSCRrs>;
///Register `ICSCR` writer
pub type W = crate::W<ICSCRrs>;
/**Field `HSICAL` reader - HSI48 clock calibration This bitfield directly acts on the HSI48 clock frequency. Its value is a sum of an internal factory-programmed number and the value of the HSITRIM\[6:0\]
bitfield. In the factory, the internal number is set to calibrate the HSI48 clock frequency to 48 MHz (with HSITRIM\[6:0\]
left at its reset value). Refer to the device datasheet for HSI48 calibration accuracy and for the frequency trimming granularity. Note: The trimming effect presents discontinuities at HSICAL\[7:0\]
multiples of 64.*/
pub type HSICAL_R = crate::FieldReader;
/**Field `HSITRIM` reader - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\[7:0\]
bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value.*/
pub type HSITRIM_R = crate::FieldReader;
/**Field `HSITRIM` writer - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\[7:0\]
bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value.*/
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    /**Bits 0:7 - HSI48 clock calibration This bitfield directly acts on the HSI48 clock frequency. Its value is a sum of an internal factory-programmed number and the value of the HSITRIM\[6:0\]
    bitfield. In the factory, the internal number is set to calibrate the HSI48 clock frequency to 48 MHz (with HSITRIM\[6:0\]
    left at its reset value). Refer to the device datasheet for HSI48 calibration accuracy and for the frequency trimming granularity. Note: The trimming effect presents discontinuities at HSICAL\[7:0\]
    multiples of 64.*/
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0xff) as u8)
    }
    /**Bits 8:14 - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\[7:0\]
    bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value.*/
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR")
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    /**Bits 8:14 - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\[7:0\]
    bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value.*/
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<ICSCRrs> {
        HSITRIM_W::new(self, 8)
    }
}
/**RCC internal clock source calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RCC:ICSCR)*/
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
///`read()` method returns [`icscr::R`](R) reader structure
impl crate::Readable for ICSCRrs {}
///`write(|w| ..)` method takes [`icscr::W`](W) writer structure
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICSCR to value 0x4000
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0x4000;
}
