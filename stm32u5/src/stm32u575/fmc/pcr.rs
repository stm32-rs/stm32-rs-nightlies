///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `PWAITEN` reader - Wait feature enable bit
pub type PWAITEN_R = crate::BitReader;
///Field `PWAITEN` writer - Wait feature enable bit
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBKEN` reader - NAND Flash memory bank enable bit
pub type PBKEN_R = crate::BitReader;
///Field `PBKEN` writer - NAND Flash memory bank enable bit
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTYP` reader - Memory type
pub type PTYP_R = crate::BitReader;
///Field `PTYP` writer - Memory type
pub type PTYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWID` reader - Data bus width
pub type PWID_R = crate::FieldReader;
///Field `PWID` writer - Data bus width
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECCEN` reader - ECC computation logic enable bit
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECC computation logic enable bit
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLR` reader - CLE to RE delay
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - CLE to RE delay
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TAR` reader - ALE to RE delay
pub type TAR_R = crate::FieldReader;
///Field `TAR` writer - ALE to RE delay
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ECCPS` reader - ECC page size
pub type ECCPS_R = crate::FieldReader;
///Field `ECCPS` writer - ECC page size
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 1 - Wait feature enable bit
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NAND Flash memory bank enable bit
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Memory type
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bits 9:12 - CLE to RE delay
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:15 - ALE to RE delay
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:19 - ECC page size
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("pwaiten", &self.pwaiten())
            .field("pbken", &self.pbken())
            .field("ptyp", &self.ptyp())
            .field("pwid", &self.pwid())
            .field("eccen", &self.eccen())
            .field("tclr", &self.tclr())
            .field("tar", &self.tar())
            .field("eccps", &self.eccps())
            .finish()
    }
}
impl W {
    ///Bit 1 - Wait feature enable bit
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - NAND Flash memory bank enable bit
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<PCRrs> {
        PBKEN_W::new(self, 2)
    }
    ///Bit 3 - Memory type
    #[inline(always)]
    #[must_use]
    pub fn ptyp(&mut self) -> PTYP_W<PCRrs> {
        PTYP_W::new(self, 3)
    }
    ///Bits 4:5 - Data bus width
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<PCRrs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<PCRrs> {
        ECCEN_W::new(self, 6)
    }
    ///Bits 9:12 - CLE to RE delay
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<PCRrs> {
        TCLR_W::new(self, 9)
    }
    ///Bits 13:15 - ALE to RE delay
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<PCRrs> {
        TAR_W::new(self, 13)
    }
    ///Bits 17:19 - ECC page size
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<PCRrs> {
        ECCPS_W::new(self, 17)
    }
}
/**NAND Flash control registers

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FMC:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCR to value 0x18
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x18;
}
