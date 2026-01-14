///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `VBE` reader - VBE
pub type VBE_R = crate::BitReader;
///Field `VBE` writer - VBE
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBRS` reader - VBRS
pub type VBRS_R = crate::BitReader;
///Field `VBRS` writer - VBRS
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRSREN` reader - DDRSREN
pub type DDRSREN_R = crate::BitReader;
///Field `DDRSREN` writer - DDRSREN
pub type DDRSREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRSRDIS` reader - DDRSRDIS
pub type DDRSRDIS_R = crate::BitReader;
///Field `DDRSRDIS` writer - DDRSRDIS
pub type DDRSRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRRETEN` reader - DDRRETEN
pub type DDRRETEN_R = crate::BitReader;
///Field `DDRRETEN` writer - DDRRETEN
pub type DDRRETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POPL` reader - POPL
pub type POPL_R = crate::FieldReader;
///Field `POPL` writer - POPL
pub type POPL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `USB33DEN` reader - USB33DEN
pub type USB33DEN_R = crate::BitReader;
///Field `USB33DEN` writer - USB33DEN
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33RDY` reader - USB33RDY
pub type USB33RDY_R = crate::BitReader;
///Field `REG18EN` reader - REG18EN
pub type REG18EN_R = crate::BitReader;
///Field `REG18EN` writer - REG18EN
pub type REG18EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG18RDY` reader - REG18RDY
pub type REG18RDY_R = crate::BitReader;
///Field `REG11EN` reader - REG11EN
pub type REG11EN_R = crate::BitReader;
///Field `REG11EN` writer - REG11EN
pub type REG11EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG11RDY` reader - REG11RDY
pub type REG11RDY_R = crate::BitReader;
impl R {
    ///Bit 8 - VBE
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBRS
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DDRSREN
    #[inline(always)]
    pub fn ddrsren(&self) -> DDRSREN_R {
        DDRSREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DDRSRDIS
    #[inline(always)]
    pub fn ddrsrdis(&self) -> DDRSRDIS_R {
        DDRSRDIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DDRRETEN
    #[inline(always)]
    pub fn ddrreten(&self) -> DDRRETEN_R {
        DDRRETEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 17:21 - POPL
    #[inline(always)]
    pub fn popl(&self) -> POPL_R {
        POPL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - USB33DEN
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - USB33RDY
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - REG18EN
    #[inline(always)]
    pub fn reg18en(&self) -> REG18EN_R {
        REG18EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - REG18RDY
    #[inline(always)]
    pub fn reg18rdy(&self) -> REG18RDY_R {
        REG18RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - REG11EN
    #[inline(always)]
    pub fn reg11en(&self) -> REG11EN_R {
        REG11EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - REG11RDY
    #[inline(always)]
    pub fn reg11rdy(&self) -> REG11RDY_R {
        REG11RDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .field("ddrsren", &self.ddrsren())
            .field("ddrsrdis", &self.ddrsrdis())
            .field("ddrreten", &self.ddrreten())
            .field("popl", &self.popl())
            .field("usb33den", &self.usb33den())
            .field("usb33rdy", &self.usb33rdy())
            .field("reg18en", &self.reg18en())
            .field("reg18rdy", &self.reg18rdy())
            .field("reg11en", &self.reg11en())
            .field("reg11rdy", &self.reg11rdy())
            .finish()
    }
}
impl W {
    ///Bit 8 - VBE
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<'_, CR3rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - VBRS
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<'_, CR3rs> {
        VBRS_W::new(self, 9)
    }
    ///Bit 10 - DDRSREN
    #[inline(always)]
    pub fn ddrsren(&mut self) -> DDRSREN_W<'_, CR3rs> {
        DDRSREN_W::new(self, 10)
    }
    ///Bit 11 - DDRSRDIS
    #[inline(always)]
    pub fn ddrsrdis(&mut self) -> DDRSRDIS_W<'_, CR3rs> {
        DDRSRDIS_W::new(self, 11)
    }
    ///Bit 12 - DDRRETEN
    #[inline(always)]
    pub fn ddrreten(&mut self) -> DDRRETEN_W<'_, CR3rs> {
        DDRRETEN_W::new(self, 12)
    }
    ///Bits 17:21 - POPL
    #[inline(always)]
    pub fn popl(&mut self) -> POPL_W<'_, CR3rs> {
        POPL_W::new(self, 17)
    }
    ///Bit 24 - USB33DEN
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W<'_, CR3rs> {
        USB33DEN_W::new(self, 24)
    }
    ///Bit 28 - REG18EN
    #[inline(always)]
    pub fn reg18en(&mut self) -> REG18EN_W<'_, CR3rs> {
        REG18EN_W::new(self, 28)
    }
    ///Bit 30 - REG11EN
    #[inline(always)]
    pub fn reg11en(&mut self) -> REG11EN_W<'_, CR3rs> {
        REG11EN_W::new(self, 30)
    }
}
/**Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0x5000_0000
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x5000_0000;
}
