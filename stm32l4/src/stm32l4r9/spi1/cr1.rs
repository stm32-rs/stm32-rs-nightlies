#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR` reader - Master selection"]
pub type MSTR_R = crate::BitReader;
#[doc = "Field `MSTR` writer - Master selection"]
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR` reader - Baud rate control"]
pub type BR_R = crate::FieldReader;
#[doc = "Field `BR` writer - Baud rate control"]
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPE` reader - SPI enable"]
pub type SPE_R = crate::BitReader;
#[doc = "Field `SPE` writer - SPI enable"]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFIRST` reader - Frame format"]
pub type LSBFIRST_R = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - Frame format"]
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI` reader - Internal slave select"]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - Internal slave select"]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Software slave management"]
pub type SSM_R = crate::BitReader;
#[doc = "Field `SSM` writer - Software slave management"]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXONLY` reader - Receive only"]
pub type RXONLY_R = crate::BitReader;
#[doc = "Field `RXONLY` writer - Receive only"]
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFF` reader - Data frame format"]
pub type DFF_R = crate::BitReader;
#[doc = "Field `DFF` writer - Data frame format"]
pub type DFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNEXT` reader - CRC transfer next"]
pub type CRCNEXT_R = crate::BitReader;
#[doc = "Field `CRCNEXT` writer - CRC transfer next"]
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode"]
pub type BIDIOE_R = crate::BitReader;
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode"]
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable"]
pub type BIDIMODE_R = crate::BitReader;
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable"]
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR1rs> {
        CPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR1rs> {
        CPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<CR1rs> {
        MSTR_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BR_W<CR1rs> {
        BR_W::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<CR1rs> {
        SPE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<CR1rs> {
        LSBFIRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<CR1rs> {
        SSI_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<CR1rs> {
        SSM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn rxonly(&mut self) -> RXONLY_W<CR1rs> {
        RXONLY_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    #[must_use]
    pub fn dff(&mut self) -> DFF_W<CR1rs> {
        DFF_W::new(self, 11)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    #[must_use]
    pub fn crcnext(&mut self) -> CRCNEXT_W<CR1rs> {
        CRCNEXT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CR1rs> {
        CRCEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    #[must_use]
    pub fn bidioe(&mut self) -> BIDIOE_W<CR1rs> {
        BIDIOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bidimode(&mut self) -> BIDIMODE_W<CR1rs> {
        BIDIMODE_W::new(self, 15)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
