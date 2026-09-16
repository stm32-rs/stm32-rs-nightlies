///Register `DCR2` reader
pub type R = crate::R<DCR2rs>;
///Register `DCR2` writer
pub type W = crate::W<DCR2rs>;
///Field `PRESCALER` reader - Clock prescaler
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - Clock prescaler
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRAPSIZE` reader - Wrap size
pub type WRAPSIZE_R = crate::FieldReader;
///Field `WRAPSIZE` writer - Wrap size
pub type WRAPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR2")
            .field("prescaler", &self.prescaler())
            .field("wrapsize", &self.wrapsize())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<'_, DCR2rs> {
        PRESCALER_W::new(self, 0)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<'_, DCR2rs> {
        WRAPSIZE_W::new(self, 16)
    }
}
/**XSPI device configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#XSPI1:DCR2)*/
pub struct DCR2rs;
impl crate::RegisterSpec for DCR2rs {
    type Ux = u32;
}
///`read()` method returns [`dcr2::R`](R) reader structure
impl crate::Readable for DCR2rs {}
///`write(|w| ..)` method takes [`dcr2::W`](W) writer structure
impl crate::Writable for DCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR2 to value 0
impl crate::Resettable for DCR2rs {}
