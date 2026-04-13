///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `PWAITEN` reader - PWAITEN
pub type PWAITEN_R = crate::BitReader;
///Field `PWAITEN` writer - PWAITEN
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBKEN` reader - PBKEN
pub type PBKEN_R = crate::BitReader;
///Field `PBKEN` writer - PBKEN
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWID` reader - PWID
pub type PWID_R = crate::FieldReader;
///Field `PWID` writer - PWID
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECCEN` reader - ECCEN
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECCEN
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCALG` reader - ECCALG
pub type ECCALG_R = crate::BitReader;
///Field `ECCALG` writer - ECCALG
pub type ECCALG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLR` reader - TCLR
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - TCLR
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TAR` reader - TAR
pub type TAR_R = crate::FieldReader;
///Field `TAR` writer - TAR
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ECCSS` reader - ECCSS
pub type ECCSS_R = crate::FieldReader;
///Field `ECCSS` writer - ECCSS
pub type ECCSS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TCEH` reader - TCEH
pub type TCEH_R = crate::FieldReader;
///Field `TCEH` writer - TCEH
pub type TCEH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BCHECC` reader - BCHECC
pub type BCHECC_R = crate::BitReader;
///Field `BCHECC` writer - BCHECC
pub type BCHECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WEN` reader - WEN
pub type WEN_R = crate::BitReader;
///Field `WEN` writer - WEN
pub type WEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - ECCALG
    #[inline(always)]
    pub fn eccalg(&self) -> ECCALG_R {
        ECCALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:19 - ECCSS
    #[inline(always)]
    pub fn eccss(&self) -> ECCSS_R {
        ECCSS_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:23 - TCEH
    #[inline(always)]
    pub fn tceh(&self) -> TCEH_R {
        TCEH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - BCHECC
    #[inline(always)]
    pub fn bchecc(&self) -> BCHECC_R {
        BCHECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - WEN
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("pwaiten", &self.pwaiten())
            .field("pbken", &self.pbken())
            .field("pwid", &self.pwid())
            .field("eccen", &self.eccen())
            .field("eccalg", &self.eccalg())
            .field("tclr", &self.tclr())
            .field("tar", &self.tar())
            .field("eccss", &self.eccss())
            .field("tceh", &self.tceh())
            .field("bchecc", &self.bchecc())
            .field("wen", &self.wen())
            .finish()
    }
}
impl W {
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W<'_, PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W<'_, PCRrs> {
        PBKEN_W::new(self, 2)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W<'_, PCRrs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<'_, PCRrs> {
        ECCEN_W::new(self, 6)
    }
    ///Bit 8 - ECCALG
    #[inline(always)]
    pub fn eccalg(&mut self) -> ECCALG_W<'_, PCRrs> {
        ECCALG_W::new(self, 8)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<'_, PCRrs> {
        TCLR_W::new(self, 9)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<'_, PCRrs> {
        TAR_W::new(self, 13)
    }
    ///Bits 17:19 - ECCSS
    #[inline(always)]
    pub fn eccss(&mut self) -> ECCSS_W<'_, PCRrs> {
        ECCSS_W::new(self, 17)
    }
    ///Bits 20:23 - TCEH
    #[inline(always)]
    pub fn tceh(&mut self) -> TCEH_W<'_, PCRrs> {
        TCEH_W::new(self, 20)
    }
    ///Bit 24 - BCHECC
    #[inline(always)]
    pub fn bchecc(&mut self) -> BCHECC_W<'_, PCRrs> {
        BCHECC_W::new(self, 24)
    }
    ///Bit 25 - WEN
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W<'_, PCRrs> {
        WEN_W::new(self, 25)
    }
}
/**NAND Flash Programmable control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR to value 0x0007_fe08
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x0007_fe08;
}
