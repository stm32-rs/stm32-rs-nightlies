///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader;
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICEN` reader - Instruction cache enable
pub type ICEN_R = crate::BitReader;
///Field `ICEN` writer - Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCEN` reader - Data cache enable
pub type DCEN_R = crate::BitReader;
///Field `DCEN` writer - Data cache enable
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICRST` reader - Instruction cache reset
pub type ICRST_R = crate::BitReader;
///Field `ICRST` writer - Instruction cache reset
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRST` reader - Data cache reset
pub type DCRST_R = crate::BitReader;
///Field `DCRST` writer - Data cache reset
pub type DCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode
pub type RUN_PD_R = crate::BitReader;
///Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_R = crate::BitReader;
///Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("icen", &self.icen())
            .field("dcen", &self.dcen())
            .field("icrst", &self.icrst())
            .field("dcrst", &self.dcrst())
            .field("run_pd", &self.run_pd())
            .field("sleep_pd", &self.sleep_pd())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<'_, ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, ACRrs> {
        DCEN_W::new(self, 10)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<'_, ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W<'_, ACRrs> {
        DCRST_W::new(self, 12)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W<'_, ACRrs> {
        RUN_PD_W::new(self, 13)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 14)
    }
}
/**Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0x0600
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
