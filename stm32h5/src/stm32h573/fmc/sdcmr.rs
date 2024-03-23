#[doc = "Register `SDCMR` reader"]
pub type R = crate::R<SDCMRrs>;
#[doc = "Register `SDCMR` writer"]
pub type W = crate::W<SDCMRrs>;
#[doc = "Field `MODE` reader - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0."]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTB2` reader - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub type CTB2_R = crate::BitReader;
#[doc = "Field `CTB2` writer - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub type CTB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTB1` reader - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub type CTB1_R = crate::BitReader;
#[doc = "Field `CTB1` writer - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub type CTB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRFS` reader - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ...."]
pub type NRFS_R = crate::FieldReader;
#[doc = "Field `NRFS` writer - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ...."]
pub type NRFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRD` reader - Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command."]
pub type MRD_R = crate::FieldReader<u16>;
#[doc = "Field `MRD` writer - Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command."]
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    pub fn ctb2(&self) -> CTB2_R {
        CTB2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    pub fn ctb1(&self) -> CTB1_R {
        CTB1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ...."]
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:21 - Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command."]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit set, the other CTB bit of the the unused bank must be kept to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SDCMRrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    #[must_use]
    pub fn ctb2(&mut self) -> CTB2_W<SDCMRrs> {
        CTB2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    #[must_use]
    pub fn ctb1(&mut self) -> CTB1_W<SDCMRrs> {
        CTB1_W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = ‘011’. ...."]
    #[inline(always)]
    #[must_use]
    pub fn nrfs(&mut self) -> NRFS_W<SDCMRrs> {
        NRFS_W::new(self, 5)
    }
    #[doc = "Bits 9:21 - Mode Register definition This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command."]
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
