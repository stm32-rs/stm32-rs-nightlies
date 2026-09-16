///Register `CFG2` reader
pub type R = crate::R<CFG2rs>;
///Register `CFG2` writer
pub type W = crate::W<CFG2rs>;
///Field `MSSI` reader - MSSI
pub type MSSI_R = crate::FieldReader;
///Field `MSSI` writer - MSSI
pub type MSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MIDI` reader - MIDI
pub type MIDI_R = crate::FieldReader;
///Field `MIDI` writer - MIDI
pub type MIDI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IOSWP` reader - IOSWP
pub type IOSWP_R = crate::BitReader;
///Field `IOSWP` writer - IOSWP
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMM` reader - COMM
pub type COMM_R = crate::FieldReader;
///Field `COMM` writer - COMM
pub type COMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SP` reader - SP
pub type SP_R = crate::FieldReader;
///Field `SP` writer - SP
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MASTER` reader - MASTER
pub type MASTER_R = crate::BitReader;
///Field `MASTER` writer - MASTER
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSBFRST` reader - LSBFRST
pub type LSBFRST_R = crate::BitReader;
///Field `LSBFRST` writer - LSBFRST
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPHA` reader - CPHA
pub type CPHA_R = crate::BitReader;
///Field `CPHA` writer - CPHA
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - CPOL
pub type CPOL_R = crate::BitReader;
///Field `CPOL` writer - CPOL
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSM` reader - SSM
pub type SSM_R = crate::BitReader;
///Field `SSM` writer - SSM
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSIOP` reader - SSIOP
pub type SSIOP_R = crate::BitReader;
///Field `SSIOP` writer - SSIOP
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSOE` reader - SSOE
pub type SSOE_R = crate::BitReader;
///Field `SSOE` writer - SSOE
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSOM` reader - SSOM
pub type SSOM_R = crate::BitReader;
///Field `SSOM` writer - SSOM
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFCNTR` reader - AFCNTR
pub type AFCNTR_R = crate::BitReader;
///Field `AFCNTR` writer - AFCNTR
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - MSSI
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MIDI
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 15 - IOSWP
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:18 - COMM
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:21 - SP
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - MASTER
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - LSBFRST
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPHA
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CPOL
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SSM
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - SSIOP
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SSOE
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SSOM
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AFCNTR
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("mssi", &self.mssi())
            .field("midi", &self.midi())
            .field("ioswp", &self.ioswp())
            .field("comm", &self.comm())
            .field("sp", &self.sp())
            .field("master", &self.master())
            .field("lsbfrst", &self.lsbfrst())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("ssm", &self.ssm())
            .field("ssiop", &self.ssiop())
            .field("ssoe", &self.ssoe())
            .field("ssom", &self.ssom())
            .field("afcntr", &self.afcntr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - MSSI
    #[inline(always)]
    pub fn mssi(&mut self) -> MSSI_W<'_, CFG2rs> {
        MSSI_W::new(self, 0)
    }
    ///Bits 4:7 - MIDI
    #[inline(always)]
    pub fn midi(&mut self) -> MIDI_W<'_, CFG2rs> {
        MIDI_W::new(self, 4)
    }
    ///Bit 15 - IOSWP
    #[inline(always)]
    pub fn ioswp(&mut self) -> IOSWP_W<'_, CFG2rs> {
        IOSWP_W::new(self, 15)
    }
    ///Bits 17:18 - COMM
    #[inline(always)]
    pub fn comm(&mut self) -> COMM_W<'_, CFG2rs> {
        COMM_W::new(self, 17)
    }
    ///Bits 19:21 - SP
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W<'_, CFG2rs> {
        SP_W::new(self, 19)
    }
    ///Bit 22 - MASTER
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W<'_, CFG2rs> {
        MASTER_W::new(self, 22)
    }
    ///Bit 23 - LSBFRST
    #[inline(always)]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<'_, CFG2rs> {
        LSBFRST_W::new(self, 23)
    }
    ///Bit 24 - CPHA
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, CFG2rs> {
        CPHA_W::new(self, 24)
    }
    ///Bit 25 - CPOL
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, CFG2rs> {
        CPOL_W::new(self, 25)
    }
    ///Bit 26 - SSM
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<'_, CFG2rs> {
        SSM_W::new(self, 26)
    }
    ///Bit 28 - SSIOP
    #[inline(always)]
    pub fn ssiop(&mut self) -> SSIOP_W<'_, CFG2rs> {
        SSIOP_W::new(self, 28)
    }
    ///Bit 29 - SSOE
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<'_, CFG2rs> {
        SSOE_W::new(self, 29)
    }
    ///Bit 30 - SSOM
    #[inline(always)]
    pub fn ssom(&mut self) -> SSOM_W<'_, CFG2rs> {
        SSOM_W::new(self, 30)
    }
    ///Bit 31 - AFCNTR
    #[inline(always)]
    pub fn afcntr(&mut self) -> AFCNTR_W<'_, CFG2rs> {
        AFCNTR_W::new(self, 31)
    }
}
/**The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPI1:CFG2)*/
pub struct CFG2rs;
impl crate::RegisterSpec for CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`cfg2::R`](R) reader structure
impl crate::Readable for CFG2rs {}
///`write(|w| ..)` method takes [`cfg2::W`](W) writer structure
impl crate::Writable for CFG2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG2 to value 0
impl crate::Resettable for CFG2rs {}
