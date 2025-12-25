///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader;
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader;
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTR` reader - Master selection
pub type MSTR_R = crate::BitReader;
///Field `MSTR` writer - Master selection
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR` reader - Baud rate control
pub type BR_R = crate::FieldReader;
///Field `BR` writer - Baud rate control
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPE` reader - SPI enable
pub type SPE_R = crate::BitReader;
///Field `SPE` writer - SPI enable
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSBFIRST` reader - Frame format
pub type LSBFIRST_R = crate::BitReader;
///Field `LSBFIRST` writer - Frame format
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSI` reader - Internal slave select
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - Internal slave select
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` reader - Software slave management
pub type SSM_R = crate::BitReader;
///Field `SSM` writer - Software slave management
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXONLY` reader - Receive only
pub type RXONLY_R = crate::BitReader;
///Field `RXONLY` writer - Receive only
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFF` reader - Data frame format
pub type DFF_R = crate::BitReader;
///Field `DFF` writer - Data frame format
pub type DFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCNEXT` reader - CRC transfer next
pub type CRCNEXT_R = crate::BitReader;
///Field `CRCNEXT` writer - CRC transfer next
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - Hardware CRC calculation enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - Hardware CRC calculation enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIDIOE` reader - Output enable in bidirectional mode
pub type BIDIOE_R = crate::BitReader;
///Field `BIDIOE` writer - Output enable in bidirectional mode
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIDIMODE` reader - Bidirectional data mode enable
pub type BIDIMODE_R = crate::BitReader;
///Field `BIDIMODE` writer - Bidirectional data mode enable
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data frame format
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("bidimode", &self.bidimode())
            .field("bidioe", &self.bidioe())
            .field("crcen", &self.crcen())
            .field("crcnext", &self.crcnext())
            .field("dff", &self.dff())
            .field("rxonly", &self.rxonly())
            .field("ssm", &self.ssm())
            .field("ssi", &self.ssi())
            .field("lsbfirst", &self.lsbfirst())
            .field("spe", &self.spe())
            .field("br", &self.br())
            .field("mstr", &self.mstr())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, CR1rs> {
        CPHA_W::new(self, 0)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, CR1rs> {
        CPOL_W::new(self, 1)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<'_, CR1rs> {
        MSTR_W::new(self, 2)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<'_, CR1rs> {
        BR_W::new(self, 3)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<'_, CR1rs> {
        SPE_W::new(self, 6)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<'_, CR1rs> {
        LSBFIRST_W::new(self, 7)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<'_, CR1rs> {
        SSI_W::new(self, 8)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<'_, CR1rs> {
        SSM_W::new(self, 9)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W<'_, CR1rs> {
        RXONLY_W::new(self, 10)
    }
    ///Bit 11 - Data frame format
    #[inline(always)]
    pub fn dff(&mut self) -> DFF_W<'_, CR1rs> {
        DFF_W::new(self, 11)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W<'_, CR1rs> {
        CRCNEXT_W::new(self, 12)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, CR1rs> {
        CRCEN_W::new(self, 13)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W<'_, CR1rs> {
        BIDIOE_W::new(self, 14)
    }
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W<'_, CR1rs> {
        BIDIMODE_W::new(self, 15)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SPI1:CR1)*/
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
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
