///Register `HCCHAR1` reader
pub type R = crate::R<HCCHAR1rs>;
///Register `HCCHAR1` writer
pub type W = crate::W<HCCHAR1rs>;
///Field `MPSIZ` reader - MPSIZ
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - MPSIZ
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `EPNUM` reader - EPNUM
pub type EPNUM_R = crate::FieldReader;
///Field `EPNUM` writer - EPNUM
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EPDIR` reader - EPDIR
pub type EPDIR_R = crate::BitReader;
///Field `EPDIR` writer - EPDIR
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSDEV` reader - LSDEV
pub type LSDEV_R = crate::BitReader;
///Field `LSDEV` writer - LSDEV
pub type LSDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPTYP` reader - EPTYP
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - EPTYP
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCNT` reader - MCNT
pub type MCNT_R = crate::FieldReader;
///Field `MCNT` writer - MCNT
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAD` reader - DAD
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - DAD
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CHDIS` reader - CHDIS
pub type CHDIS_R = crate::BitReader;
///Field `CHDIS` writer - CHDIS
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHENA` reader - CHENA
pub type CHENA_R = crate::BitReader;
///Field `CHENA` writer - CHENA
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:14 - EPNUM
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - EPDIR
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - LSDEV
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - MCNT
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:28 - DAD
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    ///Bit 30 - CHDIS
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CHENA
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR1")
            .field("mpsiz", &self.mpsiz())
            .field("epnum", &self.epnum())
            .field("epdir", &self.epdir())
            .field("lsdev", &self.lsdev())
            .field("eptyp", &self.eptyp())
            .field("mcnt", &self.mcnt())
            .field("dad", &self.dad())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<HCCHAR1rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bits 11:14 - EPNUM
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W<HCCHAR1rs> {
        EPNUM_W::new(self, 11)
    }
    ///Bit 15 - EPDIR
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W<HCCHAR1rs> {
        EPDIR_W::new(self, 15)
    }
    ///Bit 17 - LSDEV
    #[inline(always)]
    pub fn lsdev(&mut self) -> LSDEV_W<HCCHAR1rs> {
        LSDEV_W::new(self, 17)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<HCCHAR1rs> {
        EPTYP_W::new(self, 18)
    }
    ///Bits 20:21 - MCNT
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<HCCHAR1rs> {
        MCNT_W::new(self, 20)
    }
    ///Bits 22:28 - DAD
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<HCCHAR1rs> {
        DAD_W::new(self, 22)
    }
    ///Bit 30 - CHDIS
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W<HCCHAR1rs> {
        CHDIS_W::new(self, 30)
    }
    ///Bit 31 - CHENA
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W<HCCHAR1rs> {
        CHENA_W::new(self, 31)
    }
}
/**OTG host channel 1 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:HCCHAR1)*/
pub struct HCCHAR1rs;
impl crate::RegisterSpec for HCCHAR1rs {
    type Ux = u32;
}
///`read()` method returns [`hcchar1::R`](R) reader structure
impl crate::Readable for HCCHAR1rs {}
///`write(|w| ..)` method takes [`hcchar1::W`](W) writer structure
impl crate::Writable for HCCHAR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCCHAR1 to value 0
impl crate::Resettable for HCCHAR1rs {}
