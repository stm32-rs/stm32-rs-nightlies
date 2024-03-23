#[doc = "Register `SDCMR` reader"]
pub type R = crate::R<SDCMRrs>;
#[doc = "Register `SDCMR` writer"]
pub type W = crate::W<SDCMRrs>;
#[doc = "Command mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Clock Configuration Enable"]
    ClockConfigurationEnable = 1,
    #[doc = "2: PALL (All Bank Precharge) command"]
    Pall = 2,
    #[doc = "3: Auto-refresh command"]
    AutoRefreshCommand = 3,
    #[doc = "4: Load Mode Resgier"]
    LoadModeRegister = 4,
    #[doc = "5: Self-refresh command"]
    SelfRefreshCommand = 5,
    #[doc = "6: Power-down command"]
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
#[doc = "Field `MODE` writer - Command mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Normal)
    }
    #[doc = "Clock Configuration Enable"]
    #[inline(always)]
    pub fn clock_configuration_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ClockConfigurationEnable)
    }
    #[doc = "PALL (All Bank Precharge) command"]
    #[inline(always)]
    pub fn pall(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Pall)
    }
    #[doc = "Auto-refresh command"]
    #[inline(always)]
    pub fn auto_refresh_command(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::AutoRefreshCommand)
    }
    #[doc = "Load Mode Resgier"]
    #[inline(always)]
    pub fn load_mode_register(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::LoadModeRegister)
    }
    #[doc = "Self-refresh command"]
    #[inline(always)]
    pub fn self_refresh_command(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::SelfRefreshCommand)
    }
    #[doc = "Power-down command"]
    #[inline(always)]
    pub fn power_down_command(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::PowerDownCommand)
    }
}
#[doc = "Command target bank 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTB2 {
    #[doc = "0: Command not issued to SDRAM Bank 1"]
    NotIssued = 0,
    #[doc = "1: Command issued to SDRAM Bank 1"]
    Issued = 1,
}
impl From<CTB2> for bool {
    #[inline(always)]
    fn from(variant: CTB2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTB2` writer - Command target bank 2"]
pub type CTB2_W<'a, REG> = crate::BitWriter<'a, REG, CTB2>;
impl<'a, REG> CTB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command not issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut crate::W<REG> {
        self.variant(CTB2::NotIssued)
    }
    #[doc = "Command issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn issued(self) -> &'a mut crate::W<REG> {
        self.variant(CTB2::Issued)
    }
}
#[doc = "Field `CTB1` writer - Command target bank 1"]
pub use CTB2_W as CTB1_W;
#[doc = "Field `NRFS` reader - Number of Auto-refresh"]
pub type NRFS_R = crate::FieldReader;
#[doc = "Field `NRFS` writer - Number of Auto-refresh"]
pub type NRFS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `MRD` reader - Mode Register definition"]
pub type MRD_R = crate::FieldReader<u16>;
#[doc = "Field `MRD` writer - Mode Register definition"]
pub type MRD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SDCMRrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Command target bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctb2(&mut self) -> CTB2_W<SDCMRrs> {
        CTB2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Command target bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctb1(&mut self) -> CTB1_W<SDCMRrs> {
        CTB1_W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    #[must_use]
    pub fn nrfs(&mut self) -> NRFS_W<SDCMRrs> {
        NRFS_W::new(self, 5)
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    #[must_use]
    pub fn mrd(&mut self) -> MRD_W<SDCMRrs> {
        MRD_W::new(self, 9)
    }
}
#[doc = "SDRAM Command Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDCMRrs;
impl crate::RegisterSpec for SDCMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcmr::R`](R) reader structure"]
impl crate::Readable for SDCMRrs {}
#[doc = "`write(|w| ..)` method takes [`sdcmr::W`](W) writer structure"]
impl crate::Writable for SDCMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDCMR to value 0"]
impl crate::Resettable for SDCMRrs {
    const RESET_VALUE: u32 = 0;
}
