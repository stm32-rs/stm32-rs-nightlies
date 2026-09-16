///Register `SDCMR` reader
pub type R = crate::R<SDCMRrs>;
///Register `SDCMR` writer
pub type W = crate::W<SDCMRrs>;
/**Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0.

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
///Field `MODE` reader - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0.
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Normal),
            1 => Some(MODE::ClockConfigurationEnable),
            2 => Some(MODE::Pall),
            3 => Some(MODE::AutoRefreshCommand),
            4 => Some(MODE::LoadModeRegister),
            5 => Some(MODE::SelfRefreshCommand),
            6 => Some(MODE::PowerDownCommand),
            _ => None,
        }
    }
    ///Normal Mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE::Normal
    }
    ///Clock Configuration Enable
    #[inline(always)]
    pub fn is_clock_configuration_enable(&self) -> bool {
        *self == MODE::ClockConfigurationEnable
    }
    ///PALL (All Bank Precharge) command
    #[inline(always)]
    pub fn is_pall(&self) -> bool {
        *self == MODE::Pall
    }
    ///Auto-refresh command
    #[inline(always)]
    pub fn is_auto_refresh_command(&self) -> bool {
        *self == MODE::AutoRefreshCommand
    }
    ///Load Mode Resgier
    #[inline(always)]
    pub fn is_load_mode_register(&self) -> bool {
        *self == MODE::LoadModeRegister
    }
    ///Self-refresh command
    #[inline(always)]
    pub fn is_self_refresh_command(&self) -> bool {
        *self == MODE::SelfRefreshCommand
    }
    ///Power-down command
    #[inline(always)]
    pub fn is_power_down_command(&self) -> bool {
        *self == MODE::PowerDownCommand
    }
}
///Field `MODE` writer - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0.
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
/**Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not.

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
///Field `CTB2` reader - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not.
pub type CTB2_R = crate::BitReader<CTB2>;
impl CTB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTB2 {
        match self.bits {
            false => CTB2::NotIssued,
            true => CTB2::Issued,
        }
    }
    ///Command not issued to SDRAM Bank 1
    #[inline(always)]
    pub fn is_not_issued(&self) -> bool {
        *self == CTB2::NotIssued
    }
    ///Command issued to SDRAM Bank 1
    #[inline(always)]
    pub fn is_issued(&self) -> bool {
        *self == CTB2::Issued
    }
}
///Field `CTB2` writer - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not.
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
///Field `CTB1` reader - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not.
pub use CTB2_R as CTB1_R;
///Field `CTB1` writer - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not.
pub use CTB2_W as CTB1_W;
///Field `NRFS` reader - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ....
pub type NRFS_R = crate::FieldReader;
///Field `NRFS` writer - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ....
pub type NRFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `MRD` reader - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\[13:0\] bits are also used to program the extended mode register for mobile SDRAM.
pub type MRD_R = crate::FieldReader<u16>;
///Field `MRD` writer - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\[13:0\] bits are also used to program the extended mode register for mobile SDRAM.
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not.
    #[inline(always)]
    pub fn ctb2(&self) -> CTB2_R {
        CTB2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not.
    #[inline(always)]
    pub fn ctb1(&self) -> CTB1_R {
        CTB1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ....
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\[13:0\] bits are also used to program the extended mode register for mobile SDRAM.
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDCMR")
            .field("mode", &self.mode())
            .field("ctb2", &self.ctb2())
            .field("ctb1", &self.ctb1())
            .field("nrfs", &self.nrfs())
            .field("mrd", &self.mrd())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, SDCMRrs> {
        MODE_W::new(self, 0)
    }
    ///Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not.
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W<'_, SDCMRrs> {
        CTB2_W::new(self, 3)
    }
    ///Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not.
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W<'_, SDCMRrs> {
        CTB1_W::new(self, 4)
    }
    ///Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ....
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W<'_, SDCMRrs> {
        NRFS_W::new(self, 5)
    }
    ///Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\[13:0\] bits are also used to program the extended mode register for mobile SDRAM.
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<'_, SDCMRrs> {
        MRD_W::new(self, 9)
    }
}
/**This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks.

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#FMC:SDCMR)*/
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
