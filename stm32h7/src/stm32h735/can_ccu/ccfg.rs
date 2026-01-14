///Register `CCFG` reader
pub type R = crate::R<CCFGrs>;
///Register `CCFG` writer
pub type W = crate::W<CCFGrs>;
///Field `TQBT` reader - Time Quanta per Bit Time
pub type TQBT_R = crate::FieldReader;
///Field `TQBT` writer - Time Quanta per Bit Time
pub type TQBT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BCC` reader - Bypass Clock Calibration
pub type BCC_R = crate::BitReader;
///Field `BCC` writer - Bypass Clock Calibration
pub type BCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFL` reader - Calibration Field Length
pub type CFL_R = crate::BitReader;
///Field `CFL` writer - Calibration Field Length
pub type CFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCPM` reader - Oscillator Clock Periods Minimum
pub type OCPM_R = crate::FieldReader;
///Field `OCPM` writer - Oscillator Clock Periods Minimum
pub type OCPM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CDIV` reader - Clock Divider
pub type CDIV_R = crate::FieldReader;
///Field `CDIV` writer - Clock Divider
pub type CDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SWR` reader - Software Reset
pub type SWR_R = crate::BitReader;
///Field `SWR` writer - Software Reset
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Time Quanta per Bit Time
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - Bypass Clock Calibration
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Calibration Field Length
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Oscillator Clock Periods Minimum
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Clock Divider
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 31 - Software Reset
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG")
            .field("tqbt", &self.tqbt())
            .field("bcc", &self.bcc())
            .field("cfl", &self.cfl())
            .field("ocpm", &self.ocpm())
            .field("cdiv", &self.cdiv())
            .field("swr", &self.swr())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Time Quanta per Bit Time
    #[inline(always)]
    pub fn tqbt(&mut self) -> TQBT_W<'_, CCFGrs> {
        TQBT_W::new(self, 0)
    }
    ///Bit 6 - Bypass Clock Calibration
    #[inline(always)]
    pub fn bcc(&mut self) -> BCC_W<'_, CCFGrs> {
        BCC_W::new(self, 6)
    }
    ///Bit 7 - Calibration Field Length
    #[inline(always)]
    pub fn cfl(&mut self) -> CFL_W<'_, CCFGrs> {
        CFL_W::new(self, 7)
    }
    ///Bits 8:15 - Oscillator Clock Periods Minimum
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W<'_, CCFGrs> {
        OCPM_W::new(self, 8)
    }
    ///Bits 16:19 - Clock Divider
    #[inline(always)]
    pub fn cdiv(&mut self) -> CDIV_W<'_, CCFGrs> {
        CDIV_W::new(self, 16)
    }
    ///Bit 31 - Software Reset
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<'_, CCFGrs> {
        SWR_W::new(self, 31)
    }
}
/**Calibration Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#CAN_CCU:CCFG)*/
pub struct CCFGrs;
impl crate::RegisterSpec for CCFGrs {
    type Ux = u32;
}
///`read()` method returns [`ccfg::R`](R) reader structure
impl crate::Readable for CCFGrs {}
///`write(|w| ..)` method takes [`ccfg::W`](W) writer structure
impl crate::Writable for CCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCFG to value 0
impl crate::Resettable for CCFGrs {}
