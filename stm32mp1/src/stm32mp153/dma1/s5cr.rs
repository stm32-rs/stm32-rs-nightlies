///Register `S5CR` reader
pub type R = crate::R<S5CRrs>;
///Register `S5CR` writer
pub type W = crate::W<S5CRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMEIE` reader - DMEIE
pub type DMEIE_R = crate::BitReader;
///Field `DMEIE` writer - DMEIE
pub type DMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - TEIE
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - TEIE
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - HTIE
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - HTIE
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PFCTRL` reader - PFCTRL
pub type PFCTRL_R = crate::BitReader;
///Field `PFCTRL` writer - PFCTRL
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - DIR
pub type DIR_R = crate::FieldReader;
///Field `DIR` writer - DIR
pub type DIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CIRC` reader - CIRC
pub type CIRC_R = crate::BitReader;
///Field `CIRC` writer - CIRC
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINC` reader - PINC
pub type PINC_R = crate::BitReader;
///Field `PINC` writer - PINC
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MINC` reader - MINC
pub type MINC_R = crate::BitReader;
///Field `MINC` writer - MINC
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSIZE` reader - PSIZE
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - PSIZE
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSIZE` reader - MSIZE
pub type MSIZE_R = crate::FieldReader;
///Field `MSIZE` writer - MSIZE
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINCOS` reader - PINCOS
pub type PINCOS_R = crate::BitReader;
///Field `PINCOS` writer - PINCOS
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PL` reader - PL
pub type PL_R = crate::FieldReader;
///Field `PL` writer - PL
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DBM` reader - DBM
pub type DBM_R = crate::BitReader;
///Field `DBM` writer - DBM
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CT` reader - CT
pub type CT_R = crate::BitReader;
///Field `CT` writer - CT
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBURST` reader - PBURST
pub type PBURST_R = crate::FieldReader;
///Field `PBURST` writer - PBURST
pub type PBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MBURST` reader - MBURST
pub type MBURST_R = crate::FieldReader;
///Field `MBURST` writer - MBURST
pub type MBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMEIE
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TEIE
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HTIE
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PFCTRL
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - DIR
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - CIRC
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PINC
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MINC
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - PSIZE
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - MSIZE
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - PINCOS
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - PL
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - DBM
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CT
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 21:22 - PBURST
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 23:24 - MBURST
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S5CR")
            .field("en", &self.en())
            .field("dmeie", &self.dmeie())
            .field("teie", &self.teie())
            .field("htie", &self.htie())
            .field("tcie", &self.tcie())
            .field("pfctrl", &self.pfctrl())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pincos", &self.pincos())
            .field("pl", &self.pl())
            .field("dbm", &self.dbm())
            .field("ct", &self.ct())
            .field("pburst", &self.pburst())
            .field("mburst", &self.mburst())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, S5CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - DMEIE
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W<'_, S5CRrs> {
        DMEIE_W::new(self, 1)
    }
    ///Bit 2 - TEIE
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, S5CRrs> {
        TEIE_W::new(self, 2)
    }
    ///Bit 3 - HTIE
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, S5CRrs> {
        HTIE_W::new(self, 3)
    }
    ///Bit 4 - TCIE
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, S5CRrs> {
        TCIE_W::new(self, 4)
    }
    ///Bit 5 - PFCTRL
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W<'_, S5CRrs> {
        PFCTRL_W::new(self, 5)
    }
    ///Bits 6:7 - DIR
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, S5CRrs> {
        DIR_W::new(self, 6)
    }
    ///Bit 8 - CIRC
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, S5CRrs> {
        CIRC_W::new(self, 8)
    }
    ///Bit 9 - PINC
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, S5CRrs> {
        PINC_W::new(self, 9)
    }
    ///Bit 10 - MINC
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, S5CRrs> {
        MINC_W::new(self, 10)
    }
    ///Bits 11:12 - PSIZE
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, S5CRrs> {
        PSIZE_W::new(self, 11)
    }
    ///Bits 13:14 - MSIZE
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, S5CRrs> {
        MSIZE_W::new(self, 13)
    }
    ///Bit 15 - PINCOS
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W<'_, S5CRrs> {
        PINCOS_W::new(self, 15)
    }
    ///Bits 16:17 - PL
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, S5CRrs> {
        PL_W::new(self, 16)
    }
    ///Bit 18 - DBM
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<'_, S5CRrs> {
        DBM_W::new(self, 18)
    }
    ///Bit 19 - CT
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<'_, S5CRrs> {
        CT_W::new(self, 19)
    }
    ///Bits 21:22 - PBURST
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W<'_, S5CRrs> {
        PBURST_W::new(self, 21)
    }
    ///Bits 23:24 - MBURST
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W<'_, S5CRrs> {
        MBURST_W::new(self, 23)
    }
}
/**This register is used to configure the concerned stream.

You can [`read`](crate::Reg::read) this register and get [`s5cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S5CR)*/
pub struct S5CRrs;
impl crate::RegisterSpec for S5CRrs {
    type Ux = u32;
}
///`read()` method returns [`s5cr::R`](R) reader structure
impl crate::Readable for S5CRrs {}
///`write(|w| ..)` method takes [`s5cr::W`](W) writer structure
impl crate::Writable for S5CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S5CR to value 0
impl crate::Resettable for S5CRrs {}
