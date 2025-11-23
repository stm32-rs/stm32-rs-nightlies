///Register `SDCMR` reader
pub type R = crate::R<SDCMRrs>;
///Register `SDCMR` writer
pub type W = crate::W<SDCMRrs>;
/**Command mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Normal Mode
    Normal = 0,
    ///1: Clock Configuration Enable
    ClockConfigurationEnable = 1,
    ///2: PALL (All Bank Precharge) command
    Pall = 2,
    ///3: Auto-refresh command
    AutoRefreshCommand = 3,
    ///4: Load Mode Resgier
    LoadModeRegister = 4,
    ///5: Self-refresh command
    SelfRefreshCommand = 5,
    ///6: Power-down command
    PowerDownCommand = 6,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE` writer - Command mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal Mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Normal)
    }
    ///Clock Configuration Enable
    #[inline(always)]
    pub fn clock_configuration_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ClockConfigurationEnable)
    }
    ///PALL (All Bank Precharge) command
    #[inline(always)]
    pub fn pall(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Pall)
    }
    ///Auto-refresh command
    #[inline(always)]
    pub fn auto_refresh_command(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::AutoRefreshCommand)
    }
    ///Load Mode Resgier
    #[inline(always)]
    pub fn load_mode_register(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::LoadModeRegister)
    }
    ///Self-refresh command
    #[inline(always)]
    pub fn self_refresh_command(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::SelfRefreshCommand)
    }
    ///Power-down command
    #[inline(always)]
    pub fn power_down_command(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::PowerDownCommand)
    }
}
/**Command target bank 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTB2 {
    ///0: Command not issued to SDRAM Bank 1
    NotIssued = 0,
    ///1: Command issued to SDRAM Bank 1
    Issued = 1,
}
impl From<CTB2> for bool {
    #[inline(always)]
    fn from(variant: CTB2) -> Self {
        variant as u8 != 0
    }
}
///Field `CTB2` writer - Command target bank 2
pub type CTB2_W<'a, REG> = crate::BitWriter<'a, REG, CTB2>;
impl<'a, REG> CTB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command not issued to SDRAM Bank 1
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut crate::W<REG> {
        self.variant(CTB2::NotIssued)
    }
    ///Command issued to SDRAM Bank 1
    #[inline(always)]
    pub fn issued(self) -> &'a mut crate::W<REG> {
        self.variant(CTB2::Issued)
    }
}
///Field `CTB1` writer - Command target bank 1
pub use CTB2_W as CTB1_W;
///Field `NRFS` reader - Number of Auto-refresh
pub type NRFS_R = crate::FieldReader;
///Field `NRFS` writer - Number of Auto-refresh
pub type NRFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `MRD` reader - Mode Register definition
pub type MRD_R = crate::FieldReader<u16>;
///Field `MRD` writer - Mode Register definition
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 5:8 - Number of Auto-refresh
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:21 - Mode Register definition
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDCMR")
            .field("nrfs", &self.nrfs())
            .field("mrd", &self.mrd())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Command mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, SDCMRrs> {
        MODE_W::new(self, 0)
    }
    ///Bit 3 - Command target bank 2
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W<'_, SDCMRrs> {
        CTB2_W::new(self, 3)
    }
    ///Bit 4 - Command target bank 1
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W<'_, SDCMRrs> {
        CTB1_W::new(self, 4)
    }
    ///Bits 5:8 - Number of Auto-refresh
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W<'_, SDCMRrs> {
        NRFS_W::new(self, 5)
    }
    ///Bits 9:21 - Mode Register definition
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<'_, SDCMRrs> {
        MRD_W::new(self, 9)
    }
}
/**SDRAM Command Mode register

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#FMC:SDCMR)*/
pub struct SDCMRrs;
impl crate::RegisterSpec for SDCMRrs {
    type Ux = u32;
}
///`read()` method returns [`sdcmr::R`](R) reader structure
impl crate::Readable for SDCMRrs {}
///`write(|w| ..)` method takes [`sdcmr::W`](W) writer structure
impl crate::Writable for SDCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDCMR to value 0
impl crate::Resettable for SDCMRrs {}
