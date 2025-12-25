///Register `DLLGCR` reader
pub type R = crate::R<DLLGCRrs>;
///Register `DLLGCR` writer
pub type W = crate::W<DLLGCRrs>;
///Field `DRES` reader - DRES
pub type DRES_R = crate::FieldReader;
///Field `DRES` writer - DRES
pub type DRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IPUMP` reader - IPUMP
pub type IPUMP_R = crate::FieldReader;
///Field `IPUMP` writer - IPUMP
pub type IPUMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TESTEN` reader - TESTEN
pub type TESTEN_R = crate::BitReader;
///Field `TESTEN` writer - TESTEN
pub type TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTC` reader - DTC
pub type DTC_R = crate::FieldReader;
///Field `DTC` writer - DTC
pub type DTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATC` reader - ATC
pub type ATC_R = crate::FieldReader;
///Field `ATC` writer - ATC
pub type ATC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TESTSW` reader - TESTSW
pub type TESTSW_R = crate::BitReader;
///Field `TESTSW` writer - TESTSW
pub type TESTSW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MBIAS` reader - MBIAS
pub type MBIAS_R = crate::FieldReader;
///Field `MBIAS` writer - MBIAS
pub type MBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SBIAS2_0` reader - SBIAS2_0
pub type SBIAS2_0_R = crate::FieldReader;
///Field `SBIAS2_0` writer - SBIAS2_0
pub type SBIAS2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BPS200` reader - BPS200
pub type BPS200_R = crate::BitReader;
///Field `BPS200` writer - BPS200
pub type BPS200_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBIAS5_3` reader - SBIAS5_3
pub type SBIAS5_3_R = crate::FieldReader;
///Field `SBIAS5_3` writer - SBIAS5_3
pub type SBIAS5_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FDTRMSL` reader - FDTRMSL
pub type FDTRMSL_R = crate::FieldReader;
///Field `FDTRMSL` writer - FDTRMSL
pub type FDTRMSL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LOCKDET` reader - LOCKDET
pub type LOCKDET_R = crate::BitReader;
///Field `LOCKDET` writer - LOCKDET
pub type LOCKDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLRSVD2` reader - DLLRSVD2
pub type DLLRSVD2_R = crate::FieldReader;
///Field `DLLRSVD2` writer - DLLRSVD2
pub type DLLRSVD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - DRES
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - IPUMP
    #[inline(always)]
    pub fn ipump(&self) -> IPUMP_R {
        IPUMP_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bit 5 - TESTEN
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - DTC
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:10 - ATC
    #[inline(always)]
    pub fn atc(&self) -> ATC_R {
        ATC_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - TESTSW
    #[inline(always)]
    pub fn testsw(&self) -> TESTSW_R {
        TESTSW_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:19 - MBIAS
    #[inline(always)]
    pub fn mbias(&self) -> MBIAS_R {
        MBIAS_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:22 - SBIAS2_0
    #[inline(always)]
    pub fn sbias2_0(&self) -> SBIAS2_0_R {
        SBIAS2_0_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - BPS200
    #[inline(always)]
    pub fn bps200(&self) -> BPS200_R {
        BPS200_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - SBIAS5_3
    #[inline(always)]
    pub fn sbias5_3(&self) -> SBIAS5_3_R {
        SBIAS5_3_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:28 - FDTRMSL
    #[inline(always)]
    pub fn fdtrmsl(&self) -> FDTRMSL_R {
        FDTRMSL_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - LOCKDET
    #[inline(always)]
    pub fn lockdet(&self) -> LOCKDET_R {
        LOCKDET_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - DLLRSVD2
    #[inline(always)]
    pub fn dllrsvd2(&self) -> DLLRSVD2_R {
        DLLRSVD2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLLGCR")
            .field("dres", &self.dres())
            .field("ipump", &self.ipump())
            .field("testen", &self.testen())
            .field("dtc", &self.dtc())
            .field("atc", &self.atc())
            .field("testsw", &self.testsw())
            .field("mbias", &self.mbias())
            .field("sbias2_0", &self.sbias2_0())
            .field("bps200", &self.bps200())
            .field("sbias5_3", &self.sbias5_3())
            .field("fdtrmsl", &self.fdtrmsl())
            .field("lockdet", &self.lockdet())
            .field("dllrsvd2", &self.dllrsvd2())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DRES
    #[inline(always)]
    pub fn dres(&mut self) -> DRES_W<'_, DLLGCRrs> {
        DRES_W::new(self, 0)
    }
    ///Bits 2:4 - IPUMP
    #[inline(always)]
    pub fn ipump(&mut self) -> IPUMP_W<'_, DLLGCRrs> {
        IPUMP_W::new(self, 2)
    }
    ///Bit 5 - TESTEN
    #[inline(always)]
    pub fn testen(&mut self) -> TESTEN_W<'_, DLLGCRrs> {
        TESTEN_W::new(self, 5)
    }
    ///Bits 6:8 - DTC
    #[inline(always)]
    pub fn dtc(&mut self) -> DTC_W<'_, DLLGCRrs> {
        DTC_W::new(self, 6)
    }
    ///Bits 9:10 - ATC
    #[inline(always)]
    pub fn atc(&mut self) -> ATC_W<'_, DLLGCRrs> {
        ATC_W::new(self, 9)
    }
    ///Bit 11 - TESTSW
    #[inline(always)]
    pub fn testsw(&mut self) -> TESTSW_W<'_, DLLGCRrs> {
        TESTSW_W::new(self, 11)
    }
    ///Bits 12:19 - MBIAS
    #[inline(always)]
    pub fn mbias(&mut self) -> MBIAS_W<'_, DLLGCRrs> {
        MBIAS_W::new(self, 12)
    }
    ///Bits 20:22 - SBIAS2_0
    #[inline(always)]
    pub fn sbias2_0(&mut self) -> SBIAS2_0_W<'_, DLLGCRrs> {
        SBIAS2_0_W::new(self, 20)
    }
    ///Bit 23 - BPS200
    #[inline(always)]
    pub fn bps200(&mut self) -> BPS200_W<'_, DLLGCRrs> {
        BPS200_W::new(self, 23)
    }
    ///Bits 24:26 - SBIAS5_3
    #[inline(always)]
    pub fn sbias5_3(&mut self) -> SBIAS5_3_W<'_, DLLGCRrs> {
        SBIAS5_3_W::new(self, 24)
    }
    ///Bits 27:28 - FDTRMSL
    #[inline(always)]
    pub fn fdtrmsl(&mut self) -> FDTRMSL_W<'_, DLLGCRrs> {
        FDTRMSL_W::new(self, 27)
    }
    ///Bit 29 - LOCKDET
    #[inline(always)]
    pub fn lockdet(&mut self) -> LOCKDET_W<'_, DLLGCRrs> {
        LOCKDET_W::new(self, 29)
    }
    ///Bits 30:31 - DLLRSVD2
    #[inline(always)]
    pub fn dllrsvd2(&mut self) -> DLLRSVD2_W<'_, DLLGCRrs> {
        DLLRSVD2_W::new(self, 30)
    }
}
/**DDRPHYC DDR global control register

You can [`read`](crate::Reg::read) this register and get [`dllgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DLLGCR)*/
pub struct DLLGCRrs;
impl crate::RegisterSpec for DLLGCRrs {
    type Ux = u32;
}
///`read()` method returns [`dllgcr::R`](R) reader structure
impl crate::Readable for DLLGCRrs {}
///`write(|w| ..)` method takes [`dllgcr::W`](W) writer structure
impl crate::Writable for DLLGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLLGCR to value 0x0373_7000
impl crate::Resettable for DLLGCRrs {
    const RESET_VALUE: u32 = 0x0373_7000;
}
