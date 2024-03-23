#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2rs>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2rs>;
#[doc = "Field `MSSI` reader - Master SS Idleness"]
pub type MSSI_R = crate::FieldReader;
#[doc = "Field `MSSI` writer - Master SS Idleness"]
pub type MSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIDI` reader - Master Inter-Data Idleness"]
pub type MIDI_R = crate::FieldReader;
#[doc = "Field `MIDI` writer - Master Inter-Data Idleness"]
pub type MIDI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDIMM` reader - RDIMM"]
pub type RDIMM_R = crate::BitReader;
#[doc = "Field `RDIMM` writer - RDIMM"]
pub type RDIMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIOP` reader - RDIOP"]
pub type RDIOP_R = crate::BitReader;
#[doc = "Field `RDIOP` writer - RDIOP"]
pub type RDIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSWP` reader - Swap functionality of MISO and MOSI pins"]
pub type IOSWP_R = crate::BitReader;
#[doc = "Field `IOSWP` writer - Swap functionality of MISO and MOSI pins"]
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMM` reader - SPI Communication Mode"]
pub type COMM_R = crate::FieldReader;
#[doc = "Field `COMM` writer - SPI Communication Mode"]
pub type COMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SP` reader - Serial Protocol"]
pub type SP_R = crate::FieldReader;
#[doc = "Field `SP` writer - Serial Protocol"]
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASTER` reader - SPI Master"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `MASTER` writer - SPI Master"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFRST` reader - Data frame format"]
pub type LSBFRST_R = crate::BitReader;
#[doc = "Field `LSBFRST` writer - Data frame format"]
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Software management of SS signal input"]
pub type SSM_R = crate::BitReader;
#[doc = "Field `SSM` writer - Software management of SS signal input"]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSIOP` reader - SS input/output polarity"]
pub type SSIOP_R = crate::BitReader;
#[doc = "Field `SSIOP` writer - SS input/output polarity"]
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SSOE_R = crate::BitReader;
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOM` reader - SS output management in master mode"]
pub type SSOM_R = crate::BitReader;
#[doc = "Field `SSOM` writer - SS output management in master mode"]
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFCNTR` reader - Alternate function GPIOs control"]
pub type AFCNTR_R = crate::BitReader;
#[doc = "Field `AFCNTR` writer - Alternate function GPIOs control"]
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Master SS Idleness"]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Master Inter-Data Idleness"]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&self) -> RDIMM_R {
        RDIMM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RDIOP"]
    #[inline(always)]
    pub fn rdiop(&self) -> RDIOP_R {
        RDIOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - Serial Protocol"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Data frame format"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software management of SS signal input"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SS output management in master mode"]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alternate function GPIOs control"]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master SS Idleness"]
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<CFG2rs> {
        MSSI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Master Inter-Data Idleness"]
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<CFG2rs> {
        MIDI_W::new(self, 4)
    }
    #[doc = "Bit 13 - RDIMM"]
    #[inline(always)]
    #[must_use]
    pub fn rdimm(&mut self) -> RDIMM_W<CFG2rs> {
        RDIMM_W::new(self, 13)
    }
    #[doc = "Bit 14 - RDIOP"]
    #[inline(always)]
    #[must_use]
    pub fn rdiop(&mut self) -> RDIOP_W<CFG2rs> {
        RDIOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<CFG2rs> {
        IOSWP_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<CFG2rs> {
        COMM_W::new(self, 17)
    }
    #[doc = "Bits 19:21 - Serial Protocol"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<CFG2rs> {
        SP_W::new(self, 19)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<CFG2rs> {
        MASTER_W::new(self, 22)
    }
    #[doc = "Bit 23 - Data frame format"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<CFG2rs> {
        LSBFRST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CFG2rs> {
        CPHA_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CFG2rs> {
        CPOL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Software management of SS signal input"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<CFG2rs> {
        SSM_W::new(self, 26)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<CFG2rs> {
        SSIOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - SS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<CFG2rs> {
        SSOE_W::new(self, 29)
    }
    #[doc = "Bit 30 - SS output management in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<CFG2rs> {
        SSOM_W::new(self, 30)
    }
    #[doc = "Bit 31 - Alternate function GPIOs control"]
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<CFG2rs> {
        AFCNTR_W::new(self, 31)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2rs;
impl crate::RegisterSpec for CFG2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2rs {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2rs {
    const RESET_VALUE: u32 = 0;
}
