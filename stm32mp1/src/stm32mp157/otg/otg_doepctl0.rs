///Register `OTG_DOEPCTL0` reader
pub type R = crate::R<OTG_DOEPCTL0rs>;
///Register `OTG_DOEPCTL0` writer
pub type W = crate::W<OTG_DOEPCTL0rs>;
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
///Field `STALL` reader - STALL
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - CNAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - SNAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - EPDIS
pub type EPDIS_R = crate::BitReader;
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
    ///Bit 21 - STALL
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_DOEPCTL0")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("snpm", &self.snpm())
            .field("stall", &self.stall())
            .field("epdis", &self.epdis())
            .finish()
    }
}
impl W {
    ///Bit 20 - SNPM
    #[inline(always)]
    #[must_use]
    pub fn snpm(&mut self) -> SNPM_W<OTG_DOEPCTL0rs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - STALL
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<OTG_DOEPCTL0rs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<OTG_DOEPCTL0rs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<OTG_DOEPCTL0rs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<OTG_DOEPCTL0rs> {
        EPENA_W::new(self, 31)
    }
}
/**This section describes the OTG_DOEPCTL0 register.

You can [`read`](crate::Reg::read) this register and get [`otg_doepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_doepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:OTG_DOEPCTL0)*/
pub struct OTG_DOEPCTL0rs;
impl crate::RegisterSpec for OTG_DOEPCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`otg_doepctl0::R`](R) reader structure
impl crate::Readable for OTG_DOEPCTL0rs {}
///`write(|w| ..)` method takes [`otg_doepctl0::W`](W) writer structure
impl crate::Writable for OTG_DOEPCTL0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_DOEPCTL0 to value 0x8000
impl crate::Resettable for OTG_DOEPCTL0rs {
    const RESET_VALUE: u32 = 0x8000;
}
