///Register `ALRMASSR` reader
pub type R = crate::R<ALRMASSRrs>;
///Register `ALRMASSR` writer
pub type W = crate::W<ALRMASSRrs>;
///Field `SS` reader - Sub seconds value
pub type SS_R = crate::FieldReader<u16>;
///Field `SS` writer - Sub seconds value
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit
pub type MASKSS_R = crate::FieldReader;
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMASSR")
            .field("maskss", &self.maskss())
            .field("ss", &self.ss())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<'_, ALRMASSRrs> {
        SS_W::new(self, 0)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W<'_, ALRMASSRrs> {
        MASKSS_W::new(self, 24)
    }
}
/**alarm A sub second register

You can [`read`](crate::Reg::read) this register and get [`alrmassr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#RTC:ALRMASSR)*/
pub struct ALRMASSRrs;
impl crate::RegisterSpec for ALRMASSRrs {
    type Ux = u32;
}
///`read()` method returns [`alrmassr::R`](R) reader structure
impl crate::Readable for ALRMASSRrs {}
///`write(|w| ..)` method takes [`alrmassr::W`](W) writer structure
impl crate::Writable for ALRMASSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRMASSR to value 0
impl crate::Resettable for ALRMASSRrs {}
