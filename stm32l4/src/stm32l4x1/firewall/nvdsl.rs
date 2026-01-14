///Register `NVDSL` reader
pub type R = crate::R<NVDSLrs>;
///Register `NVDSL` writer
pub type W = crate::W<NVDSLrs>;
///Field `LENG` reader - Non-volatile data segment length
pub type LENG_R = crate::FieldReader<u16>;
///Field `LENG` writer - Non-volatile data segment length
pub type LENG_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    ///Bits 8:21 - Non-volatile data segment length
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVDSL").field("leng", &self.leng()).finish()
    }
}
impl W {
    ///Bits 8:21 - Non-volatile data segment length
    #[inline(always)]
    pub fn leng(&mut self) -> LENG_W<'_, NVDSLrs> {
        LENG_W::new(self, 8)
    }
}
/**Non-volatile data segment length

You can [`read`](crate::Reg::read) this register and get [`nvdsl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvdsl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#FIREWALL:NVDSL)*/
pub struct NVDSLrs;
impl crate::RegisterSpec for NVDSLrs {
    type Ux = u32;
}
///`read()` method returns [`nvdsl::R`](R) reader structure
impl crate::Readable for NVDSLrs {}
///`write(|w| ..)` method takes [`nvdsl::W`](W) writer structure
impl crate::Writable for NVDSLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NVDSL to value 0
impl crate::Resettable for NVDSLrs {}
