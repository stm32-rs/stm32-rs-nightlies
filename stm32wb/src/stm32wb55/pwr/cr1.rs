///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPMS` reader - Low-power mode selection for CPU1
pub type LPMS_R = crate::FieldReader;
///Field `LPMS` writer - Low-power mode selection for CPU1
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FPDR` reader - Flash power down mode during LPRun for CPU1
pub type FPDR_R = crate::BitReader;
///Field `FPDR` writer - Flash power down mode during LPRun for CPU1
pub type FPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPDS` reader - Flash power down mode during LPsSleep for CPU1
pub type FPDS_R = crate::BitReader;
///Field `FPDS` writer - Flash power down mode during LPsSleep for CPU1
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader;
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPR` reader - Low-power run
pub type LPR_R = crate::BitReader;
///Field `LPR` writer - Low-power run
pub type LPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Low-power mode selection for CPU1
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Flash power down mode during LPRun for CPU1
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Flash power down mode during LPsSleep for CPU1
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpr", &self.lpr())
            .field("vos", &self.vos())
            .field("dbp", &self.dbp())
            .field("fpds", &self.fpds())
            .field("fpdr", &self.fpdr())
            .field("lpms", &self.lpms())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection for CPU1
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 4 - Flash power down mode during LPRun for CPU1
    #[inline(always)]
    pub fn fpdr(&mut self) -> FPDR_W<'_, CR1rs> {
        FPDR_W::new(self, 4)
    }
    ///Bit 5 - Flash power down mode during LPsSleep for CPU1
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W<'_, CR1rs> {
        FPDS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CR1rs> {
        DBP_W::new(self, 8)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CR1rs> {
        VOS_W::new(self, 9)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    pub fn lpr(&mut self) -> LPR_W<'_, CR1rs> {
        LPR_W::new(self, 14)
    }
}
/**Power control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x0200
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0200;
}
