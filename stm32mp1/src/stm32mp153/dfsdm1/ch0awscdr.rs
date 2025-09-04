///Register `CH0AWSCDR` reader
pub type R = crate::R<CH0AWSCDRrs>;
///Register `CH0AWSCDR` writer
pub type W = crate::W<CH0AWSCDRrs>;
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
        f.debug_struct("CH0AWSCDR")
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
    pub fn scdt(&mut self) -> SCDT_W<CH0AWSCDRrs> {
        SCDT_W::new(self, 0)
    }
    ///Bits 12:15 - BKSCD
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W<CH0AWSCDRrs> {
        BKSCD_W::new(self, 12)
    }
    ///Bits 16:20 - AWFOSR
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W<CH0AWSCDRrs> {
        AWFOSR_W::new(self, 16)
    }
    ///Bits 22:23 - AWFORD
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W<CH0AWSCDRrs> {
        AWFORD_W::new(self, 22)
    }
}
/**Short-circuit detector and analog watchdog settings for channel y.

You can [`read`](crate::Reg::read) this register and get [`ch0awscdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0awscdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:CH0AWSCDR)*/
pub struct CH0AWSCDRrs;
impl crate::RegisterSpec for CH0AWSCDRrs {
    type Ux = u32;
}
///`read()` method returns [`ch0awscdr::R`](R) reader structure
impl crate::Readable for CH0AWSCDRrs {}
///`write(|w| ..)` method takes [`ch0awscdr::W`](W) writer structure
impl crate::Writable for CH0AWSCDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH0AWSCDR to value 0
impl crate::Resettable for CH0AWSCDRrs {}
