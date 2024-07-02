///Register `DCMI_CWSTRT` reader
pub type R = crate::R<DCMI_CWSTRTrs>;
///Register `DCMI_CWSTRT` writer
pub type W = crate::W<DCMI_CWSTRTrs>;
///Field `HOFFCNT` reader - HOFFCNT
pub type HOFFCNT_R = crate::FieldReader<u16>;
///Field `HOFFCNT` writer - HOFFCNT
pub type HOFFCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `VST` reader - VST
pub type VST_R = crate::FieldReader<u16>;
///Field `VST` writer - VST
pub type VST_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:13 - HOFFCNT
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:28 - VST
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCMI_CWSTRT")
            .field("hoffcnt", &self.hoffcnt())
            .field("vst", &self.vst())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - HOFFCNT
    #[inline(always)]
    #[must_use]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W<DCMI_CWSTRTrs> {
        HOFFCNT_W::new(self, 0)
    }
    ///Bits 16:28 - VST
    #[inline(always)]
    #[must_use]
    pub fn vst(&mut self) -> VST_W<DCMI_CWSTRTrs> {
        VST_W::new(self, 16)
    }
}
/**DCMI crop window start

You can [`read`](crate::Reg::read) this register and get [`dcmi_cwstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_cwstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DCMI:DCMI_CWSTRT)*/
pub struct DCMI_CWSTRTrs;
impl crate::RegisterSpec for DCMI_CWSTRTrs {
    type Ux = u32;
}
///`read()` method returns [`dcmi_cwstrt::R`](R) reader structure
impl crate::Readable for DCMI_CWSTRTrs {}
///`write(|w| ..)` method takes [`dcmi_cwstrt::W`](W) writer structure
impl crate::Writable for DCMI_CWSTRTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCMI_CWSTRT to value 0
impl crate::Resettable for DCMI_CWSTRTrs {
    const RESET_VALUE: u32 = 0;
}
