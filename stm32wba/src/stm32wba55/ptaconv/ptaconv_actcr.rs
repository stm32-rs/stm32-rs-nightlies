///Register `PTACONV_ACTCR` reader
pub type R = crate::R<PTACONV_ACTCRrs>;
///Register `PTACONV_ACTCR` writer
pub type W = crate::W<PTACONV_ACTCRrs>;
///Field `TACTIVE` reader - PTA_ACTIVE setup time in us
pub type TACTIVE_R = crate::FieldReader;
///Field `TACTIVE` writer - PTA_ACTIVE setup time in us
pub type TACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ACTPOL` reader - PTA_ACTIVE polarity
pub type ACTPOL_R = crate::BitReader;
///Field `ACTPOL` writer - PTA_ACTIVE polarity
pub type ACTPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TABORT` reader - PTA_ACTIVE delay to cease an ongoing transmission in us
pub type TABORT_R = crate::FieldReader;
///Field `TABORT` writer - PTA_ACTIVE delay to cease an ongoing transmission in us
pub type TABORT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ABORTDIS` reader - Disable PTA_ACTIVE deny to abort an ongoing transmission
pub type ABORTDIS_R = crate::BitReader;
///Field `ABORTDIS` writer - Disable PTA_ACTIVE deny to abort an ongoing transmission
pub type ABORTDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - PTA_ACTIVE setup time in us
    #[inline(always)]
    pub fn tactive(&self) -> TACTIVE_R {
        TACTIVE_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 15 - PTA_ACTIVE polarity
    #[inline(always)]
    pub fn actpol(&self) -> ACTPOL_R {
        ACTPOL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - PTA_ACTIVE delay to cease an ongoing transmission in us
    #[inline(always)]
    pub fn tabort(&self) -> TABORT_R {
        TABORT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - Disable PTA_ACTIVE deny to abort an ongoing transmission
    #[inline(always)]
    pub fn abortdis(&self) -> ABORTDIS_R {
        ABORTDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTACONV_ACTCR")
            .field("tactive", &self.tactive())
            .field("actpol", &self.actpol())
            .field("tabort", &self.tabort())
            .field("abortdis", &self.abortdis())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - PTA_ACTIVE setup time in us
    #[inline(always)]
    pub fn tactive(&mut self) -> TACTIVE_W<'_, PTACONV_ACTCRrs> {
        TACTIVE_W::new(self, 0)
    }
    ///Bit 15 - PTA_ACTIVE polarity
    #[inline(always)]
    pub fn actpol(&mut self) -> ACTPOL_W<'_, PTACONV_ACTCRrs> {
        ACTPOL_W::new(self, 15)
    }
    ///Bits 16:19 - PTA_ACTIVE delay to cease an ongoing transmission in us
    #[inline(always)]
    pub fn tabort(&mut self) -> TABORT_W<'_, PTACONV_ACTCRrs> {
        TABORT_W::new(self, 16)
    }
    ///Bit 20 - Disable PTA_ACTIVE deny to abort an ongoing transmission
    #[inline(always)]
    pub fn abortdis(&mut self) -> ABORTDIS_W<'_, PTACONV_ACTCRrs> {
        ABORTDIS_W::new(self, 20)
    }
}
/**PTACONV active control register

You can [`read`](crate::Reg::read) this register and get [`ptaconv_actcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptaconv_actcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PTACONV:PTACONV_ACTCR)*/
pub struct PTACONV_ACTCRrs;
impl crate::RegisterSpec for PTACONV_ACTCRrs {
    type Ux = u32;
}
///`read()` method returns [`ptaconv_actcr::R`](R) reader structure
impl crate::Readable for PTACONV_ACTCRrs {}
///`write(|w| ..)` method takes [`ptaconv_actcr::W`](W) writer structure
impl crate::Writable for PTACONV_ACTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTACONV_ACTCR to value 0x0005_0014
impl crate::Resettable for PTACONV_ACTCRrs {
    const RESET_VALUE: u32 = 0x0005_0014;
}
