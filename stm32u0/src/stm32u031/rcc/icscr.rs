///Register `ICSCR` reader
pub type R = crate::R<ICSCRrs>;
///Register `ICSCR` writer
pub type W = crate::W<ICSCRrs>;
///Field `MSICAL` reader - MSI clock calibration These bits are initialized at startup with the factory-programmed MSI calibration trim value. When MSITRIM is written, MSICAL is updated with the sum of MSITRIM and the factory trim value.
pub type MSICAL_R = crate::FieldReader;
///Field `MSITRIM` reader - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI.
pub type MSITRIM_R = crate::FieldReader;
///Field `MSITRIM` writer - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI.
pub type MSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HSICAL` reader - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.
pub type HSICAL_R = crate::FieldReader;
///Field `HSITRIM` reader - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%.
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%.
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:7 - MSI clock calibration These bits are initialized at startup with the factory-programmed MSI calibration trim value. When MSITRIM is written, MSICAL is updated with the sum of MSITRIM and the factory trim value.
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - HSI16 clock calibration These bits are initialized at startup with the factory-programmed HSI16 calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%.
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR")
            .field("msical", &self.msical())
            .field("msitrim", &self.msitrim())
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    ///Bits 8:15 - MSI clock trimming These bits provide an additional user-programmable trimming value that is added to the MSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim(&mut self) -> MSITRIM_W<'_, ICSCRrs> {
        MSITRIM_W::new(self, 8)
    }
    ///Bits 24:30 - HSI16 clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\[7:0\] bits. It can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI16. The default value is 64 when added to the HSICAL value, trim the HSI16 to 161MHz 1 11%.
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<'_, ICSCRrs> {
        HSITRIM_W::new(self, 24)
    }
}
/**Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:ICSCR)*/
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
///`read()` method returns [`icscr::R`](R) reader structure
impl crate::Readable for ICSCRrs {}
///`write(|w| ..)` method takes [`icscr::W`](W) writer structure
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSCR to value 0x4000_4000
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0x4000_4000;
}
