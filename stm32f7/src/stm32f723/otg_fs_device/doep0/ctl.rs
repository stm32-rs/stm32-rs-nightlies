///Register `CTL` reader
pub type R = crate::R<CTLrs>;
///Register `CTL` writer
pub type W = crate::W<CTLrs>;
///Field `MPSIZ` reader - MPSIZ
pub type MPSIZ_R = crate::FieldReader;
///Field `USBAEP` reader - USBAEP
pub type USBAEP_R = crate::BitReader;
///Field `NAKSTS` reader - NAKSTS
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - EPTYP
pub type EPTYP_R = crate::FieldReader;
///Field `SNPM` reader - SNPM
pub type SNPM_R = crate::BitReader;
///Field `SNPM` writer - SNPM
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader - STALL handshake
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - CNAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - SNAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - EPDIS
pub type EPDIS_R = crate::BitReader;
///Field `EPENA` reader - EPENA
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - EPENA
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - NAKSTS
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - SNPM
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("epena", &self.epena())
            .field("epdis", &self.epdis())
            .field("stall", &self.stall())
            .field("snpm", &self.snpm())
            .field("eptyp", &self.eptyp())
            .field("naksts", &self.naksts())
            .field("usbaep", &self.usbaep())
            .field("mpsiz", &self.mpsiz())
            .finish()
    }
}
impl W {
    ///Bit 20 - SNPM
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<CTLrs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<CTLrs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<CTLrs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<CTLrs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<CTLrs> {
        EPENA_W::new(self, 31)
    }
}
/**device endpoint-0 control register

You can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTLrs;
impl crate::RegisterSpec for CTLrs {
    type Ux = u32;
}
///`read()` method returns [`ctl::R`](R) reader structure
impl crate::Readable for CTLrs {}
///`write(|w| ..)` method takes [`ctl::W`](W) writer structure
impl crate::Writable for CTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTL to value 0x8000
impl crate::Resettable for CTLrs {
    const RESET_VALUE: u32 = 0x8000;
}
