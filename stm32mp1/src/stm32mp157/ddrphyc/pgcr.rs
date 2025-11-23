///Register `PGCR` reader
pub type R = crate::R<PGCRrs>;
///Register `PGCR` writer
pub type W = crate::W<PGCRrs>;
///Field `ITMDMD` reader - ITMDMD
pub type ITMDMD_R = crate::BitReader;
///Field `ITMDMD` writer - ITMDMD
pub type ITMDMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSCFG` reader - DQSCFG
pub type DQSCFG_R = crate::BitReader;
///Field `DQSCFG` writer - DQSCFG
pub type DQSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTCMP` reader - DFTCMP
pub type DFTCMP_R = crate::BitReader;
///Field `DFTCMP` writer - DFTCMP
pub type DFTCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFTLMT` reader - DFTLMT
pub type DFTLMT_R = crate::FieldReader;
///Field `DFTLMT` writer - DFTLMT
pub type DFTLMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOSEL` reader - DTOSEL
pub type DTOSEL_R = crate::FieldReader;
///Field `DTOSEL` writer - DTOSEL
pub type DTOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CKEN` reader - CKEN
pub type CKEN_R = crate::FieldReader;
///Field `CKEN` writer - CKEN
pub type CKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CKDV` reader - CKDV
pub type CKDV_R = crate::FieldReader;
///Field `CKDV` writer - CKDV
pub type CKDV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKINV` reader - CKINV
pub type CKINV_R = crate::BitReader;
///Field `CKINV` writer - CKINV
pub type CKINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOLB` reader - IOLB
pub type IOLB_R = crate::BitReader;
///Field `IOLB` writer - IOLB
pub type IOLB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IODDRM` reader - IODDRM
pub type IODDRM_R = crate::FieldReader;
///Field `IODDRM` writer - IODDRM
pub type IODDRM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RANKEN` reader - RANKEN
pub type RANKEN_R = crate::FieldReader;
///Field `RANKEN` writer - RANKEN
pub type RANKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ZKSEL` reader - ZKSEL
pub type ZKSEL_R = crate::FieldReader;
///Field `ZKSEL` writer - ZKSEL
pub type ZKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PDDISDX` reader - PDDISDX
pub type PDDISDX_R = crate::BitReader;
///Field `PDDISDX` writer - PDDISDX
pub type PDDISDX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFSHDT` reader - RFSHDT
pub type RFSHDT_R = crate::FieldReader;
///Field `RFSHDT` writer - RFSHDT
pub type RFSHDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LBDQSS` reader - LBDQSS
pub type LBDQSS_R = crate::BitReader;
///Field `LBDQSS` writer - LBDQSS
pub type LBDQSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBGDQS` reader - LBGDQS
pub type LBGDQS_R = crate::BitReader;
///Field `LBGDQS` writer - LBGDQS
pub type LBGDQS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBMODE` reader - LBMODE
pub type LBMODE_R = crate::BitReader;
///Field `LBMODE` writer - LBMODE
pub type LBMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ITMDMD
    #[inline(always)]
    pub fn itmdmd(&self) -> ITMDMD_R {
        ITMDMD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DQSCFG
    #[inline(always)]
    pub fn dqscfg(&self) -> DQSCFG_R {
        DQSCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DFTCMP
    #[inline(always)]
    pub fn dftcmp(&self) -> DFTCMP_R {
        DFTCMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - DFTLMT
    #[inline(always)]
    pub fn dftlmt(&self) -> DFTLMT_R {
        DFTLMT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:8 - DTOSEL
    #[inline(always)]
    pub fn dtosel(&self) -> DTOSEL_R {
        DTOSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:11 - CKEN
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:13 - CKDV
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - CKINV
    #[inline(always)]
    pub fn ckinv(&self) -> CKINV_R {
        CKINV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IOLB
    #[inline(always)]
    pub fn iolb(&self) -> IOLB_R {
        IOLB_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - IODDRM
    #[inline(always)]
    pub fn ioddrm(&self) -> IODDRM_R {
        IODDRM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - RANKEN
    #[inline(always)]
    pub fn ranken(&self) -> RANKEN_R {
        RANKEN_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:23 - ZKSEL
    #[inline(always)]
    pub fn zksel(&self) -> ZKSEL_R {
        ZKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - PDDISDX
    #[inline(always)]
    pub fn pddisdx(&self) -> PDDISDX_R {
        PDDISDX_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - RFSHDT
    #[inline(always)]
    pub fn rfshdt(&self) -> RFSHDT_R {
        RFSHDT_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bit 29 - LBDQSS
    #[inline(always)]
    pub fn lbdqss(&self) -> LBDQSS_R {
        LBDQSS_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - LBGDQS
    #[inline(always)]
    pub fn lbgdqs(&self) -> LBGDQS_R {
        LBGDQS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LBMODE
    #[inline(always)]
    pub fn lbmode(&self) -> LBMODE_R {
        LBMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGCR")
            .field("itmdmd", &self.itmdmd())
            .field("dqscfg", &self.dqscfg())
            .field("dftcmp", &self.dftcmp())
            .field("dftlmt", &self.dftlmt())
            .field("dtosel", &self.dtosel())
            .field("cken", &self.cken())
            .field("ckdv", &self.ckdv())
            .field("ckinv", &self.ckinv())
            .field("iolb", &self.iolb())
            .field("ioddrm", &self.ioddrm())
            .field("ranken", &self.ranken())
            .field("zksel", &self.zksel())
            .field("pddisdx", &self.pddisdx())
            .field("rfshdt", &self.rfshdt())
            .field("lbdqss", &self.lbdqss())
            .field("lbgdqs", &self.lbgdqs())
            .field("lbmode", &self.lbmode())
            .finish()
    }
}
impl W {
    ///Bit 0 - ITMDMD
    #[inline(always)]
    pub fn itmdmd(&mut self) -> ITMDMD_W<'_, PGCRrs> {
        ITMDMD_W::new(self, 0)
    }
    ///Bit 1 - DQSCFG
    #[inline(always)]
    pub fn dqscfg(&mut self) -> DQSCFG_W<'_, PGCRrs> {
        DQSCFG_W::new(self, 1)
    }
    ///Bit 2 - DFTCMP
    #[inline(always)]
    pub fn dftcmp(&mut self) -> DFTCMP_W<'_, PGCRrs> {
        DFTCMP_W::new(self, 2)
    }
    ///Bits 3:4 - DFTLMT
    #[inline(always)]
    pub fn dftlmt(&mut self) -> DFTLMT_W<'_, PGCRrs> {
        DFTLMT_W::new(self, 3)
    }
    ///Bits 5:8 - DTOSEL
    #[inline(always)]
    pub fn dtosel(&mut self) -> DTOSEL_W<'_, PGCRrs> {
        DTOSEL_W::new(self, 5)
    }
    ///Bits 9:11 - CKEN
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W<'_, PGCRrs> {
        CKEN_W::new(self, 9)
    }
    ///Bits 12:13 - CKDV
    #[inline(always)]
    pub fn ckdv(&mut self) -> CKDV_W<'_, PGCRrs> {
        CKDV_W::new(self, 12)
    }
    ///Bit 14 - CKINV
    #[inline(always)]
    pub fn ckinv(&mut self) -> CKINV_W<'_, PGCRrs> {
        CKINV_W::new(self, 14)
    }
    ///Bit 15 - IOLB
    #[inline(always)]
    pub fn iolb(&mut self) -> IOLB_W<'_, PGCRrs> {
        IOLB_W::new(self, 15)
    }
    ///Bits 16:17 - IODDRM
    #[inline(always)]
    pub fn ioddrm(&mut self) -> IODDRM_W<'_, PGCRrs> {
        IODDRM_W::new(self, 16)
    }
    ///Bits 18:21 - RANKEN
    #[inline(always)]
    pub fn ranken(&mut self) -> RANKEN_W<'_, PGCRrs> {
        RANKEN_W::new(self, 18)
    }
    ///Bits 22:23 - ZKSEL
    #[inline(always)]
    pub fn zksel(&mut self) -> ZKSEL_W<'_, PGCRrs> {
        ZKSEL_W::new(self, 22)
    }
    ///Bit 24 - PDDISDX
    #[inline(always)]
    pub fn pddisdx(&mut self) -> PDDISDX_W<'_, PGCRrs> {
        PDDISDX_W::new(self, 24)
    }
    ///Bits 25:28 - RFSHDT
    #[inline(always)]
    pub fn rfshdt(&mut self) -> RFSHDT_W<'_, PGCRrs> {
        RFSHDT_W::new(self, 25)
    }
    ///Bit 29 - LBDQSS
    #[inline(always)]
    pub fn lbdqss(&mut self) -> LBDQSS_W<'_, PGCRrs> {
        LBDQSS_W::new(self, 29)
    }
    ///Bit 30 - LBGDQS
    #[inline(always)]
    pub fn lbgdqs(&mut self) -> LBGDQS_W<'_, PGCRrs> {
        LBGDQS_W::new(self, 30)
    }
    ///Bit 31 - LBMODE
    #[inline(always)]
    pub fn lbmode(&mut self) -> LBMODE_W<'_, PGCRrs> {
        LBMODE_W::new(self, 31)
    }
}
/**DDRPHYC PHY global control register

You can [`read`](crate::Reg::read) this register and get [`pgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PGCR)*/
pub struct PGCRrs;
impl crate::RegisterSpec for PGCRrs {
    type Ux = u32;
}
///`read()` method returns [`pgcr::R`](R) reader structure
impl crate::Readable for PGCRrs {}
///`write(|w| ..)` method takes [`pgcr::W`](W) writer structure
impl crate::Writable for PGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PGCR to value 0x01bc_2e04
impl crate::Resettable for PGCRrs {
    const RESET_VALUE: u32 = 0x01bc_2e04;
}
