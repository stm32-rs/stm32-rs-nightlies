///Register `SAI_GCR` reader
pub type R = crate::R<SAI_GCRrs>;
///Register `SAI_GCR` writer
pub type W = crate::W<SAI_GCRrs>;
///Field `SYNCIN` reader - SYNCIN
pub type SYNCIN_R = crate::FieldReader;
///Field `SYNCIN` writer - SYNCIN
pub type SYNCIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYNCOUT` reader - SYNCOUT
pub type SYNCOUT_R = crate::FieldReader;
///Field `SYNCOUT` writer - SYNCOUT
pub type SYNCOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - SYNCIN
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - SYNCOUT
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI_GCR")
            .field("syncin", &self.syncin())
            .field("syncout", &self.syncout())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SYNCIN
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<SAI_GCRrs> {
        SYNCIN_W::new(self, 0)
    }
    ///Bits 4:5 - SYNCOUT
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<SAI_GCRrs> {
        SYNCOUT_W::new(self, 4)
    }
}
/**Global configuration register

You can [`read`](crate::Reg::read) this register and get [`sai_gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SAI1:SAI_GCR)*/
pub struct SAI_GCRrs;
impl crate::RegisterSpec for SAI_GCRrs {
    type Ux = u32;
}
///`read()` method returns [`sai_gcr::R`](R) reader structure
impl crate::Readable for SAI_GCRrs {}
///`write(|w| ..)` method takes [`sai_gcr::W`](W) writer structure
impl crate::Writable for SAI_GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAI_GCR to value 0
impl crate::Resettable for SAI_GCRrs {
    const RESET_VALUE: u32 = 0;
}
