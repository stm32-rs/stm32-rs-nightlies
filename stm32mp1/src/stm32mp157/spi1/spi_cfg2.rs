#[doc = "Register `SPI_CFG2` reader"]
pub type R = crate::R<SPI_CFG2rs>;
#[doc = "Register `SPI_CFG2` writer"]
pub type W = crate::W<SPI_CFG2rs>;
#[doc = "Field `MSSI` reader - MSSI"]
pub type MSSI_R = crate::FieldReader;
#[doc = "Field `MSSI` writer - MSSI"]
pub type MSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIDI` reader - MIDI"]
pub type MIDI_R = crate::FieldReader;
#[doc = "Field `MIDI` writer - MIDI"]
pub type MIDI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IOSWP` reader - IOSWP"]
pub type IOSWP_R = crate::BitReader;
#[doc = "Field `IOSWP` writer - IOSWP"]
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMM` reader - COMM"]
pub type COMM_R = crate::FieldReader;
#[doc = "Field `COMM` writer - COMM"]
pub type COMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SP` reader - SP"]
pub type SP_R = crate::FieldReader;
#[doc = "Field `SP` writer - SP"]
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASTER` reader - MASTER"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `MASTER` writer - MASTER"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFRST` reader - LSBFRST"]
pub type LSBFRST_R = crate::BitReader;
#[doc = "Field `LSBFRST` writer - LSBFRST"]
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - CPHA"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - CPHA"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - CPOL"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - CPOL"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - SSM"]
pub type SSM_R = crate::BitReader;
#[doc = "Field `SSM` writer - SSM"]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSIOP` reader - SSIOP"]
pub type SSIOP_R = crate::BitReader;
#[doc = "Field `SSIOP` writer - SSIOP"]
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SSOE"]
pub type SSOE_R = crate::BitReader;
#[doc = "Field `SSOE` writer - SSOE"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOM` reader - SSOM"]
pub type SSOM_R = crate::BitReader;
#[doc = "Field `SSOM` writer - SSOM"]
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFCNTR` reader - AFCNTR"]
pub type AFCNTR_R = crate::BitReader;
#[doc = "Field `AFCNTR` writer - AFCNTR"]
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - MSSI"]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MIDI"]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - IOSWP"]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - COMM"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - SP"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - MASTER"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LSBFRST"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SSM"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SSIOP"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SSOE"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SSOM"]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AFCNTR"]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MSSI"]
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<SPI_CFG2rs> {
        MSSI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - MIDI"]
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<SPI_CFG2rs> {
        MIDI_W::new(self, 4)
    }
    #[doc = "Bit 15 - IOSWP"]
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<SPI_CFG2rs> {
        IOSWP_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - COMM"]
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<SPI_CFG2rs> {
        COMM_W::new(self, 17)
    }
    #[doc = "Bits 19:21 - SP"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<SPI_CFG2rs> {
        SP_W::new(self, 19)
    }
    #[doc = "Bit 22 - MASTER"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<SPI_CFG2rs> {
        MASTER_W::new(self, 22)
    }
    #[doc = "Bit 23 - LSBFRST"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<SPI_CFG2rs> {
        LSBFRST_W::new(self, 23)
    }
    #[doc = "Bit 24 - CPHA"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPI_CFG2rs> {
        CPHA_W::new(self, 24)
    }
    #[doc = "Bit 25 - CPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPI_CFG2rs> {
        CPOL_W::new(self, 25)
    }
    #[doc = "Bit 26 - SSM"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<SPI_CFG2rs> {
        SSM_W::new(self, 26)
    }
    #[doc = "Bit 28 - SSIOP"]
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<SPI_CFG2rs> {
        SSIOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - SSOE"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<SPI_CFG2rs> {
        SSOE_W::new(self, 29)
    }
    #[doc = "Bit 30 - SSOM"]
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<SPI_CFG2rs> {
        SSOM_W::new(self, 30)
    }
    #[doc = "Bit 31 - AFCNTR"]
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<SPI_CFG2rs> {
        AFCNTR_W::new(self, 31)
    }
}
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CFG2rs;
impl crate::RegisterSpec for SPI_CFG2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cfg2::R`](R) reader structure"]
impl crate::Readable for SPI_CFG2rs {}
#[doc = "`write(|w| ..)` method takes [`spi_cfg2::W`](W) writer structure"]
impl crate::Writable for SPI_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CFG2 to value 0"]
impl crate::Resettable for SPI_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
