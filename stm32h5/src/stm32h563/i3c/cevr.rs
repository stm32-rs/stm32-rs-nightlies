///Register `CEVR` writer
pub type W = crate::W<CEVRrs>;
///Field `CFCF` writer - clear frame complete flag (whatever the I3C is acting as controller/target)
pub type CFCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRXTGTENDF` writer - clear target-initiated read end flag (when the I3C is acting as controller)
pub type CRXTGTENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERRF` writer - clear error flag (whatever the I3C is acting as controller/target)
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIBIF` writer - clear IBI request flag (when the I3C is acting as controller)
pub type CIBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIBIENDF` writer - clear IBI end flag (when the I3C is acting as target)
pub type CIBIENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRF` writer - clear controller-role request flag (when the I3C is acting as controller)
pub type CCRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRUPDF` writer - clear controller-role update flag (when the I3C is acting as target)
pub type CCRUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHJF` writer - clear hot-join flag (when the I3C is acting as controller)
pub type CHJF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWKPF` writer - clear wakeup flag (when the I3C is acting as target)
pub type CWKPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGETF` writer - clear GETxxx CCC flag (when the I3C is acting as target)
pub type CGETF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTAF` writer - clear GETSTATUS CCC flag (when the I3C is acting as target)
pub type CSTAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDAUPDF` writer - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)
pub type CDAUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMWLUPDF` writer - clear SETMWL CCC flag (when the I3C is acting as target)
pub type CMWLUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMRLUPDF` writer - clear SETMRL CCC flag (when the I3C is acting as target)
pub type CMRLUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSTF` writer - clear reset pattern flag (when the I3C is acting as target)
pub type CRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CASUPDF` writer - clear ENTASx CCC flag (when the I3C is acting as target)
pub type CASUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CINTUPDF` writer - clear ENEC/DISEC CCC flag (when the I3C is acting as target)
pub type CINTUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDEFF` writer - clear DEFTGTS CCC flag (when the I3C is acting as target)
pub type CDEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGRPF` writer - clear DEFGRPA CCC flag (when the I3C is acting as target)
pub type CGRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CEVRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 9 - clear frame complete flag (whatever the I3C is acting as controller/target)
    #[inline(always)]
    #[must_use]
    pub fn cfcf(&mut self) -> CFCF_W<CEVRrs> {
        CFCF_W::new(self, 9)
    }
    ///Bit 10 - clear target-initiated read end flag (when the I3C is acting as controller)
    #[inline(always)]
    #[must_use]
    pub fn crxtgtendf(&mut self) -> CRXTGTENDF_W<CEVRrs> {
        CRXTGTENDF_W::new(self, 10)
    }
    ///Bit 11 - clear error flag (whatever the I3C is acting as controller/target)
    #[inline(always)]
    #[must_use]
    pub fn cerrf(&mut self) -> CERRF_W<CEVRrs> {
        CERRF_W::new(self, 11)
    }
    ///Bit 15 - clear IBI request flag (when the I3C is acting as controller)
    #[inline(always)]
    #[must_use]
    pub fn cibif(&mut self) -> CIBIF_W<CEVRrs> {
        CIBIF_W::new(self, 15)
    }
    ///Bit 16 - clear IBI end flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cibiendf(&mut self) -> CIBIENDF_W<CEVRrs> {
        CIBIENDF_W::new(self, 16)
    }
    ///Bit 17 - clear controller-role request flag (when the I3C is acting as controller)
    #[inline(always)]
    #[must_use]
    pub fn ccrf(&mut self) -> CCRF_W<CEVRrs> {
        CCRF_W::new(self, 17)
    }
    ///Bit 18 - clear controller-role update flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn ccrupdf(&mut self) -> CCRUPDF_W<CEVRrs> {
        CCRUPDF_W::new(self, 18)
    }
    ///Bit 19 - clear hot-join flag (when the I3C is acting as controller)
    #[inline(always)]
    #[must_use]
    pub fn chjf(&mut self) -> CHJF_W<CEVRrs> {
        CHJF_W::new(self, 19)
    }
    ///Bit 21 - clear wakeup flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cwkpf(&mut self) -> CWKPF_W<CEVRrs> {
        CWKPF_W::new(self, 21)
    }
    ///Bit 22 - clear GETxxx CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cgetf(&mut self) -> CGETF_W<CEVRrs> {
        CGETF_W::new(self, 22)
    }
    ///Bit 23 - clear GETSTATUS CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cstaf(&mut self) -> CSTAF_W<CEVRrs> {
        CSTAF_W::new(self, 23)
    }
    ///Bit 24 - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cdaupdf(&mut self) -> CDAUPDF_W<CEVRrs> {
        CDAUPDF_W::new(self, 24)
    }
    ///Bit 25 - clear SETMWL CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cmwlupdf(&mut self) -> CMWLUPDF_W<CEVRrs> {
        CMWLUPDF_W::new(self, 25)
    }
    ///Bit 26 - clear SETMRL CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cmrlupdf(&mut self) -> CMRLUPDF_W<CEVRrs> {
        CMRLUPDF_W::new(self, 26)
    }
    ///Bit 27 - clear reset pattern flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn crstf(&mut self) -> CRSTF_W<CEVRrs> {
        CRSTF_W::new(self, 27)
    }
    ///Bit 28 - clear ENTASx CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn casupdf(&mut self) -> CASUPDF_W<CEVRrs> {
        CASUPDF_W::new(self, 28)
    }
    ///Bit 29 - clear ENEC/DISEC CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cintupdf(&mut self) -> CINTUPDF_W<CEVRrs> {
        CINTUPDF_W::new(self, 29)
    }
    ///Bit 30 - clear DEFTGTS CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cdeff(&mut self) -> CDEFF_W<CEVRrs> {
        CDEFF_W::new(self, 30)
    }
    ///Bit 31 - clear DEFGRPA CCC flag (when the I3C is acting as target)
    #[inline(always)]
    #[must_use]
    pub fn cgrpf(&mut self) -> CGRPF_W<CEVRrs> {
        CGRPF_W::new(self, 31)
    }
}
/**I3C clear event register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cevr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#I3C:CEVR)*/
pub struct CEVRrs;
impl crate::RegisterSpec for CEVRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cevr::W`](W) writer structure
impl crate::Writable for CEVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CEVR to value 0
impl crate::Resettable for CEVRrs {
    const RESET_VALUE: u32 = 0;
}
