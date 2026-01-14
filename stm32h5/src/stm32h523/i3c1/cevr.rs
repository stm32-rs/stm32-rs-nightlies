///Register `CEVR` writer
pub type W = crate::W<CEVRrs>;
///Field `CFCF` writer - Clear frame complete flag (whatever the I3C acts as controller/target)
pub type CFCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRXTGTENDF` writer - Clear target-initiated read end flag (when the I3C acts as controller)
pub type CRXTGTENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERRF` writer - Clear error flag (whatever the I3C acts as controller/target)
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIBIF` writer - Clear IBI request flag (when the I3C acts as controller)
pub type CIBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIBIENDF` writer - Clear IBI end flag (when the I3C acts as target)
pub type CIBIENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRF` writer - Clear controller-role request flag (when the I3C acts as controller)
pub type CCRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRUPDF` writer - Clear controller-role update flag (when the I3C acts as target)
pub type CCRUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHJF` writer - Clear hot-join flag (when the I3C acts as controller)
pub type CHJF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWKPF` writer - Clear wake-up flag (when the I3C acts as target)
pub type CWKPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGETF` writer - Clear GETxxx CCC flag (except GETSTATUS of format 1) (when the I3C acts as target)
pub type CGETF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTAF` writer - Clear format 1 GETSTATUS CCC flag (when the I3C acts as target)
pub type CSTAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDAUPDF` writer - Clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C acts as target)
pub type CDAUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMWLUPDF` writer - Clear SETMWL CCC flag (when the I3C acts as target)
pub type CMWLUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMRLUPDF` writer - Clear SETMRL CCC flag (when the I3C acts as target)
pub type CMRLUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSTF` writer - Clear reset pattern flag (when the I3C acts as target)
pub type CRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CASUPDF` writer - Clear ENTASx CCC flag (when the I3C acts as target)
pub type CASUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CINTUPDF` writer - Clear ENEC/DISEC CCC flag (when the I3C acts as target)
pub type CINTUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDEFF` writer - Clear DEFTGTS CCC flag (when the I3C acts as target)
pub type CDEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGRPF` writer - Clear DEFGRPA CCC flag (when the I3C acts as target)
pub type CGRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CEVRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 9 - Clear frame complete flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn cfcf(&mut self) -> CFCF_W<'_, CEVRrs> {
        CFCF_W::new(self, 9)
    }
    ///Bit 10 - Clear target-initiated read end flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn crxtgtendf(&mut self) -> CRXTGTENDF_W<'_, CEVRrs> {
        CRXTGTENDF_W::new(self, 10)
    }
    ///Bit 11 - Clear error flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn cerrf(&mut self) -> CERRF_W<'_, CEVRrs> {
        CERRF_W::new(self, 11)
    }
    ///Bit 15 - Clear IBI request flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn cibif(&mut self) -> CIBIF_W<'_, CEVRrs> {
        CIBIF_W::new(self, 15)
    }
    ///Bit 16 - Clear IBI end flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cibiendf(&mut self) -> CIBIENDF_W<'_, CEVRrs> {
        CIBIENDF_W::new(self, 16)
    }
    ///Bit 17 - Clear controller-role request flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn ccrf(&mut self) -> CCRF_W<'_, CEVRrs> {
        CCRF_W::new(self, 17)
    }
    ///Bit 18 - Clear controller-role update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn ccrupdf(&mut self) -> CCRUPDF_W<'_, CEVRrs> {
        CCRUPDF_W::new(self, 18)
    }
    ///Bit 19 - Clear hot-join flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn chjf(&mut self) -> CHJF_W<'_, CEVRrs> {
        CHJF_W::new(self, 19)
    }
    ///Bit 21 - Clear wake-up flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cwkpf(&mut self) -> CWKPF_W<'_, CEVRrs> {
        CWKPF_W::new(self, 21)
    }
    ///Bit 22 - Clear GETxxx CCC flag (except GETSTATUS of format 1) (when the I3C acts as target)
    #[inline(always)]
    pub fn cgetf(&mut self) -> CGETF_W<'_, CEVRrs> {
        CGETF_W::new(self, 22)
    }
    ///Bit 23 - Clear format 1 GETSTATUS CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cstaf(&mut self) -> CSTAF_W<'_, CEVRrs> {
        CSTAF_W::new(self, 23)
    }
    ///Bit 24 - Clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cdaupdf(&mut self) -> CDAUPDF_W<'_, CEVRrs> {
        CDAUPDF_W::new(self, 24)
    }
    ///Bit 25 - Clear SETMWL CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cmwlupdf(&mut self) -> CMWLUPDF_W<'_, CEVRrs> {
        CMWLUPDF_W::new(self, 25)
    }
    ///Bit 26 - Clear SETMRL CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cmrlupdf(&mut self) -> CMRLUPDF_W<'_, CEVRrs> {
        CMRLUPDF_W::new(self, 26)
    }
    ///Bit 27 - Clear reset pattern flag (when the I3C acts as target)
    #[inline(always)]
    pub fn crstf(&mut self) -> CRSTF_W<'_, CEVRrs> {
        CRSTF_W::new(self, 27)
    }
    ///Bit 28 - Clear ENTASx CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn casupdf(&mut self) -> CASUPDF_W<'_, CEVRrs> {
        CASUPDF_W::new(self, 28)
    }
    ///Bit 29 - Clear ENEC/DISEC CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cintupdf(&mut self) -> CINTUPDF_W<'_, CEVRrs> {
        CINTUPDF_W::new(self, 29)
    }
    ///Bit 30 - Clear DEFTGTS CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cdeff(&mut self) -> CDEFF_W<'_, CEVRrs> {
        CDEFF_W::new(self, 30)
    }
    ///Bit 31 - Clear DEFGRPA CCC flag (when the I3C acts as target)
    #[inline(always)]
    pub fn cgrpf(&mut self) -> CGRPF_W<'_, CEVRrs> {
        CGRPF_W::new(self, 31)
    }
}
/**I3C clear event register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cevr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#I3C1:CEVR)*/
pub struct CEVRrs;
impl crate::RegisterSpec for CEVRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cevr::W`](W) writer structure
impl crate::Writable for CEVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEVR to value 0
impl crate::Resettable for CEVRrs {}
