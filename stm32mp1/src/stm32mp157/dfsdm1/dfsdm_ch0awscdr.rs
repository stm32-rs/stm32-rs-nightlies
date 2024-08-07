///Register `DFSDM_CH0AWSCDR` reader
pub type R = crate::R<DFSDM_CH0AWSCDRrs>;
///Register `DFSDM_CH0AWSCDR` writer
pub type W = crate::W<DFSDM_CH0AWSCDRrs>;
///Field `SCDT` reader - SCDT
pub type SCDT_R = crate::FieldReader;
///Field `SCDT` writer - SCDT
pub type SCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BKSCD` reader - BKSCD
pub type BKSCD_R = crate::FieldReader;
///Field `BKSCD` writer - BKSCD
pub type BKSCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWFOSR` reader - AWFOSR
pub type AWFOSR_R = crate::FieldReader;
///Field `AWFOSR` writer - AWFOSR
pub type AWFOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `AWFORD` reader - AWFORD
pub type AWFORD_R = crate::FieldReader;
///Field `AWFORD` writer - AWFORD
pub type AWFORD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - SCDT
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 12:15 - BKSCD
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:20 - AWFOSR
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 22:23 - AWFORD
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_CH0AWSCDR")
            .field("scdt", &self.scdt())
            .field("bkscd", &self.bkscd())
            .field("awfosr", &self.awfosr())
            .field("awford", &self.awford())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - SCDT
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<DFSDM_CH0AWSCDRrs> {
        SCDT_W::new(self, 0)
    }
    ///Bits 12:15 - BKSCD
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<DFSDM_CH0AWSCDRrs> {
        BKSCD_W::new(self, 12)
    }
    ///Bits 16:20 - AWFOSR
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<DFSDM_CH0AWSCDRrs> {
        AWFOSR_W::new(self, 16)
    }
    ///Bits 22:23 - AWFORD
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<DFSDM_CH0AWSCDRrs> {
        AWFORD_W::new(self, 22)
    }
}
/**Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0awscdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0awscdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:DFSDM_CH0AWSCDR)*/
pub struct DFSDM_CH0AWSCDRrs;
impl crate::RegisterSpec for DFSDM_CH0AWSCDRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_ch0awscdr::R`](R) reader structure
impl crate::Readable for DFSDM_CH0AWSCDRrs {}
///`write(|w| ..)` method takes [`dfsdm_ch0awscdr::W`](W) writer structure
impl crate::Writable for DFSDM_CH0AWSCDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_CH0AWSCDR to value 0
impl crate::Resettable for DFSDM_CH0AWSCDRrs {
    const RESET_VALUE: u32 = 0;
}
