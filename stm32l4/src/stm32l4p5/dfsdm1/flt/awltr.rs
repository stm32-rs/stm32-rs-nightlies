///Register `AWLTR` reader
pub type R = crate::R<AWLTRrs>;
///Register `AWLTR` writer
pub type W = crate::W<AWLTRrs>;
///Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_R = crate::FieldReader;
///Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `AWLT` reader - Analog watchdog low threshold
pub type AWLT_R = crate::FieldReader<u32>;
///Field `AWLT` writer - Analog watchdog low threshold
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWLTR")
            .field("awlt", &self.awlt())
            .field("bkawl", &self.bkawl())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W<AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W<AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
/**analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`awltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWLTRrs;
impl crate::RegisterSpec for AWLTRrs {
    type Ux = u32;
}
///`read()` method returns [`awltr::R`](R) reader structure
impl crate::Readable for AWLTRrs {}
///`write(|w| ..)` method takes [`awltr::W`](W) writer structure
impl crate::Writable for AWLTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWLTR to value 0
impl crate::Resettable for AWLTRrs {}
