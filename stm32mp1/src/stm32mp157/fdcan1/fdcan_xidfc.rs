///Register `FDCAN_XIDFC` reader
pub type R = crate::R<FDCAN_XIDFCrs>;
///Register `FDCAN_XIDFC` writer
pub type W = crate::W<FDCAN_XIDFCrs>;
///Field `FLESA` reader - FLESA
pub type FLESA_R = crate::FieldReader<u16>;
///Field `FLESA` writer - FLESA
pub type FLESA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `LSE` reader - LSE
pub type LSE_R = crate::FieldReader;
///Field `LSE` writer - LSE
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 2:15 - FLESA
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:23 - LSE
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_XIDFC")
            .field("flesa", &self.flesa())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - FLESA
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FLESA_W<FDCAN_XIDFCrs> {
        FLESA_W::new(self, 2)
    }
    ///Bits 16:23 - LSE
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<FDCAN_XIDFCrs> {
        LSE_W::new(self, 16)
    }
}
/**Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_XIDFC)*/
pub struct FDCAN_XIDFCrs;
impl crate::RegisterSpec for FDCAN_XIDFCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_xidfc::R`](R) reader structure
impl crate::Readable for FDCAN_XIDFCrs {}
///`write(|w| ..)` method takes [`fdcan_xidfc::W`](W) writer structure
impl crate::Writable for FDCAN_XIDFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_XIDFC to value 0
impl crate::Resettable for FDCAN_XIDFCrs {
    const RESET_VALUE: u32 = 0;
}
