#[doc = "Register `PWR_CR3` reader"]
pub type R = crate::R<PWR_CR3rs>;
#[doc = "Register `PWR_CR3` writer"]
pub type W = crate::W<PWR_CR3rs>;
#[doc = "Field `VBE` reader - VBE"]
pub type VBE_R = crate::BitReader;
#[doc = "Field `VBE` writer - VBE"]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBRS` reader - VBRS"]
pub type VBRS_R = crate::BitReader;
#[doc = "Field `VBRS` writer - VBRS"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRSREN` reader - DDRSREN"]
pub type DDRSREN_R = crate::BitReader;
#[doc = "Field `DDRSREN` writer - DDRSREN"]
pub type DDRSREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRSRDIS` reader - DDRSRDIS"]
pub type DDRSRDIS_R = crate::BitReader;
#[doc = "Field `DDRSRDIS` writer - DDRSRDIS"]
pub type DDRSRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRRETEN` reader - DDRRETEN"]
pub type DDRRETEN_R = crate::BitReader;
#[doc = "Field `DDRRETEN` writer - DDRRETEN"]
pub type DDRRETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPL` reader - POPL"]
pub type POPL_R = crate::FieldReader;
#[doc = "Field `POPL` writer - POPL"]
pub type POPL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `USB33DEN` reader - USB33DEN"]
pub type USB33DEN_R = crate::BitReader;
#[doc = "Field `USB33DEN` writer - USB33DEN"]
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB33RDY` reader - USB33RDY"]
pub type USB33RDY_R = crate::BitReader;
#[doc = "Field `REG18EN` reader - REG18EN"]
pub type REG18EN_R = crate::BitReader;
#[doc = "Field `REG18EN` writer - REG18EN"]
pub type REG18EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG18RDY` reader - REG18RDY"]
pub type REG18RDY_R = crate::BitReader;
#[doc = "Field `REG11EN` reader - REG11EN"]
pub type REG11EN_R = crate::BitReader;
#[doc = "Field `REG11EN` writer - REG11EN"]
pub type REG11EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG11RDY` reader - REG11RDY"]
pub type REG11RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    pub fn ddrsren(&self) -> DDRSREN_R {
        DDRSREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    pub fn ddrsrdis(&self) -> DDRSRDIS_R {
        DDRSRDIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    pub fn ddrreten(&self) -> DDRRETEN_R {
        DDRRETEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    pub fn popl(&self) -> POPL_R {
        POPL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - USB33RDY"]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    pub fn reg18en(&self) -> REG18EN_R {
        REG18EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - REG18RDY"]
    #[inline(always)]
    pub fn reg18rdy(&self) -> REG18RDY_R {
        REG18RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    pub fn reg11en(&self) -> REG11EN_R {
        REG11EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - REG11RDY"]
    #[inline(always)]
    pub fn reg11rdy(&self) -> REG11RDY_R {
        REG11RDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<PWR_CR3rs> {
        VBE_W::new(self, 8)
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<PWR_CR3rs> {
        VBRS_W::new(self, 9)
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrsren(&mut self) -> DDRSREN_W<PWR_CR3rs> {
        DDRSREN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    #[must_use]
    pub fn ddrsrdis(&mut self) -> DDRSRDIS_W<PWR_CR3rs> {
        DDRSRDIS_W::new(self, 11)
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrreten(&mut self) -> DDRRETEN_W<PWR_CR3rs> {
        DDRRETEN_W::new(self, 12)
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    #[must_use]
    pub fn popl(&mut self) -> POPL_W<PWR_CR3rs> {
        POPL_W::new(self, 17)
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<PWR_CR3rs> {
        USB33DEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    #[must_use]
    pub fn reg18en(&mut self) -> REG18EN_W<PWR_CR3rs> {
        REG18EN_W::new(self, 28)
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    #[must_use]
    pub fn reg11en(&mut self) -> REG11EN_W<PWR_CR3rs> {
        REG11EN_W::new(self, 30)
    }
}
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_CR3rs;
impl crate::RegisterSpec for PWR_CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr3::R`](R) reader structure"]
impl crate::Readable for PWR_CR3rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr3::W`](W) writer structure"]
impl crate::Writable for PWR_CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0x5000_0000"]
impl crate::Resettable for PWR_CR3rs {
    const RESET_VALUE: u32 = 0x5000_0000;
}
