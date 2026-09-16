///Register `XIDFC` reader
pub type R = crate::R<XIDFCrs>;
///Register `XIDFC` writer
pub type W = crate::W<XIDFCrs>;
///Field `FLESA` reader - Filter list extended start address
pub type FLESA_R = crate::FieldReader<u16>;
///Field `FLESA` writer - Filter list extended start address
pub type FLESA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `LSE` reader - List size extended
pub type LSE_R = crate::FieldReader;
///Field `LSE` writer - List size extended
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 2:15 - Filter list extended start address
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:23 - List size extended
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIDFC")
            .field("flesa", &self.flesa())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Filter list extended start address
    #[inline(always)]
    pub fn flesa(&mut self) -> FLESA_W<'_, XIDFCrs> {
        FLESA_W::new(self, 2)
    }
    ///Bits 16:23 - List size extended
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W<'_, XIDFCrs> {
        LSE_W::new(self, 16)
    }
}
/**FDCAN extended ID filter configuration register

You can [`read`](crate::Reg::read) this register and get [`xidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:XIDFC)*/
pub struct XIDFCrs;
impl crate::RegisterSpec for XIDFCrs {
    type Ux = u32;
}
///`read()` method returns [`xidfc::R`](R) reader structure
impl crate::Readable for XIDFCrs {}
///`write(|w| ..)` method takes [`xidfc::W`](W) writer structure
impl crate::Writable for XIDFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets XIDFC to value 0
impl crate::Resettable for XIDFCrs {}
