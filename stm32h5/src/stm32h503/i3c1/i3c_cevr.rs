#[doc = "Register `I3C_CEVR` writer"]
pub type W = crate::W<I3C_CEVRrs>;
#[doc = "Field `CFCF` writer - clear frame complete flag (whatever the I3C is acting as controller/target)"]
pub type CFCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRXTGTENDF` writer - clear target-initiated read end flag (when the I3C is acting as controller)"]
pub type CRXTGTENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERRF` writer - clear error flag (whatever the I3C is acting as controller/target)"]
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIBIF` writer - clear IBI request flag (when the I3C is acting as controller)"]
pub type CIBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIBIENDF` writer - clear IBI end flag (when the I3C is acting as target)"]
pub type CIBIENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRF` writer - clear controller-role request flag (when the I3C is acting as controller)"]
pub type CCRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRUPDF` writer - clear controller-role update flag (when the I3C is acting as target)"]
pub type CCRUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHJF` writer - clear hot-join flag (when the I3C is acting as controller)"]
pub type CHJF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWKPF` writer - clear wakeup flag (when the I3C is acting as target)"]
pub type CWKPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGETF` writer - clear GETxxx CCC flag (when the I3C is acting as target)"]
pub type CGETF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTAF` writer - clear GETSTATUS CCC flag (when the I3C is acting as target)"]
pub type CSTAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDAUPDF` writer - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
pub type CDAUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMWLUPDF` writer - clear SETMWL CCC flag (when the I3C is acting as target)"]
pub type CMWLUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMRLUPDF` writer - clear SETMRL CCC flag (when the I3C is acting as target)"]
pub type CMRLUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSTF` writer - clear reset pattern flag (when the I3C is acting as target)"]
pub type CRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASUPDF` writer - clear ENTASx CCC flag (when the I3C is acting as target)"]
pub type CASUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CINTUPDF` writer - clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
pub type CINTUPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEFF` writer - clear DEFTGTS CCC flag (when the I3C is acting as target)"]
pub type CDEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGRPF` writer - clear DEFGRPA CCC flag (when the I3C is acting as target)"]
pub type CGRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - clear frame complete flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    #[must_use]
    pub fn cfcf(&mut self) -> CFCF_W<I3C_CEVRrs> {
        CFCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear target-initiated read end flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn crxtgtendf(&mut self) -> CRXTGTENDF_W<I3C_CEVRrs> {
        CRXTGTENDF_W::new(self, 10)
    }
    #[doc = "Bit 11 - clear error flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    #[must_use]
    pub fn cerrf(&mut self) -> CERRF_W<I3C_CEVRrs> {
        CERRF_W::new(self, 11)
    }
    #[doc = "Bit 15 - clear IBI request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn cibif(&mut self) -> CIBIF_W<I3C_CEVRrs> {
        CIBIF_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear IBI end flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cibiendf(&mut self) -> CIBIENDF_W<I3C_CEVRrs> {
        CIBIENDF_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear controller-role request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn ccrf(&mut self) -> CCRF_W<I3C_CEVRrs> {
        CCRF_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear controller-role update flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn ccrupdf(&mut self) -> CCRUPDF_W<I3C_CEVRrs> {
        CCRUPDF_W::new(self, 18)
    }
    #[doc = "Bit 19 - clear hot-join flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn chjf(&mut self) -> CHJF_W<I3C_CEVRrs> {
        CHJF_W::new(self, 19)
    }
    #[doc = "Bit 21 - clear wakeup flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cwkpf(&mut self) -> CWKPF_W<I3C_CEVRrs> {
        CWKPF_W::new(self, 21)
    }
    #[doc = "Bit 22 - clear GETxxx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cgetf(&mut self) -> CGETF_W<I3C_CEVRrs> {
        CGETF_W::new(self, 22)
    }
    #[doc = "Bit 23 - clear GETSTATUS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cstaf(&mut self) -> CSTAF_W<I3C_CEVRrs> {
        CSTAF_W::new(self, 23)
    }
    #[doc = "Bit 24 - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cdaupdf(&mut self) -> CDAUPDF_W<I3C_CEVRrs> {
        CDAUPDF_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear SETMWL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cmwlupdf(&mut self) -> CMWLUPDF_W<I3C_CEVRrs> {
        CMWLUPDF_W::new(self, 25)
    }
    #[doc = "Bit 26 - clear SETMRL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cmrlupdf(&mut self) -> CMRLUPDF_W<I3C_CEVRrs> {
        CMRLUPDF_W::new(self, 26)
    }
    #[doc = "Bit 27 - clear reset pattern flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn crstf(&mut self) -> CRSTF_W<I3C_CEVRrs> {
        CRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - clear ENTASx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn casupdf(&mut self) -> CASUPDF_W<I3C_CEVRrs> {
        CASUPDF_W::new(self, 28)
    }
    #[doc = "Bit 29 - clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cintupdf(&mut self) -> CINTUPDF_W<I3C_CEVRrs> {
        CINTUPDF_W::new(self, 29)
    }
    #[doc = "Bit 30 - clear DEFTGTS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cdeff(&mut self) -> CDEFF_W<I3C_CEVRrs> {
        CDEFF_W::new(self, 30)
    }
    #[doc = "Bit 31 - clear DEFGRPA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cgrpf(&mut self) -> CGRPF_W<I3C_CEVRrs> {
        CGRPF_W::new(self, 31)
    }
}
#[doc = "I3C clear event register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_cevr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_CEVRrs;
impl crate::RegisterSpec for I3C_CEVRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i3c_cevr::W`](W) writer structure"]
impl crate::Writable for I3C_CEVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_CEVR to value 0"]
impl crate::Resettable for I3C_CEVRrs {
    const RESET_VALUE: u32 = 0;
}
