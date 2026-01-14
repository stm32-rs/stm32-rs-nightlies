///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `PWAITEN` reader - Wait feature enable bit
pub type PWAITEN_R = crate::BitReader;
///Field `PWAITEN` writer - Wait feature enable bit
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBKEN` reader - NAND Flash memory region enable bit
pub type PBKEN_R = crate::BitReader;
///Field `PBKEN` writer - NAND Flash memory region enable bit
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWID` reader - Data bus width
pub type PWID_R = crate::FieldReader;
///Field `PWID` writer - Data bus width
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECCEN` reader - ECC computation logic enable bit
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECC computation logic enable bit
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCALG` reader - ECC algorithm
pub type ECCALG_R = crate::BitReader;
///Field `ECCALG` writer - ECC algorithm
pub type ECCALG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLR` reader - CLE to RE delay.
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - CLE to RE delay.
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TAR` reader - ALE to RE delay.
pub type TAR_R = crate::FieldReader;
///Field `TAR` writer - ALE to RE delay.
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ECCSS` reader - ECC sector size (used to access spare area)
pub type ECCSS_R = crate::FieldReader;
///Field `ECCSS` writer - ECC sector size (used to access spare area)
pub type ECCSS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BCHECC` reader - BCH error correction capability
pub type BCHECC_R = crate::BitReader;
///Field `BCHECC` writer - BCH error correction capability
pub type BCHECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WEN` reader - Write enable
pub type WEN_R = crate::BitReader;
///Field `WEN` writer - Write enable
pub type WEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Wait feature enable bit
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NAND Flash memory region enable bit
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Data bus width
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - ECC algorithm
    #[inline(always)]
    pub fn eccalg(&self) -> ECCALG_R {
        ECCALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12 - CLE to RE delay.
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:16 - ALE to RE delay.
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:19 - ECC sector size (used to access spare area)
    #[inline(always)]
    pub fn eccss(&self) -> ECCSS_R {
        ECCSS_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 24 - BCH error correction capability
    #[inline(always)]
    pub fn bchecc(&self) -> BCHECC_R {
        BCHECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Write enable
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
            .field("bchecc", &self.bchecc())
            .field("wen", &self.wen())
            .finish()
    }
}
impl W {
    ///Bit 1 - Wait feature enable bit
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W<'_, PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - NAND Flash memory region enable bit
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W<'_, PCRrs> {
        PBKEN_W::new(self, 2)
    }
    ///Bits 4:5 - Data bus width
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W<'_, PCRrs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<'_, PCRrs> {
        ECCEN_W::new(self, 6)
    }
    ///Bit 8 - ECC algorithm
    #[inline(always)]
    pub fn eccalg(&mut self) -> ECCALG_W<'_, PCRrs> {
        ECCALG_W::new(self, 8)
    }
    ///Bits 9:12 - CLE to RE delay.
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<'_, PCRrs> {
        TCLR_W::new(self, 9)
    }
    ///Bits 13:16 - ALE to RE delay.
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<'_, PCRrs> {
        TAR_W::new(self, 13)
    }
    ///Bits 17:19 - ECC sector size (used to access spare area)
    #[inline(always)]
    pub fn eccss(&mut self) -> ECCSS_W<'_, PCRrs> {
        ECCSS_W::new(self, 17)
    }
    ///Bit 24 - BCH error correction capability
    #[inline(always)]
    pub fn bchecc(&mut self) -> BCHECC_W<'_, PCRrs> {
        BCHECC_W::new(self, 24)
    }
    ///Bit 25 - Write enable
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W<'_, PCRrs> {
        WEN_W::new(self, 25)
    }
}
/**NAND Flash programmable control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:PCR)*/
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
