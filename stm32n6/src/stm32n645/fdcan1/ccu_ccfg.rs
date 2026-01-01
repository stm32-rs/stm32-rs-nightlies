///Register `CCU_CCFG` reader
pub type R = crate::R<CCU_CCFGrs>;
///Register `CCU_CCFG` writer
pub type W = crate::W<CCU_CCFGrs>;
///Field `TQBT` reader - Time quanta per bit time
pub type TQBT_R = crate::FieldReader;
///Field `TQBT` writer - Time quanta per bit time
pub type TQBT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BCC` reader - Bypass clock calibration
pub type BCC_R = crate::BitReader;
///Field `BCC` writer - Bypass clock calibration
pub type BCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFL` reader - Calibration field length
pub type CFL_R = crate::BitReader;
///Field `CFL` writer - Calibration field length
pub type CFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCPM` reader - Oscillator clock periods minimum
pub type OCPM_R = crate::FieldReader;
///Field `OCPM` writer - Oscillator clock periods minimum
pub type OCPM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CDIV` reader - Clock divider
pub type CDIV_R = crate::FieldReader;
///Field `CDIV` writer - Clock divider
pub type CDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SWR` reader - Software reset
pub type SWR_R = crate::BitReader;
///Field `SWR` writer - Software reset
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Time quanta per bit time
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - Bypass clock calibration
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Calibration field length
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Oscillator clock periods minimum
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Clock divider
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 31 - Software reset
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCU_CCFG")
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
    ///Bits 0:4 - Time quanta per bit time
    #[inline(always)]
    pub fn tqbt(&mut self) -> TQBT_W<'_, CCU_CCFGrs> {
        TQBT_W::new(self, 0)
    }
    ///Bit 6 - Bypass clock calibration
    #[inline(always)]
    pub fn bcc(&mut self) -> BCC_W<'_, CCU_CCFGrs> {
        BCC_W::new(self, 6)
    }
    ///Bit 7 - Calibration field length
    #[inline(always)]
    pub fn cfl(&mut self) -> CFL_W<'_, CCU_CCFGrs> {
        CFL_W::new(self, 7)
    }
    ///Bits 8:15 - Oscillator clock periods minimum
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W<'_, CCU_CCFGrs> {
        OCPM_W::new(self, 8)
    }
    ///Bits 16:19 - Clock divider
    #[inline(always)]
    pub fn cdiv(&mut self) -> CDIV_W<'_, CCU_CCFGrs> {
        CDIV_W::new(self, 16)
    }
    ///Bit 31 - Software reset
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<'_, CCU_CCFGrs> {
        SWR_W::new(self, 31)
    }
}
/**FDCAN Endian register

You can [`read`](crate::Reg::read) this register and get [`ccu_ccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_ccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FDCAN1:CCU_CCFG)*/
pub struct CCU_CCFGrs;
impl crate::RegisterSpec for CCU_CCFGrs {
    type Ux = u32;
}
///`read()` method returns [`ccu_ccfg::R`](R) reader structure
impl crate::Readable for CCU_CCFGrs {}
///`write(|w| ..)` method takes [`ccu_ccfg::W`](W) writer structure
impl crate::Writable for CCU_CCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCU_CCFG to value 0x8765_4321
impl crate::Resettable for CCU_CCFGrs {
    const RESET_VALUE: u32 = 0x8765_4321;
}
