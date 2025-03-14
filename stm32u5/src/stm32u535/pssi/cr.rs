///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CKPOL` reader - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
pub type CKPOL_R = crate::BitReader;
///Field `CKPOL` writer - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEPOL` reader - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
pub type DEPOL_R = crate::BitReader;
///Field `DEPOL` writer - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDYPOL` reader - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
pub type RDYPOL_R = crate::BitReader;
///Field `RDYPOL` writer - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
pub type RDYPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDM` reader - Extended data mode
pub type EDM_R = crate::FieldReader;
///Field `EDM` writer - Extended data mode
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ENABLE` reader - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DERDYCFG` reader - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
pub type DERDYCFG_R = crate::FieldReader;
///Field `DERDYCFG` writer - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
pub type DERDYCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMAEN` reader - DMA enable bit
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA enable bit
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTEN` reader - Data direction selection bit
pub type OUTEN_R = crate::BitReader;
///Field `OUTEN` writer - Data direction selection bit
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn rdypol(&self) -> RDYPOL_R {
        RDYPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 14 - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 18:20 - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
    #[inline(always)]
    pub fn derdycfg(&self) -> DERDYCFG_R {
        DERDYCFG_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Data direction selection bit
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("ckpol", &self.ckpol())
            .field("depol", &self.depol())
            .field("rdypol", &self.rdypol())
            .field("edm", &self.edm())
            .field("enable", &self.enable())
            .field("derdycfg", &self.derdycfg())
            .field("dmaen", &self.dmaen())
            .field("outen", &self.outen())
            .finish()
    }
}
impl W {
    ///Bit 5 - Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN.
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<CRrs> {
        CKPOL_W::new(self, 5)
    }
    ///Bit 6 - Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W<CRrs> {
        DEPOL_W::new(self, 6)
    }
    ///Bit 8 - Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface.
    #[inline(always)]
    pub fn rdypol(&mut self) -> RDYPOL_W<CRrs> {
        RDYPOL_W::new(self, 8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W<CRrs> {
        EDM_W::new(self, 10)
    }
    ///Bit 14 - PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 14)
    }
    ///Bits 18:20 - Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity.
    #[inline(always)]
    pub fn derdycfg(&mut self) -> DERDYCFG_W<CRrs> {
        DERDYCFG_W::new(self, 18)
    }
    ///Bit 30 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 30)
    }
    ///Bit 31 - Data direction selection bit
    #[inline(always)]
    pub fn outen(&mut self) -> OUTEN_W<CRrs> {
        OUTEN_W::new(self, 31)
    }
}
/**PSSI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PSSI:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x4000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
