///Register `FLTCR` reader
pub type R = crate::R<FLTCRrs>;
///Register `FLTCR` writer
pub type W = crate::W<FLTCRrs>;
///Field `TAMPFREQ` reader - Tamper sampling frequency
pub type TAMPFREQ_R = crate::FieldReader;
///Field `TAMPFREQ` writer - Tamper sampling frequency
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TAMPFLT` reader - TAMP_INx filter count
pub type TAMPFLT_R = crate::FieldReader;
///Field `TAMPFLT` writer - TAMP_INx filter count
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPRCH` reader - TAMP_INx precharge duration
pub type TAMPPRCH_R = crate::FieldReader;
///Field `TAMPPRCH` writer - TAMP_INx precharge duration
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPUDIS` reader - TAMP_INx pull-up disable
pub type TAMPPUDIS_R = crate::BitReader;
///Field `TAMPPUDIS` writer - TAMP_INx pull-up disable
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - TAMP_INx filter count
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - TAMP_INx precharge duration
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - TAMP_INx pull-up disable
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTCR")
            .field("tampfreq", &self.tampfreq())
            .field("tampflt", &self.tampflt())
            .field("tampprch", &self.tampprch())
            .field("tamppudis", &self.tamppudis())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<FLTCRrs> {
        TAMPFREQ_W::new(self, 0)
    }
    ///Bits 3:4 - TAMP_INx filter count
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<FLTCRrs> {
        TAMPFLT_W::new(self, 3)
    }
    ///Bits 5:6 - TAMP_INx precharge duration
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<FLTCRrs> {
        TAMPPRCH_W::new(self, 5)
    }
    ///Bit 7 - TAMP_INx pull-up disable
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<FLTCRrs> {
        TAMPPUDIS_W::new(self, 7)
    }
}
/**TAMP filter control register

You can [`read`](crate::Reg::read) this register and get [`fltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#TAMP:FLTCR)*/
pub struct FLTCRrs;
impl crate::RegisterSpec for FLTCRrs {
    type Ux = u32;
}
///`read()` method returns [`fltcr::R`](R) reader structure
impl crate::Readable for FLTCRrs {}
///`write(|w| ..)` method takes [`fltcr::W`](W) writer structure
impl crate::Writable for FLTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTCR to value 0
impl crate::Resettable for FLTCRrs {}
