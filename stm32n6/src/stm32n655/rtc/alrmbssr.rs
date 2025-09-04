///Register `ALRMBSSR` reader
pub type R = crate::R<ALRMBSSRrs>;
///Register `ALRMBSSR` writer
pub type W = crate::W<ALRMBSSRrs>;
///Field `SS` reader - Subseconds value
pub type SS_R = crate::FieldReader<u16>;
///Field `SS` writer - Subseconds value
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit
pub type MASKSS_R = crate::FieldReader;
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only)
pub type SSCLR_R = crate::BitReader;
///Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only)
pub type SSCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:14 - Subseconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 31 - Clear synchronous counter on alarm (Binary mode only)
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMBSSR")
            .field("ss", &self.ss())
            .field("maskss", &self.maskss())
            .field("ssclr", &self.ssclr())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Subseconds value
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<ALRMBSSRrs> {
        SS_W::new(self, 0)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W<ALRMBSSRrs> {
        MASKSS_W::new(self, 24)
    }
    ///Bit 31 - Clear synchronous counter on alarm (Binary mode only)
    #[inline(always)]
    pub fn ssclr(&mut self) -> SSCLR_W<ALRMBSSRrs> {
        SSCLR_W::new(self, 31)
    }
}
/**RTC alarm B subsecond register

You can [`read`](crate::Reg::read) this register and get [`alrmbssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RTC:ALRMBSSR)*/
pub struct ALRMBSSRrs;
impl crate::RegisterSpec for ALRMBSSRrs {
    type Ux = u32;
}
///`read()` method returns [`alrmbssr::R`](R) reader structure
impl crate::Readable for ALRMBSSRrs {}
///`write(|w| ..)` method takes [`alrmbssr::W`](W) writer structure
impl crate::Writable for ALRMBSSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRMBSSR to value 0
impl crate::Resettable for ALRMBSSRrs {}
