///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LPDS` reader - Low-power deep sleep
pub type LPDS_R = crate::BitReader;
///Field `LPDS` writer - Low-power deep sleep
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS` reader - Power down deepsleep
pub type PDDS_R = crate::BitReader;
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader;
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader;
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDS` reader - Flash power down in Stop mode
pub type FPDS_R = crate::BitReader;
///Field `FPDS` writer - Flash power down in Stop mode
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPLVDS` reader - Low-Power Regulator Low Voltage in deepsleep
pub type LPLVDS_R = crate::BitReader;
///Field `LPLVDS` writer - Low-Power Regulator Low Voltage in deepsleep
pub type LPLVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRLVDS` reader - Main regulator low voltage in deepsleep mode
pub type MRLVDS_R = crate::BitReader;
///Field `MRLVDS` writer - Main regulator low voltage in deepsleep mode
pub type MRLVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOS` reader - Regulator voltage scaling output selection
pub type VOS_R = crate::FieldReader;
///Field `VOS` writer - Regulator voltage scaling output selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ODEN` reader - Over-drive enable
pub type ODEN_R = crate::BitReader;
///Field `ODEN` writer - Over-drive enable
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODSWEN` reader - Over-drive switching enabled
pub type ODSWEN_R = crate::BitReader;
///Field `ODSWEN` writer - Over-drive switching enabled
pub type ODSWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDEN` reader - Under-drive enable in stop mode
pub type UDEN_R = crate::FieldReader;
///Field `UDEN` writer - Under-drive enable in stop mode
pub type UDEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-Power Regulator Low Voltage in deepsleep
    #[inline(always)]
    pub fn lplvds(&self) -> LPLVDS_R {
        LPLVDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Main regulator low voltage in deepsleep mode
    #[inline(always)]
    pub fn mrlvds(&self) -> MRLVDS_R {
        MRLVDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&self) -> ODSWEN_R {
        ODSWEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lpds", &self.lpds())
            .field("pdds", &self.pdds())
            .field("cwuf", &self.cwuf())
            .field("csbf", &self.csbf())
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("dbp", &self.dbp())
            .field("fpds", &self.fpds())
            .field("lplvds", &self.lplvds())
            .field("mrlvds", &self.mrlvds())
            .field("vos", &self.vos())
            .field("oden", &self.oden())
            .field("odswen", &self.odswen())
            .field("uden", &self.uden())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CRrs> {
        LPDS_W::new(self, 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CRrs> {
        PDDS_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W<'_, CRrs> {
        CWUF_W::new(self, 2)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, CRrs> {
        CSBF_W::new(self, 3)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CRrs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CRrs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, CRrs> {
        FPDS_W::new(self, 9)
    }
    ///Bit 10 - Low-Power Regulator Low Voltage in deepsleep
    #[inline(always)]
    pub fn lplvds(&mut self) -> LPLVDS_W<'_, CRrs> {
        LPLVDS_W::new(self, 10)
    }
    ///Bit 11 - Main regulator low voltage in deepsleep mode
    #[inline(always)]
    pub fn mrlvds(&mut self) -> MRLVDS_W<'_, CRrs> {
        MRLVDS_W::new(self, 11)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CRrs> {
        VOS_W::new(self, 14)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W<'_, CRrs> {
        ODEN_W::new(self, 16)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&mut self) -> ODSWEN_W<'_, CRrs> {
        ODSWEN_W::new(self, 17)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&mut self) -> UDEN_W<'_, CRrs> {
        UDEN_W::new(self, 18)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#PWR:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0xc000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000;
}
