///Register `PCR2` reader
pub type R = crate::R<PCR2rs>;
///Register `PCR2` writer
pub type W = crate::W<PCR2rs>;
///Field `PWAITEN` reader - PWAITEN
pub type PWAITEN_R = crate::BitReader;
///Field `PWAITEN` writer - PWAITEN
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBKEN` reader - PBKEN
pub type PBKEN_R = crate::BitReader;
///Field `PBKEN` writer - PBKEN
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTYP` reader - PTYP
pub type PTYP_R = crate::BitReader;
///Field `PTYP` writer - PTYP
pub type PTYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWID` reader - PWID
pub type PWID_R = crate::FieldReader;
///Field `PWID` writer - PWID
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECCEN` reader - ECCEN
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECCEN
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLR` reader - TCLR
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - TCLR
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TAR` reader - TAR
pub type TAR_R = crate::FieldReader;
///Field `TAR` writer - TAR
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ECCPS` reader - ECCPS
pub type ECCPS_R = crate::FieldReader;
///Field `ECCPS` writer - ECCPS
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR2")
            .field("eccps", &self.eccps())
            .field("tar", &self.tar())
            .field("tclr", &self.tclr())
            .field("eccen", &self.eccen())
            .field("pwid", &self.pwid())
            .field("ptyp", &self.ptyp())
            .field("pbken", &self.pbken())
            .field("pwaiten", &self.pwaiten())
            .finish()
    }
}
impl W {
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W<'_, PCR2rs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W<'_, PCR2rs> {
        PBKEN_W::new(self, 2)
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W<'_, PCR2rs> {
        PTYP_W::new(self, 3)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W<'_, PCR2rs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<'_, PCR2rs> {
        ECCEN_W::new(self, 6)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<'_, PCR2rs> {
        TCLR_W::new(self, 9)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<'_, PCR2rs> {
        TAR_W::new(self, 13)
    }
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W<'_, PCR2rs> {
        ECCPS_W::new(self, 17)
    }
}
/**PC Card/NAND Flash control register 2

You can [`read`](crate::Reg::read) this register and get [`pcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#FMC:PCR2)*/
pub struct PCR2rs;
impl crate::RegisterSpec for PCR2rs {
    type Ux = u32;
}
///`read()` method returns [`pcr2::R`](R) reader structure
impl crate::Readable for PCR2rs {}
///`write(|w| ..)` method takes [`pcr2::W`](W) writer structure
impl crate::Writable for PCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR2 to value 0x18
impl crate::Resettable for PCR2rs {
    const RESET_VALUE: u32 = 0x18;
}
